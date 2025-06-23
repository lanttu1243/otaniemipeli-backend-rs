#![allow(unreachable_code)]

use axum::Router;
use axum::serve::Serve;
use tokio::net::TcpListener;

mod server;
mod api;
mod utils;
mod database;

#[tokio::main]
async fn main() {
    server::start().await.unwrap()
}