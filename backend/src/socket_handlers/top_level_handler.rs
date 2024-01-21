use crate::database::comment_controller;
use crate::database::image_controller;
use crate::database::user_controller;
use crate::socket_handlers::types::PostDTO;
use crate::{
    database::post_controller, socket_handlers::types::CreatePostDTO, types::SenderWrapper,
};
use axum::extract::ws::{Message, WebSocket};
use futures::{
    future::join_all,
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use sqlx_core::database::HasStatementCache;
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::types::AppState;

use super::types::CreateCommentDTO;
use super::{on_subscribe::on_subscribe, types::SocketResponse};
pub async fn top_level_socket_handler(mut socket: WebSocket, user_id: i32, state: AppState) {
    // ---
    // append socket to app state
    // ---
    let (mut internal_sender, mut internal_receiver) = mpsc::channel::<SocketResponse>(128);
    let (mut client_sender, mut client_receiver) = socket.split();

    {
        let mut internal_conns = state.internal_conns.write().await;
        internal_conns.push(SenderWrapper {
            sender: internal_sender.clone(),
            user_id,
        });
    };

    on_subscribe(&mut client_sender, user_id).await;

    // tokio::spawn(receive_internal_msgs(client_sender, internal_receiver));
    tokio::join!(
        receive_internal_msgs(client_sender, internal_receiver),
        receive_client_msgs(client_receiver, user_id, state),
    );
}

async fn receive_internal_msgs(
    mut client_sender: SplitSink<WebSocket, Message>,
    mut internal_receiver: Receiver<SocketResponse>,
) {
    println!("Started receive internal msgs...");
    while let internal_msg = internal_receiver.recv().await.unwrap() {
        // ---
        // Messages sent internally on the server
        // ---

        println!("Received message internally, sending to client...");
        client_sender
            .send(internal_msg.serialize_for_socket())
            .await
            .unwrap();
    }
}

async fn receive_client_msgs(
    mut client_receiver: SplitStream<WebSocket>,
    user_id: i32,
    mut state: AppState,
) {
    while let client_msg = client_receiver.next().await.unwrap() {
        println!("Received message from client. {:?}", client_msg);

        let socket_resp: SocketResponse = client_msg.unwrap().into();
        println!("Received {:?} event!", socket_resp.data_type);
        match socket_resp.data_type {
            super::types::SocketEventType::PostCreated => {
                handle_create_post(
                    socket_resp.data.into_create_post_dto().unwrap(),
                    user_id,
                    state.clone(),
                )
                .await;
            }
            super::types::SocketEventType::CommentCreated => {
                handle_create_comment(
                    socket_resp.data.into_create_comment_dto().unwrap(),
                    user_id,
                    state.clone(),
                )
                .await
            }
            super::types::SocketEventType::FriendRequestSent => todo!(),
            // handle_friend_request_sent(
            //     socket_resp.data.into_create_comment_dto().unwrap(),
            //     user_id,
            //     state.clone(),
            // ),
            _ => panic!(),
        };
    }
}

async fn handle_create_post(post: CreatePostDTO, user_id: i32, state: AppState) {
    // ---
    //  Create post in database
    // ---
    let created_post = post_controller::create_post(post.image.data, user_id)
        .await
        .unwrap();
    let image = image_controller::read_image(created_post.image)
        .await
        .unwrap();
    let post_dto = PostDTO {
        id: created_post.id,
        poster_id: user_id,
        timestamp: created_post.timestamp as u128,
        image: image,
        comments: vec![],
    };
    // ---
    //  Find all friends of user and lookup their internal senders in app_state
    // ---
    let user = user_controller::read_user(user_id).await.unwrap();

    // ---
    //  Send update to all friends
    // ---
    let friend_conns = {
        // NOTE: We want to hold the read lock on state.internal_conns for as short as possible, so we take the lock here, copy the sender_wrappers,
        //          and then return them. When we go out of scope we lose the read lock, enabled writers to write.
        let conn_lock = state.internal_conns.read().await;
        let sender_wrappers = conn_lock
            .iter()
            .filter(|internal_conn| user.friends.contains(&internal_conn.user_id))
            .map(|sender_wrapper| sender_wrapper.to_owned())
            .collect::<Vec<SenderWrapper>>();
        sender_wrappers
    };
    println!("Found the following friend connections: {:?}", friend_conns);
    join_all(friend_conns.iter().map(|conn| {
        println!("Sending to user {}", conn.user_id);

        return conn.sender.send(SocketResponse {
            data_type: super::types::SocketEventType::PostCreated,
            data: super::types::SocketData::PostDTO(post_dto.clone()),
        });
    }))
    .await;
    println!("Finished sending stuff to friends");
}

async fn handle_create_comment(comment: CreateCommentDTO, user_id: i32, state: AppState) {
    let created_comment =
        comment_controller::create_comment(comment.post_id, user_id, comment.data)
            .await
            .unwrap();

    let user = user_controller::read_user(user_id).await.unwrap();

    let friend_conns = {
        // NOTE: We want to hold the read lock on state.internal_conns for as short as possible, so we take the lock here, copy the sender_wrappers,
        //          and then return them. When we go out of scope we lose the read lock, enabled writers to write.
        let conn_lock = state.internal_conns.read().await;
        let sender_wrappers = conn_lock
            .iter()
            .filter(|internal_conn| user.friends.contains(&internal_conn.user_id))
            .map(|sender_wrapper| sender_wrapper.to_owned())
            .collect::<Vec<SenderWrapper>>();
        sender_wrappers
    };

    join_all(friend_conns.iter().map(|conn| {
        println!("Sending to user {}", conn.user_id);

        return conn.sender.send(SocketResponse {
            data_type: super::types::SocketEventType::CommentCreated,
            data: super::types::SocketData::CommentDTO(created_comment.clone()),
        });
    }))
    .await;
}

async fn handle_friend_request_sent(target_user_id: i32, user_id: i32, state: AppState) {
    todo!("Not implemented yet. Requires change to db tables. I think")
    // let target_user = user_controller::read_user(target_user_id).await.unwrap();

    // let conn_lock = state.internal_conns.read().await;

    // let sender_wrapper = conn_lock.iter().filter(|internal_conn| internal_conn.user_id == target_user_id)
}
