use axum::extract::{Path, Query, State};
use axum::Json;
use deadpool_postgres::Client;
use crate::database::drinks::*;
use crate::utils::state::{AppError, AppState};
use crate::utils::types::{Drink, DrinkIngredients, DrinkIngredientsPost, Drinks, DrinksIngredients, Ingredient, IngredientIdQuery, IngredientsQuery};

pub async fn drinks_ingredients_get(
    state: State<AppState>,
) -> Result<Json<DrinksIngredients>, AppError> {
    println!("GET /drinks/ingredients");
    let client: Client = state.db.get().await?;
    match get_drinks_ingredients(&client).await {
        Ok(drinks_ingredients) if drinks_ingredients.drink_ingredients.is_empty() => {
            Err(AppError::NotFound(String::from("No drinks with ingredients")))
        },
        Ok(drinks_ingredients) => Ok(Json(drinks_ingredients)),
        Err(e) => {
            eprintln!("{}", e);
            Err(AppError::Database("The server encountered an unexpected error!".parse().unwrap()))
        }
    }
}
pub async fn drink_ingredients_post(
    state: State<AppState>,
    Json(drink_ingredients): Json<DrinkIngredientsPost>,
) -> Result<Json<DrinkIngredientsPost>, AppError> {
    println!("POST /drinks/ingredients");
    let client: Client = state.db.get().await?;
    match add_ingredients(&client, drink_ingredients.clone()).await {
        Err(e) => {
            eprintln!("{}", e);
            Err(AppError::Database("Database operations encountered an error!".parse().unwrap()))
        },
        _ => Ok(Json(drink_ingredients))
    }
}
pub async fn drink_ingredient_delete(
    Path(drink_id): Path<i32>,
    state: State<AppState>,
    query: Query<IngredientIdQuery>,
) -> Result<Json<Ingredient>, AppError> {
    let ingredient_id: i32 = query.ingredient_id;
    println!("DELETE /drinks/ingredients/{}?{}", drink_id, ingredient_id);
    let client: Client = state.db.get().await?;
    match delete_ingredient_from_drink(&client, drink_id, ingredient_id).await {
        Ok(_) => Ok(
            Json(match get_ingredient(&client, ingredient_id).await {
                Ok(ingredient) => ingredient,
                Err(e) => {
                    eprintln!("{}", e);
                    return Err(AppError::Database(format!("Ingredient {} not in database!", ingredient_id)))
                }
            }
            )),
        Err(_) => Err(AppError::Database(format!("Ingredient {ingredient_id} not in database!"))),
    }
}
pub async fn drink_ingredients_get(
    Path(drink_id): Path<i32>,
    state: State<AppState>,
) -> Result<Json<DrinkIngredients>, AppError> {
    println!("GET /drinks/ingredients");
    let client: Client = state.db.get().await?;
    let drinks: Drinks = match get_drinks(&client).await {
        Ok(drinks) => drinks,
        Err(e) => {
            eprintln!("{}", e);
            return Err(AppError::Database("The server encountered an unexpected error!".parse().unwrap()))
        }
    };
    let mut drink: Option<Drink> =  None;
    for drink_from in drinks.drinks {
        if drink_from.id == drink_id {
            drink = Some(drink_from);
        }
    };
    match drink {
        Some(_) => {},
        None => {
            return Err(AppError::Database("The server encountered an unexpected error!".parse().unwrap()))
        }
    }
    match get_drink_ingredients(&client, drink.unwrap()).await {
        Ok(drink_ingredients) => Ok(Json(drink_ingredients)),
        Err(e) => {
            eprintln!("{}", e);
            Err(AppError::Database("The server encountered an unexpected error!".parse().unwrap()))
        }
    }
}