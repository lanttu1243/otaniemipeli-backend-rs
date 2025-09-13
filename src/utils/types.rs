use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::types::{FromSql, ToSql};
pub type PgError = tokio_postgres::error::Error;
#[derive(Clone, Debug, Serialize, Deserialize, ToSql, FromSql)]
#[postgres(name = "placetype")]
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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SocketAuth {
    pub token: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageBack {
    pub ok: bool,
    pub echo: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSql, FromSql)]
#[postgres(name = "usertype")]
#[derive(PartialEq, Eq)]
pub enum UserType {
    #[postgres(name = "admin")]
    admin,
    #[postgres(name = "ie")]
    ie,
    #[postgres(name = "referee")]
    referee,
    #[postgres(name = "secretary")]
    secretary,
    #[postgres(name = "team")]
    team,
}
impl UserType {
    pub fn as_str(&self) -> &str {
        match self {
            UserType::admin => "admin",
            UserType::ie => "ie",
            UserType::referee => "referee",
            UserType::secretary => "secretary",
            UserType::team => "team",
        }
    }
}
impl core::fmt::Display for UserType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub uid: i32,
    pub username: String,
    pub email: String,
    pub user_types: UserTypes,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct UserCreateInfo {
    pub username: String,
    pub email: String,
    pub user_type: UserType,
    pub password: String,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SessionInfo {
    pub uid: i32,
    pub session_hash: String,
    pub user_types: UserTypes,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct UserSessionInfo {
    pub user: UserInfo,
    pub session: SessionInfo,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserTypes {
    pub user_types: Vec<UserType>,
}
impl UserTypes {
    pub fn new() -> Self {
        Self {
            user_types: Vec::new(),
        }
    }
    pub fn push(&mut self, user_type: UserType) {
        self.user_types.push(user_type);
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Team {
    pub team_id: i32,
    pub name: String,
    pub game_id: i32,
    pub team_name: String,
    pub team_hash: String,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Teams {
    pub teams: Vec<Team>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct GameInfo {
    pub id: i32,
    pub name: String,
    pub board: String,
    pub finished: bool,
    pub start_time: DateTime<Utc>,
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
    pub places: Vec<BoardPlace>,
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
    pub drinks: PlaceDrinks,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct PlaceDrinks {
    pub drinks: Vec<PlaceDrink>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct PlaceDrink {
    pub place_number: i32,
    pub board_id: i32,
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
pub struct IngredientIdQuery {
    pub ingredient_id: i32,
}
