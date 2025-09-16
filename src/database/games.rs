use crate::database::drinks::get_drinks;
use crate::database::team::get_teams;
use crate::utils::types::*;
use chrono::{DateTime, Utc};
use deadpool_postgres::Client;
use futures::future::join_all;
use tokio_postgres::Row;

pub async fn get_games(client: &Client) -> Result<Games, PgError> {
    let query_str = "SELECT * FROM games";

    let mut games: Vec<Game> = Vec::new();

    let query = match client.query(query_str, &[]).await {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    for row in query {
        let game: Game = build_game_from_row(&row).await;
        games.push(game);
    }
    Ok(Games { games })
}
pub async fn get_game(
    client: &Client,
    game_name: String,
    game_board: i32,
) -> Result<Game, PgError> {
    let query_str = "
    SELECT * FROM games
    WHERE games.name = $1 AND games.board_id = $2";
    let mut game: Game = Game {
        id: -100,
        name: "unknown".to_string(),
        board: -404,
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

pub async fn get_game_id(client: &Client, game_id: i32) -> Result<Game, PgError> {
    let query_str = "
    SELECT * FROM games
    WHERE games.id = $1";
    let mut game: Game = Game {
        id: -100,
        name: "unknown".to_string(),
        board: -404,
        started: false,
        finished: false,
        start_time: DateTime::parse_from_rfc3339("1986-02-13T14:00:00Z")
            .unwrap()
            .with_timezone(&Utc),
    };
    match client.query(query_str, &[&game_id]).await {
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
        finished: row.get(3),
        board: row.get(4),
        started: row.get(5),
    }
}
pub async fn get_team_data(client: &Client, game_id: i32) -> Result<GameData, PgError> {
    let game = get_game_id(client, game_id).await?;
    let teams = get_teams(client, game_id).await?;
    let teams = join_all(teams.iter().map(|team| {
        let team = team.clone();
        async move {
            let turns = get_team_turns(&client, team.team_id)
                .await
                .unwrap_or_default();
            GameTeam {
                team,
                turns,
                location: None,
            }
        }
    }))
    .await;
    Ok(GameData { game, teams })
}
pub async fn get_team_turns(client: &Client, team_id: i32) -> Result<Vec<Turn>, PgError> {
    let query_str = "\
    SELECT * FROM turns WHERE game_id = $1 ORDER BY turn_id ASC";
    let query = match client.query(query_str, &[&team_id]).await {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    let mut turns: Vec<Turn> = Vec::new();
    for row in query {
        let turn = Turn {
            turn_id: row.get(0),
            start_time: row.get(1),
            team_id: row.get(2),
            game_id: row.get(3),
            dice1: row.get(4),
            dice2: row.get(5),
            finished: row.get(6),
            end_time: row.get(7),
            drinks: get_turn_drinks(client, row.get(0))
                .await
                .unwrap_or_else(|_| Drinks { drinks: Vec::new() }),
        };
        turns.push(turn);
    }
    Ok(turns)
}
pub async fn get_turn_drinks(client: &Client, turn_id: i32) -> Result<Drinks, PgError> {
    let query_str = "\
    SELECT \
        td.drink_id \
    FROM turn_drinks as td\
     WHERE td.turn_id = $1";
    let query = match client.query(query_str, &[&turn_id]).await {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    let drink_ids: Vec<i32> = query.iter().map(|row| row.get(0)).collect();
    let drinks: Vec<Drink> = get_drinks(client)
        .await?
        .drinks
        .into_iter()
        .filter(|drink| drink_ids.contains(&drink.id))
        .collect();

    Ok(Drinks { drinks })
}
