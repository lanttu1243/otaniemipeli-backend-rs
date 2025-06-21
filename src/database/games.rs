use deadpool_postgres::{Client};
use crate::utils::types::*;

pub async fn get_games(client: Client) -> Result<Games, PgError> {

    let query_str = "
    SELECT 
        games.game_id AS game_id,
        games.name AS game_name,
        boards.name AS board_name
    FROM games
    INNER JOIN boards ON games.board = boards.board_id";
    
    let mut games: Vec<GameInfo> = Vec::new();
    
    let query = match client.query(query_str, &[]).await {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    
    for row in query {
        let game: GameInfo = GameInfo {
            id: row.get(0),
            name: row.get(1),
            board: row.get(2),
        };
        games.push(game);
    }
    Ok(Games {games})
}

pub async fn post_game(client: Client, game: PostGame) -> Result<u64, PgError> {
    
    let query_str = "\
    INSERT INTO games (name, board) VALUES ($1, $2)";
    
    match client.execute(query_str, &[&game.name, &game.board]).await {
        Ok(v) => Ok(v),
        Err(e) => Err(e),
    }
}