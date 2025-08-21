use axum::extract::State;
use axum::http::HeaderMap;
use axum::Json;
use crate::utils::state::{AppError, AppState};
use crate::utils::types::{LoginInfo, SessionInfo, UserSessionInfo};
use crate::database::login::{check_session, post_login_db};

pub async fn post_login(
    state: State<AppState>,
    Json(login): Json<LoginInfo>,
) -> Result<Json<UserSessionInfo>, AppError> {
    println!("POST /login");
    let client = state.db.get().await?;
    match post_login_db(login, &client).await {
        Ok((user, session_hash)) => {
            if user.uid == -404 && session_hash != "" {
                Err(AppError::Unauthorized("Unauthorized".parse().unwrap()))
            } else {
                match check_session(session_hash.as_str(), &client).await {
                    Ok(session) => Ok(Json(UserSessionInfo {
                        user,
                        session
                    })),
                    Err(e) => Err(AppError::Unauthorized("Unauthorized".parse().unwrap())),
                }
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            Err(AppError::Database("The server encountered an unexpected error!".parse().unwrap()))
        }
    }
}
pub async fn verify_session(
    state: State<AppState>,
    headers: HeaderMap,
) -> Result<Json<SessionInfo>, AppError>  {
    println!("UPDATE /login");

    let client = state.db.get().await?;
    let mut auth_hash = String::from("");

    match headers.get(axum::http::header::AUTHORIZATION) {
        Some(auth) => {
            auth_hash = auth.to_str().unwrap().to_string();
            println!("got auth header: {}", auth_hash);
        }
        _ => {
            return Err(AppError::Unauthorized("Unauthorized".parse().unwrap()))
        }
    }

    match check_session(auth_hash.as_str(), &client).await {
        Ok(session) => {
            Ok(Json(session))
        }
        Err(e) => {
            eprintln!("{}", e);
            Err(AppError::Database("The server encountered an unexpected error!".parse().unwrap()))
        }
    }
}