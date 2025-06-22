use deadpool_postgres::Pool;
use rouille::{
    router,
    Request,
    Response
};
use crate::utils::runtime::GLOBAL_RT;
use crate::database::games::*;
use crate::utils::types::PostGame;

pub fn router(pool: Pool) -> impl Fn(&Request) -> Response + Send + Sync + 'static {
    move |request| {
        router!(request,
            (GET) (/api/games) => {
                println!("GET /games");
                
                let client = match GLOBAL_RT.block_on(pool.get()) {
                    Ok(c) => c,
                    Err(e) => return Response::text(format!("DB error: {e}")).with_status_code(500),
                };

                match GLOBAL_RT.block_on(get_games(client)) {
                    Ok(games) if games.games.is_empty() =>
                        Response::text("There are no games").with_status_code(404),
                    Ok(games)  => Response::json(&games).with_status_code(200),
                    Err(e)     => Response::text(format!("{e}")).with_status_code(500),
                }
            },

            (POST) (/api/games) => {
                println!("POST /games");

                let client = match GLOBAL_RT.block_on(pool.get()) {
                    Ok(c) => c,
                    Err(e) => return Response::text(format!("DB error: {e}")).with_status_code(500),
                };

                let game: PostGame = match rouille::input::json_input(request) {
                    Ok(v)  => v,
                    Err(e) => return Response::text(format!("{e}")).with_status_code(400),
                };

                match GLOBAL_RT.block_on(post_game(client, game)) {
                    Err(e) => {
                        eprintln!("{}", e);
                        return Response::text("Error in DB operations").with_status_code(500)
                    }
                    _ => {}};
                Response::text("Game added successfully!").with_status_code(201)
            },
            _ => Response::empty_404()
        )
    }
}