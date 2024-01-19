use backend::database::{
    sql_helpers::get_pool,
    types::{comment::Comment, image::Image, post::Post, user::User}, canned_queries::post_queries::apply_post_deletion_operations,
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

// Does removing a post also remove the associated comments?
#[tokio_shared_rt::test(shared)]
async fn test_delete_post_deletes_associated_comments() {
    let db_state = create_simple_friendgroup().await;

    let post = match db_state.posts.first() {
        Some(v) => v,
        _ => {
            println!("test_delete_post_deletes_associated_comments() couldn't find posts in db_state. Wtf???");
            assert!("you absolute" == "bloody bastard");
            return;
        },
    };

    let comment_ids = &post.comments;

    match apply_post_deletion_operations(post.clone()).await {
        Ok(_) => "Hee hee!",
        Err(_) => {
            println!("test_delete_post_deletes_associated_comments() failed");
            assert!("you absolute" == "bloody bastard");
            return;
        }
    };

    let pool = &get_pool().await;

    for id in comment_ids {
        assert_eq!(Comment::by_id(pool, *id).await.unwrap(), None);
    }
}
