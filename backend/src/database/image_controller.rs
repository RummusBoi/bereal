use crate::general_helpers::ENV_VARS;

use super::types::image::Image;

fn get_mock_data() -> Vec<Image> {
    return vec![
        Image {
            id: "1".to_string(),
            timestamp: 0,
            data: vec![0, 1, 2],
        },
        Image {
            id: "2".to_string(),
            timestamp: 1,
            data: vec![0, 5, 10],
        },
        Image {
            id: "3".to_string(),
            timestamp: 2,
            data: vec![10, 20, 30],
        },
    ];
}

pub fn read_images<'a>(ids: &'a Vec<String>) -> impl Iterator<Item = Image> + 'a {
    if ENV_VARS.use_mocked_database {
        return get_mock_data()
            .into_iter()
            .filter(|image| ids.contains(&image.id))
            .map(|image| image.clone());
    } else {
        todo!("Implement this part of the database interaction");
    }
}
