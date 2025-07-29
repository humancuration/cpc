use crate::domain::{Sheet, CellAddress, CellCrdt, CellValue, Clock, CellMetadata};
use cpc_core::event_bus::{EventPublisher, EventSubscriber};
use uuid::Uuid;
use chrono::Utc;

/// Collaboration event types
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SheetEvent {
    CellUpdated {
        sheet_id: Uuid,
        cell: CellAddress,
        operation: CrdtOperation,
        timestamp: chrono::DateTime<Utc>,
    },
    CursorMoved {
        sheet_id: Uuid,
        user: Uuid,
        position: CellAddress,
        timestamp: chrono::DateTime<Utc>,
    },
    SheetShared {
        sheet_id: Uuid,
        shared_with: Uuid,
        permissions: crate::domain::PermissionLevel,
    },
    CommentAdded {
        sheet_id: Uuid,
        cell: CellAddress,
        user: Uuid,
        comment: String,
        timestamp: chrono::DateTime<Utc>,
    },
}

/// CRDT operation for collaborative editing
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CrdtOperation {
    Insert(CellCrdt),
    Update(Clock, CellValue, CellMetadata),
    Delete(CellAddress),
}

/// Service for real-time collaboration on sheets
pub struct CollaborationService {
    event_publisher: Box<dyn EventPublisher<SheetEvent>>,
    event_subscriber: Box<dyn EventSubscriber<SheetEvent>>,
}

impl CollaborationService {
    pub fn new(
        event_publisher: Box<dyn EventPublisher<SheetEvent>>,
        event_subscriber: Box<dyn EventSubscriber<SheetEvent>>,
    ) -> Self {
        let mut service = Self {
            event_publisher,
            event_subscriber,
        };
        
        // Start listening for events
        service.subscribe_to_events();
        service
    }
    
    fn subscribe_to_events(&mut self) {
        let callback = Box::new(|event: SheetEvent| {
            // Handle incoming collaboration events
            match event {
                SheetEvent::CellUpdated { sheet_id, cell, operation, .. } => {
                    // Apply CRDT merge operation
                    // In a real implementation, this would update the local sheet state
                    println!("Received cell update for sheet {:?}, cell {:?}", sheet_id, cell);
                }
                SheetEvent::CursorMoved { user, position, .. } => {
                    // Update collaborator cursor position
                    println!("User {:?} moved cursor to {:?}", user, position);
                }
                SheetEvent::SheetShared { sheet_id, shared_with, permissions } => {
                    // Update sharing information
                    println!("Sheet {:?} shared with {:?} with permissions {:?}", sheet_id, shared_with, permissions);
                }
                SheetEvent::CommentAdded { cell, user, comment, .. } => {
                    // Add comment to cell
                    println!("User {:?} added comment to cell {:?}: {}", user, cell, comment);
                }
            }
        });
        
        self.event_subscriber.subscribe(callback);
    }
    
    pub fn broadcast_cell_update(
        &self,
        sheet_id: Uuid,
        cell: CellAddress,
        operation: CrdtOperation,
    ) {
        let event = SheetEvent::CellUpdated {
            sheet_id,
            cell,
            operation,
            timestamp: Utc::now(),
        };
        
        self.event_publisher.publish(&event);
    }
    
    pub fn broadcast_cursor_move(
        &self,
        sheet_id: Uuid,
        user: Uuid,
        position: CellAddress,
    ) {
        let event = SheetEvent::CursorMoved {
            sheet_id,
            user,
            position,
            timestamp: Utc::now(),
        };
        
        self.event_publisher.publish(&event);
    }
    
    pub fn broadcast_sheet_share(
        &self,
        sheet_id: Uuid,
        shared_with: Uuid,
        permissions: crate::domain::PermissionLevel,
    ) {
        let event = SheetEvent::SheetShared {
            sheet_id,
            shared_with,
            permissions,
        };
        
        self.event_publisher.publish(&event);
    }
    
    pub fn broadcast_comment_added(
        &self,
        sheet_id: Uuid,
        cell: CellAddress,
        user: Uuid,
        comment: String,
    ) {
        let event = SheetEvent::CommentAdded {
            sheet_id,
            cell,
            user,
            comment,
            timestamp: Utc::now(),
        };
        
        self.event_publisher.publish(&event);
    }
}