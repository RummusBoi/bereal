use std::{collections::HashMap, fs};

use crate::{
    db_helpers::users::{add_user, get_user},
    types::{AppError, User},
};

// pub fn is_authenticated(user_id: &String, token: &String) -> Result<bool, AppError> {
//     match credentials_are_ok(user_id, token) {
//         Ok(authenticated) => Ok(authenticated),
//         Err(error) => match error {
//             AppError::UserNotFound() => Ok(add_user(user_id, token).is_none()),
//             _ => return Err(error),
//         },
//     }
// }

pub fn credentials_are_ok(user_id: &String, token: &String) -> Result<bool, AppError> {
    let user = get_user(user_id)?;
    Ok(user.token == *token)
}
