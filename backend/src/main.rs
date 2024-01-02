use std::net::SocketAddr;

use axum::{extract::WebSocketUpgrade, http::HeaderMap, response::Response, routing::get, Router};
use socket_handlers::top_level_handler::top_level_socket_handler;

mod database;
mod general_helpers;
mod socket_handlers;

async fn handle_new_socket_conn(headers: HeaderMap, ws: WebSocketUpgrade) -> Response {
    println!("Received new websocket connection.");
    let user_id = "rasmus".to_string();
    // let user_id = headers
    //     .get("user_id".to_string())
    //     .unwrap()
    //     .to_str()
    //     .unwrap()
    //     .to_string();
    return ws.on_upgrade(|socket| top_level_socket_handler(socket, user_id));
}

#[tokio::main]
async fn main() {
    // let (sender, _) = broadcast::channel(100);
    let websocket_server = Router::new().route("/websocket", get(handle_new_socket_conn));

    let addr = SocketAddr::from(([0, 0, 0, 0], 4500));
    axum::Server::bind(&addr)
        .serve(websocket_server.into_make_service())
        .await
        .unwrap();
}
