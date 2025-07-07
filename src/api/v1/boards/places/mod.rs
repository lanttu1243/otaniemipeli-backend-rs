use axum::Router;
use axum::routing::{delete, get, post};
use crate::api::v1::boards::utils::{boards_get, boards_get_id};
use crate::database::boards::get_board_places;
use crate::utils::state::AppState;

pub mod utils;
use self::utils::*;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(places_get)
            .post(places_post))
        .route("/{id}", get(board_places_get))
}