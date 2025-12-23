//! Router composition for REST API.

use axum::Router;
use yt_rs_server::ShutdownSignal;

use crate::health;
use crate::shutdown::ShutdownRoute;

/// Creates the API router with all endpoints.
pub fn create_router<S>(shutdown: ShutdownSignal) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .merge(health::routes())
        .merge(ShutdownRoute::new(shutdown).routes())
}
