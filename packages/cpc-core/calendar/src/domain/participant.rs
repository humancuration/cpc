//! Participant domain model for calendar events

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a participant in a calendar event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub user_id: Uuid,
    pub role: ParticipantRole,
    pub status: ParticipationStatus,
    pub response_time: Option<DateTime<Utc>>,
}

impl Participant {
    /// Create a new participant
    pub fn new(user_id: Uuid, role: ParticipantRole) -> Self {
        Self {
            user_id,
            role,
            status: ParticipationStatus::Pending,
            response_time: None,
        }
    }

    /// Accept the invitation
    pub fn accept(&mut self) {
        self.status = ParticipationStatus::Accepted;
        self.response_time = Some(Utc::now());
    }

    /// Decline the invitation
    pub fn decline(&mut self) {
        self.status = ParticipationStatus::Declined;
        self.response_time = Some(Utc::now());
    }

    /// Mark as tentative
    pub fn tentative(&mut self) {
        self.status = ParticipationStatus::Tentative;
        self.response_time = Some(Utc::now());
    }
}

/// Role of a participant in an event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipantRole {
    Organizer,
    Required,
    Optional,
}

/// Participation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipationStatus {
    Pending,
    Accepted,
    Declined,
    Tentative,
}

/// Participant response to an event invitation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantResponse {
    pub user_id: Uuid,
    pub event_id: Uuid,
    pub status: ParticipationStatus,
    pub comment: Option<String>,
    pub response_time: DateTime<Utc>,
}

impl ParticipantResponse {
    /// Create a new participant response
    pub fn new(
        user_id: Uuid,
        event_id: Uuid,
        status: ParticipationStatus,
        comment: Option<String>,
    ) -> Self {
        Self {
            user_id,
            event_id,
            status,
            comment,
            response_time: Utc::now(),
        }
    }
}