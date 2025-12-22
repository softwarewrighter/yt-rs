//! Integration tests for thumbnail extraction.

use std::path::PathBuf;
use uuid::Uuid;
use yt_rs_ffmpeg::{extract_thumbnail, thumbnail_path};

#[test]
fn test_thumbnail_path_format() {
    let data_dir = PathBuf::from("/data");
    let id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    let path = thumbnail_path(&data_dir, id);

    assert_eq!(
        path,
        PathBuf::from("/data/work/thumbnails/550e8400-e29b-41d4-a716-446655440000.jpg")
    );
}

#[test]
fn test_thumbnail_path_uses_jpg_extension() {
    let data_dir = PathBuf::from("/var/app");
    let id = Uuid::new_v4();
    let path = thumbnail_path(&data_dir, id);

    assert!(path.extension().map(|e| e == "jpg").unwrap_or(false));
}

#[test]
fn test_thumbnail_path_in_work_thumbnails_subdir() {
    let data_dir = PathBuf::from("/mydata");
    let id = Uuid::new_v4();
    let path = thumbnail_path(&data_dir, id);

    assert!(path.starts_with("/mydata/work/thumbnails"));
}

#[tokio::test]
async fn test_extract_thumbnail_missing_video_returns_error() {
    let output = tempfile::tempdir().unwrap();
    let output_path = output.path().join("thumb.jpg");

    let result = extract_thumbnail(
        std::path::Path::new("/nonexistent/video.mp4"),
        &output_path,
        0.0,
        320,
    )
    .await;

    assert!(result.is_err());
}

/// Integration test that requires ffmpeg and creates a test video.
/// Run with: cargo test --test thumbnail_tests -- --ignored
#[tokio::test]
#[ignore]
async fn test_extract_thumbnail_creates_file() {
    use std::process::Command;

    // Create a temporary directory for the test
    let temp = tempfile::tempdir().unwrap();
    let video_path = temp.path().join("test.mp4");
    let thumb_path = temp.path().join("thumb.jpg");

    // Generate a 2-second test video using ffmpeg
    let gen_result = Command::new("ffmpeg")
        .args([
            "-f",
            "lavfi",
            "-i",
            "testsrc=duration=2:size=320x240:rate=30",
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            "-y",
        ])
        .arg(&video_path)
        .output();

    if gen_result.is_err() {
        eprintln!("ffmpeg not available, skipping test");
        return;
    }

    // Extract thumbnail at 1 second
    let result = extract_thumbnail(&video_path, &thumb_path, 1.0, 160).await;

    assert!(result.is_ok(), "extract_thumbnail failed: {:?}", result);
    assert!(thumb_path.exists(), "thumbnail file not created");

    // Verify the file has content
    let metadata = std::fs::metadata(&thumb_path).unwrap();
    assert!(metadata.len() > 0, "thumbnail file is empty");
}
