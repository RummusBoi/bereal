use crate::database::types::post::Post;
use crate::database::types::{comment::Comment, image::Image};
use enum_as_inner::EnumAsInner;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Serialize, Deserialize, Debug, Display)]
pub enum SocketEventType {
    Error,
    InitialState,
    PostCreated,
    CommentCreated,
    FriendRequestSent,
}

#[derive(Serialize, Deserialize, Debug, Display, EnumAsInner)]
pub enum SocketData {
    InitialState(InitialState),
    String(String),
    CreatePostDTO(CreatePostDTO),
    CreateCommentDTO(CreateCommentDTO),
    CommentDTO(Comment),
    PostDTO(PostDTO),
    Post(Post),
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
    pub fn serialize_for_tung_socket(&self) -> tungstenite::Message {
        return tungstenite::Message::Text(serde_json::to_string(self).unwrap());
    }
}

impl From<axum::extract::ws::Message> for SocketResponse {
    fn from(value: axum::extract::ws::Message) -> Self {
        return match value {
            axum::extract::ws::Message::Text(data) => {
                serde_json::from_slice::<SocketResponse>(data.as_bytes()).unwrap()
            }
            _ => panic!("Received invalid socket response."),
        };
    }
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePostDTO {
    pub image: CreateImageDTO,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCommentDTO {
    pub data: String,
    pub post_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateImageDTO {
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InitialState {
    pub posts: Vec<PostDTO>,
}
