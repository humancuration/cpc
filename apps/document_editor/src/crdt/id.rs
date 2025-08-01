// DEPRECATED - Refactored to use collaboration_engine
// This file has been deprecated as part of the refactor to use the collaboration_engine package.
// The new implementation can be found in the application/collaboration_service.rs file.
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CRDTId {
    pub node_id: Uuid,       // Client/node UUID
    pub counter: u64,        // Monotonically increasing
    pub timestamp: i64,      // Logical clock timestamp
}

impl CRDTId {
    pub fn new(node_id: Uuid, counter: u64, timestamp: i64) -> Self {
        Self {
            node_id,
            counter,
            timestamp,
        }
    }
}