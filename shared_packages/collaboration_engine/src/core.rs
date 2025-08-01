//! Core Operational Transformation/CRDT algorithms for real-time document collaboration

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use event_bus::{EventBus, DomainEvent, EventError};
use serde_json::json;

/// Represents a position in a document
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

/// Represents an operation that can be applied to a document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
    Insert {
        position: Position,
        text: String,
        user_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    Delete {
        start: Position,
        end: Position,
        user_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    Replace {
        start: Position,
        end: Position,
        text: String,
        user_id: Uuid,
        timestamp: DateTime<Utc>,
    },
}

/// Represents a document in the collaboration system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: Uuid,
    pub content: String,
    pub version: u64,
    pub operations: Vec<Operation>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip)]
    pub event_bus: Option<EventBus>,
}

impl Document {
    /// Create a new document
    pub fn new(content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            content,
            version: 0,
            operations: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            event_bus: None,
        }
    }
    
    /// Set the event bus for this document
    pub fn set_event_bus(&mut self, event_bus: EventBus) {
        self.event_bus = Some(event_bus);
    }

    /// Apply an operation to the document
    pub fn apply_operation(&mut self, operation: Operation) -> Result<(), CollaborationError> {
        match &operation {
            Operation::Insert { position, text, .. } => {
                let index = self.position_to_index(position)?;
                self.content.insert_str(index, text);
            }
            Operation::Delete { start, end, .. } => {
                let start_index = self.position_to_index(start)?;
                let end_index = self.position_to_index(end)?;
                if start_index > end_index {
                    return Err(CollaborationError::InvalidRange);
                }
                self.content.drain(start_index..end_index);
            }
            Operation::Replace { start, end, text, .. } => {
                let start_index = self.position_to_index(start)?;
                let end_index = self.position_to_index(end)?;
                if start_index > end_index {
                    return Err(CollaborationError::InvalidRange);
                }
                self.content.replace_range(start_index..end_index, text);
            }
        }

        self.operations.push(operation.clone());
        self.version += 1;
        self.updated_at = Utc::now();
        
        // Publish event if event bus is available
        if let Some(ref event_bus) = self.event_bus {
            let event = DomainEvent::new_local(
                "collaboration".to_string(),
                "OperationApplied".to_string(),
                json!({
                    "document_id": self.id,
                    "operation": operation,
                    "version": self.version,
                }),
            );
            
            // We're ignoring the result here as we don't want to fail the operation
            // if event publishing fails
            let _ = event_bus.publish(event);
        }
        
        Ok(())
    }
    
    /// Handle a remote operation
    pub fn handle_remote_operation(&mut self, operation: Operation) -> Result<(), CollaborationError> {
        // Apply the operation
        self.apply_operation(operation)
    }
    
    /// Handle user joining document
    pub fn handle_user_joined(&mut self, user_id: Uuid, user_name: String) -> Result<(), CollaborationError> {
        // In a real implementation, this might initialize user-specific state
        // For now, we just log that the user joined
        tracing::debug!("User {} ({}) joined document {}", user_name, user_id, self.id);
        
        // Publish event if event bus is available
        if let Some(ref event_bus) = self.event_bus {
            let event = DomainEvent::new_local(
                "collaboration".to_string(),
                "UserJoinedDocument".to_string(),
                json!({
                    "document_id": self.id,
                    "user_id": user_id,
                    "user_name": user_name,
                }),
            );
            
            // We're ignoring the result here as we don't want to fail the operation
            // if event publishing fails
            let _ = event_bus.publish(event);
        }
        
        Ok(())
    }

    /// Convert a position to a character index in the document
    fn position_to_index(&self, position: &Position) -> Result<usize, CollaborationError> {
        let mut line = 0;
        let mut index = 0;

        for (i, ch) in self.content.chars().enumerate() {
            if line == position.line && index == position.column {
                return Ok(i);
            }

            if ch == '\n' {
                line += 1;
                index = 0;
            } else {
                index += 1;
            }
        }

        if line == position.line && index == position.column {
            return Ok(self.content.len());
        }

        Err(CollaborationError::InvalidPosition)
    }
}

/// Error types for collaboration operations
#[derive(Debug, thiserror::Error)]
pub enum CollaborationError {
    #[error("Invalid position in document")]
    InvalidPosition,
    #[error("Invalid range in document")]
    InvalidRange,
    #[error("Operation conflict detected")]
    OperationConflict,
    #[error("Document not found")]
    DocumentNotFound,
    #[error("Failed to publish event")]
    EventPublishError,
    #[error("Transformation error")]
    TransformationError,
    #[error("Resolution timeout")]
    ResolutionTimeout,
    #[error("Merge conflict")]
    MergeConflict,
}

/// CRDT-based data structure for collaborative editing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CRDTDocument {
    pub id: Uuid,
    pub content: HashMap<Uuid, Vec<CRDTOperation>>,
    pub version_vector: HashMap<Uuid, u64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Operation for CRDT-based editing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CRDTOperation {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub operation: Operation,
    pub timestamp: DateTime<Utc>,
    pub version: u64,
}

impl CRDTDocument {
    /// Create a new CRDT document
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            content: HashMap::new(),
            version_vector: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Apply an operation to the CRDT document
    pub fn apply_operation(&mut self, user_id: Uuid, operation: Operation) -> Result<(), CollaborationError> {
        let version = self.version_vector.entry(user_id).or_insert(0);
        *version += 1;

        let crdt_op = CRDTOperation {
            id: Uuid::new_v4(),
            parent_id: None,
            operation,
            timestamp: Utc::now(),
            version: *version,
        };

        self.content.entry(user_id).or_insert_with(Vec::new).push(crdt_op);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Merge operations from another CRDT document
    pub fn merge(&mut self, other: &CRDTDocument) -> Result<(), CollaborationError> {
        for (user_id, operations) in &other.content {
            let local_version = self.version_vector.entry(*user_id).or_insert(0);
            for operation in operations {
                if operation.version > *local_version {
                    self.content.entry(*user_id).or_insert_with(Vec::new).push(operation.clone());
                    *local_version = operation.version;
                }
            }
        }

        // Update version vector
        for (user_id, version) in &other.version_vector {
            let local_version = self.version_vector.entry(*user_id).or_insert(0);
            *local_version = (*local_version).max(*version);
        }

        self.updated_at = Utc::now();
        Ok(())
    }
}