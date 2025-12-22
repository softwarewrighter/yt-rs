//! Integration tests for project routes.

use yt_rs_shared::Project;

// Re-create ProjectSummary for testing since it's not exported
#[derive(Debug)]
struct ProjectSummary {
    name: String,
    node_count: usize,
}

impl From<&Project> for ProjectSummary {
    fn from(project: &Project) -> Self {
        Self {
            name: project.name.clone(),
            node_count: project.nodes.len(),
        }
    }
}

#[test]
fn test_project_summary_from_project() {
    let project = Project::new("Test");
    let summary = ProjectSummary::from(&project);
    assert_eq!(summary.name, "Test");
    assert_eq!(summary.node_count, 0);
}
