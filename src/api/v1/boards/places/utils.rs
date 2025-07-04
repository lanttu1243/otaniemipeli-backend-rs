use axum::extract::{Path, State};
use axum::Json;
use deadpool_postgres::Client;
use crate::database::drinks::delete_drink;
use crate::utils::state::{AppError, AppState};
use crate::utils::types::ResultIntJson;
/*
pub async fn places_get(
    state: State<AppState>,
) -> Result<Json<ResultIntJson>, AppError> {
    println!("DELETE /drinks/{}");

    let client: Client = state.db.get().await?;
    match get_places(&client).await {
        Ok(places) => Ok(Json(places)),
        Err(_) => Err(AppError::Database("Error getting places!".to_string())),
    }
}
*/