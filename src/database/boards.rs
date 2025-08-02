use deadpool_postgres::{Client};
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
                        connections: match get_board_place_connections(&client, board_id, row.get(5)).await {
                            Ok(r) => r,
                            Err(e) => return Err(e),
                        },
                        drinks: {
                            match get_place_drinks(&client, row.get(5), board_id).await {
                                Ok(r) => r,
                                Err(e) => return Err(e),
                            }
                        }
                    }
                )
            }
            Err(_) => continue,
        }
    }
    Ok(board_places)
}
pub async fn get_place_drinks(client: &Client, place_number: i32, board_id: i32) -> Result<PlaceDrinks, PgError> {
    let mut drinks: PlaceDrinks = PlaceDrinks { drinks: Vec::new() };
    let query_str = "\
    SELECT \
        pd.place_number, \
        pd.board_id, \
        pd.drink_id, \
        d.name, \
        pd.refill, \
        pd.optional, \
        pd.n, \
        pd.n_update \
    FROM place_drinks AS pd \
    LEFT JOIN drinks AS d \
        ON d.drink_id = pd.drink_id \
    WHERE pd.place_number = $1 AND pd.board_id = $2";

    let query = match client.query(query_str, &[&place_number, &board_id]).await {
        Ok(r) => r,
        Err(e) => return Err(e)
    };

    for row in query {
        match row.try_get::<usize, i32>(0) {
            Ok(_) => {
                drinks.drinks.push(
                    PlaceDrink {
                        place_number: row.get(0),
                        board_id: row.get(1),
                        drink: Drink {
                            id: row.get(2),
                            name: row.get(3),
                        },
                        refill: row.get(4),
                        optional: row.get(5),
                        n: row.get(6),
                        n_update: row.get(7),
                    }
                )
            }
            Err(_) => continue,
        }
    }
    Ok(drinks)
}
pub async fn add_place_drinks(client: &Client, drinks: PlaceDrinks) -> Result<u64, PgError> {

    let query_str = "\
    INSERT INTO place_drinks (drink_id, place_number, board_id, refill, optional, n, n_update) \
    VALUES ($1, $2, $3, $4, $5, $6, $7)";

    for drink in &drinks.drinks {
        client.execute(query_str, &[&drink.drink.id, &drink.place_number, &drink.board_id, &drink.refill, &drink.optional, &drink.n, &drink.n_update]).await?;
    }
    Ok(drinks.drinks.len() as u64)
}
pub async fn get_board_place_connections(client: &Client, board_id: i32, place_number: i32) -> Result<Vec<Connection>, PgError> {
    let mut connections: Vec<Connection> = Vec::new();
    let query_str = "\
    SELECT \
        board_id, \
        origin, \
        target, \
        on_land, \
        backwards, \
        dashed \
    FROM place_connections \
    WHERE board_id = $1  AND origin = $2 \
    ORDER BY target";

    let query = match client.query(query_str, &[&board_id, &place_number]).await {
        Ok(r) => r,
        Err(e) => return Err(e)
    };
    for row in query {
        match row.try_get::<usize, i32>(0) {
            Ok(_) => {
                connections.push(
                    Connection {
                        board_id: row.get(0),
                        origin: row.get(1),
                        target: row.get(2),
                        on_land: row.get(3),
                        backwards: row.get(4),
                        dashed: row.get(5),
                    }
                )
            }
            Err(e) => return Err(e),
        }
    }
    Ok(connections)
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