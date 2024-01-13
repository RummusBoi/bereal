use my_sqlx_crud::traits::Schema;
use my_sqlx_crud_macro::SqlxCrud;
use sqlx::Postgres;
use sqlx::{prelude::FromRow, Pool};

use crate::database::helpers::get_timestamp;
use crate::socket_handlers::types::AppError;
#[derive(Clone, Debug, FromRow, SqlxCrud, PartialEq)]
#[database(Postgres)]
pub struct User {
    pub id: i32,
    pub friends: Vec<i32>,
    pub timestamp: i64,
}

impl User {
    // pub async fn by_id(pool: &Pool<Postgres>, id: i32) -> Result<Option<User>, AppError> {
    //     let query = format!("select * from {} where id = {}", Self::table_name(), id);

    //     let result = sqlx::query_as::<_, Self>(query.as_str())
    //         .fetch_optional(pool)
    //         .await;

    //     result.map_err(|error| {
    //         AppError::DatabaseError(format!(
    //             "Error when fetching user by id {}. {:?}",
    //             id, error
    //         ))
    //     })
    // }
    pub fn random() -> Self {
        User {
            id: uuid::Uuid::new_v4().to_u128_le() as i32,
            friends: vec![],
            timestamp: get_timestamp(),
        }
    }

    // pub async fn create() -> Self {
    //     let query = format!(
    //         r#"INSERT INTO users ("name") VALUES ({}) RETURNING "users"."user_id"#,
    //         Self::table_name(),
    //         id
    //     );

    //     let result = sqlx::query_as::<_, Self>(query.as_str())
    //         .fetch_optional(pool)
    //         .await;
    // }
}
