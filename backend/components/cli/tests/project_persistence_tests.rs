//! Integration tests for workspace persistence to disk.

use yt_rs_cli::config::AppConfig;
use yt_rs_cli::state::{self, AppState};
use yt_rs_shared::Project;

#[tokio::test]
async fn test_save_workspace_creates_file() {
    let temp = tempfile::tempdir().unwrap();
    let state = AppState::new(temp.path().to_path_buf(), AppConfig::default());

    let project = Project::new("Test Project");
    state::save_workspace(&state, &project).await.unwrap();

    let expected_path = temp.path().join("projects/default.json");
    assert!(expected_path.exists());
}

#[tokio::test]
async fn test_load_workspace_returns_saved_project() {
    let temp = tempfile::tempdir().unwrap();
    let state = AppState::new(temp.path().to_path_buf(), AppConfig::default());

    let project = Project::new("My Project");
    state::save_workspace(&state, &project).await.unwrap();
    let loaded = state::load_workspace(&state).await.unwrap();

    assert!(loaded.is_some());
    let loaded = loaded.unwrap();
    assert_eq!(loaded.name, "My Project");
}

#[tokio::test]
async fn test_load_nonexistent_workspace_returns_none() {
    let temp = tempfile::tempdir().unwrap();
    let state = AppState::new(temp.path().to_path_buf(), AppConfig::default());

    let loaded = state::load_workspace(&state).await.unwrap();
    assert!(loaded.is_none());
}

#[tokio::test]
async fn test_save_workspace_overwrites_existing() {
    let temp = tempfile::tempdir().unwrap();
    let state = AppState::new(temp.path().to_path_buf(), AppConfig::default());

    let mut project = Project::new("Original Name");
    state::save_workspace(&state, &project).await.unwrap();

    project.name = "Updated Name".to_string();
    state::save_workspace(&state, &project).await.unwrap();

    let loaded = state::load_workspace(&state).await.unwrap().unwrap();
    assert_eq!(loaded.name, "Updated Name");
}
