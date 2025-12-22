//! Application state for the backend server.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::RwLock;
use uuid::Uuid;
use yt_rs_shared::Project;

/// Shared application state.
#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

struct AppStateInner {
    data_dir: PathBuf,
    projects: RwLock<HashMap<Uuid, Project>>,
}

impl AppState {
    /// Creates a new application state.
    pub fn new(data_dir: PathBuf) -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                data_dir,
                projects: RwLock::new(HashMap::new()),
            }),
        }
    }

    /// Saves a video file to work directory and returns the path.
    pub async fn save_video(&self, id: Uuid, name: &str, data: &[u8]) -> std::io::Result<String> {
        let work_dir = self.inner.data_dir.join("work/videos");
        tokio::fs::create_dir_all(&work_dir).await?;
        let ext = std::path::Path::new(name)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("mp4");
        let filename = format!("{}.{}", id, ext);
        let path = work_dir.join(&filename);
        tokio::fs::write(&path, data).await?;
        Ok(path.to_string_lossy().to_string())
    }

    /// Lists all projects.
    pub async fn list_projects(&self) -> Vec<Project> {
        let projects = self.inner.projects.read().await;
        projects.values().cloned().collect()
    }

    /// Gets a project by ID.
    pub async fn get_project(&self, id: Uuid) -> Option<Project> {
        let projects = self.inner.projects.read().await;
        projects.get(&id).cloned()
    }

    /// Creates or updates a project.
    pub async fn upsert_project(&self, project: Project) -> Uuid {
        let id = project.id;
        let mut projects = self.inner.projects.write().await;
        projects.insert(id, project);
        id
    }

    /// Deletes a project.
    pub async fn delete_project(&self, id: Uuid) -> Option<Project> {
        let mut projects = self.inner.projects.write().await;
        projects.remove(&id)
    }

    /// Returns the path for a video file if it exists.
    pub async fn get_video_path(&self, id: Uuid) -> Option<PathBuf> {
        let work_dir = self.inner.data_dir.join("work/videos");

        // Check for common extensions
        for ext in ["mp4", "mov", "avi", "webm", "mkv"] {
            let path = work_dir.join(format!("{}.{}", id, ext));
            if tokio::fs::try_exists(&path).await.unwrap_or(false) {
                return Some(path);
            }
        }

        None
    }

    /// Saves a project to disk as JSON.
    pub async fn save_project_to_disk(&self, project: &Project) -> std::io::Result<()> {
        let projects_dir = self.inner.data_dir.join("projects");
        tokio::fs::create_dir_all(&projects_dir).await?;

        let path = projects_dir.join(format!("{}.json", project.id));
        let json = serde_json::to_string_pretty(project)
            .map_err(|e| std::io::Error::other(e.to_string()))?;

        tokio::fs::write(&path, json).await
    }

    /// Loads a project from disk by ID.
    pub async fn load_project_from_disk(&self, id: Uuid) -> std::io::Result<Option<Project>> {
        let path = self.inner.data_dir.join(format!("projects/{}.json", id));

        if !tokio::fs::try_exists(&path).await.unwrap_or(false) {
            return Ok(None);
        }

        let json = tokio::fs::read_to_string(&path).await?;
        let project: Project =
            serde_json::from_str(&json).map_err(|e| std::io::Error::other(e.to_string()))?;

        Ok(Some(project))
    }

    /// Lists all projects from disk.
    pub async fn list_projects_from_disk(&self) -> std::io::Result<Vec<Project>> {
        let projects_dir = self.inner.data_dir.join("projects");

        if !tokio::fs::try_exists(&projects_dir).await.unwrap_or(false) {
            return Ok(Vec::new());
        }

        let mut projects = Vec::new();
        let mut entries = tokio::fs::read_dir(&projects_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "json")
                && let Ok(json) = tokio::fs::read_to_string(&path).await
                && let Ok(project) = serde_json::from_str::<Project>(&json)
            {
                projects.push(project);
            }
        }

        Ok(projects)
    }

    /// Deletes a project from disk.
    pub async fn delete_project_from_disk(&self, id: Uuid) -> std::io::Result<()> {
        let path = self.inner.data_dir.join(format!("projects/{}.json", id));

        if tokio::fs::try_exists(&path).await.unwrap_or(false) {
            tokio::fs::remove_file(&path).await?;
        }

        Ok(())
    }

    /// Gets or creates a thumbnail for a video.
    pub async fn get_or_create_thumbnail(&self, video_id: Uuid) -> std::io::Result<PathBuf> {
        let thumbnail_path = yt_rs_ffmpeg::thumbnail_path(&self.inner.data_dir, video_id);

        // Return existing thumbnail if it exists
        if tokio::fs::try_exists(&thumbnail_path)
            .await
            .unwrap_or(false)
        {
            return Ok(thumbnail_path);
        }

        // Find the video file
        let video_path = self
            .get_video_path(video_id)
            .await
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Video not found"))?;

        // Extract thumbnail at 0 seconds
        yt_rs_ffmpeg::extract_thumbnail(&video_path, &thumbnail_path, 0.0, 320)
            .await
            .map_err(|e| std::io::Error::other(e.to_string()))?;

        Ok(thumbnail_path)
    }

    /// Gets or creates a still thumbnail at a specific timestamp.
    pub async fn get_or_create_still_thumbnail(
        &self,
        video_id: Uuid,
        timestamp: f64,
    ) -> std::io::Result<PathBuf> {
        let stills_dir = self.inner.data_dir.join("work/stills");
        tokio::fs::create_dir_all(&stills_dir).await?;

        // Create filename based on video_id and timestamp
        let filename = format!("{}-{:.2}.jpg", video_id, timestamp);
        let still_path = stills_dir.join(&filename);

        // Return existing still if it exists
        if tokio::fs::try_exists(&still_path).await.unwrap_or(false) {
            return Ok(still_path);
        }

        // Find the video file
        let video_path = self
            .get_video_path(video_id)
            .await
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Video not found"))?;

        // Extract frame at the specified timestamp
        yt_rs_ffmpeg::extract_thumbnail(&video_path, &still_path, timestamp, 320)
            .await
            .map_err(|e| std::io::Error::other(e.to_string()))?;

        Ok(still_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_project_crud() {
        let state = AppState::new(PathBuf::from("./test_data"));

        // Create
        let project = Project::new("Test Project");
        let id = project.id;
        state.upsert_project(project).await;

        // Read
        let retrieved = state.get_project(id).await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Test Project");

        // List
        let all = state.list_projects().await;
        assert_eq!(all.len(), 1);

        // Delete
        let deleted = state.delete_project(id).await;
        assert!(deleted.is_some());

        // Verify deleted
        let gone = state.get_project(id).await;
        assert!(gone.is_none());
    }
}
