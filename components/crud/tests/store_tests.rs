//! Tests for FileStore implementation.

use uuid::Uuid;
use yt_rs_crud::{FileStore, ProjectStore, VideoStore};
use yt_rs_shared::Project;

#[tokio::test]
async fn test_project_save_and_load() {
    let temp = tempfile::tempdir().unwrap();
    let store = FileStore::new(temp.path());

    let project = Project::new("Test Project");
    let id = project.id;

    ProjectStore::save(&store, &project).await.unwrap();
    let loaded = store.load(id).await.unwrap();

    assert!(loaded.is_some());
    assert_eq!(loaded.unwrap().name, "Test Project");
}

#[tokio::test]
async fn test_project_load_nonexistent() {
    let temp = tempfile::tempdir().unwrap();
    let store = FileStore::new(temp.path());

    let loaded = store.load(Uuid::new_v4()).await.unwrap();
    assert!(loaded.is_none());
}

#[tokio::test]
async fn test_project_list() {
    let temp = tempfile::tempdir().unwrap();
    let store = FileStore::new(temp.path());

    let p1 = Project::new("Project 1");
    let p2 = Project::new("Project 2");

    ProjectStore::save(&store, &p1).await.unwrap();
    ProjectStore::save(&store, &p2).await.unwrap();

    let projects = store.list().await.unwrap();
    assert_eq!(projects.len(), 2);
}

#[tokio::test]
async fn test_project_delete() {
    let temp = tempfile::tempdir().unwrap();
    let store = FileStore::new(temp.path());

    let project = Project::new("To Delete");
    let id = project.id;

    ProjectStore::save(&store, &project).await.unwrap();
    assert!(store.load(id).await.unwrap().is_some());

    store.delete(id).await.unwrap();
    assert!(store.load(id).await.unwrap().is_none());
}

#[tokio::test]
async fn test_video_save() {
    let temp = tempfile::tempdir().unwrap();
    let store = FileStore::new(temp.path());

    let id = Uuid::new_v4();
    let data = b"fake video data";

    let path = VideoStore::save(&store, id, "test.mp4", data)
        .await
        .unwrap();
    assert!(path.exists());
}

#[tokio::test]
async fn test_video_get_path() {
    let temp = tempfile::tempdir().unwrap();
    let store = FileStore::new(temp.path());

    let id = Uuid::new_v4();
    VideoStore::save(&store, id, "video.mov", b"data")
        .await
        .unwrap();

    let path = store.get_path(id).await.unwrap();
    assert!(path.is_some());
}

#[tokio::test]
async fn test_video_get_path_nonexistent() {
    let temp = tempfile::tempdir().unwrap();
    let store = FileStore::new(temp.path());

    let path = store.get_path(Uuid::new_v4()).await.unwrap();
    assert!(path.is_none());
}
