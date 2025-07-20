use serde::{Deserialize, Serialize};
use tokio_postgres::types::{FromSql, ToSql};
pub type PgError = tokio_postgres::error::Error;
#[derive(Clone, Debug, Serialize, Deserialize, ToSql, FromSql)]
#[postgres(name = "place_types")]
#[derive(PartialEq, Eq)]
pub enum PlaceType {
    #[postgres(name = "normal")]
    normal,
    #[postgres(name = "food")]
    food,
    #[postgres(name = "sauna")]
    sauna,
    #[postgres(name = "special")]
    special,
    #[postgres(name = "guild")]
    guild,
}

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
    pub places: Vec<BoardPlace>
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Place {
    pub place_id: i32,
    pub place_name: String,
    pub rule: String,
    pub place_type: PlaceType,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Places {
    pub places: Vec<Place>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct BoardPlace {
    pub board_id: i32,
    pub place: Place,
    pub place_number: i32,
    pub start: bool,
    pub end: bool,
    pub x: f64,
    pub y: f64,
    pub connections: Vec<Connection>,
    pub drinks: PlaceDrinks
}
#[derive(Clone, Serialize, Deserialize)]
pub struct PlaceDrinks {
    pub drinks: Vec <PlaceDrink>
}
#[derive(Clone, Serialize, Deserialize)]
pub struct PlaceDrink {
    pub place_id: i32,
    pub drink: Drink,
    pub refill: bool,
    pub optional: bool,
    pub n: i32,
    pub n_update: String,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Connection {
    pub board_id: i32,
    pub origin: i32,
    pub target: i32,
    pub on_land: bool,
    pub backwards: bool,
    pub dashed: bool,
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