use crate::backend::{get_asset, get_leaderboard, get_page, index, leaderboard_sse, post_comment, get_all_comments, submit_grass, top_images, comments_sse, top_images_sse};
use crate::state::IslandState;
use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[macro_use]
extern crate tracing;

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
        .route("/submit_grass", post(submit_grass))
        .route("/leaderboard", get(get_leaderboard))
        .route("/leaderboard_sse", get(leaderboard_sse))
        .route("/comments_sse", get(comments_sse))
        .route("/topimages_sse", get(top_images_sse))
        .route("/topimages", get(top_images))
        .route("/comment", post(post_comment))
        .route("/all_comments", get(get_all_comments))
        .layer(CorsLayer::default().allow_origin(AllowOrigin::any()))
        .layer(DefaultBodyLimit::max(64_000_000))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    info!(?listener, "listening on");
    axum::serve(listener, app).await.unwrap();
}
