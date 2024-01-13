use backend::database::{
    sql_helpers::get_pool,
    types::{comment::Comment, image::Image, post::Post, user::User},
};
use my_sqlx_crud::traits::Crud;

mod common;
#[test]
fn test_user_table() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let pool = rt.block_on(get_pool());
    let random_user = User::random();
    println!("{:?}", random_user);
    let created_post = rt.block_on(random_user.create(&pool)).unwrap();
    let fetched_post = rt
        .block_on(User::by_id(&pool, created_post.id))
        .unwrap()
        .unwrap();

    assert!(created_post == fetched_post);
}

#[test]
fn test_comment_table() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let pool = rt.block_on(get_pool());

    let created_obj = rt.block_on(Comment::random().create(&pool)).unwrap();
    let fetched_obj = rt
        .block_on(Comment::by_id(&pool, created_obj.id))
        .unwrap()
        .unwrap();

    assert!(created_obj == fetched_obj);
}

#[test]
fn test_image_table() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let pool = rt.block_on(get_pool());

    let created_obj = rt.block_on(Image::random().create(&pool)).unwrap();
    let fetched_obj = rt
        .block_on(Image::by_id(&pool, created_obj.id))
        .unwrap()
        .unwrap();

    assert!(created_obj == fetched_obj);
}

#[test]
fn test_post_table() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let pool = rt.block_on(get_pool());

    let created_obj = rt.block_on(Post::random().create(&pool)).unwrap();
    let fetched_obj = rt
        .block_on(Post::by_id(&pool, created_obj.id))
        .unwrap()
        .unwrap();

    assert!(created_obj == fetched_obj);
}
