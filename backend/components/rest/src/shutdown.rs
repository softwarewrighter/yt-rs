//! Shutdown endpoint for graceful server termination.

use axum::{Extension, Json, Router, http::StatusCode, routing::post};
use serde::Serialize;
use yt_rs_server::ShutdownSignal;

/// Shutdown response.
#[derive(Serialize)]
pub struct ShutdownResponse {
    pub message: String,
}

/// Shutdown route configuration.
pub struct ShutdownRoute {
    signal: ShutdownSignal,
}

impl ShutdownRoute {
    /// Creates a new shutdown route with the given signal.
    pub fn new(signal: ShutdownSignal) -> Self {
        Self { signal }
    }

    /// Returns the router for shutdown endpoints.
    pub fn routes<S>(self) -> Router<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        Router::new()
            .route("/shutdown", post(shutdown_handler))
            .layer(Extension(self.signal))
    }
}

async fn shutdown_handler(
    Extension(signal): Extension<ShutdownSignal>,
) -> (StatusCode, Json<ShutdownResponse>) {
    tracing::info!("Shutdown requested via API");
    signal.trigger();
    (
        StatusCode::OK,
        Json(ShutdownResponse {
            message: "Server shutting down".to_string(),
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_shutdown_triggers_signal() {
        let signal = ShutdownSignal::default();
        let signal_clone = signal.clone();

        let handle = tokio::spawn(async move {
            signal_clone.wait().await;
            true
        });

        // Yield to ensure spawned task starts waiting
        tokio::task::yield_now().await;
        signal.trigger();

        let result = tokio::time::timeout(std::time::Duration::from_secs(1), handle).await;
        assert!(result.is_ok());
    }
}
