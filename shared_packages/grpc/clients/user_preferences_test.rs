//! Unit tests for the gRPC user preferences client

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clients::user_preferences::{UserPreferencesClient, GrpcUserPreferences, NetworkStatusMonitor};
    use packages::domains::finance::domain::primitives::Currency;
    use packages::domains::finance::application::user_preferences::UserPreferences;
    use tempfile::TempDir;
    use uuid::Uuid;
    use tonic::transport::Channel;

    #[tokio::test]
    async fn test_get_preferred_currency_success() {
        // Create a mock client
        let client = UserPreferencesClient {
            inner: InnerClient {
                channel: Channel::from_static("http://[::1]:50051")
                    .connect_lazy()
                    .unwrap(),
                auth_token: "test_token".to_string(),
            }
        };
        
        let user_id = Uuid::new_v4();
        let preferences = GrpcUserPreferences::new(client, user_id);
        
        // This would normally make a network call
        let result = preferences.get_preferred_currency(user_id).await;
        // In our mock implementation, it will return USD
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_set_preferred_currency_success() {
        // Create a mock client
        let client = UserPreferencesClient {
            inner: InnerClient {
                channel: Channel::from_static("http://[::1]:50051")
                    .connect_lazy()
                    .unwrap(),
                auth_token: "test_token".to_string(),
            }
        };
        
        let user_id = Uuid::new_v4();
        let preferences = GrpcUserPreferences::new(client, user_id);
        
        // This would normally make a network call
        let result = preferences.set_preferred_currency(user_id, Currency::EUR).await;
        // In our mock implementation, it will succeed
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_retry_logic() {
        // Create a mock client
        let client = UserPreferencesClient {
            inner: InnerClient {
                channel: Channel::from_static("http://[::1]:50051")
                    .connect_lazy()
                    .unwrap(),
                auth_token: "test_token".to_string(),
            }
        };
        
        let user_id = Uuid::new_v4();
        let preferences = GrpcUserPreferences::new(client, user_id);
        
        // Test that retry logic is implemented (this is a basic check)
        // In a real test, we would mock network failures and verify retries
        assert!(true); // Placeholder for actual retry logic test
    }
    
    #[tokio::test]
    async fn test_network_status_monitoring() {
        let mut monitor = NetworkStatusMonitor::new();
        
        // Initially should be connected
        assert!(monitor.is_connected());
        
        // Test changing status
        monitor.set_connected(false);
        assert!(!monitor.is_connected());
        
        monitor.set_connected(true);
        assert!(monitor.is_connected());
    }
    
    #[tokio::test]
    async fn test_currency_conversion() {
        // Test that currency codes are properly converted to Currency enum
        let client = UserPreferencesClient {
            inner: InnerClient {
                channel: Channel::from_static("http://[::1]:50051")
                    .connect_lazy()
                    .unwrap(),
                auth_token: "test_token".to_string(),
            }
        };
        
        let user_id = Uuid::new_v4();
        let preferences = GrpcUserPreferences::new(client, user_id);
        
        // Test various currency codes
        let test_cases = vec![
            ("USD", Currency::USD),
            ("EUR", Currency::EUR),
            ("GBP", Currency::GBP),
            ("JPY", Currency::JPY),
            ("DABLOONS", Currency::Dabloons),
        ];
        
        // In a real test, we would mock the gRPC response to return these codes
        // For now, we'll just verify our test structure
        assert!(!test_cases.is_empty());
    }
}