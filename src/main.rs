use config::Config;
use game::Game;

mod game;
mod config;

#[tokio::main]
async fn main() {
    let config = Config::new();
    let mut game = Game::new(config.game.clone());
    println!("Game initialized");

    game.update();
}
