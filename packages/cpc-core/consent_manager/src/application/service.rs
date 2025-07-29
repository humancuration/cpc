//! Consent service implementation.

use crate::domain::{
    consent::{ConsentProfile, DataSharingLevel, Domain},
    audit::{AuditEvent, Actor, ConsentAction},
    errors::ConsentError,
};
use std::collections::HashMap;

/// Trait for storage operations
#[async_trait::async_trait]
pub trait ConsentStorage: Send + Sync {
    /// Get consent profile for a user and domain
    async fn get_consent_profile(&self, user_id: &str, domain: &Domain) -> Result<Option<ConsentProfile>, ConsentError>;
    
    /// Save consent profile
    async fn save_consent_profile(&self, profile: &ConsentProfile) -> Result<(), ConsentError>;
    
    /// Revoke consent for a domain
    async fn revoke_domain(&self, user_id: &str, domain: &Domain) -> Result<(), ConsentError>;
    
    /// Get audit events for a user
    async fn get_audit_events(&self, user_id: &str) -> Result<Vec<AuditEvent>, ConsentError>;
    
    /// Save audit event
    async fn save_audit_event(&self, event: &AuditEvent) -> Result<(), ConsentError>;
}

/// Consent service orchestrates consent operations
pub struct ConsentService {
    storage: Box<dyn ConsentStorage>,
}

impl ConsentService {
    /// Create a new consent service
    pub fn new(storage: Box<dyn ConsentStorage>) -> Self {
        Self { storage }
    }

    /// Get the current consent level for a user and domain
    pub async fn get_consent_level(&self, user_id: &str, domain: Domain) -> Result<DataSharingLevel, ConsentError> {
        match self.storage.get_consent_profile(user_id, &domain).await? {
            Some(profile) => Ok(profile.level.clone()),
            None => Ok(DataSharingLevel::None), // Default to None if no profile exists
        }
    }

    /// Update the consent level for a user and domain
    pub async fn update_consent_level(
        &self,
        user_id: &str,
        domain: Domain,
        level: DataSharingLevel,
        actor: Actor,
    ) -> Result<(), ConsentError> {
        // Get existing profile or create new one
        let mut profile = match self.storage.get_consent_profile(user_id, &domain).await? {
            Some(mut profile) => {
                let previous_level = profile.level.clone();
                profile.set_level(level.clone())?;
                
                // Create audit event
                let audit_event = AuditEvent::new(
                    user_id.to_string(),
                    domain.clone(),
                    ConsentAction::Modified,
                    Some(previous_level.clone()),
                    level.clone(),
                    actor,
                );
                self.storage.save_audit_event(&audit_event).await?;
                
                profile
            },
            None => {
                let profile = ConsentProfile::new(user_id.to_string(), domain.clone(), level.clone());
                
                // Create audit event
                let audit_event = AuditEvent::new(
                    user_id.to_string(),
                    domain.clone(),
                    ConsentAction::Granted,
                    None,
                    level.clone(),
                    actor,
                );
                self.storage.save_audit_event(&audit_event).await?;
                
                profile
            }
        };

        // Save the updated profile
        self.storage.save_consent_profile(&profile).await?;
        
        // Publish consent change event (if bevy integration is enabled)
        #[cfg(feature = "bevy-integration")]
        {
            if let Some(event_channel) = crate::infrastructure::events::bevy::ConsentEventChannel::get_global() {
                let event = crate::infrastructure::events::bevy::ConsentChangeEvent {
                    user_id: user_id.to_string(),
                    domain: domain.clone(),
                    new_level: level.clone(),
                    timestamp: chrono::Utc::now(),
                };
                event_channel.publish(event);
            }
        }
        
        Ok(())
    }

    /// Revoke consent for a specific domain
    pub async fn revoke_domain(&self, user_id: &str, domain: Domain, actor: Actor) -> Result<(), ConsentError> {
        // Get existing profile
        if let Some(profile) = self.storage.get_consent_profile(user_id, &domain).await? {
            let previous_level = profile.level.clone();
            
            // Revoke in storage
            self.storage.revoke_domain(user_id, &domain).await?;
            
            // Create audit event
            let audit_event = AuditEvent::new(
                user_id.to_string(),
                domain.clone(),
                ConsentAction::Revoked,
                Some(previous_level.clone()),
                DataSharingLevel::None,
                actor,
            );
            self.storage.save_audit_event(&audit_event).await?;
            
            // Publish consent change event (if bevy integration is enabled)
            #[cfg(feature = "bevy-integration")]
            {
                if let Some(event_channel) = crate::infrastructure::events::bevy::ConsentEventChannel::get_global() {
                    let event = crate::infrastructure::events::bevy::ConsentChangeEvent {
                        user_id: user_id.to_string(),
                        domain: domain.clone(),
                        new_level: DataSharingLevel::None,
                        timestamp: chrono::Utc::now(),
                    };
                    event_channel.publish(event);
                }
            }
        }
        
        Ok(())
    }

    /// Get audit events for a user
    pub async fn get_audit_events(&self, user_id: &str) -> Result<Vec<AuditEvent>, ConsentError> {
        self.storage.get_audit_events(user_id).await
    }
}