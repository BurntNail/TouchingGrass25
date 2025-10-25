use crate::errors::IslandError;
use redis::aio::MultiplexedConnection;
use redis::{AsyncTypedCommands, Client};
use redis_macros::FromRedisValue;
use s3::creds::Credentials;
use s3::{Bucket, Region};
use serde::{Deserialize, Serialize};
use sha256::digest;
use std::cmp::Reverse;
use tokio::sync::broadcast::{Receiver, Sender, channel};
use tracing::error;
use uuid::Uuid;

#[derive(Clone)]
pub struct IslandState {
    redis_connection: MultiplexedConnection,
    update_leaderboard: Sender<Vec<LeaderboardEntry>>,
    bucket: Box<Bucket>,
    pub client: reqwest::Client,
}

#[derive(Serialize, Debug, Clone)]
pub struct LeaderboardEntry {
    pub person: String,
    pub score: u32,
}

#[derive(Serialize, Deserialize, FromRedisValue, Debug)]
struct InternalTopImage {
    person: String,
    image_score: u32,
    s3_path: String,
}

#[derive(Serialize)]
pub struct TopImageEntry {
    pub person: String,
    pub image_score: u32,
    pub image: String,
}

impl IslandState {
    pub async fn new() -> Result<Self, IslandError> {
        let (update_leaderboard, _) = channel(1);

        let redis_connection = {
            let path = std::env::var("REDIS_PATH")?;
            let client = Client::open(path)?;

            let (redis_connection, drive_future) =
                client.create_multiplexed_tokio_connection().await?;
            tokio::spawn(drive_future); //TODO: nice ending for ctrl-c

            redis_connection
        };

        let bucket = {
            let bucket_name = std::env::var("BUCKET_NAME")?;
            let access_key_id = std::env::var("AWS_ACCESS_KEY_ID")?;
            let endpoint = std::env::var("AWS_ENDPOINT_URL_S3")?;
            let region = std::env::var("AWS_REGION")?;
            let secret_access_key = std::env::var("AWS_SECRET_ACCESS_KEY")?;

            let creds = Credentials::new(
                Some(&access_key_id),
                Some(&secret_access_key),
                None,
                None,
                None,
            )?;
            let region = Region::Custom { region, endpoint };
            Bucket::new(&bucket_name, region, creds)?
        };

        Ok(Self {
            update_leaderboard,
            redis_connection,
            bucket,
            client: reqwest::Client::new(),
        })
    }

    pub async fn start_submission(&self) -> Result<Uuid, IslandError> {
        let uuid = Uuid::new_v4();

        self.redis_connection
            .clone()
            .sadd("valid_uuids", uuid.to_string())
            .await?;

        Ok(uuid)
    }

    pub async fn add_score(&self, uuid: Uuid, name: String, score: u32) -> Result<(), IslandError> {
        let mut conn = self.redis_connection.clone();

        let was_valid = conn.srem("valid_uuids", uuid.to_string()).await? > 0;

        if was_valid {
            conn.hincr("scores", name, score).await?;
            drop(conn);

            let _ = self.update_leaderboard.send(self.get_leaderboard().await?);
        } else {
            error!("tried to submit w/o getting start");
        }

        Ok(())
    }

    pub async fn get_leaderboard(&self) -> Result<Vec<LeaderboardEntry>, IslandError> {
        let mut conn = self.redis_connection.clone();

        let mut scores: Vec<LeaderboardEntry> = conn
            .hgetall("scores")
            .await?
            .into_iter()
            .map(|(person, score)| {
                Ok(LeaderboardEntry {
                    person,
                    score: score.parse()?,
                })
            })
            .collect::<Result<_, IslandError>>()?;

        scores.sort_by_key(|entry| Reverse(entry.score));
        Ok(scores)
    }

    pub fn subscribe_to_update_leaderboard(&self) -> Receiver<Vec<LeaderboardEntry>> {
        self.update_leaderboard.subscribe()
    }

    async fn get_top_info(
        conn: &mut MultiplexedConnection,
    ) -> Result<Vec<InternalTopImage>, IslandError> {
        let Some(json) = conn.get("top_img_info").await? else {
            return Ok(vec![]);
        };
        Ok(serde_json::from_str(&json)?)
    }

    pub async fn get_top_images(&self) -> Result<Vec<TopImageEntry>, IslandError> {
        let mut redis_conn = self.redis_connection.clone();

        let current_top = Self::get_top_info(&mut redis_conn).await?;

        let mut ret = vec![];
        for info in current_top {
            let image = self.bucket.presign_get(info.s3_path, 60, None).await?;

            ret.push(TopImageEntry {
                person: info.person,
                image_score: info.image_score,
                image,
            });
        }

        Ok(ret)
    }

    pub async fn set_potential_top_image(
        &self,
        person: String,
        image_score: u32,
        image: Vec<u8>,
    ) -> Result<(), IslandError> {
        let mut redis_conn = self.redis_connection.clone();

        let mut current_top = Self::get_top_info(&mut redis_conn).await?;

        let mut index_to_insert = None;
        if current_top.len() < 3 {
            index_to_insert = Some(current_top.len());
        } else {
            for (i, info) in current_top.iter().enumerate() {
                if info.image_score < image_score {
                    index_to_insert = Some(i);
                    break;
                }
            }
        }

        let Some(index_to_insert) = index_to_insert else {
            return Ok(());
        };

        let s3_path = digest(&image).to_string();
        self.bucket.put_object(&s3_path, &image).await?;

        current_top.insert(
            index_to_insert,
            InternalTopImage {
                person,
                image_score,
                s3_path,
            },
        );
        current_top.truncate(3);

        let jsoned_top = serde_json::to_string(&current_top)?;

        redis_conn.set("top_img_info", jsoned_top).await?;

        Ok(())
    }
}
