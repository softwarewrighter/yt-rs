//! Tests for GenerateDialog node type.

use yt_rs_nodes::{
    ClipDialog, ConnectorType, GenerateDialogData, GeneratedDialog, GenerationStatus, Node,
    NodeData, Position,
};

#[test]
fn test_generation_status_default() {
    let status = GenerationStatus::default();
    assert!(matches!(status, GenerationStatus::Idle));
}

#[test]
fn test_generation_status_generating() {
    let status = GenerationStatus::Generating {
        current_still: 2,
        total_stills: 5,
    };
    if let GenerationStatus::Generating {
        current_still,
        total_stills,
    } = status
    {
        assert_eq!(current_still, 2);
        assert_eq!(total_stills, 5);
    } else {
        panic!("Expected Generating variant");
    }
}

#[test]
fn test_clip_dialog_default() {
    let clip = ClipDialog::default();
    assert_eq!(clip.timestamp_seconds, 0.0);
    assert_eq!(clip.dialog_text, "");
    assert!(!clip.is_interesting);
}

#[test]
fn test_generated_dialog_default() {
    let dialog = GeneratedDialog::default();
    assert_eq!(dialog.prolog, "");
    assert!(dialog.clips.is_empty());
    assert_eq!(dialog.epilog, "");
    assert_eq!(dialog.youtube_description, "");
}

#[test]
fn test_generate_dialog_data_default() {
    let data = GenerateDialogData::default();
    assert!(matches!(data.generation_status, GenerationStatus::Idle));
    assert!(data.generated_dialog.is_none());
}

#[test]
fn test_node_new_generate_dialog_creates_node() {
    let pos = Position::new(100.0, 200.0);
    let node = Node::new_generate_dialog(pos);

    assert_eq!(node.position.x, 100.0);
    assert_eq!(node.position.y, 200.0);
    assert!(matches!(node.data, NodeData::GenerateDialog(_)));
}

#[test]
fn test_node_new_generate_dialog_has_one_input() {
    let node = Node::new_generate_dialog(Position::new(0.0, 0.0));

    assert_eq!(node.inputs.len(), 1);
    assert_eq!(node.inputs[0].name, "stills_in");
    assert!(matches!(
        node.inputs[0].connector_type,
        ConnectorType::Input
    ));
}

#[test]
fn test_node_new_generate_dialog_has_one_output() {
    let node = Node::new_generate_dialog(Position::new(0.0, 0.0));

    assert_eq!(node.outputs.len(), 1);
    assert_eq!(node.outputs[0].name, "text_out");
    assert!(matches!(
        node.outputs[0].connector_type,
        ConnectorType::Output
    ));
}

#[test]
fn test_node_new_generate_dialog_size() {
    let node = Node::new_generate_dialog(Position::new(0.0, 0.0));

    assert_eq!(node.size.width, 240.0);
    assert_eq!(node.size.height, 140.0);
}

#[test]
fn test_generate_dialog_type_name() {
    let data = NodeData::GenerateDialog(GenerateDialogData::default());
    assert_eq!(data.type_name(), "Generate Dialog");
}

#[test]
fn test_generate_dialog_data_serialization() {
    let data = GenerateDialogData {
        generation_status: GenerationStatus::Complete,
        generated_dialog: Some(GeneratedDialog {
            prolog: "Welcome to my video!".to_string(),
            clips: vec![ClipDialog {
                timestamp_seconds: 30.0,
                dialog_text: "Here's the interesting part".to_string(),
                is_interesting: true,
            }],
            epilog: "Thanks for watching!".to_string(),
            youtube_description: "A great video".to_string(),
        }),
    };

    let json = serde_json::to_string(&data).expect("serialize");
    let parsed: GenerateDialogData = serde_json::from_str(&json).expect("deserialize");

    assert!(matches!(
        parsed.generation_status,
        GenerationStatus::Complete
    ));
    assert!(parsed.generated_dialog.is_some());
    let dialog = parsed.generated_dialog.unwrap();
    assert_eq!(dialog.prolog, "Welcome to my video!");
    assert_eq!(dialog.clips.len(), 1);
    assert_eq!(dialog.clips[0].timestamp_seconds, 30.0);
}

#[test]
fn test_node_data_generate_dialog_serialization() {
    let node_data = NodeData::GenerateDialog(GenerateDialogData::default());

    let json = serde_json::to_string(&node_data).expect("serialize");
    assert!(json.contains("\"type\":\"GenerateDialog\""));

    let parsed: NodeData = serde_json::from_str(&json).expect("deserialize");
    assert!(matches!(parsed, NodeData::GenerateDialog(_)));
}
