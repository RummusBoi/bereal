use tungstenite::{connect, handshake::client::Request};
use url::Url;

pub fn connect_to_localhost(
) -> tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>> {
    let request = Url::parse("ws://localhost:4500/websocket").unwrap();

    let (mut socket, _) = connect(request).expect("Can't connect");
    return socket;
}
