//! Integration tests for connection types.

use uuid::Uuid;
use yt_rs_shared::{BezierControlPoints, Connection, PendingConnection, Position};

#[test]
fn test_bezier_control_points_horizontal() {
    let start = Position::new(0.0, 100.0);
    let end = Position::new(200.0, 100.0);
    let cp = BezierControlPoints::horizontal(start, end);

    // Control points should be horizontally offset
    assert_eq!(cp.cp1.x, 100.0);
    assert_eq!(cp.cp1.y, 100.0);
    assert_eq!(cp.cp2.x, 100.0);
    assert_eq!(cp.cp2.y, 100.0);
}

#[test]
fn test_bezier_control_points_vertical_offset() {
    let start = Position::new(0.0, 0.0);
    let end = Position::new(200.0, 100.0);
    let cp = BezierControlPoints::horizontal(start, end);

    // CP1 should be at start.y, CP2 at end.y
    assert_eq!(cp.cp1.y, 0.0);
    assert_eq!(cp.cp2.y, 100.0);
}

#[test]
fn test_connection_new() {
    let from_node = Uuid::new_v4();
    let from_conn = Uuid::new_v4();
    let to_node = Uuid::new_v4();
    let to_conn = Uuid::new_v4();

    let connection = Connection::new(from_node, from_conn, to_node, to_conn);

    assert_eq!(connection.from_node, from_node);
    assert_eq!(connection.from_connector, from_conn);
    assert_eq!(connection.to_node, to_node);
    assert_eq!(connection.to_connector, to_conn);
    assert!(connection.control_points.is_none());
}

#[test]
fn test_connection_svg_path() {
    let connection = Connection::new(
        Uuid::new_v4(),
        Uuid::new_v4(),
        Uuid::new_v4(),
        Uuid::new_v4(),
    );
    let start = Position::new(0.0, 50.0);
    let end = Position::new(100.0, 50.0);

    let path = connection.svg_path(start, end);

    assert!(path.starts_with("M 0,50"));
    assert!(path.contains("C "));
    assert!(path.ends_with("100,50"));
}

#[test]
fn test_pending_connection() {
    let from_node = Uuid::new_v4();
    let from_conn = Uuid::new_v4();
    let start = Position::new(100.0, 100.0);

    let mut pending = PendingConnection::new(from_node, from_conn, start);
    assert_eq!(pending.current_position, start);

    pending.current_position = Position::new(200.0, 150.0);
    let path = pending.svg_path();

    assert!(path.starts_with("M 100,100"));
    assert!(path.ends_with("200,150"));
}

#[test]
fn test_connection_serialization() {
    let connection = Connection::new(
        Uuid::new_v4(),
        Uuid::new_v4(),
        Uuid::new_v4(),
        Uuid::new_v4(),
    );
    let json = serde_json::to_string(&connection).unwrap();
    let deserialized: Connection = serde_json::from_str(&json).unwrap();
    assert_eq!(connection.id, deserialized.id);
}
