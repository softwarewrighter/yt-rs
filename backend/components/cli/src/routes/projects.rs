//! Project CRUD endpoints.

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yt_rs_shared::Project;

use crate::state::AppState;

/// Request to create a new project.
#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
}

/// Response for project operations.
#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    pub project: Project,
}

/// Response for listing projects.
#[derive(Debug, Serialize)]
pub struct ProjectListResponse {
    pub projects: Vec<ProjectSummary>,
}

/// Summary of a project for listing.
#[derive(Debug, Serialize)]
pub struct ProjectSummary {
    pub id: Uuid,
    pub name: String,
    pub node_count: usize,
    pub updated_at: String,
}

impl From<&Project> for ProjectSummary {
    fn from(project: &Project) -> Self {
        Self {
            id: project.id,
            name: project.name.clone(),
            node_count: project.nodes.len(),
            updated_at: project.updated_at.to_rfc3339(),
        }
    }
}

/// Returns project routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/projects", get(list_projects).post(create_project))
        .route(
            "/projects/{id}",
            get(get_project).put(update_project).delete(delete_project),
        )
}

/// Lists all projects.
async fn list_projects(State(state): State<AppState>) -> Json<ProjectListResponse> {
    let projects = state.projects().read().await;
    let summaries = projects.values().map(ProjectSummary::from).collect();
    Json(ProjectListResponse {
        projects: summaries,
    })
}

/// Creates a new project.
async fn create_project(
    State(state): State<AppState>,
    Json(request): Json<CreateProjectRequest>,
) -> (StatusCode, Json<ProjectResponse>) {
    let project = Project::new(request.name);
    state
        .projects()
        .write()
        .await
        .insert(project.id, project.clone());
    (StatusCode::CREATED, Json(ProjectResponse { project }))
}

/// Gets a project by ID.
async fn get_project(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ProjectResponse>, StatusCode> {
    state
        .projects()
        .read()
        .await
        .get(&id)
        .cloned()
        .map(|project| Json(ProjectResponse { project }))
        .ok_or(StatusCode::NOT_FOUND)
}

/// Updates a project.
async fn update_project(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(project): Json<Project>,
) -> Result<Json<ProjectResponse>, StatusCode> {
    if project.id != id {
        return Err(StatusCode::BAD_REQUEST);
    }
    state
        .projects()
        .write()
        .await
        .insert(project.id, project.clone());
    Ok(Json(ProjectResponse { project }))
}

/// Deletes a project.
async fn delete_project(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    state
        .projects()
        .write()
        .await
        .remove(&id)
        .map(|_| StatusCode::NO_CONTENT)
        .ok_or(StatusCode::NOT_FOUND)
}
