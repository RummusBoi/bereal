use axum::{
    extract::Query,
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    http::HeaderMap,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use futures_util::{
    future::select, select, sink::SinkExt, stream::StreamExt, FutureExt, Stream, TryFutureExt,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::write,
    fs,
    net::SocketAddr,
    thread,
    time::{Duration, UNIX_EPOCH},
};
use std::{sync::Arc, time::SystemTime};
use tokio::sync::{
    broadcast::{self, Receiver},
    futures, Mutex,
};
use tokio_stream::wrappers::BroadcastStream;
use types::{AppError, AppResponse, Message, User};
use websockets::handler::handler;

use crate::websockets::client::websocket_handler;

mod db_helpers;
mod helpers;
mod types;
mod websockets;

#[derive(Clone)]
enum AppData {
    SingleMessage(Message),
    MultipleMessages(Vec<Message>),
    Users(Vec<String>),
    SingleUser(User),
}

impl Serialize for AppData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            AppData::SingleMessage(message) => message.serialize(serializer),
            AppData::MultipleMessages(messages) => messages.serialize(serializer),
            AppData::Users(users) => users.serialize(serializer),
            AppData::SingleUser(user) => user.serialize(serializer),
        }
    }
}

#[derive(Deserialize)]
struct Params {
    exclude_read_messages: Option<bool>,
}

fn get_userid_and_token(headers: &HeaderMap) -> Result<(String, String), AppError> {
    let user_id = match headers.get("user_id") {
        Some(user_id) => user_id.to_str().unwrap().to_string(),
        None => {
            return Err(AppError::NoUserIDProvided(
                "Provide UserID in http headers.".to_string(),
            ))
        }
    };

    let token = match headers.get("token") {
        Some(token) => token.to_str().unwrap().to_string(),
        None => {
            return Err(AppError::NoTokenProvided(
                "Provide Token in http headers.".to_string(),
            ))
        }
    };

    Ok((user_id, token))
}

#[derive(Clone)]
pub struct AppState {
    sender: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    let (sender, _) = broadcast::channel(100);

    let mut app_state = AppState { sender: sender };

    let websocket_server = Router::new()
        .route("/websocket", get(handler))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 4500));
    axum::Server::bind(&addr)
        .serve(websocket_server.into_make_service())
        .await
        .unwrap();
}
