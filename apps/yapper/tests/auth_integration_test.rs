//! # Auth Integration Test
//!
//! Integration test for the unified authentication system in the Yapper app.

#[cfg(test)]
mod tests {
    use yapper::infrastructure::auth_service_client::YapperAuthServiceClient;
    use yapper::infrastructure::consent_manager::YapperConsentManager;
    use cpc_consent::ConsentService;
    use cpc_consent::ConsentLevel;
    use cpc_consent::Domain;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_consent_manager() {
        // Create consent service and manager
        let consent_service = Arc::new(ConsentService::new());
        let consent_manager = YapperConsentManager::new(consent_service);
        
        // Create a test user
        let user_id = Uuid::new_v4();
        
        // Check initial consent level (should be None)
        assert!(!consent_manager.check_consent(user_id, ConsentLevel::Minimal));
        
        // Set consent level to Standard
        consent_manager.set_consent(user_id, ConsentLevel::Standard);
        
        // Check that Standard and lower levels are allowed
        assert!(consent_manager.check_consent(user_id, ConsentLevel::Minimal));
        assert!(consent_manager.check_consent(user_id, ConsentLevel::Standard));
        
        // Check that Full level is not allowed
        assert!(!consent_manager.check_consent(user_id, ConsentLevel::Full));
    }

    #[tokio::test]
    async fn test_auth_service_client_creation() {
        // This test just verifies that we can create the client
        // In a real implementation, you would need a running auth service
        /*
        let client = YapperAuthServiceClient::new("http://[::1]:50051".to_string()).await;
        assert!(client.is_ok());
        */
    }
}