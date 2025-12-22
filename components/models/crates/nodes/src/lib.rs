//! Node types and graph structure for yt-rs.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// Geometry types
// ============================================================================

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

// ============================================================================
// Canvas types
// ============================================================================

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

// ============================================================================
// Connection types
// ============================================================================

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

// ============================================================================
// Connector types
// ============================================================================

/// The type of a connector (input or output).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectorType {
    Input,
    Output,
}

/// The position of a connector relative to its node.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ConnectorPosition {
    /// Left side, with Y offset from top.
    Left(f64),
    /// Right side, with Y offset from top.
    Right(f64),
}

/// A connector on a node (input or output point for connections).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Connector {
    pub id: Uuid,
    pub name: String,
    pub connector_type: ConnectorType,
    pub position: ConnectorPosition,
}

impl Connector {
    /// Creates a new connector.
    pub fn new(
        name: impl Into<String>,
        connector_type: ConnectorType,
        position: ConnectorPosition,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            connector_type,
            position,
        }
    }

    /// Creates an input connector on the left side.
    pub fn input(name: impl Into<String>, y_offset: f64) -> Self {
        Self::new(
            name,
            ConnectorType::Input,
            ConnectorPosition::Left(y_offset),
        )
    }

    /// Creates an output connector on the right side.
    pub fn output(name: impl Into<String>, y_offset: f64) -> Self {
        Self::new(
            name,
            ConnectorType::Output,
            ConnectorPosition::Right(y_offset),
        )
    }
}

// ============================================================================
// Node data types
// ============================================================================

/// Upload status for video files.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum UploadStatus {
    #[default]
    None,
    Uploading {
        progress: f32,
    },
    Complete,
    Error(String),
}

/// Data specific to a VideoInput node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct VideoInputData {
    pub file_id: Option<Uuid>,
    pub file_name: Option<String>,
    pub file_path: Option<String>,
    pub duration_seconds: Option<f64>,
    pub upload_status: UploadStatus,
}

/// Processing status for still extraction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum ProcessingStatus {
    #[default]
    Idle,
    Processing {
        progress: f32,
    },
    Complete,
    Error(String),
}

/// An extracted still image.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Still {
    pub id: Uuid,
    pub timestamp_seconds: f64,
    pub thumbnail_url: Option<String>,
}

impl Still {
    /// Creates a new still at the given timestamp.
    pub fn new(timestamp_seconds: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp_seconds,
            thumbnail_url: None,
        }
    }
}

/// Data specific to a StillSampler node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StillSamplerData {
    pub interval_seconds: u32,
    pub extracted_stills: Vec<Still>,
    pub processing_status: ProcessingStatus,
}

impl Default for StillSamplerData {
    fn default() -> Self {
        Self {
            interval_seconds: 30,
            extracted_stills: Vec::new(),
            processing_status: ProcessingStatus::Idle,
        }
    }
}

impl StillSamplerData {
    /// Generates stills based on video duration and interval.
    pub fn generate_stills(&mut self, duration_seconds: f64) {
        self.extracted_stills.clear();
        let mut timestamp = 0.0;
        while timestamp < duration_seconds {
            self.extracted_stills.push(Still::new(timestamp));
            timestamp += self.interval_seconds as f64;
        }
    }
}

/// Data specific to a Viewer node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ViewerData {
    /// Path to the generated thumbnail (relative to data dir).
    pub thumbnail_path: Option<String>,
    /// Timestamp in seconds where the thumbnail was extracted.
    pub thumbnail_timestamp: f64,
}

/// Data specific to a Selector node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SelectorData {
    /// The index of the selected still (0-based, default 0).
    pub selected_index: usize,
}

/// Data specific to a StillPreview node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StillPreviewData {
    // Marker type - resolves data from connection at render time
}

/// The type-specific data for a node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NodeData {
    VideoInput(VideoInputData),
    StillSampler(StillSamplerData),
    Viewer(ViewerData),
    Selector(SelectorData),
    StillPreview(StillPreviewData),
}

impl NodeData {
    /// Returns the display name for this node type.
    pub fn type_name(&self) -> &'static str {
        match self {
            NodeData::VideoInput(_) => "Video Input",
            NodeData::StillSampler(_) => "Still Sampler",
            NodeData::Viewer(_) => "Viewer",
            NodeData::Selector(_) => "Selector",
            NodeData::StillPreview(_) => "Still Preview",
        }
    }
}

// ============================================================================
// Node type
// ============================================================================

/// A node in the canvas.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub position: Position,
    pub size: Size,
    pub data: NodeData,
    pub inputs: Vec<Connector>,
    pub outputs: Vec<Connector>,
    pub z_index: u32,
}

impl Node {
    /// Creates a new VideoInput node at the given position.
    pub fn new_video_input(position: Position) -> Self {
        Self {
            id: Uuid::new_v4(),
            position,
            size: Size::new(200.0, 120.0),
            data: NodeData::VideoInput(VideoInputData::default()),
            inputs: Vec::new(),
            outputs: vec![Connector::output("video_out", 60.0)],
            z_index: 0,
        }
    }

    /// Creates a new StillSampler node at the given position.
    pub fn new_still_sampler(position: Position) -> Self {
        Self {
            id: Uuid::new_v4(),
            position,
            size: Size::new(220.0, 150.0),
            data: NodeData::StillSampler(StillSamplerData::default()),
            inputs: vec![Connector::input("video_in", 40.0)],
            outputs: vec![Connector::output("stills_out", 100.0)],
            z_index: 0,
        }
    }

    /// Creates a new Viewer node at the given position.
    pub fn new_viewer(position: Position) -> Self {
        Self {
            id: Uuid::new_v4(),
            position,
            size: Size::new(200.0, 140.0),
            data: NodeData::Viewer(ViewerData::default()),
            inputs: vec![Connector::input("video_in", 70.0)],
            outputs: Vec::new(),
            z_index: 0,
        }
    }

    /// Creates a new Selector node at the given position.
    pub fn new_selector(position: Position) -> Self {
        Self {
            id: Uuid::new_v4(),
            position,
            size: Size::new(180.0, 100.0),
            data: NodeData::Selector(SelectorData::default()),
            inputs: vec![Connector::input("stills_in", 50.0)],
            outputs: vec![
                Connector::output("selected_out", 30.0),
                Connector::output("array_out", 70.0),
            ],
            z_index: 0,
        }
    }

    /// Creates a new StillPreview node at the given position.
    pub fn new_still_preview(position: Position) -> Self {
        Self {
            id: Uuid::new_v4(),
            position,
            size: Size::new(200.0, 160.0),
            data: NodeData::StillPreview(StillPreviewData::default()),
            inputs: vec![Connector::input("still_in", 80.0)],
            outputs: Vec::new(),
            z_index: 0,
        }
    }
}
