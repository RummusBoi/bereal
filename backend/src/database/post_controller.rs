use crate::general_helpers::{VectorTools, ENV_VARS};

use super::types::{image::Image, post::Post};

fn get_mock_data() -> Vec<Post> {
    return vec![Post {
        id: "1".to_string(),
        poster_id: "jonathan".to_string(),
        image: "1".to_string(),
        comments: vec!["1".to_string(), "2".to_string()],
        timestamp: 0,
    }];
}

pub fn read_posts<'a>(ids: &'a Vec<String>) -> impl Iterator<Item = Post> + 'a {
    if ENV_VARS.use_mocked_database {
        let posts = get_mock_data();
        return posts.into_iter().filter(|post| ids.contains(&post.id));
    } else {
        todo!("Implement this part of the database interaction");
    }
}

pub fn read_posts_for_users<'a>(user_ids: &'a Vec<String>) -> impl Iterator<Item = Post> + 'a {
    if ENV_VARS.use_mocked_database {
        let posts = get_mock_data();
        return posts
            .into_iter()
            .filter(|post| user_ids.contains(&post.poster_id));
    } else {
        todo!("Implement this part of the database interaction");
    }
}
