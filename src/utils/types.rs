use serde::{Deserialize, Serialize};
pub type PgError = tokio_postgres::error::Error;

#[derive(Clone, Serialize, Deserialize)]
pub struct GameInfo {
    pub id: i32,
    pub name: String,
    pub board: String,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct PostGame {
    pub name: String,
    pub board: i32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Games {
    pub games: Vec<GameInfo>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Board {
    pub id: i32,
    pub name: String,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Boards {
    pub boards: Vec<Board>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct BoardPlaces {
    pub board: Board,
    pub name: String,
    pub places: Vec<BoardPlace>
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Place {
    pub id: i32,
    pub name: String,
    pub refill: bool,
    pub special_rule: String,
    pub drink: Drink,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct BoardPlace {
    pub board: Board,
    pub place: Place,
    pub number: i32
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub abv: f64,
    pub carbonated: bool,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Ingredients {
    pub ingredients: Vec<Ingredient>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Drink {
    pub id: i32,
    pub name: String,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct IngredientQty {
    pub ingredient: Ingredient,
    pub quantity: f64,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct DrinkIngredients {
    pub drink: Drink,
    pub quantity: f64,
    pub abv: f64,
    pub ingredients: Vec<IngredientQty>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct DrinkIngredientsPost {
    pub drink: Drink,
    pub ingredients: Vec<IngredientQty>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct DrinksIngredients {
    pub drink_ingredients: Vec<DrinkIngredients>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct ResultIntJson {
    pub int: i32,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Drinks {
    pub drinks: Vec<Drink>,
}

#[derive(Deserialize)]
pub struct IngredientsQuery {
    pub with_ingredients: bool,
}

#[derive(Deserialize)]
pub struct IngredientIdQuery {
    pub ingredient_id: i32,
}