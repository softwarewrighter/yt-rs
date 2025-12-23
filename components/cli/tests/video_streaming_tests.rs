//! Integration tests for video streaming and thumbnail endpoints.

use std::path::PathBuf;
use uuid::Uuid;

use yt_rs_cli::config::AppConfig;
use yt_rs_cli::state::AppState;

#[tokio::test]
async fn test_get_video_path_returns_none_for_missing_video() {
    let temp = tempfile::tempdir().unwrap();
    let state = AppState::new(temp.path().to_path_buf(), AppConfig::default());

    let id = Uuid::new_v4();
    let result = state.get_video_path(id).await;

    assert!(result.is_none());
}

#[tokio::test]
async fn test_get_video_path_finds_mp4_video() {
    let temp = tempfile::tempdir().unwrap();
    let state = AppState::new(temp.path().to_path_buf(), AppConfig::default());

    // Create video directory and file
    let video_dir = temp.path().join("work/videos");
    tokio::fs::create_dir_all(&video_dir).await.unwrap();

    let id = Uuid::new_v4();
    let video_path = video_dir.join(format!("{}.mp4", id));
    tokio::fs::write(&video_path, b"fake video data")
        .await
        .unwrap();

    let result = state.get_video_path(id).await;

    assert!(result.is_some());
    assert_eq!(result.unwrap(), video_path);
}

#[tokio::test]
async fn test_get_video_path_finds_mov_video() {
    let temp = tempfile::tempdir().unwrap();
    let state = AppState::new(temp.path().to_path_buf(), AppConfig::default());

    let video_dir = temp.path().join("work/videos");
    tokio::fs::create_dir_all(&video_dir).await.unwrap();

    let id = Uuid::new_v4();
    let video_path = video_dir.join(format!("{}.mov", id));
    tokio::fs::write(&video_path, b"fake mov data")
        .await
        .unwrap();

    let result = state.get_video_path(id).await;

    assert!(result.is_some());
    assert_eq!(result.unwrap(), video_path);
}

#[tokio::test]
async fn test_get_or_create_thumbnail_returns_error_for_missing_video() {
    let temp = tempfile::tempdir().unwrap();
    let state = AppState::new(temp.path().to_path_buf(), AppConfig::default());

    let id = Uuid::new_v4();
    let result = state.get_or_create_thumbnail(id).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_or_create_thumbnail_returns_existing_thumbnail() {
    let temp = tempfile::tempdir().unwrap();
    let state = AppState::new(temp.path().to_path_buf(), AppConfig::default());

    // Create thumbnail directory and file
    let thumb_dir = temp.path().join("work/thumbnails");
    tokio::fs::create_dir_all(&thumb_dir).await.unwrap();

    let id = Uuid::new_v4();
    let thumb_path = thumb_dir.join(format!("{}.jpg", id));
    tokio::fs::write(&thumb_path, b"fake thumbnail data")
        .await
        .unwrap();

    let result = state.get_or_create_thumbnail(id).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), thumb_path);
}

#[tokio::test]
async fn test_save_video_creates_file() {
    let temp = tempfile::tempdir().unwrap();
    let state = AppState::new(temp.path().to_path_buf(), AppConfig::default());

    let id = Uuid::new_v4();
    let data = b"test video content";

    let path = state.save_video(id, "test.mp4", data).await.unwrap();

    assert!(PathBuf::from(&path).exists());
    let content = std::fs::read(&path).unwrap();
    assert_eq!(content, data);
}

#[tokio::test]
async fn test_save_video_preserves_extension() {
    let temp = tempfile::tempdir().unwrap();
    let state = AppState::new(temp.path().to_path_buf(), AppConfig::default());

    let id = Uuid::new_v4();
    let path = state.save_video(id, "video.mov", b"data").await.unwrap();

    assert!(path.ends_with(".mov"));
}
