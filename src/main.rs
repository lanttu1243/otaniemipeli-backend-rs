#![allow(unreachable_code)]

use axum::serve::Serve;
use axum::Router;
use tokio::net::TcpListener;

mod api;
mod database;
mod login;
mod server;
mod utils;

#[tokio::main]
async fn main() {
    server::start().await.unwrap()
}
