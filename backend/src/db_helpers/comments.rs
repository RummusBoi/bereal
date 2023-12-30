use crate::types::Comment;
use sqlx::{
    postgres::{PgPoolOptions, PgQueryResult},
    Pool, Postgres,
};

const DATABASE_URL: &str = "postgresql://localhost/postgres";

pub async fn write_message_to_database(message: &Comment) -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await?;

    ensure_tables_exist(&pool).await?;

    let message_id = message.message.to_string()
        + message.post_id.as_str()
        + message.timestamp.to_string().as_str();
    sqlx::query(
        format!(
            "
        insert into posts values (\'{}\', \'{}\');
    ",
            message.post_id, message_id
        )
        .as_str(),
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        format!(
            "
        insert into messages values (\'{}\', \'{}\', \'{}\', \'{}\');
    ",
            message_id, message.message, message.timestamp, message.sender
        )
        .as_str(),
    )
    .execute(&pool)
    .await?;

    Ok(())
}

pub async fn ensure_tables_exist(
    connection_pool: &Pool<Postgres>,
) -> Result<(PgQueryResult, PgQueryResult), sqlx::Error> {
    let posts_query_result = sqlx::query(
        r#"
        create table if not exists posts (
            post_id varchar(255) PRIMARY KEY,
            message_id varchar(255) NOT NULL UNIQUE
        );
    "#,
    )
    .execute(connection_pool)
    .await?;

    let messages_query_result = sqlx::query(
        r#"
        create table if not exists messages (
            message_id varchar(255) PRIMARY KEY,
            message varchar(255) NOT NULL,
            timestamp date NOT NULL,
            sender varchar(255) NOT NULL
        );
    "#,
    )
    .execute(connection_pool)
    .await?;

    Ok((posts_query_result, messages_query_result))
}