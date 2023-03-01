use std::sync::Arc;

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
    let mut game = Game::new(config.game.clone());
    let mut api_handler = APIHandler::new(Arc::new(game));
    println!("Game initialized");

    game.update();
}
