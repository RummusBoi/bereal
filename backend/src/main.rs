use std::net::SocketAddr;

use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    http::HeaderMap,
    response::Response,
    routing::get,
    Router, Server,
};
use serde::Serialize;
use socket_handlers::top_level_handler::top_level_socket_handler;
use tokio::sync::broadcast;

use crate::database::{comment_controller::read_comments, user_controller::read_users};

mod database;
mod general_helpers;
mod socket_handlers;
// fn main() {
//     // let users = read_users(vec!["rasmus".to_string(), "jonathan".to_string()]);
//     // println!("{users:?}");
//     // let images = read_images(vec!["rasmus_img".to_string()]);
//     // println!("{images:?}");
//     let comments = read_comments(vec!["1".to_string()]);
//     println!("{comments:?}");
//     let comments = read_comments(vec!["1".to_string()]);
//     println!("{comments:?}");
// }

async fn handle_new_socket_conn(headers: HeaderMap, ws: WebSocketUpgrade) -> Response {
    let user_id = headers
        .get("user_id".to_string())
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    return ws.on_upgrade(|socket| top_level_socket_handler(socket, user_id));
}

#[tokio::main]
async fn main() {
    // let (sender, _) = broadcast::channel(100);

    let websocket_server = Router::new().route("/websocket", get(handle_new_socket_conn));
    // maybe we need app_state later
    // .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 4500));
    axum::Server::bind(&addr)
        .serve(websocket_server.into_make_service())
        .await
        .unwrap();
}
