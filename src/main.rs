#![allow(unreachable_code)]

mod server;
mod db_connection;
mod types;

fn main() {
    server::start(8000)
}