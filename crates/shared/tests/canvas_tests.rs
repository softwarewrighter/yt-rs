//! Integration tests for canvas and viewport types.

use yt_rs_shared::{CanvasState, Position, Size, Viewport};

#[test]
fn test_position_new() {
    let pos = Position::new(10.0, 20.0);
    assert_eq!(pos.x, 10.0);
    assert_eq!(pos.y, 20.0);
}

#[test]
fn test_position_distance() {
    let p1 = Position::new(0.0, 0.0);
    let p2 = Position::new(3.0, 4.0);
    assert!((p1.distance_to(&p2) - 5.0).abs() < f64::EPSILON);
}

#[test]
fn test_size_new() {
    let size = Size::new(100.0, 200.0);
    assert_eq!(size.width, 100.0);
    assert_eq!(size.height, 200.0);
}

#[test]
fn test_viewport_default() {
    let viewport = Viewport::default();
    assert_eq!(viewport.width, 1280.0);
    assert_eq!(viewport.height, 720.0);
}

#[test]
fn test_canvas_state_default() {
    let state = CanvasState::default();
    assert_eq!(state.zoom, 1.0);
    assert_eq!(state.pan_offset.x, 0.0);
    assert_eq!(state.pan_offset.y, 0.0);
}

#[test]
fn test_screen_to_canvas_no_transform() {
    let state = CanvasState::default();
    let screen = Position::new(100.0, 200.0);
    let canvas = state.screen_to_canvas(screen);
    assert_eq!(canvas.x, 100.0);
    assert_eq!(canvas.y, 200.0);
}

#[test]
fn test_screen_to_canvas_with_pan() {
    let state = CanvasState {
        pan_offset: Position::new(50.0, 50.0),
        ..Default::default()
    };
    let screen = Position::new(100.0, 100.0);
    let canvas = state.screen_to_canvas(screen);
    assert_eq!(canvas.x, 50.0);
    assert_eq!(canvas.y, 50.0);
}

#[test]
fn test_screen_to_canvas_with_zoom() {
    let state = CanvasState {
        zoom: 2.0,
        ..Default::default()
    };
    let screen = Position::new(100.0, 100.0);
    let canvas = state.screen_to_canvas(screen);
    assert_eq!(canvas.x, 50.0);
    assert_eq!(canvas.y, 50.0);
}

#[test]
fn test_canvas_to_screen_roundtrip() {
    let state = CanvasState {
        zoom: 1.5,
        pan_offset: Position::new(30.0, 40.0),
        ..Default::default()
    };
    let original = Position::new(100.0, 200.0);
    let screen = state.canvas_to_screen(original);
    let back = state.screen_to_canvas(screen);
    assert!((back.x - original.x).abs() < f64::EPSILON);
    assert!((back.y - original.y).abs() < f64::EPSILON);
}

#[test]
fn test_position_serialization() {
    let pos = Position::new(10.0, 20.0);
    let json = serde_json::to_string(&pos).unwrap();
    let deserialized: Position = serde_json::from_str(&json).unwrap();
    assert_eq!(pos, deserialized);
}

#[test]
fn test_canvas_state_serialization() {
    let state = CanvasState::default();
    let json = serde_json::to_string(&state).unwrap();
    let deserialized: CanvasState = serde_json::from_str(&json).unwrap();
    assert_eq!(state, deserialized);
}
