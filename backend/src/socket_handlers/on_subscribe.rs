use std::borrow::Borrow;

use axum::extract::ws::WebSocket;

use crate::database::{
    comment_controller::read_comments,
    image_controller::read_images,
    post_controller::read_posts,
    types::{comment::Comment, post::Post},
    user_controller::{read_user, read_users},
};

use super::types::{AppError, InitialState, PostDTO};

pub fn on_subscribe(socket: &WebSocket, user_id: &String) {
    let initial_state = fetch_initial_state(user_id);
    // TODO send initial state to user here
}

fn fetch_initial_state(user_id: &String) -> Result<InitialState, AppError> {
    /*
       Function used to fetch initial state from database. Will make multiple calls to construct an InitialState struct.
    */
    let user = read_user(user_id)?;
    let friends = read_users(
        user.friends
            .iter()
            .map(|friend| friend.borrow())
            .collect::<Vec<_>>(),
    );

    let posts = read_posts(friends.iter().map(|friend| friend.id.clone()).collect());
    let images = read_images(posts.iter().map(|post| post.image.clone()).collect());

    let all_comments = read_comments(
        posts
            .iter()
            .map(|post| post.comments.clone())
            .flatten()
            .collect(),
    );

    let post_dtos = posts
        .iter()
        .map(|post| {
            let comments = get_comments_for_post(post, &all_comments);

            PostDTO {
                id: post.id.clone(),
                timestamp: post.timestamp,
                image: images
                    .iter()
                    .find(|image| image.id == post.image)
                    .unwrap() // TODO assert that if post points to image then that image exists
                    .clone(),
                comments: comments.iter().map(|c| c.to_owned().to_owned()).collect(),
            }
        })
        .collect();

    Ok(InitialState { posts: post_dtos })
}

fn get_comments_for_post<'a>(post: &Post, all_comments: &'a Vec<Comment>) -> Vec<&'a Comment> {
    post.comments
        .iter()
        .map(|comment_id| {
            all_comments
                .iter()
                .find(|comment| &comment.id == comment_id)
                .unwrap() // TODO assert that if a post points to a comment, then that comment exists
        })
        .collect()
}
