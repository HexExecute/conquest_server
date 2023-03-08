use std::{sync::{ Arc, Mutex }, net::SocketAddr};

use config::Config;
use game::Game;
use api::APIHandler;

mod game;
mod config;
mod api;
mod bot;

#[tokio::main]
async fn main() {
    let config = Config::new();
    let mut game = Arc::new(Mutex::new(Game::new(config.game.clone())));
    println!("game initialized");

    game.lock().unwrap().update();

    let mut api_handler = APIHandler::new(
        Arc::clone(&game),
        SocketAddr::from(([127,0,0,1], 3000))
    ).await;
    println!("api initialized");
}
