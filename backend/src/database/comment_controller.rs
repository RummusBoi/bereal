use crate::{general_helpers::ENV_VARS, socket_handlers::types::AppError};

use super::{
    sql_helpers::{
        sql_array_append, sql_read_row, sql_write_row, DBColumn, PostColumn, SqlEntry, Table,
    },
    types::comment::Comment,
};

fn get_mock_data() -> Vec<Comment> {
    return vec![
        Comment {
            id: "1".to_string(),
            timestamp: 0,
            poster_id: "rasmus".to_string(),
            data: "hej".to_string(),
        },
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
        todo!("Implement this part of the database interaction");
    }
}
pub async fn write_comment(post_id: &String, comment: Comment) -> Result<(), AppError> {
    let comment_id = comment.id.clone();

    sql_write_row(SqlEntry::Comment(comment)).await?;
    sql_array_append(
        post_id.clone(),
        comment_id,
        DBColumn::Posts(PostColumn::Comments),
    )
    .await?;
    Ok(())
}

pub async fn read_comment(comment_id: &String) -> Result<Comment, AppError> {
    sql_read_row(Table::Comments, comment_id).await
}

// pub async fn write_comment(post_id: &String, comment: &Comment) -> Result<(), sqlx::Error> {
//     ensure_tables_exist(&pool).await?;

//     sqlx::query(
//         format!(
//             "
//         insert into posts values (\'{}\', \'{}\');
//     ",
//             post_id, comment.id
//         )
//         .as_str(),
//     )
//     .execute(&pool)
//     .await?;

//     Ok(())
// }
