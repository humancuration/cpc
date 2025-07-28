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
}

pub struct ElementState {
    pub value: JsonValue,
    pub created_at: i64,
    pub deleted: bool,
    pub parent_id: Option<CRDTId>,
    // Additional metadata as needed
}

impl CRDTDocument {
    pub fn new(node_id: Uuid) -> Self {
        Self {
            elements: HashMap::new(),
            version_vector: HashMap::new(),
            logical_clock: 0,
            node_id,
            operation_counter: 0,
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
                // Insert logic with conflict resolution
                let element_state = ElementState {
                    value: value.clone(),
                    created_at: id.timestamp,
                    deleted: false,
                    parent_id: parent_id.clone(),
                };
                
                self.elements.insert(id.clone(), element_state);
            }
            DocumentOperation::Delete { id, timestamp } => {
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
}