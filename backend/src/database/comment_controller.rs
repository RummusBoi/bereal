use sqlx::postgres::PgPoolOptions;

use super::{
    helpers::{ensure_tables_exist, DATABASE_URL},
    types::comment::Comment,
};

fn get_mock_data() -> Vec<Comment> {
    return vec![
        Comment::new("rasmus".to_string(), "hej".to_string()),
        Comment::new("jonathan".to_string(), "hej2".to_string()),
        Comment::new("darth vader".to_string(), "hej v3".to_string()),
    ];
}

pub fn read_comments(ids: Vec<String>) -> Vec<Comment> {
    return get_mock_data()
        .iter()
        .filter(|comment| ids.contains(&comment.id))
        .map(|comment| comment.clone())
        .collect::<Vec<Comment>>();
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
