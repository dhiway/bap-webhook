use crate::models::webhook::{Ack, AckResponse, AckStatus, WebhookPayload};
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use tracing::info;

pub async fn webhook_handler(
    Path(action): Path<String>,
    State(app_state): State<AppState>,
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    info!(
        "webhook called: action = {:?}, txn_id = {:?}",
        action, payload.context.transaction_id
    );

    let txn_id = payload.context.transaction_id.clone();
    info!(
        "webhook called: action = {:?}, txn_id = {:?} (searching in shared_state)",
        action, txn_id
    );

    let mut pending = app_state.shared_state.pending_searches.lock().await;
    if let Some(sender) = pending.remove(&txn_id) {
        info!("✅ Found pending sender. Sending payload to waiting /search...");
        let _ = sender.send(serde_json::json!(payload));
    } else {
        info!("❌ No matching sender found for txn_id = {}", txn_id);
    }

    Json(AckResponse {
        message: AckStatus {
            ack: Ack { status: "ACK" },
        },
    })
}
