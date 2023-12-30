use super::types::image::Image;

fn get_mock_data() -> Vec<Image> {
    return vec![
        Image::new(vec![5, 12, 13]),
        Image::new(vec![1, 5, 12]),
        Image::new(vec![12, 12, 15]),
    ];
}

pub fn read_images(ids: Vec<String>) -> Vec<Image> {
    return get_mock_data()
        .iter()
        .filter(|image| ids.contains(&image.id))
        .map(|image| image.clone())
        .collect::<Vec<Image>>();
}
