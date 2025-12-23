//! State management tests.

use std::rc::Rc;
use yew::Reducible;
use yt_rs_frontend::state::{AppAction, AppState};
use yt_rs_shared::{NodeData, Position, StillSamplerData, VideoInputData};

#[test]
fn test_app_state_default() {
    let state = AppState::default();
    assert!(state.nodes.is_empty());
    assert!(state.connections.is_empty());
    assert!(state.selected_node.is_none());
}

#[test]
fn test_create_video_node() {
    let state = Rc::new(AppState::default());
    let data = NodeData::VideoInput(VideoInputData::default());
    let position = Position::new(100.0, 200.0);
    let new_state = state.reduce(AppAction::CreateNode(data, position));
    assert_eq!(new_state.nodes.len(), 1);
}

#[test]
fn test_create_sampler_node() {
    let state = Rc::new(AppState::default());
    let data = NodeData::StillSampler(StillSamplerData::default());
    let position = Position::new(100.0, 200.0);
    let new_state = state.reduce(AppAction::CreateNode(data, position));
    assert_eq!(new_state.nodes.len(), 1);
}

#[test]
fn test_zoom_clamping() {
    let state = Rc::new(AppState::default());
    let state = state.reduce(AppAction::SetZoom(10.0));
    assert_eq!(state.canvas.zoom, 4.0);
    let state = state.reduce(AppAction::SetZoom(0.1));
    assert_eq!(state.canvas.zoom, 0.25);
}
