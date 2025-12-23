//! Integration tests for project persistence to disk.

use uuid::Uuid;

use yt_rs_cli::config::AppConfig;
use yt_rs_cli::state::AppState;
use yt_rs_shared::Project;

#[tokio::test]
async fn test_save_project_creates_file() {
    let temp = tempfile::tempdir().unwrap();
    let state = AppState::new(temp.path().to_path_buf(), AppConfig::default());

    let project = Project::new("Test Project");
    let id = project.id;

    state.save_project_to_disk(&project).await.unwrap();

    let expected_path = temp.path().join(format!("projects/{}.json", id));
    assert!(expected_path.exists());
}

#[tokio::test]
async fn test_load_project_returns_saved_project() {
    let temp = tempfile::tempdir().unwrap();
    let state = AppState::new(temp.path().to_path_buf(), AppConfig::default());

    let project = Project::new("My Project");
    let id = project.id;

    state.save_project_to_disk(&project).await.unwrap();
    let loaded = state.load_project_from_disk(id).await.unwrap();

    assert!(loaded.is_some());
    let loaded = loaded.unwrap();
    assert_eq!(loaded.id, id);
    assert_eq!(loaded.name, "My Project");
}

#[tokio::test]
async fn test_load_nonexistent_project_returns_none() {
    let temp = tempfile::tempdir().unwrap();
    let state = AppState::new(temp.path().to_path_buf(), AppConfig::default());

    let id = Uuid::new_v4();
    let loaded = state.load_project_from_disk(id).await.unwrap();

    assert!(loaded.is_none());
}

#[tokio::test]
async fn test_save_project_overwrites_existing() {
    let temp = tempfile::tempdir().unwrap();
    let state = AppState::new(temp.path().to_path_buf(), AppConfig::default());

    let mut project = Project::new("Original Name");
    let id = project.id;

    state.save_project_to_disk(&project).await.unwrap();

    project.name = "Updated Name".to_string();
    state.save_project_to_disk(&project).await.unwrap();

    let loaded = state.load_project_from_disk(id).await.unwrap().unwrap();
    assert_eq!(loaded.name, "Updated Name");
}

#[tokio::test]
async fn test_list_projects_from_disk() {
    let temp = tempfile::tempdir().unwrap();
    let state = AppState::new(temp.path().to_path_buf(), AppConfig::default());

    let project1 = Project::new("Project 1");
    let project2 = Project::new("Project 2");

    state.save_project_to_disk(&project1).await.unwrap();
    state.save_project_to_disk(&project2).await.unwrap();

    let projects = state.list_projects_from_disk().await.unwrap();
    assert_eq!(projects.len(), 2);
}

#[tokio::test]
async fn test_delete_project_from_disk() {
    let temp = tempfile::tempdir().unwrap();
    let state = AppState::new(temp.path().to_path_buf(), AppConfig::default());

    let project = Project::new("To Delete");
    let id = project.id;

    state.save_project_to_disk(&project).await.unwrap();
    assert!(state.load_project_from_disk(id).await.unwrap().is_some());

    state.delete_project_from_disk(id).await.unwrap();
    assert!(state.load_project_from_disk(id).await.unwrap().is_none());
}
