//! Tests for Selector node type.

use yt_rs_nodes::{ConnectorType, Node, NodeData, Position, SelectorData};

#[test]
fn test_selector_data_default() {
    let data = SelectorData::default();
    assert_eq!(data.selected_index, 0);
}

#[test]
fn test_selector_data_with_index() {
    let data = SelectorData { selected_index: 5 };
    assert_eq!(data.selected_index, 5);
}

#[test]
fn test_node_new_selector_creates_selector_node() {
    let pos = Position::new(100.0, 200.0);
    let node = Node::new_selector(pos);

    assert_eq!(node.position.x, 100.0);
    assert_eq!(node.position.y, 200.0);
    assert!(matches!(node.data, NodeData::Selector(_)));
}

#[test]
fn test_node_new_selector_has_one_input() {
    let node = Node::new_selector(Position::new(0.0, 0.0));

    assert_eq!(node.inputs.len(), 1);
    assert_eq!(node.inputs[0].name, "stills_in");
    assert!(matches!(
        node.inputs[0].connector_type,
        ConnectorType::Input
    ));
}

#[test]
fn test_node_new_selector_has_two_outputs() {
    let node = Node::new_selector(Position::new(0.0, 0.0));

    assert_eq!(node.outputs.len(), 2);
    assert_eq!(node.outputs[0].name, "selected_out");
    assert_eq!(node.outputs[1].name, "array_out");
    assert!(matches!(
        node.outputs[0].connector_type,
        ConnectorType::Output
    ));
    assert!(matches!(
        node.outputs[1].connector_type,
        ConnectorType::Output
    ));
}

#[test]
fn test_node_new_selector_size() {
    let node = Node::new_selector(Position::new(0.0, 0.0));

    assert_eq!(node.size.width, 180.0);
    assert_eq!(node.size.height, 100.0);
}

#[test]
fn test_selector_type_name() {
    let data = NodeData::Selector(SelectorData::default());
    assert_eq!(data.type_name(), "Selector");
}

#[test]
fn test_selector_data_serialization() {
    let data = SelectorData { selected_index: 3 };

    let json = serde_json::to_string(&data).expect("serialize");
    let parsed: SelectorData = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(parsed.selected_index, data.selected_index);
}

#[test]
fn test_node_data_selector_serialization() {
    let node_data = NodeData::Selector(SelectorData { selected_index: 2 });

    let json = serde_json::to_string(&node_data).expect("serialize");
    assert!(json.contains("\"type\":\"Selector\""));

    let parsed: NodeData = serde_json::from_str(&json).expect("deserialize");
    assert!(matches!(parsed, NodeData::Selector(_)));
}
