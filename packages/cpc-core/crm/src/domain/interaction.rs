//! Interaction domain entities for the CRM module
//!
//! This module contains the core business entities for managing interactions
//! with contacts, including different types of interactions and platform-native
//! interaction references.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use crate::domain::primitives::{InteractionId, ContactId, UserId, CrmPrimitiveError};
use thiserror::Error;
use uuid::Uuid;

/// Error types for interaction operations
#[derive(Error, Debug)]
pub enum InteractionError {
    #[error("Invalid interaction data: {0}")]
    InvalidData(String),
    
    #[error("Invalid duration: {0}")]
    InvalidDuration(String),
    
    #[error("Primitive error: {0}")]
    PrimitiveError(#[from] CrmPrimitiveError),
}

/// Type of interaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InteractionType {
    /// Phone call
    Call,
    
    /// Email
    Email,
    
    /// In-person or virtual meeting
    Meeting,
    
    /// Message (chat, SMS, etc.)
    Message,
    
    /// Reference to a platform-native interaction event
    PlatformEvent(PlatformEventId),
}

/// A unique identifier for platform events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PlatformEventId(pub Uuid);

impl PlatformEventId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl std::fmt::Display for PlatformEventId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for PlatformEventId {
    type Err = CrmPrimitiveError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::parse_str(s)?;
        Ok(PlatformEventId(uuid))
    }
}

/// Main interaction entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Interaction {
    /// Unique identifier for the interaction
    pub id: InteractionId,
    
    /// ID of the contact this interaction is with
    pub contact_id: ContactId,
    
    /// Type of interaction
    pub interaction_type: InteractionType,
    
    /// Summary of the interaction
    pub summary: String,
    
    /// Detailed notes about the interaction
    pub details: Option<String>,
    
    /// Timestamp of when the interaction occurred
    pub timestamp: DateTime<Utc>,
    
    /// Duration of the interaction (if applicable)
    pub duration: Option<Duration>,
    
    /// ID of the user who recorded this interaction
    pub created_by: UserId,
    
    /// Indicates if this was a platform-mediated interaction
    pub is_platform_native: bool,
}

impl Interaction {
    /// Create a new interaction
    pub fn new(
        contact_id: ContactId,
        interaction_type: InteractionType,
        summary: String,
        details: Option<String>,
        timestamp: DateTime<Utc>,
        duration: Option<Duration>,
        created_by: UserId,
        is_platform_native: bool,
    ) -> Result<Self, InteractionError> {
        // Validate interaction data
        if summary.is_empty() {
            return Err(InteractionError::InvalidData("Interaction summary cannot be empty".to_string()));
        }
        
        // Validate duration if provided
        if let Some(duration) = &duration {
            if duration.num_seconds() < 0 {
                return Err(InteractionError::InvalidDuration("Duration cannot be negative".to_string()));
            }
        }
        
        Ok(Self {
            id: InteractionId::new(),
            contact_id,
            interaction_type,
            summary,
            details,
            timestamp,
            duration,
            created_by,
            is_platform_native,
        })
    }
    
    /// Create a call interaction
    pub fn new_call(
        contact_id: ContactId,
        summary: String,
        details: Option<String>,
        timestamp: DateTime<Utc>,
        duration: Option<Duration>,
        created_by: UserId,
        is_platform_native: bool,
    ) -> Result<Self, InteractionError> {
        Self::new(
            contact_id,
            InteractionType::Call,
            summary,
            details,
            timestamp,
            duration,
            created_by,
            is_platform_native,
        )
    }
    
    /// Create an email interaction
    pub fn new_email(
        contact_id: ContactId,
        summary: String,
        details: Option<String>,
        timestamp: DateTime<Utc>,
        created_by: UserId,
        is_platform_native: bool,
    ) -> Result<Self, InteractionError> {
        Self::new(
            contact_id,
            InteractionType::Email,
            summary,
            details,
            timestamp,
            None, // Emails don't typically have duration
            created_by,
            is_platform_native,
        )
    }
    
