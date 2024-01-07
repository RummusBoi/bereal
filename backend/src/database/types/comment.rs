use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::database::helpers::get_timestamp;

#[derive(Clone, Debug, Serialize, Deserialize, Crud)]
pub struct Comment {
    pub id: String,
    pub poster_id: String,
    pub timestamp: i64,
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
