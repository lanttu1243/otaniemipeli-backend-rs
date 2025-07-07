use axum::extract::{Path, Query, State};
use axum::Json;
use deadpool_postgres::Client;
use crate::database::boards::{add_place, get_board_places, get_places};
use crate::database::drinks::post_drink;
use crate::utils::state::{AppError, AppState};
use crate::utils::types::{Board, BoardPlaces, Drink, Place, Places};

pub async fn board_places_get(
    Path(board_id): Path<i32>,
    state: State<AppState>,
) -> Result<Json<BoardPlaces>, AppError> {
    println!("GET /boards/places/{board_id}");

    let client: Client = state.db.get().await?;
    match get_board_places(&client, board_id).await {
        Ok(places) => Ok(Json(places)),
        Err(_) => Err(AppError::Database("Error getting places!".to_string())),
    }
}
pub async fn places_post(
    state: State<AppState>,
    Json(place): Json<Place>,
) -> Result<Json<Place>, AppError> {
    println!("POST /boards/places");

    let client: Client = state.db.get().await?;
    match add_place(&client, place.clone()).await {
        Err(e) => {
            eprintln!("{}", e);
            Err(AppError::Database("Database operations encountered an error!".parse().unwrap()))
        },
        _ => Ok(Json(place))
    }
}
pub async fn places_get(
    state: State<AppState>
) -> Result<Json<Places>, AppError> {
    println!("GET /boards/places");

    let client: Client = state.db.get().await?;
    match get_places(&client).await {
        Ok(places) => Ok(Json(places)),
        Err(_) => Err(AppError::Database("Error getting places!".to_string())),
    }
}