use uuid::Uuid;

use crate::general_helpers::get_timestamp;
#[derive(Clone, Debug)]

pub struct Comment {
    pub id: String,
    pub poster_id: String,
    pub timestamp: u128,
    pub data: String,
}

impl Comment {
    pub fn new(user_id: String, data: String) -> Comment {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: get_timestamp(),
            poster_id: user_id,
            data: data,
        }
    }
}
