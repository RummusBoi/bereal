use std::time::{SystemTime, UNIX_EPOCH};

use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Image {
    pub id: String,
    pub timestamp: u128,
    pub data: Vec<u8>,
}

impl Image {
    pub fn new(data: Vec<u8>) -> Image {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("wtf")
            .as_millis();
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: current_time,
            data: data,
        }
    }
}
