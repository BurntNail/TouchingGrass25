use std::cmp::Reverse;
use std::collections::HashSet;
use std::sync::{Arc};
use dashmap::DashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct IslandState {
    scores: DashMap<String, u32>,
    valid_uuids: Arc<RwLock<HashSet<Uuid>>>
}

pub struct LeaderboardEntry {
    pub person: String,
    pub score: u32,
}

impl IslandState {
    pub async fn start_submission (&self) -> Uuid {
        let uuid = Uuid::new_v4();
        self.valid_uuids.write().await.insert(uuid);
        uuid
    }

    pub async fn add_score (&self, uuid: Uuid, name: String, score: u32) {
        if self.valid_uuids.write().await.remove(&uuid) {
            *self.scores.entry(name).or_default() += score;
        }
    }

    pub fn get_leaderboard (&self) -> Vec<LeaderboardEntry> {
        let mut scores: Vec<_> = self.scores.clone().into_iter().map(|(person, score)| LeaderboardEntry {person, score}).collect();
        scores.sort_by_key(|entry| Reverse(entry.score));
        scores
    }


}