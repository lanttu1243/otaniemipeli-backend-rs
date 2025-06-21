use serde::{Deserialize, Serialize};
pub type PgError = tokio_postgres::error::Error;

#[derive(Serialize, Deserialize)]
pub struct GameInfo {
    pub id: i32,
    pub name: String,
    pub board: String,
}
#[derive(Serialize, Deserialize)]
pub struct PostGame {
    pub name: String,
    pub board: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Games {
    pub games: Vec<GameInfo>,
}
#[derive(Serialize, Deserialize)]
pub struct Board {
    pub id: i32,
    pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct Boards {
    pub boards: Vec<Board>,
}