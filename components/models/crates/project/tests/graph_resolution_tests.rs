//! Tests for graph resolution methods.

use uuid::Uuid;
use yt_rs_project::{Connection, Node, NodeData, Position, Project};

fn setup_video_node() -> Node {
    let mut node = Node::new_video_input(Position::new(0.0, 0.0));
    if let NodeData::VideoInput(ref mut data) = node.data {
        data.file_id = Some(Uuid::new_v4());
        data.file_name = Some("test.mp4".to_string());
        data.duration_seconds = Some(60.0);
    }
    node
}

fn setup_sampler_with_stills() -> Node {
    let mut node = Node::new_still_sampler(Position::new(200.0, 0.0));
    if let NodeData::StillSampler(ref mut data) = node.data {
        data.interval_seconds = 30;
        data.generate_stills(60.0); // Creates stills at 0s and 30s
    }
    node
}

#[test]
fn test_find_upstream_node_direct_connection() {
    let mut project = Project::new("Test");

    let video = setup_video_node();
    let video_id = video.id;
    let video_out = video.outputs[0].id;

    let sampler = Node::new_still_sampler(Position::new(200.0, 0.0));
    let sampler_id = sampler.id;
    let sampler_in = sampler.inputs[0].id;

    project.add_node(video);
    project.add_node(sampler);
    project.add_connection(Connection::new(video_id, video_out, sampler_id, sampler_in));

    let upstream = project.find_upstream_node(sampler_id, "video_in");
    assert!(upstream.is_some());
    assert_eq!(upstream.unwrap().id, video_id);
}

#[test]
fn test_find_upstream_node_no_connection() {
    let mut project = Project::new("Test");

    let sampler = Node::new_still_sampler(Position::new(200.0, 0.0));
    let sampler_id = sampler.id;
    project.add_node(sampler);

    let upstream = project.find_upstream_node(sampler_id, "video_in");
    assert!(upstream.is_none());
}

#[test]
fn test_find_upstream_node_wrong_input_name() {
    let mut project = Project::new("Test");

    let video = setup_video_node();
    let video_id = video.id;
    let video_out = video.outputs[0].id;

    let sampler = Node::new_still_sampler(Position::new(200.0, 0.0));
    let sampler_id = sampler.id;
    let sampler_in = sampler.inputs[0].id;

    project.add_node(video);
    project.add_node(sampler);
    project.add_connection(Connection::new(video_id, video_out, sampler_id, sampler_in));

    // Wrong input name
    let upstream = project.find_upstream_node(sampler_id, "nonexistent");
    assert!(upstream.is_none());
}

#[test]
fn test_resolve_stills_from_sampler() {
    let mut project = Project::new("Test");

    let sampler = setup_sampler_with_stills();
    let sampler_id = sampler.id;
    let sampler_out = sampler.outputs[0].id;

    let selector = Node::new_selector(Position::new(400.0, 0.0));
    let selector_id = selector.id;
    let selector_in = selector.inputs[0].id;

    project.add_node(sampler);
    project.add_node(selector);
    project.add_connection(Connection::new(
        sampler_id,
        sampler_out,
        selector_id,
        selector_in,
    ));

    let stills = project.resolve_stills(selector_id);
    assert!(stills.is_some());
    let stills = stills.unwrap();
    assert_eq!(stills.len(), 2);
    assert_eq!(stills[0].timestamp_seconds, 0.0);
    assert_eq!(stills[1].timestamp_seconds, 30.0);
}

#[test]
fn test_resolve_stills_through_selector_chain() {
    let mut project = Project::new("Test");

    let sampler = setup_sampler_with_stills();
    let sampler_id = sampler.id;
    let sampler_out = sampler.outputs[0].id;

    let selector1 = Node::new_selector(Position::new(400.0, 0.0));
    let selector1_id = selector1.id;
    let selector1_in = selector1.inputs[0].id;
    let selector1_array_out = selector1.outputs[1].id; // array_out is at index 1

    let selector2 = Node::new_selector(Position::new(600.0, 0.0));
    let selector2_id = selector2.id;
    let selector2_in = selector2.inputs[0].id;

    project.add_node(sampler);
    project.add_node(selector1);
    project.add_node(selector2);

    // sampler -> selector1
    project.add_connection(Connection::new(
        sampler_id,
        sampler_out,
        selector1_id,
        selector1_in,
    ));
    // selector1 (array_out) -> selector2
    project.add_connection(Connection::new(
        selector1_id,
        selector1_array_out,
        selector2_id,
        selector2_in,
    ));

    // selector2 should resolve stills through selector1 back to sampler
    let stills = project.resolve_stills(selector2_id);
    assert!(stills.is_some());
    assert_eq!(stills.unwrap().len(), 2);
}

