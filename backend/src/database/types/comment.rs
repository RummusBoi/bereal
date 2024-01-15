use my_sqlx_crud_macro::SqlxCrud;
use serde::{Deserialize, Serialize};

use crate::database::helpers::get_timestamp;
use crate::socket_handlers::types::AppError;
use my_sqlx_crud::traits::Schema;
use sqlx::Pool;
use sqlx::Postgres;

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow, SqlxCrud, PartialEq)]
#[database(Postgres)]
pub struct Comment {
    pub id: i32,
    pub poster_id: i32,
    pub timestamp: i64,
    pub data: String,
}
impl Comment {
    pub async fn by_id(pool: &Pool<Postgres>, id: i32) -> Result<Option<Comment>, AppError> {
        let query = format!("select * from {} where id = {}", Self::table_name(), id);

        let result = sqlx::query_as::<_, Self>(query.as_str())
            .fetch_optional(pool)
            .await;

        result.map_err(|error| {
            AppError::DatabaseError(format!(
                "Error when fetching comment by id {}. {:?}",
                id, error
            ))
        })
    }
    pub fn random() -> Self {
        Comment {
            id: uuid::Uuid::new_v4().to_u128_le() as i32,
            poster_id: uuid::Uuid::new_v4().to_u128_le() as i32,
            timestamp: get_timestamp(),
            data: "hejsa".to_string(),
        }
    }

    pub fn new(poster_id: i32, content: String) -> Self {
        Comment {
            id: uuid::Uuid::new_v4().to_u128_le() as i32,
            poster_id: poster_id,
            timestamp: get_timestamp(),
            data: content,
        }
    }
}
