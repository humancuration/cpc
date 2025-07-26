//! Represents a project within the system.

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

/// The status of a project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "project_status", rename_all = "snake_case")]
pub enum ProjectStatus {
    NotStarted,
    InProgress,
    Completed,
    OnHold,
}

/// Represents a project in the system.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    /// The unique identifier for the project.
    pub id: Uuid,
    /// The name of the project.
    pub name: String,
    /// An optional description of the project.
    pub description: Option<String>,
    /// The ID of the cooperative this project belongs to.
    pub cooperative_id: Uuid,
    /// The current status of the project.
    pub status: ProjectStatus,
    /// The optional start date of the project.
    pub start_date: Option<NaiveDate>,
    /// The optional end date of the project.
    pub end_date: Option<NaiveDate>,
    /// The timestamp when the project was created.
    pub created_at: DateTime<Utc>,
    /// The timestamp when the project was last updated.
    pub updated_at: DateTime<Utc>,
}

/// Represents the payload for updating a project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProject {
    pub name: Option<String>,
    pub description: Option<String>,
}