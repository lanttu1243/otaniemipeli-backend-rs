use axum::extract::{Path, State};
use axum::Json;
use deadpool_postgres::Client;
use crate::database::boards::{add_board_place, get_board, get_boards};
use crate::utils::state::{AppError, AppState};
use crate::utils::types::{Board, BoardPlace, Boards, Place};

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
pub async fn board_place_post(
    Path(board_id): Path<i32>,
    state: State<AppState>,
    Json(place): Json<BoardPlace>,
) -> Result<Json<u64>, AppError> {
    println!("POST /boards/places/{board_id}");
    
    let client: Client = state.db.get().await?;
    match add_board_place(&client, board_id, place).await {
        Ok(success) => Ok(Json(success)),
        Err(_) => Err(AppError::Database("Error getting places!".to_string())),
    }
}