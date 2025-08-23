use axum::Router;
use axum::routing::get;
use crate::utils::state::AppState;

pub mod utils;
use self::utils::*;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/",
               get(games_get)
                   .post(games_post))
}