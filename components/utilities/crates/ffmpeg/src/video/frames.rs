//! Video frame extraction.

use std::path::{Path, PathBuf};
use std::process::Stdio;

use tokio::process::Command;

use crate::error::FfmpegError;
use crate::video::metadata::get_video_metadata;

/// Extracts a single frame at the specified time.
pub async fn extract_frame(
    video_path: &Path,
    output_path: &Path,
    time_seconds: f64,
) -> Result<(), FfmpegError> {
    let output = Command::new("ffmpeg")
        .args(["-y", "-ss", &format!("{:.3}", time_seconds), "-i"])
        .arg(video_path)
        .args(["-frames:v", "1", "-q:v", "2"])
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

/// Extracts multiple frames at regular intervals.
pub async fn extract_frames_at_interval(
    video_path: &Path,
    output_dir: &Path,
    interval_seconds: f64,
    prefix: &str,
) -> Result<Vec<PathBuf>, FfmpegError> {
    let metadata = get_video_metadata(video_path).await?;
    let frame_count = (metadata.duration_seconds / interval_seconds).floor() as u32;

    let mut frames = Vec::with_capacity(frame_count as usize);

    for i in 0..frame_count {
        let time = i as f64 * interval_seconds;
        let output_path = output_dir.join(format!("{}_{:04}.jpg", prefix, i));
        extract_frame(video_path, &output_path, time).await?;
        frames.push(output_path);
    }

    Ok(frames)
}
