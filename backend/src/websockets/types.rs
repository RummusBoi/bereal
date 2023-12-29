use super::super::types::Message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum SocketEventType {
    InitialMessages,
    MessagePosted,
    PostMessage,
}

#[derive(Serialize, Deserialize)]
pub enum SocketData {
    MessageVec(Vec<Message>),
    MessagePosted(Message),
    NewMessage(PostMessageModel),
}

#[derive(Serialize, Deserialize)]
pub struct SocketResponse {
    pub data_type: SocketEventType,
    pub data: SocketData,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostMessageModel {
    // receiver: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct PostUserModel {
    pub user_id: String,
    pub token: String,
}
