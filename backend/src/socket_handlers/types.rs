use serde::{Deserialize, Serialize};

use crate::database::types::{comment::Comment, image::Image};

#[derive(Serialize, Deserialize)]
pub enum SocketEventType {
    Error,
    InitialState,
}

#[derive(Serialize, Deserialize)]
pub enum SocketData {}

pub enum AppError {
    UserNotFound(String),
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

// ---
// TYPES RELATED TO INITIAL STATE
// ---

// #[derive(Serialize, Clone)]
// pub struct ImageDTO {
//     id: String,
//     timestamp: u128,
//     data: Vec<u8>,
// }

// #[derive(Serialize, Clone)]
// pub struct CommentDTO {
//     id: String,
//     timestamp: u128,
//     poster_id: String,
//     data: String,
// }

#[derive(Serialize, Clone)]
pub struct PostDTO {
    pub id: String,
    pub timestamp: u128,
    pub image: Image,
    pub comments: Vec<Comment>,
}

#[derive(Serialize, Clone)]
pub struct InitialState {
    pub posts: Vec<PostDTO>,
}
