//! API routes for the backend server.

mod health;
mod projects;
mod videos;

use axum::Router;

use crate::state::AppState;

/// Creates the main router with all API routes.
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1", api_routes())
        .with_state(state)
}

/// Creates the API routes.
fn api_routes() -> Router<AppState> {
    Router::new()
        .merge(health::routes())
        .merge(projects::routes())
        .merge(videos::routes())
}
