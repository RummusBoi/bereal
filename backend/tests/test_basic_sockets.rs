use std::{collections::HashSet, thread::sleep, time::Duration};

use backend::{
    self,
    database::{image_controller, post_controller, sql_helpers::get_pool, types::user::User},
    socket_handlers::types::{
        CreateCommentDTO, CreateImageDTO, CreatePostDTO, InitialState, SocketEventType,
        SocketResponse,
    },
};
use my_sqlx_crud::traits::{Crud, Schema};

use crate::common::{
    setup_database::{create_friend_pair, create_simple_friendgroup},
    setup_socket_conn::connect_to_localhost,
};
mod common;

#[tokio_shared_rt::test(shared)]
async fn test_on_subscribe() {
    let pool = get_pool().await;
    let user = User::new(vec![]).create(&pool).await.unwrap();
    let mut socket = connect_to_localhost(user.id);
    let message = socket.read().unwrap();

    let socket_resp = match message {
        tungstenite::Message::Text(data) => {
            serde_json::from_slice::<SocketResponse>(data.as_bytes()).unwrap()
        }
        _ => todo!(),
    };
    println!("{:?}", socket_resp);
}

#[tokio_shared_rt::test(shared)]
async fn test_can_aggregate_posts() {
    let db_state = create_simple_friendgroup().await;

    let mut socket = connect_to_localhost(db_state.user.id);
    let message = socket.read().unwrap();

    let socket_resp = match message {
        tungstenite::Message::Text(data) => {
            serde_json::from_slice::<SocketResponse>(data.as_bytes()).unwrap()
        }
        _ => panic!(),
    };
    let data: InitialState = socket_resp.try_into().unwrap();

    // ---
    //  Assert that the correct posts by ID were returned.
    // ---
    let received_post_ids: HashSet<i32> = HashSet::from_iter(data.posts.iter().map(|p| p.id));
    let post_ids: HashSet<i32> = HashSet::from_iter(db_state.posts.iter().map(|p| p.id));

    assert_eq!(received_post_ids, post_ids);

    // ---
    //  Assert that each post refers to the correct user, comments and image
    // ---
    for post in db_state.posts.iter() {
        let received_post = data.posts.iter().find(|p| p.id == post.id()).unwrap();
        assert_eq!(post.image, received_post.image.id);
        assert_eq!(post.poster_id, received_post.poster_id);
        assert_eq!(
            post.comments,
            received_post
                .comments
                .iter()
                .map(|c| c.id)
                .collect::<Vec<i32>>()
        );
    }
}

#[tokio_shared_rt::test(shared)]
async fn can_send_create_post_dto() {
    // ---
    //  'User' creates the following post...
    // ---

    let pool = get_pool().await;
    let user = User::new(vec![]).create(&pool).await.unwrap();
    let image_data = vec![123, 123, 123];
    let post_dto = CreatePostDTO {
        image: CreateImageDTO {
            data: image_data.clone(),
        },
    };

    let mut socket = connect_to_localhost(user.id);

    socket.read().unwrap();

    let message = SocketResponse {
        data_type: backend::socket_handlers::types::SocketEventType::PostCreated,
        data: backend::socket_handlers::types::SocketData::CreatePostDTO(post_dto),
    };
    socket.write(message.serialize_for_tung_socket()).unwrap();
    socket.flush().unwrap();

    // ---
    //  Fetch posts made by the current user
    // ---

    sleep(Duration::from_millis(10));
    let posts = post_controller::read_posts_for_users(vec![user.id]).await;

    assert_eq!(posts.len(), 1); // we have only the post just created

    let image = image_controller::read_image(posts[0].image).await.unwrap();
    assert_eq!(image.data, image_data);
}

#[tokio_shared_rt::test(shared)]
async fn friend_receives_new_post() {
    // ---
    //  Set the user and the friend up!
    // ---

    let pool = get_pool().await;
    let mut user = User::new(vec![]).create(&pool).await.unwrap();
    let mut friend = User::new(vec![]).create(&pool).await.unwrap();

    user = User {
        id: user.id,
        friends: vec![friend.id],
        timestamp: user.timestamp,
    }
    .update(&pool)
    .await
    .unwrap();

    friend = User {
        id: friend.id,
        friends: vec![user.id],
        timestamp: friend.timestamp,
    }
    .update(&pool)
    .await
    .unwrap();

    // ---
    //  Connect the user and the friend to the server. Read on_subscribe event.
    // ---

    let mut user_conn = connect_to_localhost(user.id);
    let mut friend_conn = connect_to_localhost(friend.id);

    user_conn.read().unwrap();
    friend_conn.read().unwrap();

    let image_data = vec![32, 32, 23];
    let post_dto = CreatePostDTO {
        image: CreateImageDTO {
            data: image_data.clone(),
        },
    };
    let message = SocketResponse {
        data_type: backend::socket_handlers::types::SocketEventType::PostCreated,
        data: backend::socket_handlers::types::SocketData::CreatePostDTO(post_dto),
    };
    user_conn
        .write(message.serialize_for_tung_socket())
        .unwrap();
    user_conn.flush().unwrap();

    let socket_resp = match friend_conn.read().unwrap() {
        tungstenite::Message::Text(data) => {
            serde_json::from_slice::<SocketResponse>(data.as_bytes()).unwrap()
        }
        _ => todo!(),
    };

    assert!(matches!(
        socket_resp.data_type,
        SocketEventType::PostCreated
    ));

    let received_post = socket_resp.data.into_post_dto().unwrap();

    assert_eq!(received_post.poster_id, user.id);
    assert_eq!(received_post.image.data, image_data);
}

#[tokio_shared_rt::test(shared)]
async fn friend_receives_new_comment() {
    let (user1, user2) = create_friend_pair().await;
    let post = post_controller::create_post(vec![1, 2, 3], user1.id)
        .await
        .unwrap();

    let comment_content = "this is my comment!".to_string();
    let comment_dto = CreateCommentDTO {
        data: comment_content.clone(),
        post_id: post.id,
    };

    let mut user1_conn = connect_to_localhost(user1.id);
    let mut user2_conn = connect_to_localhost(user2.id);

    user1_conn.read().unwrap();
    user2_conn.read().unwrap();

    let message = SocketResponse {
        data_type: backend::socket_handlers::types::SocketEventType::CommentCreated,
        data: backend::socket_handlers::types::SocketData::CreateCommentDTO(comment_dto),
    };

    user1_conn
        .send(message.serialize_for_tung_socket())
        .unwrap();

    let socket_resp = match user2_conn.read().unwrap() {
        tungstenite::Message::Text(data) => {
            serde_json::from_slice::<SocketResponse>(data.as_bytes()).unwrap()
        }
        _ => todo!(),
    };

    assert!(matches!(
        socket_resp.data_type,
        SocketEventType::CommentCreated
    ));
    let received_comment = socket_resp.data.into_comment_dto().unwrap();

    assert_eq!(received_comment.data, comment_content);
    assert_eq!(received_comment.poster_id, user1.id);
}
