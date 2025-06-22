use deadpool_postgres::{Client, Manager, ManagerConfig, Pool};
use rouille::Response;
use tokio_postgres::NoTls;
use crate::utils::runtime::GLOBAL_RT;
use crate::utils::types::PgError;

pub fn make_pool(conn_str: &str) -> anyhow::Result<Pool> {
    let mgr_cfg = ManagerConfig { recycling_method: deadpool_postgres::RecyclingMethod::Fast };
    let mgr = Manager::from_config(conn_str.parse()?, NoTls, mgr_cfg);
    Ok(Pool::builder(mgr).max_size(16).build()?)
}
pub fn db_client(pool: &Pool) -> Result<Client, Response> {
    GLOBAL_RT
        .block_on(pool.get())
        .map_err(|e| {
            eprintln!("DB-pool error: {e}");
            Response::text("Error connecting to DB")
                .with_status_code(500)
        })
}