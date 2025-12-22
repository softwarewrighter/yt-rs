//! Integration tests for node types.

use yt_rs_shared::{
    Connector, ConnectorPosition, ConnectorType, Node, NodeData, Position, StillSamplerData,
    UploadStatus, VideoInputData,
};

#[test]
fn test_connector_input() {
    let conn = Connector::input("test", 50.0);
    assert_eq!(conn.name, "test");
    assert_eq!(conn.connector_type, ConnectorType::Input);
    assert_eq!(conn.position, ConnectorPosition::Left(50.0));
}

#[test]
fn test_connector_output() {
    let conn = Connector::output("out", 75.0);
    assert_eq!(conn.name, "out");
    assert_eq!(conn.connector_type, ConnectorType::Output);
    assert_eq!(conn.position, ConnectorPosition::Right(75.0));
}

#[test]
fn test_video_input_node() {
    let node = Node::new_video_input(Position::new(100.0, 200.0));
    assert_eq!(node.position.x, 100.0);
    assert_eq!(node.position.y, 200.0);
    assert!(node.inputs.is_empty());
    assert_eq!(node.outputs.len(), 1);
    assert!(matches!(node.data, NodeData::VideoInput(_)));
}

#[test]
fn test_still_sampler_node() {
    let node = Node::new_still_sampler(Position::new(300.0, 200.0));
    assert_eq!(node.inputs.len(), 1);
    assert_eq!(node.outputs.len(), 1);
    assert_eq!(node.outputs[0].name, "stills_out");
    assert!(matches!(node.data, NodeData::StillSampler(_)));
}

#[test]
fn test_still_sampler_default_interval() {
    let data = StillSamplerData::default();
    assert_eq!(data.interval_seconds, 30);
}

#[test]
fn test_generate_stills() {
    let mut data = StillSamplerData::default();
    data.generate_stills(150.0); // 2.5 minutes
    // At 30s intervals: 0, 30, 60, 90, 120 = 5 stills
    assert_eq!(data.extracted_stills.len(), 5);
    assert_eq!(data.extracted_stills[0].timestamp_seconds, 0.0);
    assert_eq!(data.extracted_stills[1].timestamp_seconds, 30.0);
    assert_eq!(data.extracted_stills[4].timestamp_seconds, 120.0);
}

#[test]
fn test_generate_stills_5_min_video() {
    let mut data = StillSamplerData::default();
    data.generate_stills(300.0); // 5 minutes
    // At 30s intervals: 0, 30, 60, 90, 120, 150, 180, 210, 240, 270 = 10 stills
    assert_eq!(data.extracted_stills.len(), 10);
}

#[test]
fn test_node_data_type_name() {
    let video = NodeData::VideoInput(VideoInputData::default());
    let sampler = NodeData::StillSampler(StillSamplerData::default());
    assert_eq!(video.type_name(), "Video Input");
    assert_eq!(sampler.type_name(), "Still Sampler");
}

#[test]
fn test_upload_status_default() {
    let status = UploadStatus::default();
    assert!(matches!(status, UploadStatus::None));
}

#[test]
fn test_node_serialization() {
    let node = Node::new_video_input(Position::new(10.0, 20.0));
    let json = serde_json::to_string(&node).unwrap();
    let deserialized: Node = serde_json::from_str(&json).unwrap();
    assert_eq!(node.position, deserialized.position);
}
