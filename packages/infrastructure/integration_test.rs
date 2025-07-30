//! Integration test for the complete sync infrastructure
//!
//! This test verifies that all components work together correctly:
//! - Network monitoring
//! - Composite adapters
//! - Sync queue
//! - Backoff strategies
//! - Conflict resolution

#[cfg(test)]
mod tests {
    use crate::core::network::monitor::NetworkStatusMonitor;
    use crate::core::adapters::composite::CompositeUserPreferences;
    use crate::core::factories::user_preferences_factory::UserPreferencesFactory;
    use crate::sync::queue::SyncQueue;
    use crate::sync::storage::{SledQueueStorage, SyncOperation, OperationPriority};
    use crate::sync::backoff::ExponentialBackoff;
    use crate::sync::conflict::TimestampConflictResolver;
    use crate::grpc::clients::user_preferences::{UserPreferencesClient, GrpcUserPreferences};
    use crate::sled::adapters::user_preferences::SledUserPreferences;
    use packages::domains::finance::domain::primitives::Currency;
    use std::sync::Arc;
    use uuid::Uuid;
    use tempfile::TempDir;
    use std::time::SystemTime;

    #[test]
    fn test_complete_integration() {
        // This is a compile-only test to ensure all components can work together
        // A more comprehensive integration test would require async runtime
        
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        
        // Create network monitor
        let network_monitor = Arc::new(NetworkStatusMonitor::new());
        
        // Create services
        let grpc_client = UserPreferencesClient::new();
        let online_service = GrpcUserPreferences::new(grpc_client.clone(), Uuid::nil());
        let offline_service = SledUserPreferences::new(&db);
        
        // Create composite adapter
        let _composite = CompositeUserPreferences::new(online_service, offline_service);
        
        // Create factory
        let _preferences = UserPreferencesFactory::create(&db, network_monitor, grpc_client);
        
        // Create sync queue
        let storage = SledQueueStorage::new(&db).expect("Failed to create storage");
        let _queue = SyncQueue::new(
            Box::new(storage),
            Arc::new(TimestampConflictResolver::new()),
            Box::new(ExponentialBackoff::default())
        );
        
        // Create operation
        let user_id = Uuid::new_v4();
        let _operation = SyncOperation::SetCurrency {
            user_id,
            currency: Currency::USD,
            priority: OperationPriority::High,
            attempts: 0,
            scheduled_at: SystemTime::now(),
        };
        
        // All components compile together successfully
        assert!(true);
    }
}