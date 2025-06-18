use crate::{models::search::SearchRequest, state::AppState};
use axum::{extract::State, Json};
use reqwest::Client;
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
    let client = Client::new();
    let now = chrono::Utc::now().to_rfc3339();

    let payload = json!({
        "context": {
            "action": "search",
            "bap_id": config.bap.id,
            "bap_uri": config.bap.caller_uri,
            "domain": config.bap.domain,
            "message_id": message_id,
            "transaction_id": txn_id,

            "timestamp": now,
            "ttl": "PT30S",
            "version": config.bap.version
        },
        "message": {
            "intent": {
                "item": {
                    "descriptor": {
                        "name": _req.query
                    }
                }
            }
        }
    });
    info!(
        "payload for search:\n{}",
        serde_json::to_string_pretty(&payload).unwrap()
    );

    let adapter_url = format!("{}/search", config.bap.caller_uri);

    // Fire the request to BAP adapter
    let res = client.post(adapter_url).json(&payload).send().await;

    if let Err(e) = res {
        return Json(json!({ "error": "Failed to send search", "details": e.to_string() }));
    }

    // Wait for `on_search` webhook
    match tokio::time::timeout(Duration::from_secs(30), rx).await {
        Ok(Ok(result)) => Json(result),
        Ok(Err(_)) => Json(json!({ "error": "response channel closed" })),
        Err(_) => Json(json!({ "error": "timeout waiting for on_search" })),
    }
}
