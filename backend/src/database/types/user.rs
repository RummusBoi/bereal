use my_sqlx_crud::traits::Schema;
use my_sqlx_crud_macro::SqlxCrud;
use sqlx::Postgres;
use sqlx::{prelude::FromRow, Pool};

use crate::database::helpers::get_timestamp;
use crate::socket_handlers::types::AppError;
#[derive(Clone, Debug, FromRow, SqlxCrud, PartialEq)]
#[database(Postgres)]
pub struct User {
    pub id: i32,
    pub friends: Vec<i32>,
    pub timestamp: i64,
}

impl User {
    pub fn random() -> Self {
        User {
            id: uuid::Uuid::new_v4().to_u128_le() as i32,
            friends: vec![],
            timestamp: get_timestamp(),
        }
    }

    pub fn new(friends: Vec<i32>) -> Self {
        User {
            id: uuid::Uuid::new_v4().to_u128_le() as i32,
            friends: friends,
            timestamp: get_timestamp(),
        }
    }
}
