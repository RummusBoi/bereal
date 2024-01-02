use super::types::user::User;
use crate::{general_helpers::ENV_VARS, socket_handlers::types::AppError};
use std::iter::Iterator;

fn get_mock_data() -> Vec<User> {
    return vec![
        User::new(
            "rasmus".to_string(),
            vec!["jonathan".to_string(), "darth vader".to_string()],
        ),
        User::new(
            "jonathan".to_string(),
            vec!["rasmus".to_string(), "darth vader".to_string()],
        ),
        User::new(
            "darth vader".to_string(),
            vec!["rasmus".to_string(), "jonathan".to_string()],
        ),
    ];
}

pub fn read_user(id: &String) -> Result<User, AppError> {
    read_users(&vec![id.clone()])
        .next()
        .and_then(|u| Some(u.clone()))
        .ok_or(AppError::UserNotFound(id.clone()))
}

pub fn read_users(ids: &Vec<String>) -> impl Iterator<Item = User> + '_ {
    if ENV_VARS.use_mocked_database {
        let data = get_mock_data();

        return data
            .into_iter()
            .filter(|user| ids.contains(&&user.id))
            .map(|user| user.clone());
    } else {
        todo!("Implement this part of the database interaction");
    }
}
