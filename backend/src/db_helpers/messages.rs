use std::fs;

use crate::types::{AppError, Message};
const MESSAGE_FILE_PATH: &str = "./messages.json";

pub fn write_message_to_file(message: &Message) -> Result<(), AppError> {
    let content = match fs::read_to_string(MESSAGE_FILE_PATH) {
        Ok(content) => content,
        Err(err_msg) => return Err(AppError::FileReadFailed(err_msg.to_string())),
    };

    let messages: Result<Vec<Message>, serde_json::Error> = serde_json::from_str(&content);
    let mut messages: Vec<Message> = match messages {
        Ok(messages) => messages,
        Err(error) => return Err(AppError::FileParseFailed(error.to_string())),
    };

    messages.push(message.clone());
    let content = match serde_json::to_string(&messages) {
        Ok(content) => content,
        Err(error) => return Err(AppError::MessageSerializationFailed(error.to_string())),
    };
    match fs::write(MESSAGE_FILE_PATH, content) {
        Ok(_) => Ok(()),
        Err(error) => return Err(AppError::MessageSerializationFailed(error.to_string())),
    }
}

pub fn read_all_messages_from_file() -> Result<Vec<Message>, AppError> {
    let content = match fs::read_to_string(MESSAGE_FILE_PATH) {
        Ok(content) => content,
        Err(err_msg) => return Err(AppError::FileReadFailed(err_msg.to_string())),
    };

    let messages: Result<Vec<Message>, serde_json::Error> = serde_json::from_str(&content);
    match messages {
        Ok(messages) => Ok(messages),
        Err(error) => return Err(AppError::FileParseFailed(error.to_string())),
    }
}
