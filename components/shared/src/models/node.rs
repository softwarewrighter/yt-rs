//! Node types and data models.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Position, Size};

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

/// The type-specific data for a node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NodeData {
    VideoInput(VideoInputData),
    StillSampler(StillSamplerData),
}

impl NodeData {
    /// Returns the display name for this node type.
    pub fn type_name(&self) -> &'static str {
        match self {
            NodeData::VideoInput(_) => "Video Input",
            NodeData::StillSampler(_) => "Still Sampler",
        }
    }
}

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
}
