use crate::database::games::{get_games, post_game};
use crate::database::login::check_session;
use crate::server::MessageIn;
use crate::utils::state::AppState;
use crate::utils::types::{Games, MessageBack, PostGame, SocketAuth};
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

    if let Err(e) = check_session(token.as_str(), &client).await {
        eprintln!("auth failed for socket {}: {e}", s.id);
        let _ = s.disconnect();
        return; // early return so we don’t continue
    }

    s.on("message", |s: SocketRef<A>, Data::<MessageIn>(data)| { // <-- generic here too
        println!(
            "got 'message' from {}:\n{{\n  message_type: {}\n  content: {}\n  value: {}\n  timestamp: {}\n}}",
            s.id, data.message_type, data.content, data.value, data.timestamp
        );

        let payload = MessageBack {
            ok: true,
            echo: format!(
                "got 'message' from {}:\n{{\n  message_type: {}\n  content: {}\n  value: {}\n  timestamp: {}\n}}",
                s.id, data.message_type, data.content, data.value, data.timestamp
            ),
        };

        if let Err(e) = s.emit("message-back", &payload) {
            eprintln!("emit error to {}: {e}", s.id);
        }
    });
    s.on(
        "start-game",
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
                            .unwrap_or_else(|e| Games { games: Vec::new() });
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
    s.on("get-games", |s: SocketRef<A>| async move {
        let client = match state.db.get().await {
            Ok(c) => c,
            Err(e) => {
                let _ = s.emit("response-error", &format!("db pool error: {e}"));
                return;
            }
        };
        let games = get_games(&client)
            .await
            .unwrap_or_else(|e| Games { games: Vec::new() });
        s.emit("reply-games", &games)
            .expect("Failed replying games");
    })
}
