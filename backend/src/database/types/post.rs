use crate::{database::helpers::get_timestamp, socket_handlers::types::AppError};
use futures::TryStreamExt;
use sqlx::{
    database::HasArguments, postgres::PgArguments, Arguments, Database, Encode, Executor, FromRow,
    Pool, Postgres,
};
use sqlx_crud::{Schema, SqlxCrud};
use uuid::uuid;

#[derive(Clone, Debug, FromRow, SqlxCrud, PartialEq)]
#[database(Postgres)]
pub struct Post {
    pub id: i32,
    pub poster_id: i32,
    pub image: i32,
    pub comments: Vec<i32>,
    pub timestamp: i64,
}

impl Post {
    pub async fn by_id(pool: &Pool<Postgres>, id: i32) -> Result<Option<Post>, AppError> {
        let query = format!("select * from {} where id = {}", Self::table_name(), id);

        let result = sqlx::query_as::<_, Self>(query.as_str())
            .fetch_optional(pool)
            .await;

        result.map_err(|error| {
            AppError::DatabaseError(format!(
                "Error when fetching post by id {}. {:?}",
                id, error
            ))
        })
    }
    pub async fn by_user_id(pool: &Pool<Postgres>, user_id: i32) -> Result<Option<Post>, AppError> {
        let query = format!(
            "select * from {} where poster_id = {}",
            Self::table_name(),
            user_id
        );

        let result = sqlx::query_as::<_, Self>(query.as_str())
            .fetch_optional(pool)
            .await;

        result.map_err(|error| {
            AppError::DatabaseError(format!(
                "Error when fetching post by user {}. {:?}",
                user_id, error
            ))
        })
    }
    pub async fn update_comment_arr_atomic(
        pool: &Pool<Postgres>,
        post_id: i32,
        comment_id: i32,
    ) -> Result<Post, AppError> {
        let query = format!(
            "update {} set comments = array_append(comments, {}) where id = {};",
            Self::table_name(),
            comment_id,
            post_id
        );

        let new_post = sqlx::query_as::<_, Self>(&query)
            .fetch_optional(pool)
            .await?;
        match new_post {
            Some(post) => Ok(post),
            None => Err(AppError::DatabaseError(format!(
                "Failed when updating comments on post."
            ))),
        }
    }
    pub fn random() -> Self {
        Post {
            id: uuid::Uuid::new_v4().to_u128_le() as i32,
            poster_id: uuid::Uuid::new_v4().to_u128_le() as i32,
            timestamp: get_timestamp(),
            image: uuid::Uuid::new_v4().to_u128_le() as i32,
            comments: vec![
                uuid::Uuid::new_v4().to_u128_le() as i32,
                uuid::Uuid::new_v4().to_u128_le() as i32,
            ],
        }
    }
}
