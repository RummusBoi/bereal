use sqlx::postgres::PgPoolOptions;

use crate::general_helpers::ENV_VARS;

use super::{
    helpers::{ensure_tables_exist, DATABASE_URL},
    types::comment::Comment,
};

fn get_mock_data() -> Vec<Comment> {
    return vec![
        Comment {
            id: "1".to_string(),
            timestamp: 0,
            poster_id: "rasmus".to_string(),
            data: "hej".to_string(),
        }, //("rasmus".to_string(), "hej".to_string()),
        Comment {
            id: "2".to_string(),
            timestamp: 1,
            poster_id: "jonathan".to_string(),
            data: "jeg er dum".to_string(),
        },
        Comment {
            id: "3".to_string(),
            timestamp: 2,
            poster_id: "anakin".to_string(),
            data: "hej gutter".to_string(),
        },
    ];
}

pub fn read_comments(ids: Vec<String>) -> Vec<Comment> {
    if ENV_VARS.use_mocked_database {
        return get_mock_data()
            .iter()
            .filter(|comment| ids.contains(&comment.id))
            .map(|comment| comment.clone())
            .collect::<Vec<Comment>>();
    } else {
        todo!("We need to implement the db behaviour here.")
    }
}

pub async fn write_comment(post_id: &String, comment: &Comment) -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await?;

    ensure_tables_exist(&pool).await?;

    sqlx::query(
        format!(
            "
        insert into posts values (\'{}\', \'{}\');
    ",
            post_id, comment.id
        )
        .as_str(),
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        format!(
            "
        insert into comment values (\'{}\', \'{}\', \'{}\', \'{}\');
    ",
            comment.id, comment.data, comment.timestamp, comment.poster_id
        )
        .as_str(),
    )
    .execute(&pool)
    .await?;

    Ok(())
}
