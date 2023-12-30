#[derive(Clone, Debug)]
pub struct Comment {
    pub sender: String,
    pub post_id: String,
    pub message: String,
    pub timestamp: u128,
}
