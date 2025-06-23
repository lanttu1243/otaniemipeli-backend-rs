use deadpool_postgres::Pool;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool,
}
impl AppState {
    pub fn new(db: Pool) -> Self {
        Self { db }
    }
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("database error: {0}")]
    Database(String),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("rate limited")]
    RateLimited,
    #[error("internal error")]
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            AppError::Validation(m)   => (StatusCode::BAD_REQUEST, m),
            AppError::Database(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
            AppError::Conflict(m)     => (StatusCode::CONFLICT, m),
            AppError::NotFound(m)     => (StatusCode::NOT_FOUND, m),
            AppError::RateLimited     => (StatusCode::TOO_MANY_REQUESTS, "too many requests".into()),
            AppError::Internal        => (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".into()),
        };
        (status, Json(ErrorBody { error: msg })).into_response()
    }
}

impl From<deadpool_postgres::PoolError> for AppError {
    fn from(e: deadpool_postgres::PoolError) -> Self {
        eprintln!("{e}");
        AppError::Database("Database operations encountered an error!".into())
    }
}