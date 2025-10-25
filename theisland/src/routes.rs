use axum::extract::{Multipart, State};
use axum::Json;
use serde::Deserialize;
use uuid::Uuid;
use crate::state::IslandState;

pub async fn start_grass (State(state): State<IslandState>) -> Json<Uuid> {
    let uuid = state.start_submission().await;
    Json(uuid)
}

#[derive(Deserialize)]
pub struct SubmitGrassForm {
    pub uuid: Uuid,
    pub name: String,
    pub image: Multipart
}

pub async fn submit_grass (State(state): State<IslandState>) -> 