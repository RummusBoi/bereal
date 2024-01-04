use axum::extract::ws::WebSocket;

use super::on_subscribe::on_subscribe;

pub async fn top_level_socket_handler(mut socket: WebSocket, user_id: String) {
    on_subscribe(&mut socket, &user_id).await;
    socket.close().await;
}
