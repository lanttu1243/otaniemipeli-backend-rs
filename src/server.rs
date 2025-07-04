use std::env;
use crate::api::router as api_router;
use crate::database::utils::make_pool;
use tokio::{
    net::TcpListener,
};
use axum::{
    routing::get,
    Router,
    serve::Serve,
};
use tracing_subscriber::FmtSubscriber;

use crate::utils::state;

pub async fn start() -> anyhow::Result<()> {

    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    dotenvy::dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| {
        eprintln!("PORT environment variable not set");
        "8000".to_string()
    });
    let db_url = match env::var("POSTGRES_URL") {
        Ok(url) => url,
        Err(_) => panic!("POSTGRES_URL must be set"),
    };
    let pool = match make_pool(&db_url) {
        Ok(pool) => pool,
        Err(_) => panic!("Failed to create pool"),
    };

    let bind = format!("0.0.0.0:{}", port);
    println!("\nServer started at port {}", port);

    let state = state::AppState::new(pool);

    let app = Router::new()
        .route("/", get(|| async { "The backend for Otaniemipeli is up and running..." }))
        .nest("/api", api_router())
        .with_state(state);

    let listener = match TcpListener::bind(&bind).await {
        Ok(listener) => listener,
        Err(error) => panic!("Could not bind to {}: {}", bind, error),
    };

    axum::serve(listener, app).await?;
    Ok(())
}