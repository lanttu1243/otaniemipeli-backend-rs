use axum::extract::{Path, State};
use axum::Json;
use deadpool_postgres::Client;
use crate::database::boards::{get_board, get_boards};
use crate::utils::state::{AppError, AppState};
use crate::utils::types::{Board, Boards};

pub async fn boards_get(
    state: State<AppState>,
) -> Result<Json<Boards>, AppError> {
    println!("GET /boards");

    let client: Client = state.db.get().await?;
    match get_boards(&client).await {
        Ok(places) => Ok(Json(places)),
        Err(_) => Err(AppError::Database("Error getting places!".to_string())),
    }
}
pub async fn boards_get_id(
    Path(board_id): Path<i32>,
    state: State<AppState>,
) -> Result<Json<Board>, AppError> {
    println!("GET /boards");

    let client: Client = state.db.get().await?;
    match get_board(&client, board_id).await {
        Ok(board) => Ok(Json(board)),
        Err(_) => Err(AppError::Database("Error getting places!".to_string())),
    }
}