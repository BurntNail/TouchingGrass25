use crate::errors::IslandError;
use crate::state::{IslandState, LeaderboardEntry};
use axum::extract::{State};
use axum::response::{IntoResponse, Sse, sse::{Event as AxumSseEvent, KeepAlive}, Redirect};
use axum::{Form, Json};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use futures::Stream;
use image::{GenericImageView, ImageReader, Pixel};
use serde::Deserialize;
use std::io::Cursor;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;
use tracing::info;
use uuid::Uuid;

pub async fn start_grass(State(state): State<IslandState>) -> Json<Uuid> {
    let uuid = state.start_submission().await;
    Json(uuid)
}

#[derive(Deserialize)]
pub struct SubmitGrassForm {
    name: String,
    file: String,
}

#[axum::debug_handler]
pub async fn submit_grass(
    State(state): State<IslandState>,
    Form(SubmitGrassForm {
        name,
        file,
    }): Form<SubmitGrassForm>,
) -> Result<impl IntoResponse, IslandError> {
    let uuid = state.start_submission().await; //TODO: make this actually not just be useless lolll

    let file_contents = BASE64_STANDARD.decode(file.as_bytes())?;
    info!("decoded");
    let img = ImageReader::new(Cursor::new(file_contents))
        .with_guessed_format()
        .unwrap()
        .decode()?;

    let mut sum_distance: f32 = 0.0;
    let mut count: f32 = 0.0;

    for (_, _, pixel) in img.pixels() {
        let [red, green, blue] = pixel.to_rgb().0;

        count += 1.0;
        sum_distance += ((red as f32).powi(2) + (255.0 - green as f32).powi(2) + (blue as f32).powi(2)).sqrt();
    }


    const MAX_DISTANCE: f32 = 441.67295593; // 255.0 * 3.0_f32.sqrt()
    let average_distance = sum_distance / count;
    let score = ((1.0 - average_distance / MAX_DISTANCE) * 100.0).max(0.0).min(100.0) as u32;

    info!(?average_distance, ?score);

    state.add_score(uuid, name, score).await;

    Ok("/")
}

pub async fn get_leaderboard(State(state): State<IslandState>) -> Json<Vec<LeaderboardEntry>> {
    Json(state.get_leaderboard().await)
}

pub async fn leaderboard_sse(
    State(state): State<IslandState>,
) -> Sse<impl Stream<Item = Result<AxumSseEvent, IslandError>>> {
    let stream = BroadcastStream::new(state.subscribe_to_update_leaderboard())
        .filter_map(Result::ok)
        .map(|lb| {
            let jsoned = serde_json::to_string(&lb)?;
            Ok(AxumSseEvent::default()
                .event("new_leaderboard")
                .data(jsoned))
        });

    Sse::new(stream).keep_alive(KeepAlive::default())
}
