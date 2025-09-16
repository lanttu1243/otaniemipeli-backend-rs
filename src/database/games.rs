use crate::utils::types::*;
use chrono::{DateTime, Utc};
use deadpool_postgres::Client;
use tokio_postgres::Row;

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
        games.start_time AS start_time
        games.name AS game_name,
        boards.name AS board_name,
        games.started AS started,
        games.finished AS finished,
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
                game = build_game_from_row(&row).await;
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
pub async fn make_first_turns(client: &Client, first_turn: &FirstTurnPost) -> Result<(), PgError> {
    println!("Making first turns for game {}", first_turn.game_id);
    let query_str = "\
    WITH ins_turns AS (
      INSERT INTO turns (team_id, game_id, dice1, dice2)
      SELECT team_id, $1::int, -1, -1
      FROM teams
      WHERE game_id = $1
      RETURNING turn_id
    ),
    drinks(drink_id, n) AS (
      -- zip drink ids and counts
      SELECT * FROM unnest($2::int[], $3::int[])
    ),
    ins_drinks AS (
      INSERT INTO turn_drinks (drink_id, turn_id, n)
      SELECT d.drink_id, it.turn_id, GREATEST(1, d.n)  -- clamp to >= 1
      FROM ins_turns it
      CROSS JOIN drinks d
      RETURNING drink_id, turn_id, n
    )
    SELECT * FROM ins_drinks
    ";

    let (drink_ids, counts): (Vec<i32>, Vec<i32>) = first_turn
        .drinks
        .iter()
        .map(|td| (td.drink.id, td.n))
        .unzip();

    match client
        .execute(
            query_str,
            &[&first_turn.game_id, &drink_ids, &counts], // len must match!
        )
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
pub async fn start_game(client: &Client, first_turn: FirstTurnPost) -> Result<Game, PgError> {
    println!("Starting game {}", first_turn.game_id);
    let query_str = "\
    UPDATE games SET \
     started = true, \
     start_time = NOW() \
    WHERE game_id = $1 \
    RETURNING *";
    make_first_turns(client, &first_turn).await?;

    match client.query_one(query_str, &[&first_turn.game_id]).await {
        Ok(row) => {
            let game = build_game_from_row(&row).await;
            Ok(game)
        }
        Err(e) => Err(e),
    }
}

async fn build_game_from_row(row: &Row) -> Game {
    Game {
        id: row.get(0),
        start_time: row.get(1),
        name: row.get(2),
        board: row.get(3),
        started: row.get(4),
        finished: row.get(5),
    }
}
