//! Tests for Viewer node type.

use yt_rs_nodes::{Node, NodeData, Position, ViewerData};

#[test]
fn test_viewer_data_default() {
    let data = ViewerData::default();
    assert!(data.thumbnail_path.is_none());
    assert_eq!(data.thumbnail_timestamp, 0.0);
}

#[test]
fn test_viewer_data_with_thumbnail() {
    let data = ViewerData {
        thumbnail_path: Some("/path/to/thumb.jpg".to_string()),
        thumbnail_timestamp: 1.5,
    };
    assert_eq!(data.thumbnail_path.as_deref(), Some("/path/to/thumb.jpg"));
    assert_eq!(data.thumbnail_timestamp, 1.5);
}

#[test]
fn test_node_new_viewer_creates_viewer_node() {
    let pos = Position::new(100.0, 200.0);
    let node = Node::new_viewer(pos);

    assert_eq!(node.position.x, 100.0);
    assert_eq!(node.position.y, 200.0);
    assert!(matches!(node.data, NodeData::Viewer(_)));
}

#[test]
fn test_node_new_viewer_has_input_connector() {
    let node = Node::new_viewer(Position::new(0.0, 0.0));

    assert_eq!(node.inputs.len(), 1);
    assert_eq!(node.inputs[0].name, "video_in");
}

#[test]
fn test_node_new_viewer_has_no_output_connectors() {
    let node = Node::new_viewer(Position::new(0.0, 0.0));

    assert!(node.outputs.is_empty());
}

#[test]
fn test_node_new_viewer_size() {
    let node = Node::new_viewer(Position::new(0.0, 0.0));

    assert_eq!(node.size.width, 200.0);
    assert_eq!(node.size.height, 160.0);
}

#[test]
fn test_viewer_type_name() {
    let data = NodeData::Viewer(ViewerData::default());
    assert_eq!(data.type_name(), "Viewer");
}

#[test]
fn test_viewer_data_serialization() {
    let data = ViewerData {
        thumbnail_path: Some("/thumb.jpg".to_string()),
        thumbnail_timestamp: 2.5,
    };

    let json = serde_json::to_string(&data).expect("serialize");
    let parsed: ViewerData = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(parsed.thumbnail_path, data.thumbnail_path);
    assert_eq!(parsed.thumbnail_timestamp, data.thumbnail_timestamp);
}

#[test]
fn test_node_data_viewer_serialization() {
    let node_data = NodeData::Viewer(ViewerData::default());

    let json = serde_json::to_string(&node_data).expect("serialize");
    assert!(json.contains("\"type\":\"Viewer\""));

    let parsed: NodeData = serde_json::from_str(&json).expect("deserialize");
    assert!(matches!(parsed, NodeData::Viewer(_)));
}
