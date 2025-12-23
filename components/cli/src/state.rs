//! Application state for the backend server.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::RwLock;
use uuid::Uuid;
use yt_rs_shared::Project;

use crate::config::AppConfig;

/// Shared application state.
#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

struct AppStateInner {
    data_dir: PathBuf,
    config: AppConfig,
    projects: RwLock<HashMap<Uuid, Project>>,
}

impl AppState {
    /// Creates a new application state.
    pub fn new(data_dir: PathBuf, config: AppConfig) -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                data_dir,
                config,
                projects: RwLock::new(HashMap::new()),
            }),
        }
    }

    /// Returns the application configuration.
    pub fn config(&self) -> &AppConfig {
        &self.inner.config
    }

    /// Returns the data directory path.
    pub fn data_dir(&self) -> &PathBuf {
        &self.inner.data_dir
    }

    /// Returns the projects cache.
    pub fn projects(&self) -> &RwLock<HashMap<Uuid, Project>> {
        &self.inner.projects
    }
}

/// Saves a video file and returns the path.
pub async fn save_video(
    state: &AppState,
    id: Uuid,
    name: &str,
    data: &[u8],
) -> std::io::Result<PathBuf> {
    let dir = state.data_dir().join("work/videos");
    tokio::fs::create_dir_all(&dir).await?;
    let ext = std::path::Path::new(name)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("mp4");
    let path = dir.join(format!("{id}.{ext}"));
    tokio::fs::write(&path, data).await?;
    Ok(path)
}

/// Gets the path to a video file if it exists.
pub async fn get_video_path(state: &AppState, id: Uuid) -> Option<PathBuf> {
    let dir = state.data_dir().join("work/videos");
    for ext in ["mp4", "mov", "avi", "webm", "mkv"] {
        let path = dir.join(format!("{id}.{ext}"));
        if tokio::fs::try_exists(&path).await.unwrap_or(false) {
            return Some(path);
        }
    }
    None
}

/// Gets or creates a thumbnail for a video.
pub async fn get_or_create_thumbnail(state: &AppState, video_id: Uuid) -> std::io::Result<PathBuf> {
    let thumbnail_path = yt_rs_ffmpeg::thumbnail_path(state.data_dir(), video_id);
    if tokio::fs::try_exists(&thumbnail_path)
        .await
        .unwrap_or(false)
    {
        return Ok(thumbnail_path);
    }
    let video_path = get_video_path(state, video_id)
        .await
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Video not found"))?;
    yt_rs_ffmpeg::extract_thumbnail(&video_path, &thumbnail_path, 0.0, 320)
        .await
        .map_err(|e| std::io::Error::other(e.to_string()))?;
    Ok(thumbnail_path)
}

/// Gets or creates a still thumbnail at a specific timestamp.
pub async fn get_or_create_still(
    state: &AppState,
    video_id: Uuid,
    timestamp: f64,
) -> std::io::Result<PathBuf> {
    let stills_dir = state.data_dir().join("work/stills");
    tokio::fs::create_dir_all(&stills_dir).await?;
    let filename = format!("{video_id}-{timestamp:.2}.jpg");
    let still_path = stills_dir.join(&filename);
    if tokio::fs::try_exists(&still_path).await.unwrap_or(false) {
        return Ok(still_path);
    }
    let video_path = get_video_path(state, video_id)
        .await
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Video not found"))?;
    yt_rs_ffmpeg::extract_thumbnail(&video_path, &still_path, timestamp, 320)
        .await
        .map_err(|e| std::io::Error::other(e.to_string()))?;
    Ok(still_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_project_cache() {
        let state = AppState::new(PathBuf::from("./test_data"), AppConfig::default());
        let project = Project::new("Test Project");
        let id = project.id;

        state.projects().write().await.insert(id, project);
        let projects = state.projects().read().await;
        assert!(projects.contains_key(&id));
        assert_eq!(projects.get(&id).unwrap().name, "Test Project");
    }
}
