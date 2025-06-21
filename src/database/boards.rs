use deadpool_postgres::{Client};
use crate::utils::types::*;
pub async fn get_boards(client: Client) -> Result<Boards, PgError> {

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