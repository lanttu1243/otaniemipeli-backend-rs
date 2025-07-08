use deadpool_postgres::{Client};
use crate::api::v1::boards::places;
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
pub async fn get_places(client: &Client) -> Result<Places, PgError> {

    let query_str = "\
    SELECT place_id, place_name, place_type, rule FROM places;";

    let mut places: Places = Places { places: Vec::new() };

    let query = match client.query(query_str, &[]).await {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    for row in query {
        let place: Place = Place {
            place_id: row.get(0),
            place_name: row.get(1),
            place_type: row.get(2),
            rule: row.get(3),
        };
        places.places.push(place);
    }
    Ok(places)
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

pub async fn get_board_places(client: &Client, board_id: i32) -> Result<BoardPlaces, PgError> {
    let board: Board = get_board(client, board_id).await?;
    let mut board_places: BoardPlaces = BoardPlaces {
        board,
        places: vec![],
    };
    let query_str = "\
    SELECT \
        bp.board_id, \
        p.place_id, \
        p.place_name, \
        p.rule, \
        p.place_type, \
        bp.place_number, \
        bp.start,\
        bp.end, \
        bp.x, \
        bp.y \
    FROM board_places AS bp \
    LEFT JOIN places AS p \
        ON bp.place_id = p.place_id \
    WHERE bp.board_id = $1 \
    ORDER BY bp.place_number";

    let query = match client.query(query_str, &[&board_id]).await {
        Ok(r) => r,
        Err(e) => return Err(e)
    };
    for row in query {
        match row.try_get::<usize, i32>(0) {
            Ok(_) => {
                board_places.places.push(
                    BoardPlace {
                        board_id,
                        place: Place {
                            place_id: row.get(1),
                            place_name: row.get(2),
                            rule: row.get(3),
                            place_type: row.get(4),
                        },
                        place_number: row.get(5),
                        start: row.get(6),
                        end: row.get(7),
                        x: row.get(8),
                        y: row.get(9),
                    }
                )
            }
            Err(e) => return Err(e),
        }
    }
    Ok(board_places)
}
pub async fn add_place(client: &Client, place: Place) -> Result<u64, PgError> {
    let query_str = "\
    INSERT INTO places (place_name, rule, place_type) \
    VALUES ($1, $2, $3)";

    client.execute(query_str, &[&place.place_name, &place.rule, &place.place_type]).await
}
pub async fn add_board_place(client: &Client, board_id: i32, place: BoardPlace) -> Result<u64, PgError> {
    let query_str = "\
    INSERT INTO board_places (board_id, place_number, place_id, start, \"end\", x, y) \
    VALUES ($1, $2, $3, $4, $5, $6, $7)";

    client.execute(query_str, &[&board_id, &place.place_number, &place.place.place_id, &place.start, &place.end, &place.x, &place.y] ).await
}
pub async fn update_coordinates(client: &Client, board_id: i32, place: &BoardPlace) -> Result<u64, PgError> {
    let query_str = "\
    UPDATE board_places SET x = $1, y = $2 WHERE board_id = $3 AND place_number = $4";

    client.execute(query_str, &[&place.x, &place.y, &board_id, &place.place_number]).await
}