use std::env;
use rouille::{router, Request, Response};
use crate::api::router as api_router;
use crate::database::utils::make_pool;
use tokio::task;


pub async fn start() -> anyhow::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| String::from("8000"));
    let db_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => panic!("DATABASE_URL must be set"),
    };
    let pool = make_pool(&db_url)?;
    
    match task::spawn_blocking(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Could not build tokio runtime");
        
        let _guard = rt.enter();
        
        let bind = format!("0.0.0.0:{}", port);
        println!("\nServer started at port {}", port);
        
        rouille::start_server(bind, move |request: &Request| {
            router!(request,
                (GET) (/) => {
                    println!("GET /");
                    Response::text("The backend for Otaniemipeli is up and running...")
                },
                _ => api_router(pool.clone())(request)
            )
        });
    })
    .await {
        Ok(res) => res,
        Err(e) => Err(anyhow::Error::new(e))
    }
}