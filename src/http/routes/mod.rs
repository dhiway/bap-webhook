pub mod search;
pub mod webhook;

use crate::models::webhook::HealthResponse;
use crate::state::AppState;
use axum::{response::IntoResponse, routing::get, Json, Router};
use chrono::Utc;

async fn health_check() -> impl IntoResponse {
    let response = HealthResponse {
        status: "OK",
        timestamp: Utc::now().to_rfc3339(),
    };

    Json(response)
}

pub fn create_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(health_check))
        .nest("/api", search::routes(app_state.clone()))
        .merge(webhook::routes(app_state))
}
