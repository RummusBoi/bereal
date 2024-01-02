use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::{postgres::PgQueryResult, Pool, Postgres};

pub fn get_timestamp() -> u128 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("wtf")
        .as_millis();
}

pub const DATABASE_URL: &str = "postgresql://localhost/postgres";

pub async fn ensure_tables_exist(
    connection_pool: &Pool<Postgres>,
) -> Result<(PgQueryResult, PgQueryResult, PgQueryResult), sqlx::Error> {
    let posts_query_result = sqlx::query(
        r#"
        create table if not exists posts (
            post_id text PRIMARY KEY,
            poster_id text NOT NULL,
            image_id text NOT NULL,
            caption text NOT NULL UNIQUE,
            comment_ids text[]
        );
    "#,
    )
    .execute(connection_pool)
    .await?;

    let messages_query_result = sqlx::query(
        r#"
        create table if not exists comments (
            message_id text PRIMARY KEY,
            message text NOT NULL,
            timestamp date NOT NULL,
            sender text NOT NULL
        );
    "#,
    )
    .execute(connection_pool)
    .await?;

    let images_query_result = sqlx::query(
        r#"
            create table if not exists images (
                image_id text NOT NULL,
                data text NOT NULL,
                timestamp date NOT NULL
            );
        "#,
    )
    .execute(connection_pool)
    .await?;

    Ok((posts_query_result, messages_query_result, images_query_result))
}
