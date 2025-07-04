use deadpool_postgres::{Client};
use crate::utils::state::AppError;
use crate::utils::types::*;
pub async fn get_boards(client: &Client) -> Result<Boards, PgError> {

    let query_str = "\
    SELECT board_id, name FROM boards;";

    let mut boards: Vec<Board> = Vec::new();

    let query = match client.query(query_str, &[]).await {
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
pub async fn get_board(client: &Client, board_id: i32) -> Result<Board, PgError> {
    let query_str = "\
    SELECT board_id, name FROM boards WHERE board_id = $1";
    let query = match client.query(query_str, &[&board_id]).await {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    if query.len() == 0 {
        return Ok(Board {
            id: -1,
            name: "No Boards!".to_string(),
        })
    }
    Ok(Board {
        id: query.first().unwrap().get(0),
        name: query.first().unwrap().get(1)
    })
    
}