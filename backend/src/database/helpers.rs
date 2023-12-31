use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::{postgres::PgQueryResult, Pool, Postgres};

use crate::general_helpers::ENV_VARS;

pub fn get_timestamp() -> u128 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("wtf")
        .as_millis();
}

pub const DATABASE_URL: &str = "postgresql://localhost/postgres";

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

pub fn get_post_id_for_user(user_id: &String) -> String {
    if ENV_VARS.use_mocked_database {
        return format!("{user_id}-day-1");
    } else {
        todo!("Implement this")
    }
}
