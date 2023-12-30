use sqlx::{postgres::{PgPoolOptions, PgQueryResult}, Pool, Postgres};
use crate::types::Message;

const DATABASE_URL: &str = "postgresql://localhost/postgres";

//Messages are written to a table with primary key post_id, and columns sender, timestamp, and
//messaage.
pub async fn write_message_to_database(message: &Message) -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
                            .max_connections(5)
                            .connect(DATABASE_URL)
                            .await?;

    ensure_tables_exist(&pool).await?;

    // Now post the message to DB: first (post_id, message_id) to the posts table.
    let message_id = message.message.to_string() + message.post_id.as_str() + message.timestamp.to_string().as_str();
    sqlx::query(format!("
        insert into posts values (\'{}\', \'{}\');
    ", message.post_id, message_id).as_str()).execute(&pool).await?;
    
    // Then post (message_id, message, timestamp, sender) to the messages table.
    sqlx::query(format!("
        insert into messages values (\'{}\', \'{}\', \'{}\', \'{}\');
    ", message_id, message.message, message.timestamp, message.sender).as_str()).execute(&pool).await?;

    Ok(())
}

// Takes an sqlx connection pool to query through.
pub async fn ensure_tables_exist(connection_pool: &Pool<Postgres>) -> Result<(PgQueryResult, PgQueryResult), sqlx::Error> {
    // If the table for posts doesn't exist, create it.
    let posts_query_result = sqlx::query(r#"
        create table if not exists posts (
            post_id varchar(255) PRIMARY KEY,
            message_id varchar(255) NOT NULL UNIQUE
        );
    "#).execute(connection_pool).await?;

    // If the table for messages doesn't exist, create it.
    let messages_query_result = sqlx::query(r#"
        create table if not exists messages (
            message_id varchar(255) PRIMARY KEY,
            message varchar(255) NOT NULL,
            timestamp date NOT NULL,
            sender varchar(255) NOT NULL
        );
    "#).execute(connection_pool).await?;

    Ok((posts_query_result, messages_query_result))
}
