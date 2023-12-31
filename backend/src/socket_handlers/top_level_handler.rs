use axum::extract::ws::WebSocket;

use super::on_subscribe::on_subscribe;

pub async fn top_level_socket_handler(socket: WebSocket, user_id: String) {
    on_subscribe(&socket, &user_id);
}
