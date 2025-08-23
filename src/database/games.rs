use deadpool_postgres::{Client};
use crate::utils::types::*;

pub async fn get_games(client: &Client) -> Result<Games, PgError> {

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
pub async fn get_game(client: &Client, game_name: String, game_board: i32) -> Result<GameInfo, PgError> {
    let query_str = "SELECT
        games.game_id AS game_id,
        games.name AS game_name,
        boards.name AS board_name
    FROM games
    INNER JOIN boards ON games.board = boards.board_id
    WHERE games.name = $1 AND games.board = $2";
    let mut game: GameInfo = GameInfo {
        id: -100,
        name: "unknown".to_string(),
        board: "unknown".to_string(),
    };
    match client.query(query_str, &[&game_name, &game_board]).await {
        Ok(query) => {
            for row in query {
                game = GameInfo {
                    id: row.get(0),
                    name: row.get(1),
                    board: row.get(2),
                };
            }
            Ok(game)
        },
        Err(e) => Err(e),
    }
}

pub async fn post_game(client: &Client, game: PostGame) -> Result<GameInfo, PgError> {
    
    let query_str = "\
    INSERT INTO games (name, board) VALUES ($1, $2)";
    
    match client.execute(query_str, &[&game.name, &game.board]).await {
        Ok(v) => {
            get_game(client, game.name, game.board).await
        },
        Err(e) => Err(e),
    }

}