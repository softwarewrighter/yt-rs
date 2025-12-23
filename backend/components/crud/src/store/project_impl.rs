//! ProjectStore implementation for FileStore.

use std::path::Path;

use async_trait::async_trait;
use uuid::Uuid;
use yt_rs_shared::Project;

use crate::error::CrudError;
use crate::project::ProjectStore;
use crate::store::FileStore;

#[async_trait]
impl ProjectStore for FileStore {
    async fn save(&self, project: &Project) -> Result<(), CrudError> {
        let dir = self.base_path.join("projects");
        tokio::fs::create_dir_all(&dir).await?;
        let path = dir.join(format!("{}.json", project.id));
        let json = serde_json::to_string_pretty(project)?;
        tokio::fs::write(&path, json).await?;
        Ok(())
    }

    async fn load(&self, id: Uuid) -> Result<Option<Project>, CrudError> {
        let path = self.base_path.join(format!("projects/{}.json", id));
        if !path.exists() {
            return Ok(None);
        }
        let json = tokio::fs::read_to_string(&path).await?;
        let project: Project = serde_json::from_str(&json)?;
        Ok(Some(project))
    }

    async fn list(&self) -> Result<Vec<Project>, CrudError> {
        let dir = self.base_path.join("projects");
        if !dir.exists() {
            return Ok(Vec::new());
        }
        let mut projects = Vec::new();
        let mut entries = tokio::fs::read_dir(&dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            if let Some(project) = load_project_file(&entry.path()).await? {
                projects.push(project);
            }
        }
        Ok(projects)
    }

    async fn delete(&self, id: Uuid) -> Result<(), CrudError> {
        let path = self.base_path.join(format!("projects/{}.json", id));
        if path.exists() {
            tokio::fs::remove_file(&path).await?;
        }
        Ok(())
    }
}

async fn load_project_file(path: &Path) -> Result<Option<Project>, CrudError> {
    if path.extension().is_none_or(|ext| ext != "json") {
        return Ok(None);
    }
    let json = tokio::fs::read_to_string(path).await?;
    let project: Project = serde_json::from_str(&json)?;
    Ok(Some(project))
}
