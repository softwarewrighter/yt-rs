//! FFmpeg utilities for video processing.
//!
//! Provides async functions to run ffmpeg/ffprobe commands via spawned processes.

pub mod error;
pub mod video;

pub use error::FfmpegError;
pub use video::duration::get_duration;
pub use video::frames::{extract_frame, extract_frames_at_interval};
pub use video::metadata::{VideoMetadata, get_video_metadata};
