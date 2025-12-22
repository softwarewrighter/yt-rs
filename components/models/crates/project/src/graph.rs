//! Graph resolution methods for traversing node connections.

use uuid::Uuid;

use crate::{Node, NodeData, Project, Still, VideoInputData};

impl Project {
    /// Finds the node connected to a specific input of the given node.
    pub fn find_upstream_node(&self, node_id: Uuid, input_name: &str) -> Option<&Node> {
        let node = self.find_node(node_id)?;
        let input_conn = node.inputs.iter().find(|c| c.name == input_name)?;
        let connection = self
            .connections
            .iter()
            .find(|c| c.to_node == node_id && c.to_connector == input_conn.id)?;
        self.find_node(connection.from_node)
    }

    /// Resolves the stills array for a node by traversing upstream.
    /// Works for Selector nodes connected to StillSampler or other Selectors.
    pub fn resolve_stills(&self, node_id: Uuid) -> Option<&Vec<Still>> {
        let upstream = self.find_upstream_node(node_id, "stills_in")?;
        match &upstream.data {
            NodeData::StillSampler(data) => Some(&data.extracted_stills),
            NodeData::Selector(_) => {
                // Selector's array_out passes through the stills array
                self.resolve_stills(upstream.id)
            }
            _ => None,
        }
    }

    /// Resolves the selected Still for a StillPreview node.
    /// Traverses upstream to find the connected Selector and returns its selected still.
    pub fn resolve_selected_still(&self, node_id: Uuid) -> Option<&Still> {
        let upstream = self.find_upstream_node(node_id, "still_in")?;
        match &upstream.data {
            NodeData::Selector(data) => {
                let stills = self.resolve_stills(upstream.id)?;
                stills.get(data.selected_index)
            }
            _ => None,
        }
    }

    /// Resolves the source VideoInput for any downstream node.
    /// Traverses upstream through all connections until finding a VideoInput node.
    pub fn resolve_video_source(&self, node_id: Uuid) -> Option<&VideoInputData> {
        let node = self.find_node(node_id)?;
        // Traverse upstream through any input
        for input in &node.inputs {
            let maybe_conn = self
                .connections
                .iter()
                .find(|c| c.to_node == node_id && c.to_connector == input.id);
            if let Some(conn) = maybe_conn {
                let upstream = self.find_node(conn.from_node)?;
                match &upstream.data {
                    NodeData::VideoInput(data) => return Some(data),
                    _ => {
                        if let Some(data) = self.resolve_video_source(upstream.id) {
                            return Some(data);
                        }
                    }
                }
            }
        }
        None
    }
}
