use futures::future::join_all;
use itertools::{Either, Itertools};
use sqlx_crud::Crud;

use crate::{
    general_helpers::{VectorTools, ENV_VARS},
    socket_handlers::types::AppError,
};

use super::{
    sql_helpers::get_pool,
    types::{image::Image, post::Post},
};

pub async fn read_posts<'a>(ids: Vec<i32>) -> Vec<Post> {
    let pool = get_pool().await;
    let results: Vec<Result<Option<Post>, AppError>> =
        join_all(ids.iter().map(|id| Post::by_id(&pool, *id))).await;

    // ---
    // --- Map returned results into a Vector of succesfully retrieved posts and one of errors.
    // --- If for a given ID that comment was not found in the DB, then an error will be appended to errors.
    // ---

    let (posts, errors): (Vec<_>, Vec<_>) =
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
    // --- Log all errors. This includes errors for posts that weren't found.
    // ---

    errors.iter().for_each(|error| println!("{:?}", error));

    posts
}

pub async fn read_posts_for_users(user_ids: Vec<i32>) -> Vec<Post> {
    let pool = get_pool().await;

    let results = join_all(
        user_ids
            .iter()
            .map(|user_id| Post::by_user_id(&pool, *user_id)),
    )
    .await;

    // ---
    // --- Map returned results into a Vector of succesfully retrieved posts and one of errors.
    // --- If for a given ID that comment was not found in the DB, then an error will be appended to errors.
    // ---

    let (posts, errors): (Vec<_>, Vec<_>) =
        results
            .into_iter()
            .enumerate()
            .partition_map(|(index, res)| match res {
                Ok(c) => match c {
                    Some(c) => Either::Left(c),
                    None => Either::Right(AppError::DatabaseError(format!(
                        "Comment with ID {:?} not found!",
                        user_ids.get(index)
                    ))),
                },
                Err(e) => Either::Right(AppError::DatabaseError(format!(
                    "Failed when querying comment {:?}. {:?}",
                    user_ids.get(index),
                    e
                ))),
            });

    // ---
    // --- Log all errors. This includes errors for posts that weren't found.
    // ---

    errors.iter().for_each(|error| println!("{:?}", error));

    posts
}
