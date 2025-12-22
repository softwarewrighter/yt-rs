//! Canvas and viewport state models.

use serde::{Deserialize, Serialize};

/// A 2D position in canvas coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    /// Creates a new position.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Calculates the distance to another position.
    pub fn distance_to(&self, other: &Position) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// A 2D size (width and height).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    /// Creates a new size.
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

/// The viewport state (visible area of the canvas).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Viewport {
    pub width: f64,
    pub height: f64,
    pub scroll_x: f64,
    pub scroll_y: f64,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            width: 1280.0,
            height: 720.0,
            scroll_x: 0.0,
            scroll_y: 0.0,
        }
    }
}

/// The canvas state including pan, zoom, and viewport.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanvasState {
    pub viewport: Viewport,
    pub zoom: f64,
    pub pan_offset: Position,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self {
            viewport: Viewport::default(),
            zoom: 1.0,
            pan_offset: Position::default(),
        }
    }
}

impl CanvasState {
    /// Converts screen coordinates to canvas coordinates.
    pub fn screen_to_canvas(&self, screen_pos: Position) -> Position {
        Position {
            x: (screen_pos.x - self.pan_offset.x) / self.zoom,
            y: (screen_pos.y - self.pan_offset.y) / self.zoom,
        }
    }

    /// Converts canvas coordinates to screen coordinates.
    pub fn canvas_to_screen(&self, canvas_pos: Position) -> Position {
        Position {
            x: canvas_pos.x * self.zoom + self.pan_offset.x,
            y: canvas_pos.y * self.zoom + self.pan_offset.y,
        }
    }
}
