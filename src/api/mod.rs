pub mod v1;
pub mod referee;

use axum::Router;
use crate::utils::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/v1", v1::router())
}