use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{oneshot, Mutex};

pub type OnSearchResponse = serde_json::Value;

use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub shared_state: SharedState,
}

#[derive(Clone, Default)]
pub struct SharedState {
    pub pending_searches: Arc<Mutex<HashMap<String, oneshot::Sender<OnSearchResponse>>>>,
}