    /// Create a meeting interaction
    pub fn new_meeting(
        contact_id: ContactId,
        summary: String,
        details: Option<String>,
        timestamp: DateTime<Utc>,
        duration: Option<Duration>,
        created_by: UserId,
        is_platform_native: bool,
    ) -> Result<Self, InteractionError> {
        Self::new(
            contact_id,
            InteractionType::Meeting,
            summary,
            details,
            timestamp,
            duration,
            created_by,
            is_platform_native,
        )
    }
    
    /// Create a message interaction
    pub fn new_message(
        contact_id: ContactId,
        summary: String,
        details: Option<String>,
        timestamp: DateTime<Utc>,
        created_by: UserId,
        is_platform_native: bool,
    ) -> Result<Self, InteractionError> {
        Self::new(
            contact_id,
            InteractionType::Message,
            summary,
            details,
            timestamp,
            None, // Messages don't typically have duration
            created_by,
            is_platform_native,
        )
    }
    
    /// Create a platform event interaction
    pub fn new_platform_event(
        contact_id: ContactId,
        event_id: PlatformEventId,
        summary: String,
        details: Option<String>,
        timestamp: DateTime<Utc>,
        duration: Option<Duration>,
        created_by: UserId,
    ) -> Result<Self, InteractionError> {
        Self::new(
            contact_id,
            InteractionType::PlatformEvent(event_id),
            summary,
            details,
            timestamp,
            duration,
            created_by,
            true, // Platform events are always platform-native
        )
    }
    
    /// Update interaction details
    pub fn update_details(
        &mut self,
        summary: Option<String>,
        details: Option<String>,
    ) -> Result<(), InteractionError> {
        if let Some(summary) = summary {
            if summary.is_empty() {
                return Err(InteractionError::InvalidData("Interaction summary cannot be empty".to_string()));
            }
            self.summary = summary;
        }
        
        if let Some(details) = details {
            self.details = Some(details);
        }
        
        Ok(())
    }
    
    /// Validate the interaction
    pub fn validate(&self) -> Result<(), InteractionError> {
        if self.summary.is_empty() {
            return Err(InteractionError::InvalidData("Interaction summary cannot be empty".to_string()));
        }
        
        // Validate duration if provided
        if let Some(duration) = &self.duration {
            if duration.num_seconds() < 0 {
                return Err(InteractionError::InvalidDuration("Duration cannot be negative".to_string()));
            }
        }
        
        Ok(())
    }
    
    /// Check if this is a platform event interaction
    pub fn is_platform_event(&self) -> bool {
        matches!(self.interaction_type, InteractionType::PlatformEvent(_))
    }
    
