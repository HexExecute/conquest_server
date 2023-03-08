use std::{sync::{Arc, Mutex}, net::SocketAddr};

use axum::{Router, Extension, routing::get};
use crate::game::Game;

use self::websocket::SocketHandler;

mod websocket;

#[derive(Clone)]
struct State;

pub struct APIHandler;

impl APIHandler {
    pub async fn new(game: Arc<Mutex<Game>>, address: SocketAddr) -> Self {
        let app: Router = Router::new()
                .route("/ws", get(|ws| SocketHandler::ws_handler(ws, game)))
                .layer(Extension(State));

        axum::Server::bind(&address)
            .serve(app.into_make_service())
            .await
            .unwrap();

        Self
    }
}
