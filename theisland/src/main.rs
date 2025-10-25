use crate::backend::{
    get_asset, get_leaderboard, get_page, index, leaderboard_sse, start_grass, submit_grass,
    top_images,
};
use crate::state::IslandState;
use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing::info;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

mod backend;
mod errors;
mod state;

#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_env_filter(EnvFilter::from_default_env())
            .finish(),
    )
    .unwrap();

    let state = IslandState::new().await.unwrap();

    let app = Router::new()
        .route("/", get(index))
        .route("/{path}", get(get_page))
        .route("/Assets/{path}", get(get_asset))
        .route("/start_grass", post(start_grass))
        .route("/submit_grass", post(submit_grass))
        .route("/leaderboard", get(get_leaderboard))
        .route("/leaderboard_sse", get(leaderboard_sse))
        .route("/topimages", get(top_images))
        .layer(CorsLayer::default().allow_origin(AllowOrigin::any()))
        .layer(DefaultBodyLimit::max(64_000_000))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    info!(?listener, "listening on");
    axum::serve(listener, app).await.unwrap();
}
