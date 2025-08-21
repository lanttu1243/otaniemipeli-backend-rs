use axum::Router;
use axum::routing::post;
use crate::login::utils::verify_session;
use crate::utils::state::AppState;

pub mod utils;
use self::utils::{post_login};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(post_login).put(verify_session))
}