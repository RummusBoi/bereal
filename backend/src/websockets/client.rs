use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::db_helpers::messages::write_message_to_file;
use crate::types::Message;
use crate::websockets::types::{SocketData, SocketEventType, SocketResponse};

use axum::extract::ws::WebSocket;
use tokio::sync::{broadcast, Mutex};
use tokio_stream::StreamExt;

pub async fn websocket_handler(
    mut user_receiver: ::futures::prelude::stream::SplitStream<WebSocket>,
    broadcast_sender: Arc<Mutex<broadcast::Sender<String>>>,
    user_id: &String,
    token: &String,
) {
    println!("Starting loop in websocket receiver");
    while let Some(msg) = user_receiver.next().await {
        let socket_resp = if let Ok(msg) = msg {
            match msg {
                axum::extract::ws::Message::Text(content) => {
                    match serde_json::from_str::<SocketResponse>(&content) {
                        Ok(deserialized_content) => deserialized_content,
                        Err(error) => {
                            println!("Received invalid data from broadcast socket. {error}");
                            println!("Text Content: {content}");
                            return;
                        }
                    }
                }

                axum::extract::ws::Message::Binary(_) => todo!(),
                axum::extract::ws::Message::Ping(_) => todo!(),
                axum::extract::ws::Message::Pong(_) => todo!(),
                axum::extract::ws::Message::Close(_) => todo!(),
            }
        } else {
            // client disconnected
            return;
        };

        let message = match socket_resp.data_type {
            SocketEventType::PostMessage => match socket_resp.data {
                SocketData::NewMessage(message) => message,
                _ => {
                    println!("Data format in event didn't correspond to the expected format.");
                    return;
                }
            },
            _ => {
                println!("Didn't expect this event type here.");
                return;
            }
        };

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("wtf")
            .as_millis();

        let message_to_write = Message {
            sender: user_id.clone(),
            // receiver: message.receiver,
            message: message.clone().message,
            timestamp: current_time,
            // read: false,
        };

        if let Err(error) = write_message_to_file(&message_to_write) {
            println!("Could not write message to file. {error:?}");
            return;
        };

        // serialize respnse and send it to user
        let socket_resp = SocketResponse {
            data_type: SocketEventType::MessagePosted,
            data: SocketData::MessagePosted(message_to_write),
        };

        let serialized_response = serde_json::to_string(&socket_resp).unwrap();

        let m_lock = broadcast_sender.lock().await;
        let write_future = m_lock.send(serialized_response);
        if write_future.is_err() {
            println!("Failed while getting write result");
        }
    }
}
