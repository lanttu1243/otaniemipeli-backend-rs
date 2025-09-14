use crate::utils::types::*;
use chrono::{DateTime, Utc};
use deadpool_postgres::Client;

pub async fn get_games(client: &Client) -> Result<Games, PgError> {
    let query_str = "
    SELECT 
        games.game_id AS game_id,
        games.name AS game_name,
        boards.name AS board_name,
        games.started AS started,
        games.finished AS finished,
        games.start_time AS start_time
    FROM games
    INNER JOIN boards ON games.board_id = boards.board_id";

    let mut games: Vec<Game> = Vec::new();

    let query = match client.query(query_str, &[]).await {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    for row in query {
        let game: Game = Game {
            id: row.get(0),
            name: row.get(1),
            board: row.get(2),
            started: row.get(3),
            finished: row.get(4),
            start_time: row.get(5),
        };
        games.push(game);
    }
    Ok(Games { games })
}
pub async fn get_game(
    client: &Client,
    game_name: String,
    game_board: i32,
) -> Result<Game, PgError> {
    let query_str = "SELECT
        games.game_id AS game_id,
        games.name AS game_name,
        boards.name AS board_name,
        games.started AS started,
        games.finished AS finished,
        games.start_time AS start_time
    FROM games
    INNER JOIN boards ON games.board_id = boards.board_id
    WHERE games.name = $1 AND games.board_id = $2";
    let mut game: Game = Game {
        id: -100,
        name: "unknown".to_string(),
        board: "unknown".to_string(),
        started: false,
        finished: false,
        start_time: DateTime::parse_from_rfc3339("1986-02-13T14:00:00Z")
            .unwrap()
            .with_timezone(&Utc),
    };
    match client.query(query_str, &[&game_name, &game_board]).await {
        Ok(query) => {
            for row in query {
                game = Game {
                    id: row.get(0),
                    name: row.get(1),
                    board: row.get(2),
                    started: row.get(3),
                    finished: row.get(4),
                    start_time: row.get(5),
                };
            }
            Ok(game)
        }
        Err(e) => Err(e),
    }
}

pub async fn post_game(client: &Client, game: PostGame) -> Result<Game, PgError> {
    let query_str = "\
    INSERT INTO games (name, board_id) VALUES ($1, $2)";

    match client.execute(query_str, &[&game.name, &game.board]).await {
        Ok(_) => get_game(client, game.name, game.board).await,
        Err(e) => Err(e),
    }
}
pub async fn start_game(client: &Client, game_id: i32) -> Result<Game, PgError> {
    let query_str = "\
    UPDATE games SET \
     started = true, \
     start_time = NOW() \
    WHERE game_id = $1 \
    RETURNING *";

    match client.query_one(query_str, &[&game_id]).await {
        Ok(row) => {
            let game = Game {
                id: row.get(0),
                name: row.get(1),
                board: row.get(2),
                started: row.get(3),
                finished: row.get(4),
                start_time: row.get(5),
            };
            Ok(game)
        }
        Err(e) => Err(e),
    }
}
