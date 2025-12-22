//! Project serialization for yt-rs.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use yt_rs_nodes::{
    BezierControlPoints, CanvasState, Connection, Connector, ConnectorPosition, ConnectorType,
    Node, NodeData, PendingConnection, Position, ProcessingStatus, Size, Still, StillSamplerData,
    UploadStatus, VideoInputData, Viewport,
};

/// A project containing the full canvas state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub canvas_state: CanvasState,
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
}

impl Project {
    /// Creates a new empty project with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            created_at: now,
            updated_at: now,
            canvas_state: CanvasState::default(),
            nodes: Vec::new(),
            connections: Vec::new(),
        }
    }

    /// Adds a node to the project.
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
        self.updated_at = Utc::now();
    }

    /// Removes a node by ID, also removing any connections to/from it.
    pub fn remove_node(&mut self, node_id: Uuid) -> Option<Node> {
        // Remove connections involving this node
        self.connections
            .retain(|c| c.from_node != node_id && c.to_node != node_id);

        // Remove the node
        if let Some(pos) = self.nodes.iter().position(|n| n.id == node_id) {
            self.updated_at = Utc::now();
            Some(self.nodes.remove(pos))
        } else {
            None
        }
    }

    /// Finds a node by ID.
    pub fn find_node(&self, node_id: Uuid) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == node_id)
    }

    /// Finds a node by ID (mutable).
    pub fn find_node_mut(&mut self, node_id: Uuid) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.id == node_id)
    }

    /// Adds a connection to the project.
    pub fn add_connection(&mut self, connection: Connection) {
        self.connections.push(connection);
        self.updated_at = Utc::now();
    }

    /// Removes a connection by ID.
    pub fn remove_connection(&mut self, connection_id: Uuid) -> Option<Connection> {
        if let Some(pos) = self.connections.iter().position(|c| c.id == connection_id) {
            self.updated_at = Utc::now();
            Some(self.connections.remove(pos))
        } else {
            None
        }
    }
}

impl Default for Project {
    fn default() -> Self {
        Self::new("Untitled Project")
    }
}
