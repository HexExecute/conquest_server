use axum::{extract::{ws::WebSocket, WebSocketUpgrade}, response::IntoResponse};
use crate::game::Game;

#[derive(Clone)]
pub struct SocketHandler<'a> {
    game: &'a Game
}

impl<'a> SocketHandler<'a> {
    pub fn new(game: &Game) -> Self {
        Self { game }
    }

    pub async fn ws_handler(ws: WebSocketUpgrade, game: &Game) -> impl IntoResponse {
        ws.on_upgrade(move |socket| SocketHandler::new(game).handle_socket(socket))
    }
    
    async fn handle_socket(&self, mut socket: WebSocket) {
        while let Some(msg) = socket.recv().await {
            let msg = if let Ok(msg) = msg {
                msg
            } else { return; };

            if socket.send(msg).await.is_err() { return; }
        }
    }
}