    /// Get the platform event ID if this is a platform event interaction
    pub fn get_platform_event_id(&self) -> Option<&PlatformEventId> {
        match &self.interaction_type {
            InteractionType::PlatformEvent(event_id) => Some(event_id),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_create_call_interaction() {
        let contact_id = ContactId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let user_id = UserId::from_str("111e8400-e29b-41d4-a716-446655440000").unwrap();
        let duration = Duration::minutes(30);
        let timestamp = Utc::now();
        
        let interaction = Interaction::new_call(
            contact_id.clone(),
            "Discussed project requirements".to_string(),
            Some("Client wants additional features".to_string()),
            timestamp,
            Some(duration),
            user_id.clone(),
            true,
        ).unwrap();
        
        assert_eq!(interaction.contact_id, contact_id);
        assert_eq!(interaction.interaction_type, InteractionType::Call);
        assert_eq!(interaction.summary, "Discussed project requirements");
        assert_eq!(interaction.details, Some("Client wants additional features".to_string()));
        assert_eq!(interaction.duration, Some(duration));
        assert_eq!(interaction.created_by, user_id);
        assert!(interaction.is_platform_native);
    }

    #[test]
    fn test_create_email_interaction() {
        let contact_id = ContactId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let user_id = UserId::from_str("111e8400-e29b-41d4-a716-446655440000").unwrap();
        let timestamp = Utc::now();
        
        let interaction = Interaction::new_email(
            contact_id.clone(),
            "Follow-up on proposal".to_string(),
            Some("Sent proposal v2 with updated pricing".to_string()),
            timestamp,
            user_id.clone(),
            false,
        ).unwrap();
        
        assert_eq!(interaction.contact_id, contact_id);
        assert_eq!(interaction.interaction_type, InteractionType::Email);
        assert_eq!(interaction.summary, "Follow-up on proposal");
        assert_eq!(interaction.details, Some("Sent proposal v2 with updated pricing".to_string()));
        assert_eq!(interaction.duration, None);
        assert_eq!(interaction.created_by, user_id);
        assert!(!interaction.is_platform_native);
    }

    #[test]
    fn test_create_platform_event_interaction() {
        let contact_id = ContactId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let user_id = UserId::from_str("111e8400-e29b-41d4-a716-446655440000").unwrap();
        let event_id = PlatformEventId::new();
        let duration = Duration::minutes(45);
        let timestamp = Utc::now();
        
        let interaction = Interaction::new_platform_event(
            contact_id.clone(),
            event_id.clone(),
            "Shared document in platform".to_string(),
            Some("Shared project plan document".to_string()),
            timestamp,
            Some(duration),
            user_id.clone(),
        ).unwrap();
        
        assert_eq!(interaction.contact_id, contact_id);
        assert_eq!(interaction.interaction_type, InteractionType::PlatformEvent(event_id));
        assert_eq!(interaction.summary, "Shared document in platform");
        assert_eq!(interaction.details, Some("Shared project plan document".to_string()));
        assert_eq!(interaction.duration, Some(duration));
        assert_eq!(interaction.created_by, user_id);
        assert!(interaction.is_platform_native);
        assert!(interaction.is_platform_event());
    }

    #[test]
    fn test_update_interaction_details() {
        let mut interaction = create_test_interaction();
        
        interaction.update_details(
            Some("Updated summary".to_string()),
            Some("Updated details".to_string()),
        ).unwrap();
        
        assert_eq!(interaction.summary, "Updated summary");
        assert_eq!(interaction.details, Some("Updated details".to_string()));
    }

    #[test]
    fn test_interaction_validation() {
        let interaction = create_test_interaction();
        assert!(interaction.validate().is_ok());
        
        // Test invalid interaction with empty summary
        let invalid_interaction = Interaction {
            summary: "".to_string(),
            ..create_test_interaction()
        };
        assert!(invalid_interaction.validate().is_err());
    }

    #[test]
    fn test_invalid_duration() {
        let contact_id = ContactId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let user_id = UserId::from_str("111e8400-e29b-41d4-a716-446655440000").unwrap();
        let timestamp = Utc::now();
        let negative_duration = Duration::seconds(-300);
        
        let result = Interaction::new_call(
            contact_id,
            "Test call".to_string(),
            None,
            timestamp,
            Some(negative_duration),
            user_id,
            false,
        );
        
        assert!(result.is_err());
        match result.unwrap_err() {
            InteractionError::InvalidDuration(_) => {},
            _ => panic!("Expected InvalidDuration error"),
        }
    }

    fn create_test_interaction() -> Interaction {
        let contact_id = ContactId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let user_id = UserId::from_str("111e8400-e29b-41d4-a716-446655440000").unwrap();
        let timestamp = Utc::now();
        
        Interaction::new_call(
            contact_id,
            "Test call".to_string(),
            Some("Test details".to_string()),
            timestamp,
            Some(Duration::minutes(15)),
            user_id,
            false,
        ).unwrap()
    }
}