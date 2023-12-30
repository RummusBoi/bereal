use crate::database::user_controller::read_users;

mod database;
mod general_helpers;
fn main() {
    let users = read_users(vec!["rasmus".to_string(), "jonathan".to_string()]);
    println!("{users:?}");
    // let images = read_images(vec!["rasmus_img".to_string()]);
    // println!("{images:?}");
    // let comments = read_comments(images[0].comments.clone());
    // println!("{comments:?}");
}
