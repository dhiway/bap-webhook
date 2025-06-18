use crate::{
    config::AppConfig,
    http::routes::create_routes,
    state::{AppState, SharedState},
};
use std::sync::Arc;
use tokio::{net::TcpListener, sync::watch, task::JoinHandle};
use tracing::info;

pub async fn start_http_server(
    config: AppConfig,
    shutdown_rx: watch::Receiver<()>,
) -> Result<
    JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>,
    Box<dyn std::error::Error + Send + Sync>,
> {
    let http_addr = format!("{}:{}", config.http.address, config.http.port);
    let listener = tokio::net::TcpListener::bind(http_addr.clone()).await?;
    info!("🚀 Starting BAP-ADAPTER server on {:?}", http_addr);

    let shared_state = SharedState::default();
    let app_state = AppState {
        config: Arc::new(config.clone()),
        shared_state,
    };

    let http_server = tokio::spawn(run_http_server(listener, shutdown_rx, app_state));

    Ok(http_server)
}

pub async fn run_http_server(
    listener: TcpListener,
    mut shutdown_rx: watch::Receiver<()>,
    app_state: AppState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = create_routes(app_state);

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(async move {
            shutdown_rx.changed().await.ok();
        })
        .await?;

    Ok(())
}
