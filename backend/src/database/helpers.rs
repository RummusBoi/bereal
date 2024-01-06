use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::{postgres::PgQueryResult, Pool, Postgres};

pub fn get_timestamp() -> i64 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("wtf")
        .as_millis() as i64;
}

pub const DATABASE_URL: &str = "postgresql://localhost/postgres";