#[test]
fn test_resolve_selected_still_index_0() {
    let mut project = Project::new("Test");

    let sampler = setup_sampler_with_stills();
    let sampler_id = sampler.id;
    let sampler_out = sampler.outputs[0].id;

    let mut selector = Node::new_selector(Position::new(400.0, 0.0));
    if let NodeData::Selector(ref mut data) = selector.data {
        data.selected_index = 0;
    }
    let selector_id = selector.id;
    let selector_in = selector.inputs[0].id;
    let selector_selected_out = selector.outputs[0].id;

    let preview = Node::new_still_preview(Position::new(600.0, 0.0));
    let preview_id = preview.id;
    let preview_in = preview.inputs[0].id;

    project.add_node(sampler);
    project.add_node(selector);
    project.add_node(preview);

    project.add_connection(Connection::new(
        sampler_id,
        sampler_out,
        selector_id,
        selector_in,
    ));
    project.add_connection(Connection::new(
        selector_id,
        selector_selected_out,
        preview_id,
        preview_in,
    ));

    let still = project.resolve_selected_still(preview_id);
    assert!(still.is_some());
    assert_eq!(still.unwrap().timestamp_seconds, 0.0);
}

#[test]
fn test_resolve_selected_still_index_1() {
    let mut project = Project::new("Test");

    let sampler = setup_sampler_with_stills();
    let sampler_id = sampler.id;
    let sampler_out = sampler.outputs[0].id;

    let mut selector = Node::new_selector(Position::new(400.0, 0.0));
    if let NodeData::Selector(ref mut data) = selector.data {
        data.selected_index = 1;
    }
    let selector_id = selector.id;
    let selector_in = selector.inputs[0].id;
    let selector_selected_out = selector.outputs[0].id;

    let preview = Node::new_still_preview(Position::new(600.0, 0.0));
    let preview_id = preview.id;
    let preview_in = preview.inputs[0].id;

    project.add_node(sampler);
    project.add_node(selector);
    project.add_node(preview);

    project.add_connection(Connection::new(
        sampler_id,
        sampler_out,
        selector_id,
        selector_in,
    ));
    project.add_connection(Connection::new(
        selector_id,
        selector_selected_out,
        preview_id,
        preview_in,
    ));

    let still = project.resolve_selected_still(preview_id);
    assert!(still.is_some());
    assert_eq!(still.unwrap().timestamp_seconds, 30.0);
}

#[test]
fn test_resolve_video_source_from_preview() {
    let mut project = Project::new("Test");

    let video = setup_video_node();
    let video_id = video.id;
    let video_out = video.outputs[0].id;
    let expected_file_id = match &video.data {
        NodeData::VideoInput(data) => data.file_id,
        _ => None,
    };

    let sampler = setup_sampler_with_stills();
    let sampler_id = sampler.id;
    let sampler_in = sampler.inputs[0].id;
    let sampler_out = sampler.outputs[0].id;

    let selector = Node::new_selector(Position::new(400.0, 0.0));
    let selector_id = selector.id;
    let selector_in = selector.inputs[0].id;
    let selector_selected_out = selector.outputs[0].id;

    let preview = Node::new_still_preview(Position::new(600.0, 0.0));
    let preview_id = preview.id;
    let preview_in = preview.inputs[0].id;

    project.add_node(video);
    project.add_node(sampler);
    project.add_node(selector);
    project.add_node(preview);

    project.add_connection(Connection::new(video_id, video_out, sampler_id, sampler_in));
    project.add_connection(Connection::new(
        sampler_id,
        sampler_out,
        selector_id,
        selector_in,
    ));
    project.add_connection(Connection::new(
        selector_id,
        selector_selected_out,
        preview_id,
        preview_in,
    ));

    let video_source = project.resolve_video_source(preview_id);
    assert!(video_source.is_some());
    let video_data = video_source.unwrap();
    assert_eq!(video_data.file_id, expected_file_id);
    assert_eq!(video_data.file_name.as_deref(), Some("test.mp4"));
}

