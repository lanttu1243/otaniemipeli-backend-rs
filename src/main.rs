#![allow(unreachable_code)]

use std::env;

mod server;
mod db_connection;
mod types;

fn main() {
    // Search if PORT is in .env. If not default to port 8000
    let port = env::var("PORT").unwrap_or_else(|_| String::from("8000"));
    server::start(port)
}