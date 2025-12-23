//! Tests for TextView node type.

use yt_rs_nodes::{ConnectorType, Node, NodeData, Position, TextViewData};

#[test]
fn test_text_view_data_default() {
    let data = TextViewData::default();
    // TextViewData is a marker type with no fields
    let _ = data;
}

#[test]
fn test_node_new_text_view_creates_node() {
    let pos = Position::new(100.0, 200.0);
    let node = Node::new_text_view(pos);

    assert_eq!(node.position.x, 100.0);
    assert_eq!(node.position.y, 200.0);
    assert!(matches!(node.data, NodeData::TextView(_)));
}

#[test]
fn test_node_new_text_view_has_one_input() {
    let node = Node::new_text_view(Position::new(0.0, 0.0));

    assert_eq!(node.inputs.len(), 1);
    assert_eq!(node.inputs[0].name, "text_in");
    assert!(matches!(
        node.inputs[0].connector_type,
        ConnectorType::Input
    ));
}

#[test]
fn test_node_new_text_view_has_no_outputs() {
    let node = Node::new_text_view(Position::new(0.0, 0.0));

    assert!(node.outputs.is_empty());
}

#[test]
fn test_node_new_text_view_size() {
    let node = Node::new_text_view(Position::new(0.0, 0.0));

    assert_eq!(node.size.width, 280.0);
    assert_eq!(node.size.height, 200.0);
}

#[test]
fn test_text_view_type_name() {
    let data = NodeData::TextView(TextViewData::default());
    assert_eq!(data.type_name(), "Text View");
}

#[test]
fn test_text_view_data_serialization() {
    let data = TextViewData::default();

    let json = serde_json::to_string(&data).expect("serialize");
    let parsed: TextViewData = serde_json::from_str(&json).expect("deserialize");

    let _ = parsed; // Marker type has no fields to verify
}

#[test]
fn test_node_data_text_view_serialization() {
    let node_data = NodeData::TextView(TextViewData::default());

    let json = serde_json::to_string(&node_data).expect("serialize");
    assert!(json.contains("\"type\":\"TextView\""));

    let parsed: NodeData = serde_json::from_str(&json).expect("deserialize");
    assert!(matches!(parsed, NodeData::TextView(_)));
}
