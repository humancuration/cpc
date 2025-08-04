//! Domain models for collaborative workspace
//!
//! Defines core entities for collaborative documents, project boards,
//! file versions, and meeting rooms as per ADR-0008.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Collaborative document metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborativeDocument {
    /// Unique identifier for the document
    pub id: Uuid,
    /// Human-readable title
    pub title: String,
    /// User who created the document
    pub created_by: Uuid,
    /// Creation timestamp (UTC)
    pub created_at: DateTime<Utc>,
    /// Last update timestamp (UTC)
    pub updated_at: DateTime<Utc>,
}

/// Project board root entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectBoard {
    /// Unique identifier for the board
    pub id: Uuid,
    /// Board title
    pub title: String,
    /// Owner/user who created/owns the board
    pub owner_id: Uuid,
    /// Creation timestamp (UTC)
    pub created_at: DateTime<Utc>,
}

/// Column within a project board (ordering via position)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectColumn {
    /// Unique identifier for the column
    pub id: Uuid,
    /// Parent board id
    pub board_id: Uuid,
    /// Column title
    pub title: String,
    /// Column order within board (0-based or 1-based defined by service)
    pub position: i32,
}

/// Task within a project column (ordering via position)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTask {
    /// Unique identifier for the task
    pub id: Uuid,
    /// Parent column id
    pub column_id: Uuid,
    /// Task title
    pub title: String,
    /// Optional long-form description
    pub description: Option<String>,
    /// Position within column
    pub position: i32,
    /// Creation timestamp (UTC)
    pub created_at: DateTime<Utc>,
    /// Last update timestamp (UTC)
    pub updated_at: Option<DateTime<Utc>>,
}

/// File version metadata (content is stored elsewhere)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileVersion {
    /// Unique identifier for this version row
    pub id: Uuid,
    /// Logical file identifier
    pub file_id: Uuid,
    /// Monotonic version number per file
    pub version: i32,
    /// User who created this version
    pub created_by: Uuid,
    /// Creation timestamp (UTC)
    pub created_at: DateTime<Utc>,
}

/// Meeting room metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingRoom {
    /// Unique identifier for the room
    pub id: Uuid,
    /// Room title/topic
    pub title: String,
    /// Owner/user who created the room
    pub owner_id: Uuid,
    /// Creation timestamp (UTC)
    pub created_at: DateTime<Utc>,
    /// End timestamp (UTC) if meeting has ended
    pub ended_at: Option<DateTime<Utc>>,
}