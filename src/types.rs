use postgres;
use serde::{Deserialize, Serialize};

pub type PgError = postgres::error::Error;

#[derive(Serialize)]
pub struct GameInfo {
    pub id: i32,
    pub name: String,
    pub board: String,
}
#[derive(Deserialize)]
pub struct PostGame {
    pub name: String,
    pub board: i32,
}

#[derive(Serialize)]
pub struct Games {
    pub games: Vec<GameInfo>,
}
#[derive(Serialize)]
pub struct Board {
    pub id: i32,
    pub name: String,
}
#[derive(Serialize)]
pub struct Boards {
    pub boards: Vec<Board>,
}