//! Message types for real-time signaling

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use ot::{Operation, TextOperation as OtTextOperation};

/// Position in a document
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

/// Range of selected text
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SelectionRange {
    pub start: Position,
    pub end: Position,
}

/// Cursor position update
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CursorPosition {
    pub document_id: Uuid,
    pub user_id: Uuid,
    pub position: Position,
    pub timestamp: DateTime<Utc>,
}

/// Presence update message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresenceUpdate {
    pub document_id: Uuid,
    pub user_id: Uuid,
    pub cursor: Option<Position>,
    pub selection: Option<SelectionRange>,
    pub is_typing: bool,
    pub avatar_url: Option<String>,
    pub color: String,
    pub last_active: DateTime<Utc>,
    pub timestamp: DateTime<Utc>,
}

/// Viewport update message for synchronizing viewport positions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ViewportUpdate {
    pub user_id: Uuid,
    pub document_id: Uuid,
    pub viewport: Rect,
    pub resolution: f64, // LOD resolution for cursors
    pub timestamp: DateTime<Utc>,
}

/// Rectangle representing a viewport or region
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Presence summary message for efficient broadcasting
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresenceSummary {
    pub users: std::collections::HashMap<Uuid, PresenceUser>,
    pub expires_at: DateTime<Utc>,
}

/// Individual user presence information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresenceUser {
    pub avatar_url: Option<String>,
    pub color: String,
    pub status: PresenceStatus,
}

/// User presence status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PresenceStatus {
    Online,
    Away,
    Busy,
    Offline,
}

/// Core signaling message types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SignalingMessage {
    /// User has joined a document
    JoinDocument {
        document_id: Uuid,
        user_id: Uuid,
    },
    
    /// User has left a document
    LeaveDocument {
        document_id: Uuid,
        user_id: Uuid,
    },
    
    /// Presence update for a user
    PresenceUpdate(PresenceUpdate),
    
    /// Presence summary for efficient broadcasting
    PresenceSummary(PresenceSummary),
    
    /// Cursor position update
    CursorUpdate(CursorPosition),
    
    /// Text selection update
    SelectionUpdate {
        document_id: Uuid,
        user_id: Uuid,
        selection: Option<SelectionRange>,
        timestamp: DateTime<Utc>,
    },
    
    /// User is typing
    TypingIndicator {
        document_id: Uuid,
        user_id: Uuid,
        is_typing: bool,
        timestamp: DateTime<Utc>,
    },
    
    /// Error message
    Error {
        code: String,
        message: String,
    },
    
    /// Annotation message
    Annotation {
        document_id: Uuid,
        user_id: Uuid,
        position: Position,
        content: String,
        timestamp: DateTime<Utc>,
    },
    
    /// Comment message
    Comment {
        document_id: Uuid,
        user_id: Uuid,
        position: Position,
        content: String,
        timestamp: DateTime<Utc>,
    },
    
    /// Presence status update
    PresenceStatus {
        document_id: Uuid,
        user_id: Uuid,
        status: String, // e.g., "online", "away", "busy"
        timestamp: DateTime<Utc>,
    },
    
    /// Text operation for collaborative editing
    TextOperation(OtTextOperation),
    
    /// Viewport update for synchronizing viewport positions
    ViewportUpdate(ViewportUpdate),
}