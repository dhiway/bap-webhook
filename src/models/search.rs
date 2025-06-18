use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub query: Value,
}
