use axum::extract::{Path, State};
use axum::Json;
use deadpool_postgres::Client;
use crate::database::drinks::{delete_drink, delete_ingredient, get_drinks, get_drinks_ingredients, get_ingredient, post_drink, post_ingredient};
use crate::utils::remove_ingredients;
use crate::utils::state::{AppError, AppState};
use crate::utils::types::{Drink, Drinks, DrinksIngredients, Ingredient, ResultIntJson};

pub async fn drinks_get(
    state: State<AppState>
) -> Result<Json<DrinksIngredients>, AppError> {
    println!("GET /drinks");
    let client: Client = state.db.get().await?;
    match get_drinks_ingredients(&client).await {
        Ok(drinks) => Ok(Json(remove_ingredients(drinks))),
        Err(e) => {
            eprintln!("{}", e);
            Err(AppError::Database("The server encountered an unexpected error!".parse().unwrap()))
        }
    }
}

pub async fn drinks_post(
    state: State<AppState>,
    Json(drink): Json<Drink>,
) -> Result<Json<Drink>, AppError> {
    println!("POST /drinks");
    let client: Client = state.db.get().await?;
    match post_drink(&client, drink.clone()).await {
        Err(e) => {
            eprintln!("{}", e);
            Err(AppError::Database("Database operations encountered an error!".parse().unwrap()))
        },
        _ => Ok(Json(drink))
    }
}

pub async fn drink_delete(
    Path(drink_id): Path<i32>,
    state: State<AppState>,
) -> Result<Json<ResultIntJson>, AppError> {
    println!("DELETE /drinks/{}", drink_id);

    let client: Client = state.db.get().await?;
    match delete_drink(&client, drink_id).await {
        Ok(_) => Ok(Json(ResultIntJson { int: drink_id })),
        Err(_) => Err(AppError::Database(format!("Drink {drink_id} not in database!"))),
    }
}