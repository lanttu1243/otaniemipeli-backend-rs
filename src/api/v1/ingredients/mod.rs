use axum::Router;
use axum::routing::{delete, get};
use crate::utils::state::AppState;

pub mod utils;
use self::utils::*;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/ingredients", 
               get(ingredients_get)
                   .post(ingredients_post))
        .route("/ingredients/{id}",
               delete(ingredient_delete)
        )
}