use crate::general_helpers::ENV_VARS;
use uuid::Uuid;
use sqlx::{postgres::{PgPoolOptions, PgQueryResult}, QueryBuilder, Pool, Postgres, Row};

use super::{
    types::{image::Image, post::Post},
    helpers::{DATABASE_URL, ensure_tables_exist},
};

fn get_mock_data() -> Vec<Image> {
    return vec![
        Image {
            id: "1".to_string(),
            data: vec![0, 1, 2],
            timestamp: 0,
        },
        Image {
            id: "2".to_string(),
            data: vec![0, 5, 10],
            timestamp: 1,
        },
        Image {
            id: "3".to_string(),
            data: vec![10, 20, 30],
            timestamp: 2,
        },
    ];
}

pub async fn read_images(ids: Vec<String>) -> Result<Vec<Image>, sqlx::Error> {
    if ENV_VARS.use_mocked_database {
        return Ok(get_mock_data()
            .iter()
            .filter(|image| ids.contains(&image.id))
            .map(|image| image.clone())
            .collect::<Vec<Image>>());
    } else {
        return Ok(read_images_from_db(ids).await?);
    }
}

async fn read_images_from_db(ids: Vec<String>) -> Result<Vec<Image>, sqlx::Error> {
    let pool = PgPoolOptions::new().max_connections(5).connect(DATABASE_URL).await?;

    let mut querybuilder_stub = QueryBuilder::new("select * from images where image_id =");
    let finalized_querybuilder = querybuilder_stub.push(ids.join(" or image_id = "));
    let sql_query = finalized_querybuilder.build();

    let query_result = sql_query.fetch_all(&pool).await?;

    Ok(query_result.iter().filter_map(
        // Issue: sqlx only supports ex. i64 (not u64) in the "get" method (else it complains
        // that trait bound is not satisfied)
        |row| {
            let id = row.try_get::<String, _>("id").ok()?;
            let data = row.try_get::<Vec<u8>, _>("data").ok()?;
            let timestamp: u128 = row.try_get::<i64, _>("timestamp").ok()?.try_into().ok()?;
            Some(Image {id, data, timestamp})
        }
    ).collect::<Vec<Image>>())
}

// TODO: Images in comments? (Just a new field on comment and extra image uploads.)
pub async fn write_image_to_db(post: &Post, image: &Image) -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await?;

    ensure_tables_exist(&pool).await?;

    sqlx::query(
        format!(
            "
                insert into images values (\'{}\', \'{:?}\', \'{}\');
            ",
            image.id, image.data, image.timestamp
        )
        .as_str(),
    )
    .execute(&pool)
    .await?;

    Ok(())
}

// Better ways to do this?
pub async fn generate_image_id(image_data: &Vec<u8>) -> Uuid {
    return Uuid::new_v4();
}
