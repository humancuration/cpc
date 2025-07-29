//! Supply chain network entity and related types

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

use super::primitives::{NodeId, DomainError, Result, GeoLocation};
use super::consent::NetworkConsentSettings;

/// Types of nodes in a supply chain network
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeType {
    Supplier,
    Warehouse,
    DistributionCenter,
    RetailLocation,
    ManufacturingPlant,
    Customer,
}

/// A node in the supply chain network
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkNode {
    pub id: NodeId,
    pub name: String,
    pub node_type: NodeType,
    pub location: GeoLocation,
    pub capacity: Option<i32>, // Optional capacity constraint
    pub operating_hours: Option<String>, // Optional operating hours
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl NetworkNode {
    pub fn new(
        id: NodeId,
        name: String,
        node_type: NodeType,
        location: GeoLocation,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            node_type,
            location,
            capacity: None,
            operating_hours: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_capacity(mut self, capacity: i32) -> Self {
        self.capacity = Some(capacity);
        self
    }

    pub fn with_operating_hours(mut self, hours: String) -> Self {
        self.operating_hours = Some(hours);
        self
    }
}

/// Connection between two nodes in the supply chain network
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkConnection {
    pub id: Uuid,
    pub start_node_id: NodeId,
    pub end_node_id: NodeId,
    pub lead_time_days: i32,
    pub cost_per_unit: Option<f64>, // Optional cost for this connection
    pub transport_mode: String, // e.g., "truck", "ship", "air"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl NetworkConnection {
    pub fn new(
        id: Uuid,
        start_node_id: NodeId,
        end_node_id: NodeId,
        lead_time_days: i32,
        transport_mode: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            start_node_id,
            end_node_id,
            lead_time_days,
            cost_per_unit: None,
            transport_mode,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_cost(mut self, cost: f64) -> Self {
        self.cost_per_unit = Some(cost);
        self
    }
}

/// Supply chain network entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SupplyChainNetwork {
    pub id: Uuid,
    pub owner_id: Uuid, // Cooperative member ID
    pub name: String,
    pub nodes: Vec<NetworkNode>,
    pub connections: Vec<NetworkConnection>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub consent_settings: NetworkConsentSettings,
}

impl SupplyChainNetwork {
    /// Create a new supply chain network
    pub fn new(owner_id: Uuid, name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            owner_id,
            name,
            nodes: Vec::new(),
            connections: Vec::new(),
            created_at: now,
            updated_at: now,
            consent_settings: NetworkConsentSettings::default(),
        }
    }

    /// Add a node to the network
    pub fn add_node(&mut self, node: NetworkNode) -> Result<()> {
        // Validate that node doesn't already exist
        if self.nodes.iter().any(|n| n.id == node.id) {
            return Err(DomainError::ValidationError {
                message: "Node with this ID already exists in the network".to_string(),
            });
        }
        
        self.nodes.push(node);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Remove a node from the network
    pub fn remove_node(&mut self, node_id: NodeId) -> Result<()> {
        // Check if node exists
        if !self.nodes.iter().any(|n| n.id == node_id) {
            return Err(DomainError::NotFound);
        }
        
        // Remove connections to/from this node
        self.connections.retain(|c| {
            c.start_node_id != node_id && c.end_node_id != node_id
        });
        
        // Remove the node
        self.nodes.retain(|n| n.id != node_id);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Add a connection between nodes
    pub fn add_connection(&mut self, connection: NetworkConnection) -> Result<()> {
        // Validate that both nodes exist
        let start_exists = self.nodes.iter().any(|n| n.id == connection.start_node_id);
        let end_exists = self.nodes.iter().any(|n| n.id == connection.end_node_id);
        
        if !start_exists || !end_exists {
            return Err(DomainError::ValidationError {
                message: "Both start and end nodes must exist in the network".to_string(),
            });
        }
        
        // Validate that connection doesn't already exist
        if self.connections.iter().any(|c| {
            c.start_node_id == connection.start_node_id && 
            c.end_node_id == connection.end_node_id
        }) {
            return Err(DomainError::ValidationError {
                message: "Connection between these nodes already exists".to_string(),
            });
        }
        
        self.connections.push(connection);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Remove a connection from the network
    pub fn remove_connection(&mut self, connection_id: Uuid) -> Result<()> {
        let initial_count = self.connections.len();
        self.connections.retain(|c| c.id != connection_id);
        
        if self.connections.len() == initial_count {
            return Err(DomainError::NotFound);
        }
        
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Validate the network structure
    pub fn validate(&self) -> Result<()> {
        // Must have at least one supplier and one warehouse node
        let has_supplier = self.nodes.iter().any(|n| n.node_type == NodeType::Supplier);
        let has_warehouse = self.nodes.iter().any(|n| n.node_type == NodeType::Warehouse);
        
        if !has_supplier || !has_warehouse {
            return Err(DomainError::NetworkValidationError {
                message: "Network must contain at least one supplier and one warehouse node".to_string(),
            });
        }
        
        // All connections must have valid start/end nodes
        for connection in &self.connections {
            let start_exists = self.nodes.iter().any(|n| n.id == connection.start_node_id);
            let end_exists = self.nodes.iter().any(|n| n.id == connection.end_node_id);
            
            if !start_exists || !end_exists {
                return Err(DomainError::NetworkValidationError {
                    message: format!(
                        "Connection {} references non-existent nodes", 
                        connection.id
                    ),
                });
            }
            
            // Connection lead times must be positive
            if connection.lead_time_days <= 0 {
                return Err(DomainError::NetworkValidationError {
                    message: "Connection lead times must be positive values".to_string(),
                });
            }
        }
        
        // Network must be connected (no isolated nodes)
        let all_node_ids: std::collections::HashSet<NodeId> = self.nodes.iter().map(|n| n.id).collect();
        let connected_node_ids: std::collections::HashSet<NodeId> = self.connections
            .iter()
            .flat_map(|c| vec![c.start_node_id, c.end_node_id])
            .collect();
        
        let isolated_nodes: Vec<NodeId> = all_node_ids
            .difference(&connected_node_ids)
            .copied()
            .collect();
        
        if !isolated_nodes.is_empty() {
            return Err(DomainError::NetworkValidationError {
                message: "Network contains isolated nodes".to_string(),
            });
        }
        
        Ok(())
    }

    /// Calculate lead time between two nodes using Dijkstra's algorithm
    pub fn calculate_lead_time(&self, start_node: NodeId, end_node: NodeId) -> Result<i32> {
        // Validate that both nodes exist
        let start_exists = self.nodes.iter().any(|n| n.id == start_node);
        let end_exists = self.nodes.iter().any(|n| n.id == end_node);
        
        if !start_exists || !end_exists {
            return Err(DomainError::NotFound);
        }
        
        // If start and end are the same, lead time is 0
        if start_node == end_node {
            return Ok(0);
        }
        
        // Implementation of Dijkstra's algorithm for shortest path
        let mut distances: HashMap<NodeId, i32> = HashMap::new();
        let mut visited: std::collections::HashSet<NodeId> = std::collections::HashSet::new();
        let mut previous: HashMap<NodeId, Option<NodeId>> = HashMap::new();
        
        // Initialize distances
        for node in &self.nodes {
            distances.insert(node.id, i32::MAX);
            previous.insert(node.id, None);
        }
        distances.insert(start_node, 0);
        
        while !visited.contains(&end_node) {
            // Find unvisited node with smallest distance
            let current = distances
                .iter()
                .filter(|(node_id, _)| !visited.contains(node_id))
                .min_by_key(|(_, &distance)| distance)
                .map(|(node_id, _)| *node_id);
            
            let current = match current {
                Some(node_id) => node_id,
                None => break, // No path exists
            };
            
            visited.insert(current);
            
            // Update distances to neighbors
            for connection in &self.connections {
                let neighbor = if connection.start_node_id == current {
                    connection.end_node_id
                } else if connection.end_node_id == current {
                    connection.start_node_id
                } else {
                    continue; // Not connected to current node
                };
                
                if visited.contains(&neighbor) {
                    continue;
                }
                
                let new_distance = distances[&current] + connection.lead_time_days;
                if new_distance < distances[&neighbor] {
                    distances.insert(neighbor, new_distance);
                    previous.insert(neighbor, Some(current));
                }
            }
        }
        
        if distances[&end_node] == i32::MAX {
            Err(DomainError::NetworkValidationError {
                message: "No path exists between the specified nodes".to_string(),
            })
        } else {
            Ok(distances[&end_node])
        }
    }

    /// Set consent settings for the network
    pub fn set_consent_settings(&mut self, settings: NetworkConsentSettings) {
        self.consent_settings = settings;
        self.updated_at = Utc::now();
    }

    /// Check if the network is valid for sharing based on consent settings
    pub fn can_share_topology(&self) -> bool {
        matches!(
            self.consent_settings.share_topology,
            super::primitives::DataSharingLevel::ViewOnly | 
            super::primitives::DataSharingLevel::Editable | 
            super::primitives::DataSharingLevel::FullAccess
        )
    }

    /// Check if lead times can be shared
    pub fn can_share_lead_times(&self) -> bool {
        matches!(
            self.consent_settings.share_lead_times,
            super::primitives::DataSharingLevel::ViewOnly | 
            super::primitives::DataSharingLevel::Editable | 
            super::primitives::DataSharingLevel::FullAccess
        )
    }
}