use uuid::Uuid;

use crate::general_helpers::get_timestamp;

#[derive(Clone, Debug)]
pub struct Post {
    pub id: String,
    pub poster_id: String,
    pub image: String,
    pub comments: Vec<String>,
    pub timestamp: u128,
}

impl Post {
    pub fn new(poster_id: String, image: String, comments: Vec<String>) -> Post {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: get_timestamp(),
            poster_id: poster_id,
            image: image,
            comments: comments,
        }
    }
}
