use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_timestamp() -> u128 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("wtf")
        .as_millis();
}
