use std::{
    collections::HashMap,
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    get_userid_and_token,
    types::{AppError, User},
};
const USERS_FILE_PATH: &str = "./users.json";

fn read_user_file() -> Result<HashMap<String, User>, AppError> {
    let content = match fs::read_to_string(USERS_FILE_PATH) {
        Ok(content) => content,
        Err(err_msg) => return Err(AppError::FileReadFailed(err_msg.to_string())),
    };

    let users: Result<HashMap<String, User>, AppError> = serde_json::from_str(&content)
        .map_err(|_| AppError::FileParseFailed("Could not parse users.json".to_string()));

    return users;
}

pub fn get_user(user_id: &String) -> Result<User, AppError> {
    let users = read_user_file()?;
    match users.get(user_id) {
        Some(user) => Ok(user.clone()),
        None => Err(AppError::UserNotFound()),
    }
}

pub fn add_user(user_id: &String, token: &String) -> Result<(), AppError> {
    let content = match fs::read_to_string(USERS_FILE_PATH) {
        Ok(content) => content,
        Err(err_msg) => return Err(AppError::FileReadFailed(err_msg.to_string())),
    };

    let users: Result<HashMap<String, User>, AppError> = serde_json::from_str(&content)
        .map_err(|_| AppError::FileParseFailed("Could not parse users.json".to_string()));
    let mut users = match users {
        Ok(users) => users,
        Err(error) => return Err(error),
    };

    let new_user = User {
        user_id: user_id.clone(),
        token: token.clone(),
        created: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("wtf")
            .as_millis(),
    };
    users.insert(user_id.clone(), new_user.clone());

    let content = match serde_json::to_string(&users) {
        Ok(content) => content,
        Err(error) => return Err(AppError::MessageSerializationFailed(error.to_string())),
    };
    match fs::write(USERS_FILE_PATH, content) {
        Ok(_) => return Ok(()),
        Err(error) => return Err(AppError::MessageSerializationFailed(error.to_string())),
    }
}
