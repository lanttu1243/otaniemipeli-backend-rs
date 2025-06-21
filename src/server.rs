extern crate rouille;
extern crate postgres;
#[macro_use]

use postgres::{Client, NoTls};
use dotenvy::dotenv;
use std::env;
use rouille::{router, RequestBody};
use rouille::{Request, Response};
use crate::db_connection::*;
use super::types::*;
use std::io::Read;

pub fn start(port: u16) {
    let conn_str = build_connection_string();
    println!("{:?}", conn_str);
    println!("\nServer started at port {}", port);
    
    rouille::start_server(format!("0.0.0.0:{}", port), move |request: &Request| {
        router!(request,
            (GET) (/) => {
                println!("GET /");
                Response::text("The backend for Otaniemipeli is up and running...")
            },

            (GET) (/games) => {
                println!("GET /games");
                
                let client = match Client::connect(&conn_str, NoTls) {
                    Ok(c) => {
                        println!("Successfully connected to Otaniemipeli server");
                        c
                    },
                    Err(e) => 
                    return Response::text(format!("DB connection error: {}", e)).with_status_code(500),
                };
                
                let result = get_games(client);
                if !result.is_ok() {
                    panic!("Connection to database failed! \n\t{}", result.err().unwrap());
                }
                let games: Games = result.unwrap();
                if games.games.is_empty() {
                    return Response::text("There are no games!").with_status_code(400)
                }
                
                let json = serde_json::to_string(&games).unwrap();
                
                Response::from_data("application/json", json)
            },
            
            (POST) (/games) => {
                println!("POST /games");
                
                let client = match Client::connect(&conn_str, NoTls) {
                    Ok(c) => {
                        println!("Successfully connected to Otaniemipeli server");
                        c
                    },
                    Err(e) => {
                        eprintln!("Failed to connect to Otaniemipeli server! \n\t{}", e);
                        return Response::text("DB connection error").with_status_code(500)
                    }
                };
                
                let mut data: RequestBody = request.data().expect("Error retrieving request");
                let mut buf = Vec::new();
                match data.read_to_end(&mut buf) {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Error reading response: {}", e);
                        return Response::text("Error reading request").with_status_code(500)
                    }
                }
                
                let game: PostGame = match serde_json::from_slice(&buf) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("JSON parse error: {e}\nPayload: {}", String::from_utf8_lossy(&buf));
                        return Response::text("Error parsing JSON-body").with_status_code(500);
                    }
                };
                
                match post_game(client, game) {
                    Err(e) => {
                        eprintln!("{}", e);
                        return Response::text("Error in DB operations").with_status_code(500)
                    }
                _ => {}};
                Response::text("Game added successfully!").with_status_code(201)
            },

            // Fallback response
            _ => Response::empty_404()
        )
    });
}
fn build_connection_string() -> String {
    dotenv().ok(); // Loads from .env file
    let keys: Vec<&str> = Vec::from(
        ["USER",
            "PASSWORD",
            "DBNAME",
            "HOST",
            "PORT"]
    );
    let mut result = String::new();
    for key in keys {
        result = format!("{}{}={} ", result, key.to_ascii_lowercase(), get_env(key));
        println!("SUCCESS!: '{}'", result);
    }
    result.pop();
    println!("{}", result);
    result
}
fn get_env(key: &str) -> String {
    let new_key = "POSTGRES_".to_string() + key;
    println!("Getting environment variable {}", new_key);
    env::var(new_key).unwrap_or_else(|_| panic!("Environment variable `{}` not set", key))
}