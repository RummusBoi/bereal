use futures::future::join_all;
use itertools::{Either, Itertools};
use my_sqlx_crud::traits::Crud;

use crate::socket_handlers::types::AppError;

use super::{sql_helpers::get_pool, types::image::Image};

pub async fn read_images(ids: Vec<i32>) -> Vec<Image> {
    let pool = get_pool().await;
    let results: Vec<Result<Option<Image>, sqlx::Error>> =
        join_all(ids.iter().map(|id| Image::by_id(&pool, *id))).await;

    // ---
    // --- Map returned results into a Vector of succesfully retrieved images and one of errors.
    // --- If for a given ID that image was not found in the DB, then an error will be appended to errors.
    // ---

    let (images, errors): (Vec<_>, Vec<_>) =
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
    // --- Log all errors. This includes errors for images that weren't found.
    // ---

    errors.iter().for_each(|error| println!("{:?}", error));

    images
}

pub async fn read_image(id: i32) -> Result<Image, AppError> {
    let pool = get_pool().await;
    match Image::by_id(&pool, id).await? {
        Some(c) => Ok(c),
        None => Err(AppError::DatabaseError(format!(
            "Could not fetch image with ID {}",
            id
        ))),
    }
}
