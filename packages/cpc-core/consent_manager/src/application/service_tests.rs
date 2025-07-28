//! Tests for the consent service.

#[cfg(test)]
mod tests {
    use super::super::service::*;
    use crate::domain::{
        consent::{ConsentProfile, DataSharingLevel, Domain},
        audit::{AuditEvent, Actor, ConsentAction},
        errors::ConsentError,
    };
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    /// Mock storage implementation for testing
    struct MockStorage {
        profiles: Arc<Mutex<HashMap<(String, Domain), ConsentProfile>>>,
        audit_events: Arc<Mutex<HashMap<String, Vec<AuditEvent>>>>,
    }

    impl MockStorage {
        fn new() -> Self {
            Self {
                profiles: Arc::new(Mutex::new(HashMap::new())),
                audit_events: Arc::new(Mutex::new(HashMap::new())),
            }
        }
    }

    #[async_trait::async_trait]
    impl ConsentStorage for MockStorage {
        async fn get_consent_profile(&self, user_id: &str, domain: &Domain) -> Result<Option<ConsentProfile>, ConsentError> {
            let profiles = self.profiles.lock().await;
            Ok(profiles.get(&(user_id.to_string(), domain.clone())).cloned())
        }

        async fn save_consent_profile(&self, profile: &ConsentProfile) -> Result<(), ConsentError> {
            let mut profiles = self.profiles.lock().await;
            profiles.insert((profile.user_id.clone(), profile.domain.clone()), profile.clone());
            Ok(())
        }

        async fn revoke_domain(&self, user_id: &str, domain: &Domain) -> Result<(), ConsentError> {
            let mut profiles = self.profiles.lock().await;
            profiles.remove(&(user_id.to_string(), domain.clone()));
            Ok(())
        }

        async fn get_audit_events(&self, user_id: &str) -> Result<Vec<AuditEvent>, ConsentError> {
            let audit_events = self.audit_events.lock().await;
            Ok(audit_events.get(user_id).cloned().unwrap_or_default())
        }

        async fn save_audit_event(&self, event: &AuditEvent) -> Result<(), ConsentError> {
            let mut audit_events = self.audit_events.lock().await;
            let user_events = audit_events.entry(event.user_id.clone()).or_insert_with(Vec::new);
            user_events.push(event.clone());
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_get_consent_level_default_none() {
        let storage = Box::new(MockStorage::new());
        let service = ConsentService::new(storage);
        
        let level = service
            .get_consent_level("user123", Domain::FinancialData)
            .await
            .unwrap();
            
        assert_eq!(level, DataSharingLevel::None);
    }

    #[tokio::test]
    async fn test_update_consent_level_new_profile() {
        let storage = Box::new(MockStorage::new());
        let service = ConsentService::new(storage);
        
        let result = service
            .update_consent_level(
                "user123",
                Domain::FinancialData,
                DataSharingLevel::Standard,
                Actor::User("user123".to_string()),
            )
            .await;
            
        assert!(result.is_ok());
        
        // Verify the level was set
        let level = service
            .get_consent_level("user123", Domain::FinancialData)
            .await
            .unwrap();
            
        assert_eq!(level, DataSharingLevel::Standard);
    }

    #[tokio::test]
    async fn test_update_consent_level_existing_profile() {
        let storage = Box::new(MockStorage::new());
        let service = ConsentService::new(storage);
        
        // First set a level
        service
            .update_consent_level(
                "user123",
                Domain::FinancialData,
                DataSharingLevel::Minimal,
                Actor::User("user123".to_string()),
            )
            .await
            .unwrap();
            
        // Then update it
        let result = service
            .update_consent_level(
                "user123",
                Domain::FinancialData,
                DataSharingLevel::Standard,
                Actor::User("user123".to_string()),
            )
            .await;
            
        assert!(result.is_ok());
        
        // Verify the level was updated
        let level = service
            .get_consent_level("user123", Domain::FinancialData)
            .await
            .unwrap();
            
        assert_eq!(level, DataSharingLevel::Standard);
    }

    #[tokio::test]
    async fn test_revoke_domain() {
        let storage = Box::new(MockStorage::new());
        let service = ConsentService::new(storage);
        
        // First set a level
        service
            .update_consent_level(
                "user123",
                Domain::FinancialData,
                DataSharingLevel::Standard,
                Actor::User("user123".to_string()),
            )
            .await
            .unwrap();
            
        // Verify the level was set
        let level = service
            .get_consent_level("user123", Domain::FinancialData)
            .await
            .unwrap();
        assert_eq!(level, DataSharingLevel::Standard);
            
        // Then revoke it
        let result = service
            .revoke_domain(
                "user123",
                Domain::FinancialData,
                Actor::User("user123".to_string()),
            )
            .await;
            
        assert!(result.is_ok());
        
        // Verify the level was revoked (defaults to None)
        let level = service
            .get_consent_level("user123", Domain::FinancialData)
            .await
            .unwrap();
            
        assert_eq!(level, DataSharingLevel::None);
    }

    #[tokio::test]
    async fn test_get_audit_events() {
        let storage = Box::new(MockStorage::new());
        let service = ConsentService::new(storage);
        
        // Perform some operations
        service
            .update_consent_level(
                "user123",
                Domain::FinancialData,
                DataSharingLevel::Standard,
                Actor::User("user123".to_string()),
            )
            .await
            .unwrap();
            
        service
            .update_consent_level(
                "user123",
                Domain::HealthData,
                DataSharingLevel::Minimal,
                Actor::User("user123".to_string()),
            )
            .await
            .unwrap();
            
        // Get audit events
        let events = service
            .get_audit_events("user123")
            .await
            .unwrap();
            
        assert_eq!(events.len(), 2);
        
        // Verify event details
        let financial_event = events.iter().find(|e| e.domain == Domain::FinancialData).unwrap();
        assert_eq!(financial_event.action, ConsentAction::Granted);
        assert_eq!(financial_event.new_level, DataSharingLevel::Standard);
        
        let health_event = events.iter().find(|e| e.domain == Domain::HealthData).unwrap();
        assert_eq!(health_event.action, ConsentAction::Granted);
        assert_eq!(health_event.new_level, DataSharingLevel::Minimal);
    }
}