//! Integration with external services for the Messenger application

use async_trait::async_trait;
use uuid::Uuid;
use consent_manager::domain::consent::{Domain, DataSharingLevel};
use consent_manager::application::service::ConsentService;
use std::sync::Arc;
use messenger_domain::errors::MessengerError;

/// Integration with the consent manager
#[async_trait]
pub trait ConsentManager: Send + Sync {
    /// Verify that a user has given consent for messaging
    async fn verify_messaging_consent(&self, user_id: Uuid) -> Result<(), String>;
    
    /// Verify that a user has given consent for media sharing
    async fn verify_media_sharing_consent(&self, user_id: Uuid) -> Result<(), String>;
}

/// Implementation of ConsentManager using the core consent manager
pub struct ConsentManagerImpl {
    consent_service: Arc<ConsentService>,
}

impl ConsentManagerImpl {
    /// Create a new consent manager integration
    pub fn new(consent_service: Arc<ConsentService>) -> Self {
        Self { consent_service }
    }
}

#[async_trait]
impl ConsentManager for ConsentManagerImpl {
    async fn verify_messaging_consent(&self, user_id: Uuid) -> Result<(), String> {
        let level = self.consent_service
            .get_consent_level(&user_id.to_string(), Domain::CrmData)
            .await
            .map_err(|e| format!("Failed to get consent level: {}", e))?;
            
        if level == DataSharingLevel::None {
            return Err("Messaging consent not granted".to_string());
        }
        
        Ok(())
    }
    
    async fn verify_media_sharing_consent(&self, user_id: Uuid) -> Result<(), String> {
        let level = self.consent_service
            .get_consent_level(&user_id.to_string(), Domain::CrmData)
            .await
            .map_err(|e| format!("Failed to get consent level: {}", e))?;
            
        // For media sharing, we might require a higher level of consent
        if level.priority() < DataSharingLevel::Standard.priority() {
            return Err("Media sharing consent not granted".to_string());
        }
        
        Ok(())
    }
}