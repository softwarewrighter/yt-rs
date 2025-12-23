//! Shutdown endpoint for graceful server termination.

use std::sync::Arc;

use axum::{Extension, Json, Router, http::StatusCode, routing::post};
use serde::Serialize;
use tokio::sync::Notify;

use crate::state::AppState;

/// Shutdown state shared between route and server.
#[derive(Clone)]
pub struct ShutdownState {
    notify: Arc<Notify>,
}

impl ShutdownState {
    /// Creates a new shutdown state.
    pub fn new() -> Self {
        Self {
            notify: Arc::new(Notify::new()),
        }
    }

    /// Returns a future that completes when shutdown is triggered.
    pub async fn wait(&self) {
        self.notify.notified().await;
    }

    /// Triggers the shutdown signal.
    pub fn trigger(&self) {
        self.notify.notify_one();
    }
}

impl Default for ShutdownState {
    fn default() -> Self {
        Self::new()
    }
}

/// Shutdown response.
#[derive(Serialize)]
pub struct ShutdownResponse {
    pub message: String,
}

/// Returns shutdown routes.
pub fn routes(shutdown: ShutdownState) -> Router<AppState> {
    Router::new()
        .route("/shutdown", post(shutdown_handler))
        .layer(axum::Extension(shutdown))
}

/// Shutdown handler triggers graceful server shutdown.
async fn shutdown_handler(
    Extension(shutdown): Extension<ShutdownState>,
) -> (StatusCode, Json<ShutdownResponse>) {
    tracing::info!("Shutdown requested via API");
    shutdown.trigger();
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
    async fn test_shutdown_state_trigger() {
        let state = ShutdownState::new();
        let state_clone = state.clone();

        // Spawn a task that waits for shutdown
        let handle = tokio::spawn(async move {
            state_clone.wait().await;
            true
        });

        // Trigger shutdown
        state.trigger();

        // The wait should complete
        let result = tokio::time::timeout(std::time::Duration::from_secs(1), handle).await;
        assert!(result.is_ok());
    }
}
