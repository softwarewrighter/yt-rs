//! API routes for the backend server.

mod health;
mod projects;
mod videos;

use axum::Router;
use axum::extract::DefaultBodyLimit;

use crate::state::AppState;

/// Maximum upload size: 500 MB
const MAX_UPLOAD_SIZE: usize = 500 * 1024 * 1024;

/// Creates the main router with all API routes.
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1", api_routes())
        .layer(DefaultBodyLimit::max(MAX_UPLOAD_SIZE))
        .with_state(state)
}

/// Creates the API routes.
fn api_routes() -> Router<AppState> {
    Router::new()
        .merge(health::routes())
        .merge(projects::routes())
        .merge(videos::routes())
}
