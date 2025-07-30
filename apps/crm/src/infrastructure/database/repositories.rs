//! Database repositories for the CRM module
//!
//! This module contains the SQLx repository implementations for CRM entities.

use sqlx::PgPool;
use uuid::Uuid;
use crate::infrastructure::database::models::*;
use crate::domain::contact::{Contact, ContactType, ConsentSettings, ExternalContactData, DataSharingLevel};
use crate::domain::interaction::{Interaction, InteractionType, PlatformEventId};
use crate::domain::pipeline::{Pipeline, PipelineStage};
use crate::domain::deal::{Deal, DealNote};
use crate::domain::primitives::*;
use chrono::{Duration, DateTime, Utc};
use std::collections::HashMap;
use thiserror::Error;

/// Error types for repository operations
#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Contact not found: {0}")]
    ContactNotFound(ContactId),
    
    #[error("Interaction not found: {0}")]
    InteractionNotFound(InteractionId),
    
    #[error("Pipeline not found: {0}")]
    PipelineNotFound(PipelineId),
    
    #[error("Deal not found: {0}")]
    DealNotFound(DealId),
    
    #[error("Conversion error: {0}")]
    ConversionError(String),
}

/// Repository for contact operations
pub struct ContactRepository {
    pool: PgPool,
}

impl ContactRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    /// Create a new contact
    pub async fn create_contact(&self, contact: &Contact) -> Result<(), RepositoryError> {
        // Convert domain contact to database model
        let model = self.convert_contact_to_model(contact)?;
        
        // Insert into database
        sqlx::query!(
            r#"
            INSERT INTO crm_contacts (
                id, user_id, contact_type, platform_user_id, name, primary_email,
                primary_phone, company, tags, consent_settings, external_data,
                created_at, updated_at, last_interaction
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14
            )
            "#,
            model.id,
            model.user_id,
            model.contact_type,
            model.platform_user_id,
            model.name,
            model.primary_email,
            model.primary_phone,
            model.company,
            model.tags as Option<sqlx::types::Json<Vec<String>>>,
            model.consent_settings as Option<sqlx::types::Json<ConsentSettingsModel>>,
            model.external_data as Option<sqlx::types::Json<ExternalContactDataModel>>,
            model.created_at,
            model.updated_at,
            model.last_interaction,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Get a contact by ID
    pub async fn get_contact_by_id(&self, id: &ContactId) -> Result<Contact, RepositoryError> {
        let uuid = id.0;
        
        let row = sqlx::query_as!(
            ContactModel,
            r#"
            SELECT id, user_id, contact_type, platform_user_id, name, primary_email,
                   primary_phone, company, tags, consent_settings, external_data,
                   created_at, updated_at, last_interaction
            FROM crm_contacts
            WHERE id = $1
            "#,
            uuid
        )
        .fetch_optional(&self.pool)
        .await?;
        
        let model = row.ok_or_else(|| RepositoryError::ContactNotFound(id.clone()))?;
        let contact = self.convert_model_to_contact(model)?;
        
        Ok(contact)
    }
    
    /// Update a contact
    pub async fn update_contact(&self, contact: &Contact) -> Result<(), RepositoryError> {
        let model = self.convert_contact_to_model(contact)?;
        
        sqlx::query!(
            r#"
            UPDATE crm_contacts
            SET user_id = $1, contact_type = $2, platform_user_id = $3, name = $4,
                primary_email = $5, primary_phone = $6, company = $7, tags = $8,
                consent_settings = $9, external_data = $10, updated_at = $11,
                last_interaction = $12
            WHERE id = $13
            "#,
            model.user_id,
            model.contact_type,
            model.platform_user_id,
            model.name,
            model.primary_email,
            model.primary_phone,
            model.company,
            model.tags as Option<sqlx::types::Json<Vec<String>>>,
            model.consent_settings as Option<sqlx::types::Json<ConsentSettingsModel>>,
            model.external_data as Option<sqlx::types::Json<ExternalContactDataModel>>,
            model.updated_at,
            model.last_interaction,
            model.id,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Delete a contact
    pub async fn delete_contact(&self, id: &ContactId) -> Result<(), RepositoryError> {
        let uuid = id.0;
        
        sqlx::query!(
            "DELETE FROM crm_contacts WHERE id = $1",
            uuid
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Get all contacts for a user
    pub async fn get_contacts_for_user(&self, user_id: &UserId) -> Result<Vec<Contact>, RepositoryError> {
        let uuid = user_id.0;
        
        let rows = sqlx::query_as!(
            ContactModel,
            r#"
            SELECT id, user_id, contact_type, platform_user_id, name, primary_email,
                   primary_phone, company, tags, consent_settings, external_data,
                   created_at, updated_at, last_interaction
            FROM crm_contacts
            WHERE user_id = $1
            ORDER BY name
            "#,
            uuid
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut contacts = Vec::new();
        for model in rows {
            let contact = self.convert_model_to_contact(model)?;
            contacts.push(contact);
        }
        
        Ok(contacts)
    }
    
    /// Convert a domain contact to a database model
    fn convert_contact_to_model(&self, contact: &Contact) -> Result<ContactModel, RepositoryError> {
        let (contact_type, platform_user_id, consent_settings, external_data) = match &contact.contact_type {
            ContactType::PlatformNative(user_id, consent) => {
                let consent_model = ConsentSettingsModel {
                    share_profile: match consent.share_profile {
                        DataSharingLevel::None => "none".to_string(),
                        DataSharingLevel::ViewOnly => "view_only".to_string(),
                        DataSharingLevel::Editable => "editable".to_string(),
                    },
                    share_interaction_history: match consent.share_interaction_history {
                        DataSharingLevel::None => "none".to_string(),
                        DataSharingLevel::ViewOnly => "view_only".to_string(),
                        DataSharingLevel::Editable => "editable".to_string(),
                    },
                    share_preferences: match consent.share_preferences {
                        DataSharingLevel::None => "none".to_string(),
                        DataSharingLevel::ViewOnly => "view_only".to_string(),
                        DataSharingLevel::Editable => "editable".to_string(),
                    },
                    custom_fields: consent.custom_fields.clone(),
                };
                
                (
                    "platform_native".to_string(),
                    Some(user_id.0),
                    Some(sqlx::types::Json(consent_model)),
                    None,
                )
            }
            ContactType::External(data) => {
                let external_model = ExternalContactDataModel {
                    email: data.email.as_ref().map(|e| e.0.clone()),
                    phone: data.phone.as_ref().map(|p| p.0.clone()),
                    address: data.address.clone(),
                    social_media: data.social_media.clone(),
                };
                
                (
                    "external".to_string(),
                    None,
                    None,
                    Some(sqlx::types::Json(external_model)),
                )
            }
        };
        
        Ok(ContactModel {
            id: contact.id.0,
            user_id: contact.get_user_id().map(|u| u.0).unwrap_or_else(Uuid::new_v4), // This should be improved
            contact_type,
            platform_user_id,
            name: contact.name.clone(),
            primary_email: contact.primary_email.as_ref().map(|e| e.0.clone()),
            primary_phone: contact.primary_phone.as_ref().map(|p| p.0.clone()),
            company: contact.company.clone(),
            tags: Some(sqlx::types::Json(contact.tags.clone())),
            consent_settings,
            external_data,
            created_at: contact.created_at,
            updated_at: contact.updated_at,
            last_interaction: contact.last_interaction,
        })
    }
    
    /// Convert a database model to a domain contact
    fn convert_model_to_contact(&self, model: ContactModel) -> Result<Contact, RepositoryError> {
        let contact_type = match model.contact_type.as_str() {
            "platform_native" => {
                let user_id = model.platform_user_id
                    .ok_or_else(|| RepositoryError::ConversionError("Missing platform_user_id for platform_native contact".to_string()))?;
                
                let consent_settings = model.consent_settings
                    .ok_or_else(|| RepositoryError::ConversionError("Missing consent_settings for platform_native contact".to_string()))?;
                
                let share_profile = match consent_settings.0.share_profile.as_str() {
                    "none" => DataSharingLevel::None,
                    "view_only" => DataSharingLevel::ViewOnly,
                    "editable" => DataSharingLevel::Editable,
                    _ => return Err(RepositoryError::ConversionError("Invalid share_profile value".to_string())),
                };
                
                let share_interaction_history = match consent_settings.0.share_interaction_history.as_str() {
                    "none" => DataSharingLevel::None,
                    "view_only" => DataSharingLevel::ViewOnly,
                    "editable" => DataSharingLevel::Editable,
                    _ => return Err(RepositoryError::ConversionError("Invalid share_interaction_history value".to_string())),
                };
                
                let share_preferences = match consent_settings.0.share_preferences.as_str() {
                    "none" => DataSharingLevel::None,
                    "view_only" => DataSharingLevel::ViewOnly,
                    "editable" => DataSharingLevel::Editable,
                    _ => return Err(RepositoryError::ConversionError("Invalid share_preferences value".to_string())),
                };
                
                ContactType::PlatformNative(
                    UserId::from_uuid(user_id),
                    ConsentSettings {
                        share_profile,
                        share_interaction_history,
                        share_preferences,
                        custom_fields: consent_settings.0.custom_fields,
                    }
                )
            }
            "external" => {
                let external_data = model.external_data
                    .ok_or_else(|| RepositoryError::ConversionError("Missing external_data for external contact".to_string()))?;
                
                ContactType::External(ExternalContactData {
                    email: external_data.0.email.map(|e| Email::new(e)).transpose()
                        .map_err(|e| RepositoryError::ConversionError(format!("Invalid email: {}", e)))?,
                    phone: external_data.0.phone.map(|p| Phone::new(p)).transpose()
                        .map_err(|e| RepositoryError::ConversionError(format!("Invalid phone: {}", e)))?,
                    address: external_data.0.address,
                    social_media: external_data.0.social_media,
                })
            }
            _ => return Err(RepositoryError::ConversionError("Invalid contact_type value".to_string())),
        };
        
        let primary_email = model.primary_email.map(|e| Email::new(e))
            .transpose()
            .map_err(|e| RepositoryError::ConversionError(format!("Invalid primary_email: {}", e)))?;
        
        let primary_phone = model.primary_phone.map(|p| Phone::new(p))
            .transpose()
            .map_err(|e| RepositoryError::ConversionError(format!("Invalid primary_phone: {}", e)))?;
        
        Ok(Contact {
            id: ContactId::from_uuid(model.id),
            contact_type,
            name: model.name,
            primary_email,
            primary_phone,
            company: model.company,
            tags: model.tags.map(|t| t.0).unwrap_or_default(),
            created_at: model.created_at,
            updated_at: model.updated_at,
            last_interaction: model.last_interaction,
        })
    }
}

/// Repository for interaction operations
pub struct InteractionRepository {
    pool: PgPool,
}

impl InteractionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    /// Create a new interaction
    pub async fn create_interaction(&self, interaction: &Interaction) -> Result<(), RepositoryError> {
        let model = self.convert_interaction_to_model(interaction)?;
        
        sqlx::query!(
            r#"
            INSERT INTO crm_interactions (
                id, contact_id, interaction_type, platform_event_id, summary, details,
                timestamp, duration_seconds, created_by, is_platform_native,
                created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
            )
            "#,
            model.id,
            model.contact_id,
            model.interaction_type,
            model.platform_event_id,
            model.summary,
            model.details,
            model.timestamp,
            model.duration_seconds,
            model.created_by,
            model.is_platform_native,
            model.created_at,
            model.updated_at,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Get an interaction by ID
    pub async fn get_interaction_by_id(&self, id: &InteractionId) -> Result<Interaction, RepositoryError> {
        let uuid = id.0;
        
        let row = sqlx::query_as!(
            InteractionModel,
            r#"
            SELECT id, contact_id, interaction_type, platform_event_id, summary, details,
                   timestamp, duration_seconds, created_by, is_platform_native,
                   created_at, updated_at
            FROM crm_interactions
            WHERE id = $1
            "#,
            uuid
        )
        .fetch_optional(&self.pool)
        .await?;
        
        let model = row.ok_or_else(|| RepositoryError::InteractionNotFound(id.clone()))?;
        let interaction = self.convert_model_to_interaction(model)?;
        
        Ok(interaction)
    }
    
    /// Get interactions for a contact
    pub async fn get_interactions_for_contact(&self, contact_id: &ContactId) -> Result<Vec<Interaction>, RepositoryError> {
        let uuid = contact_id.0;
        
        let rows = sqlx::query_as!(
            InteractionModel,
            r#"
            SELECT id, contact_id, interaction_type, platform_event_id, summary, details,
                   timestamp, duration_seconds, created_by, is_platform_native,
                   created_at, updated_at
            FROM crm_interactions
            WHERE contact_id = $1
            ORDER BY timestamp DESC
            "#,
            uuid
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut interactions = Vec::new();
        for model in rows {
            let interaction = self.convert_model_to_interaction(model)?;
            interactions.push(interaction);
        }
        
        Ok(interactions)
    }
    
    /// Convert a domain interaction to a database model
    fn convert_interaction_to_model(&self, interaction: &Interaction) -> Result<InteractionModel, RepositoryError> {
        let (interaction_type, platform_event_id) = match &interaction.interaction_type {
            InteractionType::Call => ("call".to_string(), None),
            InteractionType::Email => ("email".to_string(), None),
            InteractionType::Meeting => ("meeting".to_string(), None),
            InteractionType::Message => ("message".to_string(), None),
            InteractionType::PlatformEvent(event_id) => ("platform_event".to_string(), Some(event_id.0)),
        };
        
        Ok(InteractionModel {
            id: interaction.id.0,
            contact_id: interaction.contact_id.0,
            interaction_type,
            platform_event_id,
            summary: interaction.summary.clone(),
            details: interaction.details.clone(),
            timestamp: interaction.timestamp,
            duration_seconds: interaction.duration.map(|d| d.num_seconds()),
            created_by: interaction.created_by.0,
            is_platform_native: interaction.is_platform_native,
            created_at: interaction.timestamp, // Using timestamp as created_at for simplicity
            updated_at: interaction.timestamp, // Using timestamp as updated_at for simplicity
        })
    }
    
    /// Convert a database model to a domain interaction
    fn convert_model_to_interaction(&self, model: InteractionModel) -> Result<Interaction, RepositoryError> {
        let interaction_type = match model.interaction_type.as_str() {
            "call" => InteractionType::Call,
            "email" => InteractionType::Email,
            "meeting" => InteractionType::Meeting,
            "message" => InteractionType::Message,
            "platform_event" => {
                let event_id = model.platform_event_id
                    .ok_or_else(|| RepositoryError::ConversionError("Missing platform_event_id for platform_event interaction".to_string()))?;
                InteractionType::PlatformEvent(PlatformEventId::from_uuid(event_id))
            }
            _ => return Err(RepositoryError::ConversionError("Invalid interaction_type value".to_string())),
        };
        
        Ok(Interaction {
            id: InteractionId::from_uuid(model.id),
            contact_id: ContactId::from_uuid(model.contact_id),
            interaction_type,
            summary: model.summary,
            details: model.details,
            timestamp: model.timestamp,
            duration: model.duration_seconds.map(Duration::seconds),
            created_by: UserId::from_uuid(model.created_by),
            is_platform_native: model.is_platform_native,
        })
    }
}

// Note: Implementations for PipelineRepository and DealRepository would follow similar patterns
// For brevity, I'm not including them in this example, but they would be implemented similarly

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPoolOptions;
    
    // Note: These tests would require a running PostgreSQL database
    // They are included for illustration but would be marked as #[ignore] in a real implementation
    
    #[ignore] // This test requires a database connection
    #[tokio::test]
    async fn test_contact_repository() {
        /*
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgresql://user:password@localhost/test_db")
            .await
            .expect("Failed to connect to database");
            
        let repo = ContactRepository::new(pool);
        
        // Test would continue here...
        */
    }
}