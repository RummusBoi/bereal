use futures::future::join_all;
use sqlx_crud::Crud;

use super::{
    sql_helpers::get_pool,
    types::{comment::Comment, post::Post},
};
use crate::socket_handlers::types::AppError;
use itertools::{Either, Itertools};

pub async fn read_comments(ids: Vec<i32>) -> Result<Vec<Comment>, AppError> {
    let pool = get_pool().await;
    let results: Vec<Result<Option<Comment>, AppError>> =
        join_all(ids.iter().map(|id| Comment::by_id(&pool, *id))).await;

    // ---
    // --- Map returned results into a Vector of succesfully retrieved comments and one of errors.
    // --- If for a given ID that comment was not found in the DB, then an error will be appended to errors.
    // ---

    let (comments, errors): (Vec<_>, Vec<_>) =
        results
            .into_iter()
            .enumerate()
            .partition_map(|(index, res)| match res {
                Ok(c) => match c {
                    Some(c) => Either::Left(c),
                    None => Either::Right(AppError::DatabaseError(format!(
                        "Comment with ID {:?} not found!",
                        ids.get(index)
                    ))),
                },
                Err(e) => Either::Right(AppError::DatabaseError(format!(
                    "Failed when querying comment {:?}. {:?}",
                    ids.get(index),
                    e
                ))),
            });

    // ---
    // --- Log all errors. This includes errors for comments that weren't found.
    // ---

    errors.iter().for_each(|error| println!("{:?}", error));

    Ok(comments)
}

pub async fn write_comment(post_id: i32, comment: Comment) -> Result<Post, AppError> {
    let pool = get_pool().await;

    let comment_id = comment.id;
    let comment = comment.create(&pool).await?;
    let update_result = Post::update_comment_arr_atomic(&pool, post_id, comment_id).await;
    match update_result {
        // if updating comment array on post fails, then revert the comment creation from earlier
        Ok(post) => Ok(post),
        Err(error) => {
            return match comment.delete(&pool).await {
                Ok(_) => Err(error),
                Err(inner_error) => {
                    println!("{}. Failed when updating comment array on Post {}. Tried reverting creation of comment {}, but that operation failed.", inner_error, post_id, comment_id);
                    Err(error)
                }
            }
        }
    }
}

pub async fn read_comment(id: i32) -> Result<Comment, AppError> {
    let pool = get_pool().await;
    match Comment::by_id(&pool, id).await? {
        Some(c) => Ok(c),
        None => Err(AppError::DatabaseError(format!(
            "Could not fetch comment with ID {}",
            id
        ))),
    }
}
