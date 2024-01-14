use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::database::types::{comment::Comment, image::Image};

#[derive(Serialize, Deserialize, Debug, Display)]
pub enum SocketEventType {
    Error,
    InitialState,
}

#[derive(Serialize, Deserialize, Debug, Display)]
pub enum SocketData {
    InitialState(InitialState),
    String(String),
}

impl TryFrom<SocketResponse> for InitialState {
    type Error = AppError;

    fn try_from(resp: SocketResponse) -> Result<Self, Self::Error> {
        match resp.data_type {
            SocketEventType::InitialState => (),
            _ => {
                return Err(AppError::IncorrectSocketFormat(format!(
                    "Expected 'InitialState' event but got {}",
                    resp.data_type
                )));
            }
        };

        match resp.data {
            SocketData::InitialState(state) => Ok(state),
            _ => Err(AppError::IncorrectSocketFormat(format!(
                "Expected 'InitialState' data but found {}",
                resp.data
            ))),
        }
    }
}

#[derive(Debug)]
pub enum AppError {
    UserNotFound(String),
    DatabaseError(String),
    IncorrectSocketFormat(String),
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
    pub id: i32,
    pub poster_id: i32,
    pub timestamp: u128,
    pub image: Image,
    pub comments: Vec<Comment>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InitialState {
    pub posts: Vec<PostDTO>,
}
