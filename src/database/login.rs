use axum::Json;
use tokio_postgres::Client;
use crate::utils::state::AppError;
use crate::utils::types::{LoginInfo, PgError, UserInfo};

pub async fn post_login_db(login_info: LoginInfo, client: &Client) -> Result<Json<UserInfo>, PgError> {
    let query_str = "\
    SELECT \
        u.uid, \
        u.username, \
        u.email, \
        ut.user_type \
    FROM users as u \
    LEFT JOIN user_types as ut \
        ON u.uid = ut.uid \
    WHERE u.username = $1 AND u.password = $2";

    let query = match client.query(query_str, &[&login_info.username, &login_info.password]).await {
        Ok(r) => r,
        Err(e) => return Err(e)
    };
    let mut user: UserInfo = UserInfo {
        uid: -404,
        username: "NOT FOUND".to_string(),
        email: "NOT.FOUND@tietokilta.fi".to_string(),
        user_types: Vec::new(),
    };
    for row in query {
        match row.try_get::<usize, i32>(0) {
            Ok(_) => {
                user.uid = row.get(0);
                user.username = row.get(1);
                user.email = row.get(2);
                user.user_types.push(row.get(3));
            }
            Err(_) => continue,
        }
    }
    Ok(Json(user))
}