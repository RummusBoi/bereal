use crate::general_helpers::ENV_VARS;

use super::types::{image::Image, post::Post};

fn get_mock_data() -> Vec<Post> {
    return vec![Post {
        id: "1".to_string(),
        poster_id: "1".to_string(),
        image: "1".to_string(),
        comments: vec!["1".to_string(), "2".to_string()],
        timestamp: 0,
    }];
}

pub fn read_posts(ids: Vec<String>) -> Vec<Post> {
    if ENV_VARS.use_mocked_database {
        return get_mock_data()
            .iter()
            .filter(|image| ids.contains(&image.id))
            .map(|image| image.clone())
            .collect::<Vec<Post>>();
    } else {
        todo!("Implement this part of the database interaction");
    }
}
