#![allow(unreachable_code)]
mod server;
mod api;
mod utils;
mod database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server::start().await
}