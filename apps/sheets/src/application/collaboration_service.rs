use crate::domain::{Sheet, CellAddress, CellCrdt, CellValue, Clock, CellMetadata};
use cpc_core::event_bus::{EventPublisher, EventSubscriber};
use uuid::Uuid;
use chrono::Utc;

/// Collaboration event types
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SheetEvent {
    CellUpdated {
        sheet_id: Uuid,
        position: String, // A1 notation
        operation: CrdtOperation,
        cache_version: String,
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
                SheetEvent::CellUpdated { sheet_id, position, operation, .. } => {
                    // Apply CRDT merge operation
                    // In a real implementation, this would update the local sheet state
                    println!("Received cell update for sheet {:?}, position {:?}", sheet_id, position);
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
        // Convert cell address to A1 notation
        let position = self.cell_address_to_a1(cell);
        let event = SheetEvent::CellUpdated {
            sheet_id,
            position,
            operation,
            cache_version: self.generate_cache_version(),
            timestamp: Utc::now(),
        };
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
    
    /// Convert cell address to A1 notation
    fn cell_address_to_a1(&self, address: CellAddress) -> String {
        // Convert column number to letter (A, B, ..., Z, AA, AB, ...)
        let column_letter = self.column_number_to_letter(address.column + 1);
        format!("{}{}", column_letter, address.row + 1)
    }
    
    /// Convert column number to letter notation (1 -> A, 2 -> B, ..., 27 -> AA)
    fn column_number_to_letter(&self, column: u32) -> String {
        let mut result = String::new();
        let mut col = column;
        
        while col > 0 {
            let remainder = (col - 1) % 26;
            result.insert(0, (b'A' + remainder as u8) as char);
            col = (col - 1) / 26;
        }
        
        result
    }
    
    /// Generate a cache version string based on current timestamp
    fn generate_cache_version(&self) -> String {
        // In a real implementation, this might be based on a more sophisticated
        // versioning scheme, but for now we'll use a timestamp-based approach
        format!("{:x}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
    }
}