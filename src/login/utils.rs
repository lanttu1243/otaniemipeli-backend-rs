use axum::extract::State;
use axum::http::HeaderMap;
use axum::Json;
use crate::utils::state::{AppError, AppState};
use crate::utils::types::{LoginInfo, SessionInfo, UserSessionInfo};
use crate::database::login::{check_session, delete_all_sessions, delete_session, post_login_db};

async fn get_auth(headers: &HeaderMap) -> String {
    match headers.get(axum::http::header::AUTHORIZATION) {
        Some(auth) => {
            auth.to_str().unwrap().to_string()
        }
        _ => {
            "".to_string()
        }
    }
}

pub async fn start_session(
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
    let auth_hash = get_auth(&headers).await;
    if auth_hash == "" {
        return Err(AppError::Unauthorized("There is no authorization header".parse().unwrap()))
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
pub async fn end_session(
    state: State<AppState>,
    headers: HeaderMap,
) -> Result<Json<()>, AppError> {
    println!("DELETE /login");
    let client = state.db.get().await?;
    let auth_hash = get_auth(&headers).await;
    if auth_hash == "" {
        return Err(AppError::Unauthorized("There is no authorization header".parse().unwrap()))
    };

    match delete_session(auth_hash.as_str(), &client).await {
        Ok(_) => Ok(Json(())),
        Err(_) => Err(AppError::Database("The server encountered an unexpected error!".parse().unwrap()))
    }
}
pub async fn end_all_sessions(
    state: State<AppState>,
    headers: HeaderMap
) -> Result<Json<()>, AppError> {
    println!("DELETE /logout");
    let client = state.db.get().await?;
    let auth_hash = get_auth(&headers).await;
    if auth_hash == "" {
        return Err(AppError::Unauthorized("There is no authorization header".parse().unwrap()))
    };
    match check_session(auth_hash.as_str(), &client).await {
        Ok(session) => {
            match delete_all_sessions(session.uid, &client).await {
                Ok(_) => Ok(Json(())),
                Err(_) => Err(AppError::Database("The server encountered an unexpected error!".parse().unwrap()))
            }
        },
        Err(_) => Err(AppError::Database("The server encountered an unexpected error!".parse().unwrap()))
    }
}