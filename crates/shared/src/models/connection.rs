//! Connection models for node editor.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Position;

/// Control points for a cubic bezier curve.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BezierControlPoints {
    pub cp1: Position,
    pub cp2: Position,
}

impl BezierControlPoints {
    /// Creates control points for a horizontal bezier curve.
    pub fn horizontal(start: Position, end: Position) -> Self {
        let dx = (end.x - start.x).abs() * 0.5;
        Self {
            cp1: Position::new(start.x + dx, start.y),
            cp2: Position::new(end.x - dx, end.y),
        }
    }
}

/// A connection between two node connectors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Connection {
    pub id: Uuid,
    pub from_node: Uuid,
    pub from_connector: Uuid,
    pub to_node: Uuid,
    pub to_connector: Uuid,
    pub control_points: Option<BezierControlPoints>,
}

impl Connection {
    /// Creates a new connection between two connectors.
    pub fn new(from_node: Uuid, from_connector: Uuid, to_node: Uuid, to_connector: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            from_node,
            from_connector,
            to_node,
            to_connector,
            control_points: None,
        }
    }

    /// Generates the SVG path data for this connection.
    pub fn svg_path(&self, start: Position, end: Position) -> String {
        let cp = self
            .control_points
            .clone()
            .unwrap_or_else(|| BezierControlPoints::horizontal(start, end));

        format!(
            "M {},{} C {},{} {},{} {},{}",
            start.x, start.y, cp.cp1.x, cp.cp1.y, cp.cp2.x, cp.cp2.y, end.x, end.y
        )
    }
}

/// A pending connection being drawn by the user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PendingConnection {
    pub from_node: Uuid,
    pub from_connector: Uuid,
    pub start_position: Position,
    pub current_position: Position,
}

impl PendingConnection {
    /// Creates a new pending connection.
    pub fn new(from_node: Uuid, from_connector: Uuid, start_position: Position) -> Self {
        Self {
            from_node,
            from_connector,
            start_position,
            current_position: start_position,
        }
    }

    /// Generates the SVG path data for the pending connection.
    pub fn svg_path(&self) -> String {
        let cp = BezierControlPoints::horizontal(self.start_position, self.current_position);
        format!(
            "M {},{} C {},{} {},{} {},{}",
            self.start_position.x,
            self.start_position.y,
            cp.cp1.x,
            cp.cp1.y,
            cp.cp2.x,
            cp.cp2.y,
            self.current_position.x,
            self.current_position.y
        )
    }
}
