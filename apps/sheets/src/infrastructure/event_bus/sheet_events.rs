//! Sheet events for the event bus

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use cpc_core::event_bus::Event;
use crate::domain::{CellAddress, PermissionLevel};
use crate::application::collaboration_service::{SheetEvent, CrdtOperation};

impl Event for SheetEvent {
    fn event_type(&self) -> String {
        match self {
            SheetEvent::CellUpdated { .. } => "CELL_UPDATE".to_string(),
            SheetEvent::CursorMoved { .. } => "sheets.cursor.moved".to_string(),
            SheetEvent::SheetShared { .. } => "sheets.sheet.shared".to_string(),
            SheetEvent::CommentAdded { .. } => "sheets.comment.added".to_string(),
        }
    }
    
    fn serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(serde_json::to_vec(self)?)
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_slice(data)?)
    }
}

/// Publisher for sheet events
pub struct SheetEventPublisher;

impl SheetEventPublisher {
    pub fn new() -> Self {
        Self
    }
    
    pub fn publish(&self, event: &SheetEvent) {
        // In a real implementation, this would publish to the event bus
        // For now, we'll just log the event
        tracing::info!("Publishing sheet event: {}", event.event_type());
    }
}