use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgRow},
    PgPool, Postgres,
};

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

    // fn gen_query(table: &String, column: &String) -> String {
    //     return format!("COL_LENGTH ('{}', '{}')", table, column);
    // }

    // let results = columns.iter().all(|column| {
    //     let query = gen_query(&table, column);
    //     let column_length = sqlx::query_as::<_, i32>(&query).fetch_optional(&pool).await;
    // });
    Ok(true)
}

// pub async fn ensure_all_tables_correct() {
//     let types: Vec<Crud> = vec![User, Comment, Image];
//     let tables_exist: Vec<bool> = join_all(
//         tables_columns
//             .iter()
//             .map(|(table, columns)| table_exists_with_columns(table, columns)),
//     )
//     .await
//     .into_iter()
//     // perform unwrap here. If we failed communication with just one of the tables, we want to crash immediately
//     .map(|res| res.unwrap())
//     .collect();
//     tables_exist
//         .iter()
//         .enumerate()
//         .for_each(|(index, table_exists)| if !table_exists {});
//     println!("{:?}", results);
// }

pub async fn ensure_tables_exist() -> Result<(), AppError> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(ENV_VARS.database_url.as_str())
        .await?;
    sqlx::query(
        r#"
        create table if not exists posts (
            post_id varchar(255) PRIMARY KEY,
            message_id varchar(255) NOT NULL UNIQUE
        );
    "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        create table if not exists messages (
            message_id varchar(255) PRIMARY KEY,
            message varchar(255) NOT NULL,
            timestamp date NOT NULL,
            sender varchar(255) NOT NULL
        );
    "#,
    )
    .execute(&pool)
    .await?;

    Ok(())
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError::DatabaseError(value.to_string())
    }
}
pub async fn get_pool() -> PgPool {
    let address = "postgres://localhost:5432";
    println!("{}", address);
    let options = PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .database("postgres");
    return PgPoolOptions::new().connect_with(options).await.unwrap();
}
