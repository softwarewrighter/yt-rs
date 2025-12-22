//! Integration tests for project types.

use uuid::Uuid;
use yt_rs_shared::{Connection, Node, Position, Project};

#[test]
fn test_project_new() {
    let project = Project::new("Test Project");
    assert_eq!(project.name, "Test Project");
    assert!(project.nodes.is_empty());
    assert!(project.connections.is_empty());
}

#[test]
fn test_project_default() {
    let project = Project::default();
    assert_eq!(project.name, "Untitled Project");
}

#[test]
fn test_add_node() {
    let mut project = Project::new("Test");
    let node = Node::new_video_input(Position::new(0.0, 0.0));
    let node_id = node.id;

    project.add_node(node);

    assert_eq!(project.nodes.len(), 1);
    assert!(project.find_node(node_id).is_some());
}

#[test]
fn test_remove_node() {
    let mut project = Project::new("Test");
    let node = Node::new_video_input(Position::new(0.0, 0.0));
    let node_id = node.id;

    project.add_node(node);
    let removed = project.remove_node(node_id);

    assert!(removed.is_some());
    assert!(project.nodes.is_empty());
}

#[test]
fn test_remove_node_cascades_connections() {
    let mut project = Project::new("Test");

    let video = Node::new_video_input(Position::new(0.0, 0.0));
    let sampler = Node::new_still_sampler(Position::new(300.0, 0.0));

    let video_id = video.id;
    let video_out = video.outputs[0].id;
    let sampler_in = sampler.inputs[0].id;
    let sampler_id = sampler.id;

    project.add_node(video);
    project.add_node(sampler);

    let connection = Connection::new(video_id, video_out, sampler_id, sampler_in);
    project.add_connection(connection);

    assert_eq!(project.connections.len(), 1);

    // Remove video node should also remove connection
    project.remove_node(video_id);

    assert!(project.connections.is_empty());
}

#[test]
fn test_find_node_mut() {
    let mut project = Project::new("Test");
    let node = Node::new_video_input(Position::new(0.0, 0.0));
    let node_id = node.id;

    project.add_node(node);

    if let Some(node) = project.find_node_mut(node_id) {
        node.position = Position::new(100.0, 100.0);
    }

    let node = project.find_node(node_id).unwrap();
    assert_eq!(node.position.x, 100.0);
}

#[test]
fn test_remove_connection() {
    let mut project = Project::new("Test");
    let connection = Connection::new(
        Uuid::new_v4(),
        Uuid::new_v4(),
        Uuid::new_v4(),
        Uuid::new_v4(),
    );
    let conn_id = connection.id;

    project.add_connection(connection);
    assert_eq!(project.connections.len(), 1);

    let removed = project.remove_connection(conn_id);
    assert!(removed.is_some());
    assert!(project.connections.is_empty());
}

#[test]
fn test_project_serialization() {
    let mut project = Project::new("Test");
    project.add_node(Node::new_video_input(Position::new(10.0, 20.0)));

    let json = serde_json::to_string(&project).unwrap();
    let deserialized: Project = serde_json::from_str(&json).unwrap();

    assert_eq!(project.name, deserialized.name);
    assert_eq!(project.nodes.len(), deserialized.nodes.len());
}
