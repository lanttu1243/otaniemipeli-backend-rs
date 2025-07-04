use axum::Router;
use crate::utils::state::AppState;

pub mod ingredients;
pub mod drinks;
pub mod games;
pub mod boards;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(ingredients::router())
        .nest("/drinks", drinks::router())
        .nest("/boards", boards::router())
}