use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use serde_json::Value as JsonValue;

use crate::crdt::{CRDTId, DocumentOperation};
use crate::domain::models::DocumentContent;

pub struct CRDTDocument {
    elements: HashMap<CRDTId, ElementState>,
    version_vector: HashMap<Uuid, i64>, // Node ID to logical clock
    logical_clock: i64,
    node_id: Uuid,
    operation_counter: u64,
    // Ratchet sessions for end-to-end encryption
    ratchet_sessions: HashMap<Uuid, Vec<u8>>, // Node ID to ratchet session state
}

pub struct ElementState {
    pub value: JsonValue,
    pub created_at: i64,
    pub deleted: bool,
    pub parent_id: Option<CRDTId>,
    // Additional metadata as needed
}

/// Result of comparing two version vectors
#[derive(Debug, Clone, PartialEq)]
pub enum VersionVectorComparison {
    /// Version vectors are equal
    Equal,
    /// Local version vector is ahead of remote
    LocalAhead(Vec<Uuid>),
    /// Remote version vector is ahead of local
    RemoteAhead(Vec<Uuid>),
    /// Version vectors are concurrent (conflicting updates)
    Concurrent {
        local_ahead: Vec<Uuid>,
        remote_ahead: Vec<Uuid>,
        concurrent: Vec<Uuid>,
    },
}

impl CRDTDocument {
    pub fn new(node_id: Uuid) -> Self {
        Self {
            elements: HashMap::new(),
            version_vector: HashMap::new(),
            logical_clock: 0,
            node_id,
            operation_counter: 0,
            ratchet_sessions: HashMap::new(),
        }
    }
    
    pub fn generate_id(&mut self) -> CRDTId {
        self.operation_counter += 1;
        self.logical_clock += 1;
        
        CRDTId {
            node_id: self.node_id,
            counter: self.operation_counter,
            timestamp: self.logical_clock,
        }
    }
    
    pub fn apply_operation(&mut self, operation: &DocumentOperation, source_node: Uuid) -> bool {
        // Update version vector for the source node
        let current = self.version_vector.get(&source_node).cloned().unwrap_or(0);
        self.version_vector.insert(source_node, std::cmp::max(current, operation.timestamp()));
        
        match operation {
            DocumentOperation::Insert { position, value, id, parent_id } => {
                // Check for conflicts using version vector comparison
                // In a real implementation, this would be more sophisticated
                let element_state = ElementState {
                    value: value.clone(),
                    created_at: id.timestamp,
                    deleted: false,
                    parent_id: parent_id.clone(),
                };
                
                self.elements.insert(id.clone(), element_state);
            }
            DocumentOperation::Delete { id, timestamp } => {
                // Check for conflicts using version vector comparison
                // In a real implementation, this would be more sophisticated
                
                // Mark element as deleted if not already
                if let Some(element_state) = self.elements.get_mut(id) {
                    element_state.deleted = true;
                } else {
                    // Create a tombstone for the element if it doesn't exist locally
                    let element_state = ElementState {
                        value: JsonValue::Null,
                        created_at: *timestamp,
                        deleted: true,
                        parent_id: None,
                    };
                    self.elements.insert(id.clone(), element_state);
                }
            }
            DocumentOperation::Update { id, value, timestamp } => {
                // Check for conflicts using version vector comparison
                // In a real implementation, this would be more sophisticated
                
                if let Some(element_state) = self.elements.get_mut(id) {
                    element_state.value = value.clone();
                    element_state.created_at = *timestamp;
                } else {
                    // Create the element if it doesn't exist locally
                    let element_state = ElementState {
                        value: value.clone(),
                        created_at: *timestamp,
                        deleted: false,
                        parent_id: None,
                    };
                    self.elements.insert(id.clone(), element_state);
                }
            }
            DocumentOperation::Formatting { id, style: _, timestamp } => {
                // Formatting operations would typically be handled differently
                // For now, we'll just ensure the element exists
                if !self.elements.contains_key(id) {
                    let element_state = ElementState {
                        value: JsonValue::Null,
                        created_at: *timestamp,
                        deleted: false,
                        parent_id: None,
                    };
                    self.elements.insert(id.clone(), element_state);
                }
            }
        }
        
        true
    }
    
    pub fn to_document_content(&self) -> DocumentContent {
        // Convert CRDT state to DocumentContent
        // This is a simplified implementation that just serializes all non-deleted elements
        let mut content_map = serde_json::Map::new();
        
        for (id, state) in &self.elements {
            if !state.deleted {
                content_map.insert(
                    format!("{}-{}-{}", id.node_id, id.counter, id.timestamp),
                    state.value.clone()
                );
            }
        }
        
        DocumentContent::new(serde_json::Value::Object(content_map))
    }
    
    pub fn get_version_vector(&self) -> HashMap<Uuid, i64> {
        self.version_vector.clone()
    }
    
    pub fn get_elements(&self) -> &HashMap<CRDTId, ElementState> {
        &self.elements
    }
    
    pub fn get_ratchet_sessions(&self) -> &HashMap<Uuid, Vec<u8>> {
        &self.ratchet_sessions
    }
    
    pub fn add_ratchet_session(&mut self, node_id: Uuid, session_state: Vec<u8>) {
        self.ratchet_sessions.insert(node_id, session_state);
    }
    
    /// Compare version vectors to detect conflicts
    pub fn compare_version_vectors(&self, other: &HashMap<Uuid, i64>) -> VersionVectorComparison {
        let mut local_ahead = Vec::new();
        let mut remote_ahead = Vec::new();
        let mut concurrent = Vec::new();
        
        // Check all nodes in the local version vector
        for (node_id, local_counter) in &self.version_vector {
            if let Some(remote_counter) = other.get(node_id) {
                if local_counter > remote_counter {
                    local_ahead.push(*node_id);
                } else if remote_counter > local_counter {
                    remote_ahead.push(*node_id);
                } else {
                    concurrent.push(*node_id);
                }
            } else {
                // Node exists in local but not remote
                local_ahead.push(*node_id);
            }
        }
        
        // Check for nodes that exist in remote but not local
        for (node_id, remote_counter) in other {
            if !self.version_vector.contains_key(node_id) {
                remote_ahead.push(*node_id);
            }
        }
        
        if local_ahead.is_empty() && remote_ahead.is_empty() {
            VersionVectorComparison::Equal
        } else if local_ahead.is_empty() {
            VersionVectorComparison::RemoteAhead(remote_ahead)
        } else if remote_ahead.is_empty() {
            VersionVectorComparison::LocalAhead(local_ahead)
        } else {
            VersionVectorComparison::Concurrent {
                local_ahead,
                remote_ahead,
                concurrent,
            }
        }
    }
}