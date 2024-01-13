use backend::{self, socket_handlers::types::SocketResponse};
use common::setup_database::setup_database;

use crate::common::setup_socket_conn::connect_to_localhost;
mod common;

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
    println!("Hvad så negere!");
}

#[test]
fn test_can_create_simple_friend_group() {
    let posts = setup_database();

    assert!(posts.len() == 6);
}
