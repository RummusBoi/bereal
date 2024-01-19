use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{State, WebSocketUpgrade},
    http::HeaderMap,
    response::Response,
    routing::get,
    Router,
};
use socket_handlers::top_level_handler::top_level_socket_handler;
use tokio::sync::{Mutex, RwLock};
use types::AppState;

mod database;
mod general_helpers;
mod socket_handlers;
mod types;
extern crate my_sqlx_crud;
extern crate my_sqlx_crud_macro;
async fn handle_new_socket_conn(
    headers: HeaderMap,
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    println!("Received new websocket connection.");
    // let user_id = 123;
    let user_id = headers
        .get("user_id".to_string())
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        .parse::<i32>()
        .unwrap();

    // let mut connections = state.connections.lock().await;
    // connections.push(user_id.to_string());

    return ws.on_upgrade(move |socket| top_level_socket_handler(socket, user_id, state));
}

#[tokio::main]
async fn main() {
    let state = AppState {
        internal_conns: Arc::new(RwLock::new(Vec::new())),
    };
    // let (sender, _) = broadcast::channel(100);
    let websocket_server = Router::new()
        .route("/websocket", get(handle_new_socket_conn))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 4500));
    axum::Server::bind(&addr)
        .serve(websocket_server.into_make_service())
        .await
        .unwrap();
}
