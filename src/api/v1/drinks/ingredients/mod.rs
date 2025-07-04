use axum::{Router};
use axum::routing::{delete, get};
use crate::utils::state::{AppState};

pub mod utils;
use self::utils::*;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/ingredients", get(drinks_ingredients_get)
            .post(drink_ingredients_post))
        .route("/ingredients/{drink_id}",
        get(drink_ingredients_get).delete(drink_ingredient_delete))
}