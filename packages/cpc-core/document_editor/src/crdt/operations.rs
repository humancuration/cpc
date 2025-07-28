use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::crdt::id::CRDTId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FormattingStyle {
    Bold,
    Italic,
    Underline,
    Heading(usize),
    // Add other formatting types as needed
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
}