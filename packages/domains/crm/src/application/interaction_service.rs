//! Interaction service for the CRM module
//!
//! This module contains the application service for managing interactions,
//! including consent checks and platform-native interaction tracking.

use crate::domain::interaction::{Interaction, InteractionType, InteractionError};
use crate::domain::primitives::{InteractionId, ContactId, UserId, PlatformEventId};
use chrono::{DateTime, Utc, Duration};
use thiserror::Error;
use std::collections::HashMap;

/// Error types for interaction service operations
#[derive(Error, Debug)]
pub enum InteractionServiceError {
    #[error("Interaction error: {0}")]
    InteractionError(#[from] InteractionError),
    
    #[error("Interaction not found: {0}")]
    InteractionNotFound(InteractionId),
    
    #[error("Consent denied for interaction: {0}")]
    ConsentDenied(String),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

/// Service for recording and managing interactions
pub struct InteractionService;

impl InteractionService {
    /// Create a new call interaction
    pub fn create_call_interaction(
        contact_id: ContactId,
        summary: String,
        details: Option<String>,
        timestamp: DateTime<Utc>,
        duration: Option<Duration>,
        created_by: UserId,
        is_platform_native: bool,
    ) -> Result<Interaction, InteractionServiceError> {
        let interaction = Interaction::new_call(
            contact_id,
            summary,
            details,
            timestamp,
            duration,
            created_by,
            is_platform_native,
        )?;
        
        Ok(interaction)
    }
    
    /// Create a new email interaction
    pub fn create_email_interaction(
        contact_id: ContactId,
        summary: String,
        details: Option<String>,
        timestamp: DateTime<Utc>,
        created_by: UserId,
        is_platform_native: bool,
    ) -> Result<Interaction, InteractionServiceError> {
        let interaction = Interaction::new_email(
            contact_id,
            summary,
            details,
            timestamp,
            created_by,
            is_platform_native,
        )?;
        
        Ok(interaction)
    }
    
    /// Create a new meeting interaction
    pub fn create_meeting_interaction(
        contact_id: ContactId,
        summary: String,
        details: Option<String>,
        timestamp: DateTime<Utc>,
        duration: Option<Duration>,
        created_by: UserId,
        is_platform_native: bool,
    ) -> Result<Interaction, InteractionServiceError> {
        let interaction = Interaction::new_meeting(
            contact_id,
            summary,
            details,
            timestamp,
            duration,
            created_by,
            is_platform_native,
        )?;
        
        Ok(interaction)
    }
    
    /// Create a new message interaction
    pub fn create_message_interaction(
        contact_id: ContactId,
        summary: String,
        details: Option<String>,
        timestamp: DateTime<Utc>,
        created_by: UserId,
        is_platform_native: bool,
    ) -> Result<Interaction, InteractionServiceError> {
        let interaction = Interaction::new_message(
            contact_id,
            summary,
            details,
            timestamp,
            created_by,
            is_platform_native,
        )?;
        
        Ok(interaction)
    }
    
    /// Create a new platform event interaction
    pub fn create_platform_event_interaction(
        contact_id: ContactId,
        event_id: PlatformEventId,
        summary: String,
        details: Option<String>,
        timestamp: DateTime<Utc>,
        duration: Option<Duration>,
        created_by: UserId,
    ) -> Result<Interaction, InteractionServiceError> {
        let interaction = Interaction::new_platform_event(
            contact_id,
            event_id,
            summary,
            details,
            timestamp,
            duration,
            created_by,
        )?;
        
        Ok(interaction)
    }
    
    /// Update interaction details
    pub fn update_interaction_details(
        interaction: &mut Interaction,
        summary: Option<String>,
        details: Option<String>,
    ) -> Result<(), InteractionServiceError> {
        interaction.update_details(summary, details)?;
        Ok(())
    }
    
    /// Validate an interaction
    pub fn validate_interaction(interaction: &Interaction) -> Result<(), InteractionServiceError> {
        interaction.validate()?;
        Ok(())
    }
    
    /// Check if an interaction requires consent based on contact type
    pub fn requires_consent(interaction: &Interaction, is_contact_platform_native: bool) -> bool {
        // Platform-native interactions always require consent management
        if interaction.is_platform_native {
            return true;
        }
        
        // For external contacts, only platform event interactions require consent
        interaction.is_platform_event()
    }
    
    /// Record an interaction with consent checks
    pub fn record_interaction_with_consent(
        interaction: Interaction,
        has_consent: bool,
    ) -> Result<Interaction, InteractionServiceError> {
        // If this is a platform-native interaction, we need consent
        if interaction.is_platform_native && !has_consent {
            return Err(InteractionServiceError::ConsentDenied(
                "Consent required for platform-native interaction".to_string()
            ));
        }
        
        // If this is a platform event interaction, we need consent
        if interaction.is_platform_event() && !has_consent {
            return Err(InteractionServiceError::ConsentDenied(
                "Consent required for platform event interaction".to_string()
            ));
        }
        
        Ok(interaction)
    }
    
    /// Get interaction history for a contact
    pub fn get_interaction_history(
        interactions: &[Interaction],
        contact_id: &ContactId,
    ) -> Vec<&Interaction> {
        interactions.iter()
            .filter(|interaction| &interaction.contact_id == contact_id)
            .collect()
    }
    
    /// Get interaction history sorted by timestamp (newest first)
    pub fn get_sorted_interaction_history(
        interactions: &[Interaction],
        contact_id: &ContactId,
    ) -> Vec<&Interaction> {
        let mut history = Self::get_interaction_history(interactions, contact_id);
        history.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        history
    }
    
    /// Calculate interaction frequency for a contact
    pub fn calculate_interaction_frequency(
        interactions: &[Interaction],
        contact_id: &ContactId,
        days: i64,
    ) -> f64 {
        let cutoff = Utc::now() - Duration::days(days);
        let recent_interactions: Vec<&Interaction> = interactions.iter()
            .filter(|interaction| {
                &interaction.contact_id == contact_id && 
                interaction.timestamp > cutoff
            })
            .collect();
        
        // Calculate average days between interactions
        if recent_interactions.len() < 2 {
            return 0.0;
        }
        
        let mut sorted_interactions = recent_interactions;
        sorted_interactions.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        
        let mut total_duration = 0;
        let mut count = 0;
        
        for window in sorted_interactions.windows(2) {
            let duration = window[1].timestamp.signed_duration_since(window[0].timestamp);
            total_duration += duration.num_days();
            count += 1;
        }
        
        if count > 0 {
            total_duration as f64 / count as f64
        } else {
            0.0
        }
    }
    
    /// Get interaction type distribution
    pub fn get_interaction_type_distribution(
        interactions: &[Interaction],
        contact_id: &ContactId,
    ) -> HashMap<String, usize> {
        let mut distribution = HashMap::new();
        
        for interaction in interactions.iter() {
            if &interaction.contact_id == contact_id {
                let type_name = match interaction.interaction_type {
                    InteractionType::Call => "Call",
                    InteractionType::Email => "Email",
                    InteractionType::Meeting => "Meeting",
                    InteractionType::Message => "Message",
                    InteractionType::PlatformEvent(_) => "PlatformEvent",
                };
                
                *distribution.entry(type_name.to_string()).or_insert(0) += 1;
            }
        }
        
        distribution
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
        
        let interaction = InteractionService::create_call_interaction(
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
        assert_eq!(interaction.duration, Some(duration));
        assert_eq!(interaction.created_by, user_id);
        assert!(interaction.is_platform_native);
    }

    #[test]
    fn test_create_platform_event_interaction() {
        let contact_id = ContactId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let user_id = UserId::from_str("111e8400-e29b-41d4-a716-446655440000").unwrap();
        let event_id = PlatformEventId::new();
        let duration = Duration::minutes(45);
        let timestamp = Utc::now();
        
        let interaction = InteractionService::create_platform_event_interaction(
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
        assert_eq!(interaction.duration, Some(duration));
        assert_eq!(interaction.created_by, user_id);
        assert!(interaction.is_platform_native);
    }

    #[test]
    fn test_record_interaction_with_consent() {
        let contact_id = ContactId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let user_id = UserId::from_str("111e8400-e29b-41d4-a716-446655440000").unwrap();
        let timestamp = Utc::now();
        
        // Platform-native interaction without consent should fail
        let platform_interaction = Interaction::new_call(
            contact_id.clone(),
            "Test call".to_string(),
            None,
            timestamp,
            None,
            user_id.clone(),
            true,
        ).unwrap();
        
        let result = InteractionService::record_interaction_with_consent(
            platform_interaction,
            false, // No consent
        );
        
        assert!(result.is_err());
        match result.unwrap_err() {
            InteractionServiceError::ConsentDenied(_) => {},
            _ => panic!("Expected ConsentDenied error"),
        }
        
        // Platform-native interaction with consent should succeed
        let platform_interaction = Interaction::new_call(
            contact_id,
            "Test call".to_string(),
            None,
            timestamp,
            None,
            user_id,
            true,
        ).unwrap();
        
        let result = InteractionService::record_interaction_with_consent(
            platform_interaction,
            true, // With consent
        );
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_interaction_history() {
        let interactions = create_test_interactions();
        let contact_id = ContactId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        
        let history = InteractionService::get_interaction_history(&interactions, &contact_id);
        assert_eq!(history.len(), 2); // Two interactions for this contact
    }

    #[test]
    fn test_get_sorted_interaction_history() {
        let interactions = create_test_interactions();
        let contact_id = ContactId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        
        let history = InteractionService::get_sorted_interaction_history(&interactions, &contact_id);
        assert_eq!(history.len(), 2);
        
        // Should be sorted newest first
        assert!(history[0].timestamp > history[1].timestamp);
    }

    #[test]
    fn test_interaction_type_distribution() {
        let interactions = create_test_interactions();
        let contact_id = ContactId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        
        let distribution = InteractionService::get_interaction_type_distribution(&interactions, &contact_id);
        assert_eq!(distribution.get("Call"), Some(&1));
        assert_eq!(distribution.get("Email"), Some(&1));
    }

    fn create_test_interactions() -> Vec<Interaction> {
        let contact_id1 = ContactId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let contact_id2 = ContactId::from_str("666e8400-e29b-41d4-a716-446655440000").unwrap();
        let user_id = UserId::from_str("111e8400-e29b-41d4-a716-446655440000").unwrap();
        let timestamp = Utc::now();
        
        vec![
            Interaction::new_call(
                contact_id1.clone(),
                "Call 1".to_string(),
                None,
                timestamp,
                None,
                user_id.clone(),
                false,
            ).unwrap(),
            Interaction::new_email(
                contact_id1.clone(),
                "Email 1".to_string(),
                None,
                timestamp,
                user_id.clone(),
                false,
            ).unwrap(),
            Interaction::new_meeting(
                contact_id2.clone(),
                "Meeting 1".to_string(),
                None,
                timestamp,
                None,
                user_id.clone(),
                false,
            ).unwrap(),
        ]
    }
}