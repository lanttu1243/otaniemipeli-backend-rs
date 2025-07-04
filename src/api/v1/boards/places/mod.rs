use axum::Router;
use axum::routing::{delete, get};
use crate::utils::state::AppState;

pub mod utils;
use self::utils::*;
