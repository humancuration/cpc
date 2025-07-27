//! Database models for the calendar module

use chrono::{DateTime, Utc, NaiveTime, Duration};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Database model for calendar events
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CalendarEventModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub event_type: String, // Serialized EventType
    pub visibility: String, // Serialized EventVisibility
    pub recurrence: Option<String>, // Serialized RecurrenceRule
    pub location: Option<String>, // Serialized Location
    pub attachments: Option<String>, // Serialized Vec<EventAttachment>
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for work shifts
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct WorkShiftModel {
    pub id: Uuid,
    pub cooperative_id: Uuid,
    pub position: String,
    pub schedule: String, // Serialized ShiftSchedule
    pub coverage: String, // Serialized ShiftCoverage
    pub created_at: DateTime<Utc>,
}

/// Database model for event reminders
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct EventReminderModel {
    pub id: Uuid,
    pub event_id: Uuid,
    pub user_id: Uuid,
    pub trigger_time: DateTime<Utc>,
    pub method: String, // Serialized ReminderMethod
    pub escalation_level: i32,
    pub status: String, // Serialized ReminderStatus
    pub created_at: DateTime<Utc>,
    pub message: String,
}

/// Database model for participants
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ParticipantModel {
    pub id: Uuid,
    pub event_id: Uuid,
    pub user_id: Uuid,
    pub role: String, // Serialized ParticipantRole
    pub status: String, // Serialized ParticipationStatus
    pub response_time: Option<DateTime<Utc>>,
}

/// Database model for shift assignments
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ShiftAssignmentModel {
    pub id: Uuid,
    pub shift_id: Uuid,
    pub user_id: Uuid,
    pub assigned_at: DateTime<Utc>,
}

/// Database model for availability slots
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AvailabilitySlotModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub availability_type: String, // Serialized AvailabilityType
}