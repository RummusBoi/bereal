use my_sqlx_crud_macro::SqlxCrud;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::database::helpers::get_timestamp;

type ImageId = i32;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow, SqlxCrud, PartialEq)]
#[database(Postgres)]
pub struct Image {
    pub id: ImageId,
    pub timestamp: i64,
    pub data: Vec<u8>,
}

impl Image {
    pub fn random() -> Self {
        Image {
            id: uuid::Uuid::new_v4().to_u128_le() as i32,
            timestamp: get_timestamp(),
            data: vec![1, 2, 3],
        }
    }
    pub fn new(data: Vec<u8>) -> Self {
        let image = Image {
            id: uuid::Uuid::new_v4().to_u128_le() as i32,
            timestamp: get_timestamp(),
            data: data,
        };
        println!("Created {:?}", image);
        image
    }
}