#[test]
fn test_full_chain_two_selectors_different_indices() {
    let mut project = Project::new("Test");

    // VideoInput -> StillSampler -> Selector(0) -> StillPreview (0s)
    //                                    │
    //                                    └─(array)─> Selector(1) -> StillPreview (30s)

    let video = setup_video_node();
    let video_id = video.id;
    let video_out = video.outputs[0].id;

    let sampler = setup_sampler_with_stills();
    let sampler_id = sampler.id;
    let sampler_in = sampler.inputs[0].id;
    let sampler_out = sampler.outputs[0].id;

    let mut selector1 = Node::new_selector(Position::new(400.0, 0.0));
    if let NodeData::Selector(ref mut data) = selector1.data {
        data.selected_index = 0;
    }
    let selector1_id = selector1.id;
    let selector1_in = selector1.inputs[0].id;
    let selector1_selected_out = selector1.outputs[0].id;
    let selector1_array_out = selector1.outputs[1].id;

    let mut selector2 = Node::new_selector(Position::new(400.0, 200.0));
    if let NodeData::Selector(ref mut data) = selector2.data {
        data.selected_index = 1;
    }
    let selector2_id = selector2.id;
    let selector2_in = selector2.inputs[0].id;
    let selector2_selected_out = selector2.outputs[0].id;

    let preview1 = Node::new_still_preview(Position::new(600.0, 0.0));
    let preview1_id = preview1.id;
    let preview1_in = preview1.inputs[0].id;

    let preview2 = Node::new_still_preview(Position::new(600.0, 200.0));
    let preview2_id = preview2.id;
    let preview2_in = preview2.inputs[0].id;

    project.add_node(video);
    project.add_node(sampler);
    project.add_node(selector1);
    project.add_node(selector2);
    project.add_node(preview1);
    project.add_node(preview2);

    // Connect: video -> sampler
    project.add_connection(Connection::new(video_id, video_out, sampler_id, sampler_in));
    // Connect: sampler -> selector1
    project.add_connection(Connection::new(
        sampler_id,
        sampler_out,
        selector1_id,
        selector1_in,
    ));
    // Connect: selector1 (selected_out) -> preview1
    project.add_connection(Connection::new(
        selector1_id,
        selector1_selected_out,
        preview1_id,
        preview1_in,
    ));
    // Connect: selector1 (array_out) -> selector2
    project.add_connection(Connection::new(
        selector1_id,
        selector1_array_out,
        selector2_id,
        selector2_in,
    ));
    // Connect: selector2 (selected_out) -> preview2
    project.add_connection(Connection::new(
        selector2_id,
        selector2_selected_out,
        preview2_id,
        preview2_in,
    ));

    // Preview1 should show still at 0.0s (selector1 index 0)
    let still1 = project.resolve_selected_still(preview1_id);
    assert!(still1.is_some());
    assert_eq!(still1.unwrap().timestamp_seconds, 0.0);

    // Preview2 should show still at 30.0s (selector2 index 1)
    let still2 = project.resolve_selected_still(preview2_id);
    assert!(still2.is_some());
    assert_eq!(still2.unwrap().timestamp_seconds, 30.0);

    // Both previews should resolve to the same video source
    let video_source1 = project.resolve_video_source(preview1_id);
    let video_source2 = project.resolve_video_source(preview2_id);
    assert!(video_source1.is_some());
    assert!(video_source2.is_some());
    assert_eq!(
        video_source1.unwrap().file_id,
        video_source2.unwrap().file_id
    );
}

#[test]
fn test_resolve_selected_still_out_of_bounds() {
    let mut project = Project::new("Test");

    let sampler = setup_sampler_with_stills(); // 2 stills
    let sampler_id = sampler.id;
    let sampler_out = sampler.outputs[0].id;

    let mut selector = Node::new_selector(Position::new(400.0, 0.0));
    if let NodeData::Selector(ref mut data) = selector.data {
        data.selected_index = 999; // Out of bounds
    }
    let selector_id = selector.id;
    let selector_in = selector.inputs[0].id;
    let selector_selected_out = selector.outputs[0].id;

    let preview = Node::new_still_preview(Position::new(600.0, 0.0));
    let preview_id = preview.id;
    let preview_in = preview.inputs[0].id;

    project.add_node(sampler);
    project.add_node(selector);
    project.add_node(preview);

    project.add_connection(Connection::new(
        sampler_id,
        sampler_out,
        selector_id,
        selector_in,
    ));
    project.add_connection(Connection::new(
        selector_id,
        selector_selected_out,
        preview_id,
        preview_in,
    ));

    // Out of bounds index should return None
    let still = project.resolve_selected_still(preview_id);
    assert!(still.is_none());
}
