pub mod v1;
pub mod referee;

use axum::{middleware, Router};
use crate::utils::state::{auth_middleware, AppState};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/v1", v1::router())
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
}