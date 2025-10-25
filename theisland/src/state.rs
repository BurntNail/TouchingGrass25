use serde::Serialize;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::broadcast::{Receiver, Sender, channel};
use tracing::error;
use uuid::Uuid;

#[derive(Clone)]
pub struct IslandState {
    scores: Arc<RwLock<HashMap<String, u32>>>,
    valid_uuids: Arc<RwLock<HashSet<Uuid>>>,
    update_leaderboard: Sender<Vec<LeaderboardEntry>>,
}

impl Default for IslandState {
    fn default() -> Self {
        let (update_leaderboard, _) = channel(1);
        Self {
            scores: Arc::new(RwLock::new(HashMap::new())),
            valid_uuids: Arc::new(RwLock::new(HashSet::new())),
            update_leaderboard,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct LeaderboardEntry {
    pub person: String,
    pub score: u32,
}

impl IslandState {
    pub async fn start_submission(&self) -> Uuid {
        let uuid = Uuid::new_v4();
        self.valid_uuids.write().await.insert(uuid);
        uuid
    }

    pub async fn add_score(&self, uuid: Uuid, name: String, score: u32) {
        if self.valid_uuids.write().await.remove(&uuid) {
            *self.scores.write().await.entry(name).or_default() += score;

            let _ = self.update_leaderboard.send(self.get_leaderboard().await);
        } else {
            error!("tried to submit w/o getting start");
        }
    }

    pub async fn get_leaderboard(&self) -> Vec<LeaderboardEntry> {
        let mut scores: Vec<_> = self
            .scores
            .read().await
            .iter()
            .map(|(person, score)| LeaderboardEntry {
                person: person.clone(),
                score: *score,
            })
            .collect();
        scores.sort_by_key(|entry| Reverse(entry.score));
        scores
    }

    pub fn subscribe_to_update_leaderboard(&self) -> Receiver<Vec<LeaderboardEntry>> {
        self.update_leaderboard.subscribe()
    }
}
