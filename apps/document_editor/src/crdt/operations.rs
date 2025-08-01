// DEPRECATED - Refactored to use collaboration_engine
// This file has been deprecated as part of the refactor to use the collaboration_engine package.
// The new implementation can be found in the application/collaboration_service.rs file.
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::crdt::id::CRDTId;
use ciborium;
use collaboration_engine::core::Position;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FormattingStyle {
    Bold,
    Italic,
    Underline,
    Heading(usize),
    // Add other formatting types as needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormatType {
    Bold,
    Italic,
    Underline,
    ListItem,
    // ... other formats
}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DocumentOperation {
    Insert {
        position: usize,
        value: serde_json::Value,
        id: CRDTId,
        parent_id: Option<CRDTId>,
    },
    Delete {
        id: CRDTId,
        timestamp: i64,
    },
    Update {
        id: CRDTId,
        value: serde_json::Value,
        timestamp: i64,
    },
    Formatting {
        id: CRDTId,
        style: FormattingStyle,
        timestamp: i64,
    },
    Format {
        range: (Position, Position),
        format: FormatType,
    },
    InsertImage {
        position: Position,
        image_id: Uuid,
        caption: String,
    },
}

impl DocumentOperation {
    pub fn timestamp(&self) -> i64 {
        match self {
            DocumentOperation::Insert { id, .. } => id.timestamp,
            DocumentOperation::Delete { timestamp, .. } => *timestamp,
            DocumentOperation::Update { timestamp, .. } => *timestamp,
            DocumentOperation::Formatting { timestamp, .. } => *timestamp,
        }
    }
    
    pub fn id(&self) -> Option<&CRDTId> {
        match self {
            DocumentOperation::Insert { id, .. } => Some(id),
            DocumentOperation::Delete { id, .. } => Some(id),
            DocumentOperation::Update { id, .. } => Some(id),
            DocumentOperation::Formatting { id, .. } => Some(id),
        }
    }
    
    pub fn to_cbor(&self) -> Result<Vec<u8>, ciborium::ser::Error<std::io::Error>> {
        let mut buffer = Vec::new();
        ciborium::ser::into_writer(self, &mut buffer)?;
        Ok(buffer)
    }
    
    pub fn from_cbor(data: &[u8]) -> Result<Self, ciborium::de::Error<std::io::Error>> {
        ciborium::de::from_reader(data)
    }
}