use axum::extract::ws::{Message, WebSocket};
use futures::{stream::SplitSink, SinkExt};

use crate::database::{
    comment_controller::read_comments,
    image_controller::read_images,
    post_controller::read_posts_for_users,
    types::{comment::Comment, post::Post},
    user_controller::read_user,
};

use super::types::{AppError, InitialState, PostDTO, SocketData, SocketEventType, SocketResponse};

pub async fn on_subscribe(client_sender: &mut SplitSink<WebSocket, Message>, user_id: i32) {
    println!("Subscribed!");
    let socket_resp = match fetch_initial_state(user_id).await {
        Ok(initial_state) => SocketResponse {
            data_type: SocketEventType::InitialState,
            data: SocketData::InitialState(initial_state),
        },
        Err(error) => {
            println!("{:?}", error);
            SocketResponse {
                data_type: SocketEventType::Error,
                data: SocketData::String(format!("Failed when fetching posts. Try again later.")),
            }
        }
    };
    println!("Sending data {:?}", socket_resp);
    client_sender.send(socket_resp.serialize_for_socket()).await;
}

async fn fetch_initial_state(user_id: i32) -> Result<InitialState, AppError> {
    /*
       Function used to fetch initial state from database. Will make multiple calls to construct an InitialState struct.
    */

    let asd = read_user(user_id).await;
    let user = match asd {
        Ok(u) => u,
        Err(e) => {
            println!("{:?}", e);
            return Err(e);
        }
    };

    let posts = read_posts_for_users([user.friends, vec![user.id]].concat()).await;
    let image_ids = posts.iter().map(|post| post.image).collect();

    let images = read_images(image_ids).await;

    let all_comments = read_comments(
        posts
            .iter()
            .flat_map(|post| post.comments.clone())
            .collect(),
    )
    .await?;
    let post_dtos = posts
        .iter()
        .map(|post| PostDTO {
            id: post.id.clone(),
            poster_id: post.poster_id.clone(),
            timestamp: post.timestamp as u128,
            image: images
                .iter()
                .find(|image| image.id == post.image)
                .unwrap()
                .clone(),
            comments: get_comments_for_post(post, &all_comments)
                .iter()
                .map(|&comment| comment.clone())
                .collect(),
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
