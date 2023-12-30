use super::types::comment::Comment;

fn get_mock_data() -> Vec<Comment> {
    return vec![
        Comment::new("rasmus".to_string(), "hej".to_string()),
        Comment::new("jonathan".to_string(), "hej2".to_string()),
        Comment::new("darth vader".to_string(), "hej v3".to_string()),
    ];
}

pub fn read_comments(ids: Vec<String>) -> Vec<Comment> {
    return get_mock_data()
        .iter()
        .filter(|comment| ids.contains(&comment.id))
        .map(|comment| comment.clone())
        .collect::<Vec<Comment>>();
}
