use crate::general_helpers::get_timestamp;

#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub friends: Vec<String>,
    pub timestamp: u128,
}

impl User {
    pub fn new(user_id: String, friends: Vec<String>) -> User {
        Self {
            id: user_id,
            timestamp: get_timestamp(),
            friends: friends,
        }
    }
}
