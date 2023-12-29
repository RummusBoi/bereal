use std::{sync::Arc, thread, time::Duration};

use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    http::HeaderMap,
    response::Response,
};
use futures_util::{future::select, SinkExt, StreamExt};
use tokio::sync::{broadcast::Receiver, Mutex};

use crate::{
    db_helpers::messages::read_all_messages_from_file,
    get_userid_and_token,
    types::AppResponse,
    websockets::{
        client::websocket_handler,
        types::{SocketData, SocketEventType, SocketResponse},
    },
    AppState,
};

pub async fn handle_socket(
    mut socket: WebSocket,
    app_state: AppState,
    user_id: String,
    token: String,
) {
    println!("Sending message");
    let all_messages = match read_all_messages_from_file() {
        Ok(messages) => messages,
        Err(app_error) => {
            println!("{app_error:?}");
            socket.close();
            return;
        }
    };
    let socket_data = SocketResponse {
        data_type: SocketEventType::InitialMessages,
        data: SocketData::MessageVec(all_messages),
    };
    socket
        .send(axum::extract::ws::Message::Text(
            serde_json::to_string(&socket_data).unwrap(),
        ))
        .await;
    println!("Sent");

    // let broadcast_sender = Arc::new(Mutex::new(app_state.sender));
    let broadcast_receiver = app_state.sender.subscribe();
    let broadcast_sender = Arc::new(Mutex::new(app_state.sender.clone()));
    let (user_sender, user_receiver) = socket.split();
    let user_sender_mut = Arc::new(Mutex::new(user_sender));

    async fn broadcast_handler(
        mut broadcast_receiver: Receiver<String>,
        user_sender_mut: Arc<
            Mutex<::futures::prelude::stream::SplitSink<WebSocket, axum::extract::ws::Message>>,
        >,
    ) {
        println!("Starting loop in broadcast receiver");
        while let Ok(content) = broadcast_receiver.recv().await {
            let socket_resp = match serde_json::from_str::<SocketResponse>(&content) {
                Ok(content) => content,
                Err(error) => {
                    println!("{error}");
                    return;
                }
            };

            match socket_resp.data_type {
                SocketEventType::InitialMessages => todo!(),
                SocketEventType::MessagePosted => {
                    let message = match socket_resp.data {
                        SocketData::MessageVec(_) => todo!(),
                        SocketData::MessagePosted(message) => message,
                        SocketData::NewMessage(_) => todo!(),
                    };
                    println!("Received content {content}");

                    let socket_resp = SocketResponse {
                        data_type: SocketEventType::MessagePosted,
                        data: SocketData::MessagePosted(message),
                    };

                    let serialized_response = serde_json::to_string(&socket_resp).unwrap();
                    let mut m_lock = user_sender_mut.lock().await;
                    let write_future =
                        m_lock.send(axum::extract::ws::Message::Text(serialized_response));
                    if write_future.await.is_err() {
                        println!("Failed while getting write result");
                        break;
                    }
                }
                SocketEventType::PostMessage => todo!(),
            }
        }
    }

    let user_sender_mut_clone = user_sender_mut.clone();
    let broadcast_sender_mut_clone = broadcast_sender.clone();

    select(
        Box::pin(websocket_handler(
            user_receiver,
            broadcast_sender_mut_clone,
            &user_id,
            &token,
        )),
        Box::pin(broadcast_handler(broadcast_receiver, user_sender_mut_clone)),
    )
    .await;
    // t.join();
}

pub async fn handler(
    headers: HeaderMap,
    ws: WebSocketUpgrade,
    State(app_state): State<AppState>,
) -> Response {
    let (user_id, token) = match get_userid_and_token(&headers) {
        Ok(result) => result,
        Err(error) => return AppResponse::from(error).into(),
    };
    println!("User '{}'authenticated", user_id);
    ws.on_upgrade(|socket| handle_socket(socket, app_state, user_id, token))
}
