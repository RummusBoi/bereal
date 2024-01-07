use std::collections::HashMap;

use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    query_as, Column, FromRow, Row, ValueRef,
};

use crate::{general_helpers::ENV_VARS, socket_handlers::types::AppError};

use super::types::{comment::Comment, post::Post};

pub async fn sql_array_append(
    row_id: String,
    value: String,
    column: DBColumn,
) -> Result<(), AppError> {
    let (table, column_name) = column.into();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(ENV_VARS.database_url.as_str())
        .await?;
    sqlx::query(
        format!(
            "
            update {} set {} = array_append({}, {}) where id = {};
            ",
            table, column_name, column_name, value, row_id
        )
        .as_str(),
    )
    .execute(&pool)
    .await?;

    Ok(())
}

pub async fn sql_write_row(entry: SqlEntry) -> Result<(), AppError> {
    let (table, data) = entry.into();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(ENV_VARS.database_url.as_str())
        .await?;
    sqlx::query(
        format!(
            "
            insert into {} values (\'{}\');
            ",
            table,
            data.join("\', \'")
        )
        .as_str(),
    )
    .execute(&pool)
    .await?;

    Ok(())
}

pub async fn sql_read_row<T>(table: Table, id: &String) -> Result<T, AppError>
where
    for<'a> T: FromRow<'a, PgRow> + Send + Unpin,
{
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(ENV_VARS.database_url.as_str())
        .await?;

    let row = sqlx::query_as::<_, T>(
        format!(
            "
        select * from {} where id = {}
        ",
            table, id
        )
        .as_str(),
    )
    .fetch_one(&pool)
    .await?;

    Ok(row)

    // T::try_from(row).map_err(|err| err.into())
}

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

#[derive(strum_macros::Display)]
pub enum Table {
    Comments,
    Posts,
}

#[derive(strum_macros::Display)]
pub enum CommentColumn {
    Id,
    Poster,
}
#[derive(strum_macros::Display)]
pub enum PostColumn {
    Comments,
}

pub enum DBColumn {
    Comments(CommentColumn),
    Posts(PostColumn),
}

pub enum SqlEntry {
    Comment(Comment),
    Post(Post),
}

pub enum SqlEntryArrayAppend {
    CommentToPost(Comment),
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError::DatabaseError(value.to_string())
    }
}

// macro_rules! unwrap {
//     ($value:expr, $pattern:pat => $result:expr) => {
//         match $value {
//             E::VarA($pattern) => $result,
//             E::VarB($pattern) => $result,
//         }
//     };
// }

impl From<DBColumn> for (Table, String) {
    fn from(value: DBColumn) -> Self {
        use DBColumn::*;

        match value {
            Comments(column) => match column {
                CommentColumn::Id => (Table::Posts, format!("{}", CommentColumn::Poster)),
                CommentColumn::Poster => todo!(),
            },
            Posts(column) => match column {
                PostColumn::Comments => (Table::Posts, format!("{}", PostColumn::Comments)),
            },
        }
    }
}

impl From<SqlEntry> for (Table, Vec<String>) {
    fn from(value: SqlEntry) -> Self {
        match value {
            SqlEntry::Comment(comment) => (
                Table::Comments,
                vec![
                    comment.id,
                    comment.data,
                    comment.timestamp.to_string(),
                    comment.poster_id,
                ],
            ),
            SqlEntry::Post(post) => (
                Table::Posts,
                vec![post.id, format!("{{{}}}", post.comments.join(","))],
            ),
        }
    }
}

fn row_to_string(row: &PgRow) -> (String, String) {
    let mut column_string = String::new();
    let mut row_string = String::new();
    for col in row.columns() {
        let value = row.try_get_raw(col.ordinal()).unwrap();
        let value = match value.is_null() {
            true => "NULL".to_string(),
            false => value.as_str().unwrap().to_string(),
        };
        column_string = column_string + (col.name());
        row_string = row_string + value.as_str();
    }

    (column_string, row_string)
}
