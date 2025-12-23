//! Video thumbnail extraction.

use std::path::{Path, PathBuf};
use std::process::Stdio;

use tokio::process::Command;
use uuid::Uuid;

use crate::error::FfmpegError;

/// Extracts a thumbnail at the specified time with given width.
/// Height is automatically calculated to maintain aspect ratio.
pub async fn extract_thumbnail(
    video_path: &Path,
    output_path: &Path,
    time_seconds: f64,
    width: u32,
) -> Result<(), FfmpegError> {
    // Ensure parent directory exists
    if let Some(parent) = output_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(FfmpegError::Io)?;
    }

    let output = Command::new("ffmpeg")
        .args(["-y", "-ss", &format!("{:.3}", time_seconds), "-i"])
        .arg(video_path)
        .args([
            "-vframes",
            "1",
            "-vf",
            &format!("scale={}:-1", width),
            "-q:v",
            "2",
        ])
        .arg(output_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                FfmpegError::FfmpegNotFound
            } else {
                FfmpegError::Io(e)
            }
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(FfmpegError::ProcessFailed(stderr.to_string()));
    }

    Ok(())
}

/// Generates a thumbnail path based on video ID.
pub fn thumbnail_path(data_dir: &Path, video_id: Uuid) -> PathBuf {
    data_dir
        .join("work/thumbnails")
        .join(format!("{}.jpg", video_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_thumbnail_path() {
        let data_dir = PathBuf::from("/data");
        let id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let path = thumbnail_path(&data_dir, id);
        assert_eq!(
            path,
            PathBuf::from("/data/work/thumbnails/550e8400-e29b-41d4-a716-446655440000.jpg")
        );
    }
}
