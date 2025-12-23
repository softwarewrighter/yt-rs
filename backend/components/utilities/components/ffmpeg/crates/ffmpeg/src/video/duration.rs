//! Video duration extraction.

use std::path::Path;

use crate::error::FfmpegError;
use crate::video::metadata::get_video_metadata;

/// Gets video duration in seconds.
pub async fn get_duration(path: &Path) -> Result<f64, FfmpegError> {
    let metadata = get_video_metadata(path).await?;
    Ok(metadata.duration_seconds)
}
