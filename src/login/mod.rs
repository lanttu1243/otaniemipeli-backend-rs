use axum::Router;
use axum::routing::{delete, post};
use crate::login::utils::{end_all_sessions, end_session, start_session, verify_session};
use crate::utils::state::AppState;

pub mod utils;


pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(start_session).put(verify_session).delete(end_session))
        .route("/all", delete(end_all_sessions))
}