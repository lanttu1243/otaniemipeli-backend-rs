use crate::utils::state::AppState;
use axum::routing::{delete, get, post};
use axum::Router;

pub mod utils;
use self::utils::{boards_get, boards_get_id};

pub mod places;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/places", places::router())
        .route("/{id}", get(boards_get_id))
        .route("/", get(boards_get))
}
