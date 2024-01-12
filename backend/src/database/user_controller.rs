use futures::future::join_all;
use itertools::{Either, Itertools};
use sqlx_crud::Crud;

use super::{sql_helpers::get_pool, types::user::User};
use crate::socket_handlers::types::AppError;
use std::iter::Iterator;

pub async fn read_user(id: i32) -> Result<User, AppError> {
    let pool = get_pool().await;
    println!("{}", id);
    match User::by_id(&pool, id).await? {
        Some(u) => Ok(u),
        None => Err(AppError::DatabaseError(format!(
            "Could not fetch user with ID {}",
            id
        ))),
    }
}

pub async fn read_users(ids: &Vec<i32>) -> Result<Vec<User>, AppError> {
    let pool = get_pool().await;
    let results: Vec<Result<Option<User>, AppError>> =
        join_all(ids.iter().map(|id| User::by_id(&pool, *id))).await;

    // ---
    // --- Map returned results into a Vector of succesfully retrieved users and one of errors.
    // --- If for a given ID that user was not found in the DB, then an error will be appended to errors.
    // ---

    let (users, errors): (Vec<_>, Vec<_>) =
        results
            .into_iter()
            .enumerate()
            .partition_map(|(index, res)| match res {
                Ok(c) => match c {
                    Some(c) => Either::Left(c),
                    None => Either::Right(AppError::DatabaseError(format!(
                        "User with ID {:?} not found!",
                        ids.get(index)
                    ))),
                },
                Err(e) => Either::Right(AppError::DatabaseError(format!(
                    "Failed when querying user {:?}. {:?}",
                    ids.get(index),
                    e
                ))),
            });

    // ---
    // --- Log all errors. This includes errors for users that weren't found.
    // ---

    errors.iter().for_each(|error| println!("{:?}", error));

    Ok(users)
}
