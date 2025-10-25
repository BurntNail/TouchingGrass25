use crate::errors::IslandError;
use crate::state::{IslandState, LeaderboardEntry, TopImageEntry};
use axum::extract::{Path, State};
use axum::response::{
    IntoResponse, Response, Sse,
    sse::{Event as AxumSseEvent, KeepAlive},
};
use axum::{Form, Json};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use futures::Stream;
use image::{GenericImageView, ImageReader, Pixel};
use reqwest::Client;
use serde::Deserialize;
use std::fmt::Display;
use std::io::Cursor;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;
use tracing::info;
use uuid::Uuid;

pub async fn start_grass(State(state): State<IslandState>) -> Result<Json<Uuid>, IslandError> {
    let uuid = state.start_submission().await?;
    Ok(Json(uuid))
}

#[derive(Deserialize)]
pub struct SubmitGrassForm {
    name: String,
    file: String,
}

#[axum::debug_handler]
pub async fn submit_grass(
    State(state): State<IslandState>,
    Form(SubmitGrassForm { name, file }): Form<SubmitGrassForm>,
) -> Result<impl IntoResponse, IslandError> {
    let uuid = state.start_submission().await?; //TODO: make this actually not just be useless lolll

    let file_contents = BASE64_STANDARD.decode(file.as_bytes())?;
    info!("decoded");
    let img = ImageReader::new(Cursor::new(&file_contents))
        .with_guessed_format()
        .unwrap()
        .decode()?;

    let mut sum_distance: f32 = 0.0;
    let mut count: f32 = 0.0;

    for (_, _, pixel) in img.pixels() {
        let [red, green, blue] = pixel.to_rgb().0;

        count += 1.0;
        sum_distance +=
            ((red as f32).powi(2) + (255.0 - green as f32).powi(2) + (blue as f32).powi(2)).sqrt();
    }

    const MAX_DISTANCE: f32 = 441.672_94; // 255.0 * 3.0_f32.sqrt()
    let average_distance = sum_distance / count;
    let score = ((1.0 - average_distance / MAX_DISTANCE) * 100.0)
        .clamp(0.0, 100.0) as u32;

    info!(?average_distance, ?score);

    state.add_score(uuid, name.clone(), score).await?;
    state
        .set_potential_top_image(name, score, file_contents)
        .await?;

    Ok("/")
}

pub async fn get_leaderboard(
    State(state): State<IslandState>,
) -> Result<Json<Vec<LeaderboardEntry>>, IslandError> {
    Ok(Json(state.get_leaderboard().await?))
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

pub async fn top_images(
    State(state): State<IslandState>,
) -> Result<Json<Vec<TopImageEntry>>, IslandError> {
    state.get_top_images().await.map(Json)
}

pub async fn get_page(
    State(state): State<IslandState>,
    path: Option<Path<String>>,
) -> Result<impl IntoResponse, IslandError> {
    Ok(match path {
        Some(Path(path)) => get_with_path(state.client, path).await?.into_response(),
        None => index(State(state)).await?.into_response(),
    })
}

pub async fn get_asset(
    State(state): State<IslandState>,
    Path(asset): Path<String>,
) -> Result<impl IntoResponse, IslandError> {
    get_with_path(state.client, format!("Assets/{asset}")).await
}

pub async fn index(State(state): State<IslandState>) -> Result<impl IntoResponse, IslandError> {
    get_with_path(state.client, "index.html").await
}

pub async fn get_with_path(
    client: Client,
    path: impl Display + AsRef<std::path::Path>,
) -> Result<Response, IslandError> {
    let path_path = path.as_ref();
    let url = format!(
        "https://raw.githubusercontent.com/BurntNail/TouchingGrass25/refs/heads/main/Html/{}",
        path
    );
    let rsp = client.get(url).send().await?;

    let mime = mime_guess::from_path(path_path)
        .first()
        .unwrap()
        .to_string();
    info!(?mime);
    let bytes = rsp.bytes().await?.to_vec();

    Ok(([(http::header::CONTENT_TYPE, mime)], bytes).into_response())
}
