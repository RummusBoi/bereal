use crate::general_helpers::ENV_VARS;
use uuid::Uuid;
use sqlx::postgres::PgPoolOptions;

use super::{
    types::{image::Image, post::Post},
    helpers::{DATABASE_URL, ensure_tables_exist},
};

fn get_mock_data() -> Vec<Image> {
    return vec![
        Image {
            id: "1".to_string(),
            timestamp: 0,
            data: vec![0, 1, 2],
        },
        Image {
            id: "2".to_string(),
            timestamp: 1,
            data: vec![0, 5, 10],
        },
        Image {
            id: "3".to_string(),
            timestamp: 2,
            data: vec![10, 20, 30],
        },
    ];
}

pub fn read_images(ids: Vec<String>) -> Vec<Image> {
    if ENV_VARS.use_mocked_database {
        return get_mock_data()
            .iter()
            .filter(|image| ids.contains(&image.id))
            .map(|image| image.clone())
            .collect::<Vec<Image>>();
    } else {
        todo!("Implement this part of the database interaction");
    }
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
