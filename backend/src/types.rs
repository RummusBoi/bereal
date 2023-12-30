use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::AppData;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Message {
    pub sender: String,
    pub post_id: String,
    pub message: String,
    pub timestamp: u128,
    // read: bool,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct User {
    pub user_id: String,
    pub token: String,
    pub created: u128,
}

#[derive(Debug)]
pub enum AppError {
    FileReadFailed(String),
    FileParseFailed(String),
    MessageSerializationFailed(String),
    NoUserIDProvided(String),
    NoTokenProvided(String),
    Unauthenticated(String),
    IncorrectDataFormat(String),
    UserNotFound(),
}

#[derive(Clone, Serialize)]
pub struct AppResponse {
    data: Option<AppData>,
    message: Option<String>,
    status: u32,
}

impl From<User> for AppResponse {
    fn from(user: User) -> Self {
        AppResponse {
            data: Some(AppData::SingleUser(user)),
            message: Some("Successfully created user".to_string()),
            status: 200,
        }
    }
}
impl From<AppError> for AppResponse {
    fn from(error: AppError) -> Self {
        println!(
            "Received Error from app: {:?}. Returning error to user.",
            error
        );
        let message = match error {
            AppError::FileReadFailed(message) => message,
            AppError::FileParseFailed(message) => message,
            AppError::MessageSerializationFailed(message) => message,
            AppError::NoUserIDProvided(message) => message,
            AppError::NoTokenProvided(message) => message,
            AppError::Unauthenticated(message) => message,
            AppError::IncorrectDataFormat(message) => message,
            AppError::UserNotFound() => "what?".to_string(),
        };
        return AppResponse {
            data: None,
            message: Some(message),
            status: 500,
        };
    }
}

impl From<Message> for AppResponse {
    fn from(message: Message) -> Self {
        return AppResponse {
            data: Some(AppData::SingleMessage(message)),
            message: None,
            status: 200,
        };
    }
}

impl From<Vec<Message>> for AppResponse {
    fn from(messages: Vec<Message>) -> Self {
        return AppResponse {
            data: Some(AppData::MultipleMessages(messages)),
            message: None,
            status: 200,
        };
    }
}

impl From<Vec<String>> for AppResponse {
    fn from(users: Vec<String>) -> Self {
        return AppResponse {
            data: Some(AppData::Users(users)),
            message: None,
            status: 200,
        };
    }
}

impl<T, E> From<Result<T, E>> for AppResponse
where
    T: Into<AppResponse>,
    E: Into<AppResponse>,
{
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(v) => v.into(),
            Err(e) => e.into(),
        }
    }
}

// impl From<Vec<Message>> for String {
//     fn from(value: Vec<Message>) -> Self {

//     }
// }

impl IntoResponse for AppResponse {
    fn into_response(self) -> Response {
        let j = Json::from(self);
        return j.into_response();
    }
}
impl Into<Response> for AppResponse {
    fn into(self) -> Response {
        let j = Json::from(self);
        return j.into_response();
    }
}
