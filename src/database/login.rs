use std::env;
use chrono::prelude::*;
use axum::Json;
use tokio_postgres::Client;
use crate::utils::types::{LoginInfo, PgError, SessionInfo, UserInfo, UserTypes};
use sha2::{Sha256, Digest};

pub async fn post_login_db(login_info: LoginInfo, client: &Client) -> Result<(UserInfo, String), PgError> {
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
        user_types: UserTypes::new(),
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
    let session_hash = create_session(user.uid, client).await.unwrap_or_else(|_| "".to_string());
    Ok((user, session_hash))
}

pub async fn create_session(uid: i32, client: &Client) -> Result<String, PgError> {
    let query_str = "\
    INSERT INTO sessions (uid, session_hash) \
    SELECT u.uid, $2 \
    FROM users as u \
    WHERE u.uid = $1 \
    RETURNING uid";

    let salt = env::var("SALT").unwrap_or_else(|_| {
        eprintln!("SALT environment variable not set");
        "".to_string()
    });

    let session_prehash = format!("{}{}{}", Utc::now(), salt, uid);
    let mut hasher = Sha256::new();
    hasher.update(&session_prehash);
    let session_hash = format!("{:X}", hasher.finalize());

    match client.execute(query_str, &[&uid, &session_hash]).await {
        Ok(_) => Ok(session_hash),
        Err(e) => Err(e),
    }
}
pub async fn update_session(session_hash: &str, client: &Client) -> (Result<u64, PgError>, Result<u64, PgError>) {
    let query_str1 = "\
    UPDATE sessions SET last_active = now(), expires = GREATEST(expires, now() + interval '1 hour') WHERE session_hash = $1";
    let query_str2 = "\
    DELETE FROM sessions WHERE expires < now()";
    (client.execute(query_str2, &[]).await,
    client.execute(query_str1, &[&session_hash]).await)
}
pub async fn check_session(session_hash: &str, client: &Client) -> Result<SessionInfo, PgError> {
    let query_str = "\
    SELECT \
        s.uid, \
        s.session_hash, \
        ut.user_type \
        FROM sessions as s \
         LEFT JOIN user_types as ut ON s.uid = ut.uid \
         WHERE s.session_hash = $1 AND s.expires > now()";

    let update = update_session(session_hash, client).await;

    let query = match client.query(query_str, &[&session_hash]).await {
        Ok(r) => r,
        Err(e) => return Err(e)
    };
    let mut session: SessionInfo = SessionInfo {
        uid: -404,
        session_hash: "".to_string(),
        user_types: UserTypes::new(),
    };
    for row in query {
        match row.try_get::<usize, i32>(0) {
            Ok(_) => {
                session.uid = row.get(0);
                session.session_hash = row.get(1);
                session.user_types.push(row.get(2));
            }
            Err(_) => continue,
        }
    };
    let ses = session.clone();
    Ok(ses)
}
pub async fn delete_session(session_hash: &str, client: &Client) -> Result<u64, PgError> {
    let query_str = "\
    DELETE FROM sessions WHERE session_hash = $1";
    client.execute(query_str, &[&session_hash]).await
}
pub async fn delete_all_sessions(uid: i32, client: &Client) -> Result<u64, PgError> {
    let query_str = "\
    DELETE FROM sessions WHERE uid = $1";
    client.execute(query_str, &[&uid]).await
}