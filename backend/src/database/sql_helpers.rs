use std::time::Duration;

use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgRow},
    PgPool, Postgres,
};
use tokio::sync::OnceCell;

use crate::{general_helpers::ENV_VARS, socket_handlers::types::AppError};

use super::types::{comment::Comment, image::Image, post::Post, user::User};

async fn table_exists_with_columns(table: &str, columns: &[&str]) -> Result<bool, sqlx::Error>
where
{
    let pool = get_pool().await;
    let table_exists_query = format!("SELECT EXISTS (SELECT * FROM {});", table);

    let table_exists = match sqlx::query::<_>(&table_exists_query).fetch_one(&pool).await {
        Ok(_) => Ok(true),
        Err(e) => {
            if e.to_string().contains("does not exist") {
                Ok(false)
            } else {
                Err(e)
            }
        }
    }?;

    if !table_exists {
        return Ok(false);
    }

    Ok(true)
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError::DatabaseError(value.to_string())
    }
}

pub async fn get_pool() -> PgPool {
    let id = uuid::Uuid::new_v4();
    println!("Acquiring pool... {}", id);
    let pool = POOL
        .get_or_init(|| async {
            println!("Inside the body :O... {}", id);
            let options = PgConnectOptions::new()
                .host("localhost")
                .port(5432)
                .database("postgres");
            PgPoolOptions::new()
                .max_connections(1024)
                // .max_lifetime(Duration::from_secs(1))
                .connect_with(options)
                .await
                .expect("asd")
        })
        .await
        .clone();
    println!("Acquired pool... {}", id);
    return pool;
}

static POOL: OnceCell<PgPool> = OnceCell::const_new();
