use std::sync::Arc;

use tokio::sync::RwLock;

use crate::socket_handlers::types::SocketResponse;

#[derive(Clone, Debug)]
pub struct SenderWrapper {
    pub sender: tokio::sync::mpsc::Sender<SocketResponse>,
    pub user_id: i32,
}

#[derive(Clone)]
pub struct AppState {
    pub internal_conns: Arc<RwLock<Vec<SenderWrapper>>>,
}
