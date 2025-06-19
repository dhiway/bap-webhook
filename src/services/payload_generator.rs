use crate::config::AppConfig;
use chrono::Utc;
use serde_json::{json, Value};

fn generate_context(config: &AppConfig, txn_id: &str, message_id: &str) -> Value {
    let now = Utc::now().to_rfc3339();

    json!({
        "action": "search",
        "bap_id": config.bap.id,
        "bap_uri": config.bap.caller_uri,
        "domain": config.bap.domain,
        "message_id": message_id,
        "transaction_id": txn_id,
        "timestamp": now,
        "ttl": "PT30S",
        "version": config.bap.version
    })
}

pub fn generate_search_payload(
    config: &AppConfig,
    txn_id: &str,
    message_id: &str,
    query: &Value,
) -> Value {
    let context = generate_context(config, txn_id, message_id);

    json!({
        "context": context,
        "message": {
            "intent": {
                "item": {
                    "descriptor": {
                        "name": query
                    }
                }
            }
        }
    })
}
