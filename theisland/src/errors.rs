use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IslandError {
    #[error("error serialising/deserialising json")]
    Json(#[from] serde_json::Error),
    #[error("error parsing score")]
    InvalidScore(#[from] std::num::ParseIntError),
    #[error("error with multipart")]
    Multipart(#[from] axum::extract::multipart::MultipartError),
    #[error("error decoding base64")]
    B64Decode(#[from] base64::DecodeError),
    #[error("error with image")]
    Image(#[from] image::error::ImageError),
    #[error("missing env var")]
    Env(#[from] std::env::VarError),
    #[error("error with redis")]
    Redis(#[from] redis::RedisError),
    #[error("error with S3 creds")]
    S3Creds(#[from] s3::creds::error::CredentialsError),
    #[error("error with S3")]
    S3(#[from] s3::error::S3Error),
}

impl IntoResponse for IslandError {
    fn into_response(self) -> Response {
        eprintln!("{self:?}");
        match self {
            Self::Json(_) | Self::Image(_) | Self::Env(_) | Self::Redis(_) | Self::S3Creds(_) | Self::S3(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidScore(_) | Self::B64Decode(_) => StatusCode::BAD_REQUEST,
            Self::Multipart(m) => m.status(),
        }.into_response()
    }
}
