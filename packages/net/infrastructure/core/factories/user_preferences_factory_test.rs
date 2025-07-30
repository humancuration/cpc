//! Tests for the user preferences factory

#[cfg(test)]
mod tests {
    use packages::infra::core::factories::user_preferences_factory::UserPreferencesFactory;
    use packages::infra::grpc::clients::user_preferences::{UserPreferencesClient, NetworkStatusMonitor};
    use packages::domains::finance::application::user_preferences::UserPreferences;
    use packages::domains::finance::domain::primitives::Currency;
    use uuid::Uuid;
    use std::sync::Arc;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_factory_creates_online_implementation() {
        // Create a temporary database for testing
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        
        // Create a mock gRPC client
        let client = UserPreferencesClient::new();
        
        // Create a network monitor and set it to connected
        let mut network_monitor = NetworkStatusMonitor::new();
        network_monitor.set_connected(true);
        let network_monitor = Arc::new(network_monitor);
        
        let user_id = Uuid::new_v4();
        
        // Create the user preferences implementation using the factory
        let preferences = UserPreferencesFactory::create(&db, network_monitor, client, user_id);
        
        // Verify that we can use the preferences
        let result = preferences.set_preferred_currency(user_id, Currency::EUR).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_factory_creates_offline_implementation() {
        // Create a temporary database for testing
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        
        // Create a mock gRPC client
        let client = UserPreferencesClient::new();
        
        // Create a network monitor and set it to disconnected
        let mut network_monitor = NetworkStatusMonitor::new();
        network_monitor.set_connected(false);
        let network_monitor = Arc::new(network_monitor);
        
        let user_id = Uuid::new_v4();
        
        // Create the user preferences implementation using the factory
        let preferences = UserPreferencesFactory::create(&db, network_monitor, client, user_id);
        
        // Verify that we can use the preferences
        let result = preferences.set_preferred_currency(user_id, Currency::EUR).await;
        assert!(result.is_ok());
        
        // Verify that we can get the currency back
        let currency = preferences.get_preferred_currency(user_id).await;
        assert!(currency.is_ok());
        assert_eq!(currency.unwrap(), Currency::EUR);
    }
}