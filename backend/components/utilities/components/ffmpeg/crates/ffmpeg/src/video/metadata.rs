//! Video metadata extraction via ffprobe.

use std::path::Path;
use std::process::Stdio;

use serde::Deserialize;
use tokio::process::Command;

use crate::error::FfmpegError;

/// Video metadata from ffprobe.
#[derive(Debug, Clone)]
pub struct VideoMetadata {
    pub duration_seconds: f64,
    pub width: u32,
    pub height: u32,
    pub codec: String,
    pub frame_rate: f64,
}

/// ffprobe JSON output structures.
#[derive(Debug, Deserialize)]
struct FfprobeOutput {
    streams: Vec<FfprobeStream>,
    format: FfprobeFormat,
}

#[derive(Debug, Deserialize)]
struct FfprobeStream {
    codec_type: String,
    codec_name: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    r_frame_rate: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FfprobeFormat {
    duration: Option<String>,
}

/// Gets video metadata using ffprobe.
pub async fn get_video_metadata(path: &Path) -> Result<VideoMetadata, FfmpegError> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "quiet",
            "-print_format",
            "json",
            "-show_format",
            "-show_streams",
        ])
        .arg(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                FfmpegError::FfprobeNotFound
            } else {
                FfmpegError::Io(e)
            }
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(FfmpegError::ProcessFailed(stderr.to_string()));
    }

    let probe: FfprobeOutput = serde_json::from_slice(&output.stdout)
        .map_err(|e| FfmpegError::ParseError(e.to_string()))?;

    let video_stream = probe
        .streams
        .iter()
        .find(|s| s.codec_type == "video")
        .ok_or_else(|| FfmpegError::ParseError("no video stream found".to_string()))?;

    let duration_seconds = probe
        .format
        .duration
        .as_ref()
        .and_then(|d| d.parse::<f64>().ok())
        .unwrap_or(0.0);

    let frame_rate = video_stream
        .r_frame_rate
        .as_ref()
        .map(|r| parse_frame_rate(r))
        .unwrap_or(0.0);

    Ok(VideoMetadata {
        duration_seconds,
        width: video_stream.width.unwrap_or(0),
        height: video_stream.height.unwrap_or(0),
        codec: video_stream.codec_name.clone().unwrap_or_default(),
        frame_rate,
    })
}

/// Parses frame rate string like "30/1" or "29.97" to f64.
fn parse_frame_rate(rate: &str) -> f64 {
    if let Some((num, den)) = rate.split_once('/') {
        let num: f64 = num.parse().unwrap_or(0.0);
        let den: f64 = den.parse().unwrap_or(1.0);
        if den != 0.0 {
            return num / den;
        }
    }
    rate.parse().unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frame_rate_fraction() {
        assert!((parse_frame_rate("30/1") - 30.0).abs() < 0.001);
        assert!((parse_frame_rate("30000/1001") - 29.97).abs() < 0.01);
    }

    #[test]
    fn test_parse_frame_rate_decimal() {
        assert!((parse_frame_rate("29.97") - 29.97).abs() < 0.001);
    }

    #[test]
    fn test_parse_frame_rate_invalid() {
        assert_eq!(parse_frame_rate("invalid"), 0.0);
        assert_eq!(parse_frame_rate("30/0"), 0.0);
    }
}
