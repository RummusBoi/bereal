mod db_helpers;
mod types;

use database::{
    comments::read_comments, images::read_images, types::comment::Comment, users::read_users,
};

mod database;

fn main() {
    let users = read_users(vec!["rasmus".to_string(), "jonathan".to_string()]);
    println!("{users:?}");
    // let images = read_images(vec!["rasmus_img".to_string()]);
    // println!("{images:?}");
    // let comments = read_comments(images[0].comments.clone());
    // println!("{comments:?}");
}
