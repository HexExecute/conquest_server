use std::sync::Arc;

use axum::{Router, Extension, routing::get};
use crate::game::Game;

use self::websocket::SocketHandler;

mod websocket;

#[derive(Clone)]
struct State;

pub struct APIHandler {
    app: Router
}

impl APIHandler {
    pub fn new(game: Arc<Game>) -> Self {
        Self {
            app: Router::new()
                .route("/ws", get(|ws| SocketHandler::ws_handler(ws, game)))
                .layer(Extension(State))
        }
    }

    pub fn init(&self) {
        //...
    }
}
