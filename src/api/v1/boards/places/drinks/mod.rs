

pub mod utils;

use axum::Router;
use axum::routing::post;
use crate::utils::state::AppState;
use self::utils::*;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(post_place_drinks))
}