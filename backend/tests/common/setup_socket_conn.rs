use std::str::FromStr;

use tungstenite::handshake::client::generate_key;
use tungstenite::http::Uri;
use tungstenite::{
    connect,
    http::{self},
};

pub fn connect_to_localhost(
    user_id: i32,
) -> tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>> {
    let uri = Uri::from_str("ws://localhost:4500/websocket").unwrap();
    let request = http::request::Builder::new()
        .uri(uri.to_string())
        .header("user_id", user_id)
        .header("Sec-WebSocket-Key", generate_key())
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", 13)
        .header("Host", uri.host().unwrap())
        .body(())
        .unwrap();
    let (mut socket, _) = connect(request).expect("Can't connect");
    return socket;
}
