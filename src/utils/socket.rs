use crate::database::login::check_session;
use crate::utils::state::AppState;
use crate::utils::types::UserType;
use socketioxide::adapter::Adapter;
use socketioxide::extract::SocketRef;

pub async fn check_auth<A: Adapter>(
    token: &str,
    s: &SocketRef<A>,
    state: &AppState,
    user_type: UserType,
) -> bool {
    let client = match state.db.get().await {
        Ok(c) => c,
        Err(e) => {
            return false;
        }
    };

    match check_session(token, &client).await {
        Ok(session) => {
            println!("session: {:?}", session);
            return if session
                .user_types
                .user_types
                .iter()
                .all(|t| t != &user_type)
            {
                eprintln!("auth failed for socket {}: invalid user type", s.id);
                eprintln!("user types: {:?}", session.user_types.user_types);
                s.emit("unauthorized", "invalid user type")
                    .expect("Failed sending unauthorized");
                false
            } else {
                println!("user_types OK");
                s.emit("authorized", &session)
                    .expect("Failed sending authorized");
                true
            };
        }
        Err(e) => {
            eprintln!("auth failed for socket {}: {e}", s.id);
            s.emit("unauthorized", "invalid token")
                .expect("Failed sending unauthorized");
            false
        }
    }
}
