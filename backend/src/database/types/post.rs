use crate::{database::helpers::get_timestamp, socket_handlers::types::AppError};
use my_sqlx_crud::traits::Schema;
use my_sqlx_crud_macro::SqlxCrud;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};

type PostId = i32;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow, SqlxCrud, PartialEq)]
#[database(Postgres)]
pub struct Post {
    pub id: PostId,
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

        sqlx::query(&query).execute(pool).await?;
        let new_post = Post::by_id(pool, post_id).await.unwrap();

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

    pub fn new(poster_id: i32, image_id: i32, comment_ids: Vec<i32>) -> Self {
        let post = Post {
            id: uuid::Uuid::new_v4().to_u128_le() as i32,
            poster_id: poster_id,
            timestamp: get_timestamp(),
            image: image_id,
            comments: comment_ids,
        };
        println!("Created {:?}", post);
        post
    }
}
