//! Tests for StillPreview node type.

use yt_rs_nodes::{ConnectorType, Node, NodeData, Position, StillPreviewData};

#[test]
fn test_still_preview_data_default() {
    let data = StillPreviewData::default();
    // StillPreviewData is a marker type with no fields
    assert_eq!(data, StillPreviewData {});
}

#[test]
fn test_node_new_still_preview_creates_still_preview_node() {
    let pos = Position::new(100.0, 200.0);
    let node = Node::new_still_preview(pos);

    assert_eq!(node.position.x, 100.0);
    assert_eq!(node.position.y, 200.0);
    assert!(matches!(node.data, NodeData::StillPreview(_)));
}

#[test]
fn test_node_new_still_preview_has_one_input() {
    let node = Node::new_still_preview(Position::new(0.0, 0.0));

    assert_eq!(node.inputs.len(), 1);
    assert_eq!(node.inputs[0].name, "still_in");
    assert!(matches!(
        node.inputs[0].connector_type,
        ConnectorType::Input
    ));
}

#[test]
fn test_node_new_still_preview_has_one_output() {
    let node = Node::new_still_preview(Position::new(0.0, 0.0));

    assert_eq!(node.outputs.len(), 1);
    assert_eq!(node.outputs[0].name, "still_out");
    assert!(matches!(
        node.outputs[0].connector_type,
        ConnectorType::Output
    ));
}

#[test]
fn test_node_new_still_preview_size() {
    let node = Node::new_still_preview(Position::new(0.0, 0.0));

    // Larger size to accommodate thumbnail display
    assert_eq!(node.size.width, 200.0);
    assert_eq!(node.size.height, 160.0);
}

#[test]
fn test_still_preview_type_name() {
    let data = NodeData::StillPreview(StillPreviewData::default());
    assert_eq!(data.type_name(), "Still Preview");
}

#[test]
fn test_still_preview_data_serialization() {
    let data = StillPreviewData::default();

    let json = serde_json::to_string(&data).expect("serialize");
    let parsed: StillPreviewData = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(parsed, data);
}

#[test]
fn test_node_data_still_preview_serialization() {
    let node_data = NodeData::StillPreview(StillPreviewData::default());

    let json = serde_json::to_string(&node_data).expect("serialize");
    assert!(json.contains("\"type\":\"StillPreview\""));

    let parsed: NodeData = serde_json::from_str(&json).expect("deserialize");
    assert!(matches!(parsed, NodeData::StillPreview(_)));
}
