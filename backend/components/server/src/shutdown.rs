//! Graceful shutdown handling.

use std::sync::Arc;
use tokio::sync::Notify;

/// Signal for graceful shutdown.
#[derive(Clone, Default)]
pub struct ShutdownSignal {
    notify: Arc<Notify>,
}

impl ShutdownSignal {
    /// Triggers the shutdown signal.
    pub fn trigger(&self) {
        self.notify.notify_waiters();
    }

    /// Waits for the shutdown signal.
    pub async fn wait(&self) {
        self.notify.notified().await;
    }
}
