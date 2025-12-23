//! Workspace save/restore endpoints.

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use serde::Serialize;
use yt_rs_shared::Project;

use crate::state::{self, AppState};

/// Response for project operations.
#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    pub project: Project,
}

/// Returns workspace routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/workspace/save", post(save_workspace))
        .route("/workspace/restore", get(restore_workspace))
}

/// Saves the current workspace to disk.
async fn save_workspace(
    State(state): State<AppState>,
    Json(project): Json<Project>,
) -> Result<StatusCode, StatusCode> {
    state::save_workspace(&state, &project)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Restores the workspace from disk.
async fn restore_workspace(
    State(state): State<AppState>,
) -> Result<Json<ProjectResponse>, StatusCode> {
    state::load_workspace(&state)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(|project| Json(ProjectResponse { project }))
        .ok_or(StatusCode::NOT_FOUND)
}
