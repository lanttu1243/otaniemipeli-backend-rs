use crate::api::referee::get_db_client;
use crate::database::games::{end_turn, get_games, get_team_data};
use crate::utils::socket::check_auth;
use crate::utils::state::AppState;
use crate::utils::types::{Games, SocketAuth, UserType};
use socketioxide::adapter::Adapter;
use socketioxide::extract::{Data, SocketRef, State};

pub async fn secretary_on_connect<A: Adapter>(
    auth: Data<SocketAuth>,
    s: SocketRef<A>,
    State(state): State<AppState>,
) {
    let token = auth.token.clone();
    match check_auth(&token, &s, &state, UserType::secretary).await {
        true => {}
        false => {
            let _ = s.disconnect();
            return;
        }
    }
    s.on(
        "verify-login",
        |s: SocketRef<A>, Data(auth): Data<SocketAuth>, State(state): State<AppState>| async move {
            let token = auth.token.clone();
            match check_auth(&token, &s, &state, UserType::secretary).await {
                true => {}
                false => {
                    let _ = s.disconnect();
                    return;
                }
            }
        },
    );
    s.on(
        "finish-turn",
        |s: SocketRef<A>, Data(turn_id): Data<i32>, State(state): State<AppState>| async move {
            let client = match state.db.get().await {
                Ok(c) => c,
                Err(e) => {
                    let _ = s.emit("response-error", &format!("db pool error: {e}"));
                    return;
                }
            };
            match end_turn(&client, turn_id).await {
                Ok(turn) => {
                    if let Err(send) = s.emit("response", &turn) {
                        eprintln!("send error: {send}");
                    } else {
                        s.emit("turn-end-reply", &turn)
                            .expect("Failed replying turn");
                    }
                }
                Err(e) => {
                    let _ = s.emit("response-error", &format!("db error: {e}"));
                }
            }
        },
    );
    s.on("get-games", |s: SocketRef<A>| async move {
        let client = match crate::api::referee::get_db_client(&state, &s).await {
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
