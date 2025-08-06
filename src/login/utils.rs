use axum::extract::State;
use axum::Json;
use crate::utils::state::{AppError, AppState};
use crate::utils::types::{LoginInfo, UserInfo};
use crate::database::login::post_login_db;

pub async fn post_login(
    state: State<AppState>,
    Json(login): Json<LoginInfo>,
) -> Result<Json<UserInfo>, AppError> {
    println!("POST /login");
    let client = state.db.get().await?;
    match post_login_db(login, &client).await {
        Ok(user) => {
            if user.uid == -404 {
                Err(AppError::Unauthorized("Unauthorized".parse().unwrap()))
            } else {
                Ok(user)
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            Err(AppError::Database("The server encountered an unexpected error!".parse().unwrap()))
        }
    }
}