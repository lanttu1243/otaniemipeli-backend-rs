pub mod games;
pub mod drinks;

use axum::Router;
use deadpool_postgres::Pool;
use crate::utils::state::AppState;

pub fn router() -> Router<AppState> {
    
    Router::new()
        .merge(drinks::router())
        // .route("/games", games::router)
}