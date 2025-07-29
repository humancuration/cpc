//! Consent management for cross-module data sharing
//!
//! This module implements the consent checking required by our privacy policies
//! and handles data minimization for cross-module integrations using the
//! centralized Consent Manager.

use crate::domain::CalendarError;
use uuid::Uuid;
use std::sync::Arc;
use consent_manager::{
    domain::{
        consent::{DataSharingLevel, Domain},
    },
    application::service::ConsentService as NewConsentService,
};

/// Service for checking and managing user consent using the new Consent Manager
pub struct ConsentService {
    consent_service: Arc<NewConsentService>,
}

impl ConsentService {
    /// Create a new consent service
    pub fn new(consent_service: Arc<NewConsentService>) -> Self {
        Self { consent_service }
    }
    
    /// Check if user has granted consent for CRM integration
    pub async fn has_crm_consent(&self, user_id: Uuid) -> Result<bool, CalendarError> {
        let user_id_str = user_id.to_string();
        let level = self.consent_service
            .get_consent_level(&user_id_str, Domain::CrmData)
            .await
            .map_err(|e| CalendarError::InvalidData(format!("Consent service error: {:?}", e)))?;
        
        // For CRM integration, we need at least Minimal level
        Ok(level.priority() >= DataSharingLevel::Minimal.priority())
    }
    
    /// Check if user has granted consent for Invoicing integration
    pub async fn has_invoicing_consent(&self, user_id: Uuid) -> Result<bool, CalendarError> {
        let user_id_str = user_id.to_string();
        let level = self.consent_service
            .get_consent_level(&user_id_str, Domain::FinancialData)
            .await
            .map_err(|e| CalendarError::InvalidData(format!("Consent service error: {:?}", e)))?;
        
        // For Invoicing integration, we need at least Minimal level
        Ok(level.priority() >= DataSharingLevel::Minimal.priority())
    }
    
    /// Get the consent level for calendar data sharing
    pub async fn get_calendar_consent_level(&self, user_id: Uuid) -> Result<DataSharingLevel, CalendarError> {
        let user_id_str = user_id.to_string();
        let level = self.consent_service
            .get_consent_level(&user_id_str, Domain::CalendarData)
            .await
            .map_err(|e| CalendarError::InvalidData(format!("Consent service error: {:?}", e)))?;
        
        Ok(level)
    }
    
    /// Update the consent level for calendar data sharing
    pub async fn update_calendar_consent_level(
        &self,
        user_id: Uuid,
        level: DataSharingLevel,
    ) -> Result<(), CalendarError> {
        let user_id_str = user_id.to_string();
        let actor = consent_manager::domain::audit::Actor::User(user_id_str.clone());
        
        self.consent_service
            .update_consent_level(&user_id_str, Domain::CalendarData, level, actor)
            .await
            .map_err(|e| CalendarError::InvalidData(format!("Consent service error: {:?}", e)))?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use consent_manager::domain::consent::DataSharingLevel as NewDataSharingLevel;
    use consent_manager::domain::consent::Domain as NewDomain;
    use consent_manager::domain::errors::ConsentError;
    use consent_manager::application::service::{ConsentService as NewConsentService, ConsentStorage};
    use async_trait::async_trait;
    use std::collections::HashMap;
    
    struct MockConsentStorage {
        profiles: HashMap<String, consent_manager::domain::consent::ConsentProfile>,
    }
    
    impl MockConsentStorage {
        fn new() -> Self {
            Self {
                profiles: HashMap::new(),
            }
        }
    }
    
    #[async_trait]
    impl ConsentStorage for MockConsentStorage {
        async fn get_consent_profile(&self, user_id: &str, domain: &NewDomain) -> Result<Option<consent_manager::domain::consent::ConsentProfile>, ConsentError> {
            let key = format!("{}:{:?}", user_id, domain);
            Ok(self.profiles.get(&key).cloned())
        }
        
        async fn save_consent_profile(&self, profile: &consent_manager::domain::consent::ConsentProfile) -> Result<(), ConsentError> {
            // In a real implementation, we would save to a database
            Ok(())
        }
        
        async fn revoke_domain(&self, user_id: &str, domain: &NewDomain) -> Result<(), ConsentError> {
            // In a real implementation, we would revoke consent in the database
            Ok(())
        }
        
        async fn get_audit_events(&self, user_id: &str) -> Result<Vec<consent_manager::domain::audit::AuditEvent>, ConsentError> {
            // In a real implementation, we would fetch audit events from a database
            Ok(vec![])
        }
        
        async fn save_audit_event(&self, event: &consent_manager::domain::audit::AuditEvent) -> Result<(), ConsentError> {
            // In a real implementation, we would save audit events to a database
            Ok(())
        }
    }
    
    #[tokio::test]
    async fn test_consent_service() {
        let storage = Box::new(MockConsentStorage::new());
        let new_consent_service = Arc::new(NewConsentService::new(storage));
        let service = ConsentService::new(new_consent_service);
        
        let user_id = Uuid::new_v4();
        
        // Test CRM consent
        let has_crm_consent = service.has_crm_consent(user_id).await.unwrap();
        // By default, consent level is None, so this should be false
        assert!(!has_crm_consent);
        
        // Test Invoicing consent
        let has_invoicing_consent = service.has_invoicing_consent(user_id).await.unwrap();
        // By default, consent level is None, so this should be false
        assert!(!has_invoicing_consent);
    }
}