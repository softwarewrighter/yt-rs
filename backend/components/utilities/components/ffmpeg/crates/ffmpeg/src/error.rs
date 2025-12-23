//! Error types for ffmpeg operations.

use thiserror::Error;

/// Errors from ffmpeg operations.
#[derive(Debug, Error)]
pub enum FfmpegError {
    #[error("ffprobe not found in PATH")]
    FfprobeNotFound,

    #[error("ffmpeg not found in PATH")]
    FfmpegNotFound,

    #[error("process failed: {0}")]
    ProcessFailed(String),

    #[error("failed to parse output: {0}")]
    ParseError(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
