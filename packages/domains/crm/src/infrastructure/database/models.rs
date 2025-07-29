//! Database models for the CRM module
//!
//! This module contains the SQLx models that map to database tables
//! for CRM entities.

use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Database model for contacts
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ContactModel {
    pub id: Uuid,
    pub user_id: Uuid, // Owner of the contact
    pub contact_type: String, // "platform_native" or "external"
    pub platform_user_id: Option<Uuid>, // For platform-native contacts
    pub name: String,
    pub primary_email: Option<String>,
    pub primary_phone: Option<String>,
    pub company: Option<String>,
    pub tags: Option<sqlx::types::Json<Vec<String>>>,
    pub consent_settings: Option<sqlx::types::Json<ConsentSettingsModel>>,
    pub external_data: Option<sqlx::types::Json<ExternalContactDataModel>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_interaction: Option<DateTime<Utc>>,
}

/// Database model for consent settings
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ConsentSettingsModel {
    pub share_profile: String, // "none", "view_only", "editable"
    pub share_interaction_history: String,
    pub share_preferences: String,
    pub custom_fields: HashMap<String, String>,
}

/// Database model for external contact data
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ExternalContactDataModel {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub social_media: HashMap<String, String>,
}

/// Database model for interactions
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct InteractionModel {
    pub id: Uuid,
    pub contact_id: Uuid,
    pub interaction_type: String, // "call", "email", "meeting", "message", "platform_event"
    pub platform_event_id: Option<Uuid>, // For platform event interactions
    pub summary: String,
    pub details: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub duration_seconds: Option<i64>, // Duration in seconds
    pub created_by: Uuid,
    pub is_platform_native: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for pipelines
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PipelineModel {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub is_shared: bool,
    pub shared_with: Option<sqlx::types::Json<Vec<Uuid>>>,
    pub custom_fields: Option<sqlx::types::Json<HashMap<String, String>>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for pipeline stages
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PipelineStageModel {
    pub id: Uuid,
    pub pipeline_id: Uuid,
    pub name: String,
    pub probability: i32, // 0-100
    pub estimated_value_cents: Option<i64>, // In cents
    pub estimated_value_currency: Option<String>,
    pub position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for deals
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DealModel {
    pub id: Uuid,
    pub contact_id: Uuid,
    pub pipeline_id: Uuid,
    pub current_stage_id: Uuid,
    pub title: String,
    pub value_cents: i64, // In cents
    pub value_currency: String,
    pub expected_close_date: Option<DateTime<Utc>>,
    pub is_platform_deal: bool,
    pub owner_id: Uuid,
    pub custom_fields: Option<sqlx::types::Json<HashMap<String, String>>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for deal notes
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DealNoteModel {
    pub id: Uuid,
    pub deal_id: Uuid,
    pub content: String,
    pub created_by: Uuid,
    pub is_shared_with_contact: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_model_serialization() {
        let model = ContactModel {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            contact_type: "platform_native".to_string(),
            platform_user_id: Some(Uuid::new_v4()),
            name: "John Doe".to_string(),
            primary_email: Some("john@example.com".to_string()),
            primary_phone: Some("+1-555-123-4567".to_string()),
            company: Some("Acme Corp".to_string()),
            tags: Some(sqlx::types::Json(vec!["customer".to_string(), "vip".to_string()])),
            consent_settings: Some(sqlx::types::Json(ConsentSettingsModel {
                share_profile: "view_only".to_string(),
                share_interaction_history: "none".to_string(),
                share_preferences: "none".to_string(),
                custom_fields: HashMap::new(),
            })),
            external_data: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_interaction: None,
        };
        
        let json = serde_json::to_string(&model).unwrap();
        let deserialized: ContactModel = serde_json::from_str(&json).unwrap();
        
        assert_eq!(model.name, deserialized.name);
        assert_eq!(model.primary_email, deserialized.primary_email);
    }

    #[test]
    fn test_interaction_model_serialization() {
        let model = InteractionModel {
            id: Uuid::new_v4(),
            contact_id: Uuid::new_v4(),
            interaction_type: "call".to_string(),
            platform_event_id: None,
            summary: "Discussed project requirements".to_string(),
            details: Some("Client wants additional features".to_string()),
            timestamp: Utc::now(),
            duration_seconds: Some(1800), // 30 minutes
            created_by: Uuid::new_v4(),
            is_platform_native: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let json = serde_json::to_string(&model).unwrap();
        let deserialized: InteractionModel = serde_json::from_str(&json).unwrap();
        
        assert_eq!(model.summary, deserialized.summary);
        assert_eq!(model.duration_seconds, deserialized.duration_seconds);
    }
}