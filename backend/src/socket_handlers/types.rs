use serde::{Deserialize, Serialize};

use crate::database::types::{comment::Comment, image::Image};

#[derive(Serialize, Deserialize, Debug)]
pub enum SocketEventType {
    Error,
    InitialState,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SocketData {
    InitialState(InitialState),
    ErrorMessage(String),
}

#[derive(Debug)]
pub enum AppError {
    UserNotFound(String),
    DatabaseError(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SocketResponse {
    pub data_type: SocketEventType,
    pub data: SocketData,
}

impl SocketResponse {
    pub fn serialize_for_socket(&self) -> axum::extract::ws::Message {
        return axum::extract::ws::Message::Text(serde_json::to_string(self).unwrap());
    }
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

// ---
// TYPES RELATED TO INITIAL STATE
// ---

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostDTO {
    pub id: String,
    pub timestamp: u128,
    pub image: Image,
    pub comments: Vec<Comment>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InitialState {
    pub posts: Vec<PostDTO>,
}
