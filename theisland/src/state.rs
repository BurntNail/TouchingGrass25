use serde::Serialize;
use std::cmp::Reverse;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::broadcast::{Receiver, Sender, channel};
use tracing::error;
use uuid::Uuid;
use redis::aio::MultiplexedConnection;
use redis::{AsyncTypedCommands, Client};
use crate::errors::IslandError;

#[derive(Clone)]
pub struct IslandState {
    redis_connection: MultiplexedConnection,
    update_leaderboard: Sender<Vec<LeaderboardEntry>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct LeaderboardEntry {
    pub person: String,
    pub score: u32,
}

impl IslandState {
    pub async fn new () -> Result<Self, IslandError> {
        let (update_leaderboard, _) = channel(1);

        let path = std::env::var("REDIS_PATH")?;
        let client = Client::open(path)?;

        let (redis_connection, drive_future) = client.create_multiplexed_tokio_connection().await?;
        tokio::spawn(drive_future); //TODO: nice ending for ctrl-c

        Ok(Self {
            update_leaderboard, redis_connection
        })
    }

    pub async fn start_submission(&self) -> Result<Uuid, IslandError> {
        let uuid = Uuid::new_v4();

        self.redis_connection.clone().sadd("valid_uuids", uuid.to_string()).await?;

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

        let mut scores: Vec<LeaderboardEntry> = conn.hgetall("scores").await?
            .into_iter()
            .map(|(person, score)| {
                Ok(LeaderboardEntry {
                    person, score: score.parse()?
                })
            })
            .collect::<Result<_, IslandError>>()?;

        scores.sort_by_key(|entry| Reverse(entry.score));
        Ok(scores)
    }

    pub fn subscribe_to_update_leaderboard(&self) -> Receiver<Vec<LeaderboardEntry>> {
        self.update_leaderboard.subscribe()
    }
}
