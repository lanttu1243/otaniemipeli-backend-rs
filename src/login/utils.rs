use axum::extract::State;
use axum::http::HeaderMap;
use axum::Json;
use rand::distr::Alphanumeric;
use rand::Rng;
use sha2::{Digest, Sha256};
use tracing_subscriber::fmt::format;
use crate::utils::state::{AppError, AppState};
use crate::utils::types::{LoginInfo, SessionInfo, UserCreateInfo, UserInfo, UserSessionInfo, UserTypes};
use crate::database::login::{check_session, delete_all_sessions, delete_session, email_exist, post_login_db, update_session, user_create, users_exist};

async fn get_auth(headers: &HeaderMap) -> String {
    match headers.get(http::header::AUTHORIZATION) {
        Some(auth) => {
            auth.to_str().unwrap().to_string()
        }
        _ => {
            "".to_string()
        }
    }
}

fn hash_password(pw: String) -> String {
    let salt: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    println!("{}", salt);
    let pre_hash = format!("{}{}", salt, pw);
    let mut hasher = Sha256::new();
    hasher.update(&pre_hash);
    format!("{salt}{:X}", hasher.finalize())
}
fn compare_pw_to_db(pw_post: String, pw_db: String) -> bool {
    let salt: String = pw_db.chars().take(32).collect();
    let mut hasher = Sha256::new();
    hasher.update(&format!("{salt}{pw_post}"));
    let pw_hash = format!("{salt}{:X}", hasher.finalize());
    pw_hash == pw_db
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
pub async fn exist_users(state: State<AppState>) -> Result<Json<bool>, AppError> {
    let client = state.db.get().await?;
    match users_exist(&client).await {
        Ok(bool) => Ok(Json(bool)),
        Err(_) => Err(AppError::Database("The server encountered an unexpected error!".parse().unwrap()))
    }
}
pub async fn create_user(
    state: State<AppState>,
    headers: HeaderMap,
    Json(user_info): Json<UserCreateInfo>
) -> Result<Json<UserSessionInfo>, AppError> {
    println!("POST /login/create_user");
    let client = state.db.get().await?;
    let auth_hash = get_auth(&headers).await;
    let user_exist = match users_exist(&client).await {
        Ok(bool) => bool,
        Err(_) => return Err(AppError::Database("The server encountered an unexpected error!".parse().unwrap()))
    };
    match email_exist(&client, &user_info.email).await {
        Ok(bool) => {
            if bool {
                return Err(AppError::Conflict(format!("There already exists a user with email {}!", &user_info.email)));
            }
        },
        Err(_) => return Err(AppError::Database("The server encountered an unexpected error!".parse().unwrap()))
    }
    if !user_exist {
        return match user_create(&client, user_info).await {
            Ok((user, session)) => {
                Ok(Json(
                    UserSessionInfo {
                        user,
                        session
                    }
                ))
            },
            Err(_) => Err(AppError::Database("The server encountered an unexpected error!".parse().unwrap()))
        }
    } else if auth_hash != "" {
        let on_session = check_session(&auth_hash, &client).await;
        match on_session {
                Ok(session) => {
                    if !session.user_types.user_types.contains(&user_info.user_type) {
                        return Err(AppError::Unauthorized(format!("You are not authorized to create user with type {}!", &user_info.user_type)));
                    }
                },
                Err(_) => return Err(AppError::Database("The server encountered an unexpected error!".parse().unwrap()))
        };
        return match user_create(&client, user_info).await {
            Ok((user, _)) => {
                Ok(Json(
                    UserSessionInfo {
                        user,
                        session: SessionInfo {
                            uid: -1,
                            session_hash: "".to_string(),
                            user_types: UserTypes {
                                user_types: Vec::new()
                            }
                        }
                    }
                ))
            },
            Err(_) => Err(AppError::Database("The server encountered an unexpected error!".parse().unwrap()))
        }
    } else {
        return Err(AppError::Unauthorized("You are not authorized to perform this!".parse().unwrap()).into())
    }


    Err(AppError::Database("The server encountered an unexpected error!".parse().unwrap()))
}