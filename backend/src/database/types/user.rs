use my_sqlx_crud_macro::SqlxCrud;
use sqlx::prelude::FromRow;

use crate::database::helpers::get_timestamp;

type UserId = i32;

#[derive(Clone, Debug, FromRow, SqlxCrud, PartialEq)]
#[database(Postgres)]
pub struct User {
    pub id: UserId,
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
        let user = User {
            id: uuid::Uuid::new_v4().to_u128_le() as i32,
            friends: friends,
            timestamp: get_timestamp(),
        };

        println!("Created {:?}", user);
        user
    }
}
