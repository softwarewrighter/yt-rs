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

    /// Saves a video file and returns the path.
    pub async fn save_video(&self, id: Uuid, name: &str, data: &[u8]) -> std::io::Result<String> {
        let work_dir = self.inner.data_dir.join("videos");
        tokio::fs::create_dir_all(&work_dir).await?;
        let ext = std::path::Path::new(name).extension().and_then(|e| e.to_str()).unwrap_or("mp4");
        let filename = format!("{}.{}", id, ext);
        let path = work_dir.join(&filename);
        tokio::fs::write(&path, data).await?;
        Ok(format!("/data/videos/{}", filename))
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
