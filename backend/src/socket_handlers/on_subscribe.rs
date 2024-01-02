use std::borrow::Borrow;

use axum::extract::ws::WebSocket;

use crate::{
    database::{
        comment_controller::read_comments,
        image_controller::read_images,
        post_controller::read_posts_for_users,
        types::{comment::Comment, image::Image, post::Post},
        user_controller::read_user,
    },
    general_helpers::VectorTools,
};

use super::types::{AppError, InitialState, PostDTO, SocketData, SocketEventType, SocketResponse};

pub async fn on_subscribe(socket: &mut WebSocket, user_id: &String) {
    println!("Subscribed!");
    let socket_resp = match fetch_initial_state(user_id) {
        Ok(initial_state) => SocketResponse {
            data_type: SocketEventType::InitialState,
            data: SocketData::InitialState(initial_state),
        },
        Err(error) => {
            println!("{:?}", error);
            SocketResponse {
                data_type: SocketEventType::Error,
                data: SocketData::ErrorMessage(format!(
                    "Failed when fetching posts. Try again later."
                )),
            }
        }
    };
    println!("Sending data {:?}", socket_resp);
    socket.send(socket_resp.serialize_for_socket()).await;
}

fn fetch_initial_state(user_id: &String) -> Result<InitialState, AppError> {
    /*
       Function used to fetch initial state from database. Will make multiple calls to construct an InitialState struct.
    */
    let user = read_user(user_id)?;
    let posts = read_posts_for_users(&user.friends).collect::<Vec<Post>>();
    let image_ids = &posts.map(|post| post.image.clone());

    let images: Vec<Image> = read_images(image_ids).collect();

    let all_comments = read_comments(posts.flat_map(|post| post.comments.clone()));
    let post_dtos = posts.map(|post| PostDTO {
        id: post.id.clone(),
        timestamp: post.timestamp,
        image: images.find(|image| image.id == post.image).unwrap().clone(),
        comments: get_comments_for_post(post, &all_comments)
            .iter()
            .map(|comment| comment.clone().clone())
            .collect(),
    });

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
