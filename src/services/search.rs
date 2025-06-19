use crate::services::payload_generator::generate_search_payload;
use crate::utils::http_client::post_json;
use crate::{models::search::SearchRequest, state::AppState};
use axum::{extract::State, Json};
use serde_json::json;
use std::time::Duration;
use tokio::sync::oneshot;
use tracing::info;
use uuid::Uuid;

pub async fn handle_search(
    State(app_state): State<AppState>,
    Json(_req): Json<SearchRequest>,
) -> impl axum::response::IntoResponse {
    let txn_id = format!("txn-{}", Uuid::new_v4());
    let message_id = format!("msg-{}", Uuid::new_v4());

    // Set up oneshot channel
    let (tx, rx) = oneshot::channel();
    app_state
        .shared_state
        .pending_searches
        .lock()
        .await
        .insert(txn_id.clone(), tx);

    let config = app_state.config.clone();

    // Construct the payload for the search request
    let payload = generate_search_payload(&config, &txn_id, &message_id, &_req.query);
    info!(
        "payload for search:\n{}",
        serde_json::to_string_pretty(&payload).unwrap()
    );

    let adapter_url = format!("{}/search", config.bap.caller_uri);

    // Call the `post_json` function to send the payload
    match post_json(&adapter_url, payload).await {
        Ok(_) => {
            // Wait for `on_search` webhook response
            match tokio::time::timeout(Duration::from_secs(30), rx).await {
                Ok(Ok(result)) => Json(result),
                Ok(Err(_)) => Json(json!({ "error": "response channel closed" })),
                Err(_) => Json(json!({ "error": "timeout waiting for on_search" })),
            }
        }
        Err(e) => Json(json!({ "error": "Failed to send search", "details": e.to_string() })),
    }
}
