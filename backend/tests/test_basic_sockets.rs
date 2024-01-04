use setup_socket_conn::connect_to_localhost;

use backend::{self, socket_handlers::types::SocketResponse};
mod setup_socket_conn;

#[test]
fn test_on_subscribe() {
    let mut socket = connect_to_localhost();
    let message = socket.read().unwrap();

    let socket_resp = match message {
        tungstenite::Message::Text(data) => {
            serde_json::from_slice::<SocketResponse>(data.as_bytes()).unwrap()
        }
        tungstenite::Message::Binary(_) => todo!(),
        tungstenite::Message::Ping(_) => todo!(),
        tungstenite::Message::Pong(_) => todo!(),
        tungstenite::Message::Close(_) => todo!(),
        tungstenite::Message::Frame(_) => todo!(),
    };
    println!("{:?}", socket_resp);
}