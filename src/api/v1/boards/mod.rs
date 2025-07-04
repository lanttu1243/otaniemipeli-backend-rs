use axum::Router;
use axum::routing::{delete, get, post};
use crate::utils::state::AppState;

pub mod utils;
use self::utils::{boards_get, boards_get_id};

pub mod places;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{id}", get(boards_get_id))
        .route("/",
               get(boards_get))
}