use backend::database::{
    sql_helpers::get_pool,
    types::{comment::Comment, image::Image, post::Post, user::User},
};
use my_sqlx_crud::traits::Crud;

use crate::common::setup_database::create_simple_friendgroup;

mod common;
#[tokio_shared_rt::test(shared)]

async fn test_user_table() {
    let pool = get_pool().await;
    let random_user = User::random();
    println!("{:?}", random_user);
    let created_post = random_user.create(&pool).await.unwrap();
    let fetched_post = User::by_id(&pool, created_post.id).await.unwrap().unwrap();

    assert!(created_post == fetched_post);
}

#[tokio_shared_rt::test(shared)]
async fn test_comment_table() {
    let pool = get_pool().await;

    let created_obj = Comment::random().create(&pool).await.unwrap();
    let fetched_obj = Comment::by_id(&pool, created_obj.id)
        .await
        .unwrap()
        .unwrap();

    assert!(created_obj == fetched_obj);
}

#[tokio_shared_rt::test(shared)]
async fn test_image_table() {
    let pool = get_pool().await;

    let created_obj = Image::random().create(&pool).await.unwrap();
    let fetched_obj = Image::by_id(&pool, created_obj.id).await.unwrap().unwrap();

    assert!(created_obj == fetched_obj);
}

#[tokio_shared_rt::test(shared)]
async fn test_post_table() {
    let pool = get_pool().await;

    let created_obj = Post::random().create(&pool).await.unwrap();
    let fetched_obj = Post::by_id(&pool, created_obj.id).await.unwrap().unwrap();

    assert!(created_obj == fetched_obj);
}

#[tokio_shared_rt::test(shared)]
async fn test_can_create_simple_friend_group() {
    let db_state = create_simple_friendgroup().await;

    assert!(db_state.posts.len() == 6);
}
