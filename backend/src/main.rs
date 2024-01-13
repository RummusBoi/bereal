use std::net::SocketAddr;

use axum::{extract::WebSocketUpgrade, http::HeaderMap, response::Response, routing::get, Router};
use backend::database::sql_helpers::ensure_tables_exist;
use socket_handlers::top_level_handler::top_level_socket_handler;

mod database;
mod general_helpers;
mod socket_handlers;
extern crate my_sqlx_crud;
extern crate my_sqlx_crud_macro;
async fn handle_new_socket_conn(headers: HeaderMap, ws: WebSocketUpgrade) -> Response {
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
    return ws.on_upgrade(move |socket| top_level_socket_handler(socket, user_id));
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
