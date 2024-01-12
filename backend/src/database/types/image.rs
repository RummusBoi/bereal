use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use sqlx_crud::Schema;
use sqlx_crud::SqlxCrud;

use crate::database::helpers::get_timestamp;
use crate::socket_handlers::types::AppError;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow, SqlxCrud, PartialEq)]
#[database(Postgres)]
pub struct Image {
    pub id: i32,
    pub timestamp: i64,
    pub data: Vec<u8>,
}
impl Image {
    pub async fn by_id(pool: &Pool<Postgres>, id: i32) -> Result<Option<Image>, AppError> {
        let query = format!("select * from {} where id = {}", Self::table_name(), id);

        let result = sqlx::query_as::<_, Self>(query.as_str())
            .fetch_optional(pool)
            .await;

        result.map_err(|error| {
            AppError::DatabaseError(format!(
                "Error when fetching image by id {}. {:?}",
                id, error
            ))
        })
    }
    pub fn random() -> Self {
        Image {
            id: uuid::Uuid::new_v4().to_u128_le() as i32,
            timestamp: get_timestamp(),
            data: vec![1, 2, 3],
        }
    }
}
