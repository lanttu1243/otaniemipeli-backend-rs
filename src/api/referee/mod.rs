use crate::database::drinks::get_drinks_ingredients;
use crate::database::games::{get_games, get_team_data, post_game, start_game};
use crate::database::login::check_session;
use crate::database::team::{create_team, get_teams};
use crate::utils::state::AppState;
use crate::utils::types::{FirstTurnPost, Games, PostGame, SocketAuth, Team, Teams, UserType};
use deadpool_postgres::Client;
use socketioxide::adapter::Adapter;
use socketioxide::extract::{Data, SocketRef, State};

pub async fn referee_on_connect<A: Adapter>(
    auth: Data<SocketAuth>,
    s: SocketRef<A>,
    State(state): State<AppState>,
) {
    let token = auth.token.clone();

    let client = match state.db.get().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("db pool error: {e}");
            let _ = s.disconnect();
            return;
        }
    };

    match check_session(token.as_str(), &client).await {
        Ok(session) => {
            s.emit("authorized", &session)
                .expect("Failed sending authorized");
        }
        Err(e) => {
            eprintln!("auth failed for socket {}: {e}", s.id);
            s.emit("unauthorized", "invalid token")
                .expect("Failed sending unauthorized");
            let _ = s.disconnect();
            return;
        }
    };
    s.on(
        "verify-login",
        |s: SocketRef<A>, Data(auth): Data<SocketAuth>, State(state): State<AppState>| async move {
            let token = auth.token.clone();
            let client = match state.db.get().await {
                Ok(c) => c,
                Err(e) => {
                    s.emit("unauthorized", "db error")
                        .expect("Failed sending unauthorized");
                    let _ = s.disconnect();
                    return;
                }
            };
            match check_session(token.as_str(), &client).await {
                Ok(session) => {
                    if session
                        .user_types
                        .user_types
                        .iter()
                        .all(|t| *t != UserType::referee)
                    {
                        s.emit("unauthorized", "Wrong user type")
                            .expect("Failed sending unauthorized");
                        let _ = s.disconnect();
                        return;
                    }
                    s.emit("authorized", &session)
                        .expect("Failed sending authorized");
                }
                Err(e) => {
                    eprintln!("auth failed for socket {}: {e}", s.id);
                    s.emit("unauthorized", "invalid token")
                        .expect("Failed sending unauthorized");
                    let _ = s.disconnect();
                    return;
                }
            };
        },
    );
    s.on(
        "create-game",
        |s: SocketRef<A>, Data(game): Data<PostGame>, State(state): State<AppState>| async move {
            let client = match state.db.get().await {
                Ok(c) => c,
                Err(e) => {
                    let _ = s.emit("response-error", &format!("db pool error: {e}"));
                    return;
                }
            };

            match post_game(&client, game).await {
                Ok(game) => {
                    if let Err(send) = s.emit("response", &game) {
                        eprintln!("send error: {send}");
                    } else {
                        let games = get_games(&client)
                            .await
                            .unwrap_or_else(|_| Games { games: Vec::new() });
                        s.emit("reply-games", &games)
                            .expect("Failed replying games");
                    }
                }
                Err(e) => {
                    let _ = s.emit("response-error", &format!("db error: {e}"));
                }
            }
        },
    );
    s.on(
        "start-game",
        |s: SocketRef<A>, Data(first_turn): Data<FirstTurnPost>, State(state): State<AppState>| async move {
            let client = match get_db_client(&state, &s).await {
                Some(c) => c,
                None => return,
            };
            match start_game(&client, first_turn).await {
                Ok(game) => {
                    s.emit("reply-game", &game).expect("Failed replying game");
                }
                Err(e) => {
                    s.emit("response-error", &format!("db error: {e}"))
                        .expect("Failed sending error");
                }
            }
        },
    );
    s.on("get-games", |s: SocketRef<A>| async move {
        let client = match get_db_client(&state, &s).await {
            Some(c) => c,
            None => return,
        };
        let games = get_games(&client)
            .await
            .unwrap_or_else(|_| Games { games: Vec::new() });
        s.emit("reply-games", &games)
            .expect("Failed replying games");
    });
    s.on(
        "create-team",
        |s: SocketRef<A>, Data(team): Data<Team>, State(state): State<AppState>| async move {
            let client = match get_db_client(&state, &s).await {
                Some(c) => c,
                None => return,
            };
            let team: Team = match create_team(&client, team).await {
                Ok(team) => team,
                Err(e) => {
                    let _ = s.emit("response-error", &format!("db error: {e}"));
                    return;
                }
            };
            let teams: Teams = match get_teams(&client, team.game_id).await {
                Ok(teams) => Teams { teams },
                Err(e) => {
                    let _ = s.emit("response-error", &format!("db error: {e}"));
                    return;
                }
            };
            s.emit("reply-teams", &teams).expect("Failed replying team");
        },
    );
    s.on(
        "get-teams",
        |s: SocketRef<A>, Data(game_id): Data<i32>, State(state): State<AppState>| async move {
            let client = match get_db_client(&state, &s).await {
                Some(c) => c,
                None => return,
            };
            let teams: Teams = match get_teams(&client, game_id).await {
                Ok(teams) => Teams { teams },
                Err(e) => {
                    let _ = s.emit("response-error", &format!("db error: {e}"));
                    return;
                }
            };
            s.emit("reply-teams", &teams).expect("Failed replying team");
        },
    );
    s.on(
        "get-drinks",
        |s: SocketRef<A>, State(state): State<AppState>| async move {
            let client = match get_db_client(&state, &s).await {
                Some(c) => c,
                None => return,
            };
            match get_drinks_ingredients(&client).await {
                Ok(drinks) => {
                    s.emit("reply-drinks", &drinks)
                        .expect("Failed replying drinks");
                }
                Err(e) => {
                    let _ = s.emit("response-error", &format!("db error: {e}"));
                }
            }
        },
    );
    s.on(
        "game-data",
        |s: SocketRef<A>, Data(game_id): Data<i32>, State(state): State<AppState>| async move {
            let client = match get_db_client(&state, &s).await {
                Some(c) => c,
                None => return,
            };

            match get_team_data(&client, game_id).await {
                Ok(game_data) => {
                    s.emit("reply-game-data", &game_data)
                        .expect("Failed replying game");
                }
                Err(e) => {
                    let _ = s.emit("response-error", &format!("db error: {e}"));
                }
            }
        },
    );
}
async fn get_db_client(state: &AppState, s: &SocketRef<impl Adapter>) -> Option<Client> {
    match state.db.get().await {
        Ok(c) => Some(c),
        Err(e) => {
            let _ = s.emit("response-error", &format!("db pool error: {e}"));
            None
        }
    }
}
