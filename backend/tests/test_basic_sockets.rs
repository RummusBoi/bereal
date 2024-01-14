use std::collections::HashSet;

use backend::{
    self,
    socket_handlers::types::{InitialState, SocketResponse},
};
use my_sqlx_crud::traits::Schema;

use crate::common::{
    setup_database::create_simple_friendgroup, setup_socket_conn::connect_to_localhost,
};
mod common;

#[test]
fn test_on_subscribe() {
    let mut socket = connect_to_localhost(0);
    let message = socket.read().unwrap();

    let socket_resp = match message {
        tungstenite::Message::Text(data) => {
            serde_json::from_slice::<SocketResponse>(data.as_bytes()).unwrap()
        }
        tungstenite::Message::Binary(_) => todo!(),
        tungstenite::Message::Ping(_) => todo!(),
        tungstenite::Message::Pong(_) => todo!(),
        tungstenite::Message::Close(_) => todo!(),
        tungstenite::Message::Frame(_) => todo!(),
    };
    println!("{:?}", socket_resp);
}

#[test]
fn test_can_aggregate_posts() {
    let db_state = create_simple_friendgroup();

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
