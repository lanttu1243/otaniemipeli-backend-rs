extern crate postgres;

use std::ptr::read;
use postgres::{Client};
use super::types::*;

pub fn get_games(mut client: Client) -> Result<Games, PgError> {

    let query_str = "
    SELECT 
        games.game_id AS game_id,
        games.name AS game_name,
        boards.name AS board_name
    FROM games
    INNER JOIN boards ON games.board = boards.board_id";
    
    let mut games: Vec<GameInfo> = Vec::new();
    
    let query = match client.query(query_str, &[]) {
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

fn get_boards(mut client: Client) -> Result<Boards, PgError> {
    
    let query_str = "\
    SELECT board_id, name FROM boards;";
    
    let mut boards: Vec<Board> = Vec::new();
    
    let query = match client.query(query_str, &[]) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    
    for row in query {
        let board: Board = Board {
            id: row.get(0),
            name: row.get(1),
        };
        boards.push(board);
    }
    Ok(Boards {boards})
    
}

pub fn post_game(mut client: Client, game: PostGame) -> Result<u64, PgError> {
    
    let query_str = "\
    INSERT INTO games (name, board) VALUES ($1, $2)";
    
    match client.execute(query_str, &[&game.name, &game.board]) {
        Ok(v) => Ok(v),
        Err(e) => Err(e),
    }
}