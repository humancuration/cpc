//! Compile test to verify all components work together
//!
//! This file is used to verify that all the new components can be imported
//! and compiled together without errors.

// Test imports of all new components
use packages::infra::core::network::monitor::NetworkStatusMonitor;
use packages::infra::core::adapters::composite::CompositeUserPreferences;
use packages::infra::core::factories::user_preferences_factory::UserPreferencesFactory;
use packages::infra::sync::queue::SyncQueue;
use packages::infra::sync::storage::{SledQueueStorage, SyncOperation, OperationPriority};
use packages::infra::sync::backoff::ExponentialBackoff;
use packages::infra::sync::conflict::TimestampConflictResolver;
use packages::infra::sync::network_fault_mock::NetworkFaultMockClient;
use packages::infra::sync::worker::SyncWorker;
use packages::infra::grpc::clients::user_preferences::{UserPreferencesClient, GrpcUserPreferences};
use packages::infra::sled::adapters::user_preferences::SledUserPreferences;
use packages::domains::finance::domain::primitives::Currency;
use std::sync::Arc;
use uuid::Uuid;
use tempfile::TempDir;
use std::time::SystemTime;

/// This test just needs to compile - it verifies that all components can be imported together
#[test]
fn test_all_components_compile() {
    // This test passes if it compiles successfully
    assert!(true);
}

/// This test verifies that the main components can be instantiated together
#[test]
fn test_components_can_be_created() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let db = sled::open(temp_dir.path())?;
    
    // Create network monitor
    let _monitor = NetworkStatusMonitor::new();
    
    // Create services
    let grpc_client = UserPreferencesClient::new();
    let online_service = GrpcUserPreferences::new(grpc_client.clone(), Uuid::nil());
    let offline_service = SledUserPreferences::new(&db);
    
    // Create composite adapter
    let _composite = CompositeUserPreferences::new(online_service, offline_service);
    
    // Create factory
    let _preferences = UserPreferencesFactory::create(&db, Arc::new(NetworkStatusMonitor::new()), grpc_client);
    
    // Create sync queue components
    let storage = SledQueueStorage::new(&db)?;
    let _queue = SyncQueue::new(
        Box::new(storage),
        Arc::new(TimestampConflictResolver::new()),
        Box::new(ExponentialBackoff::default())
    );
    
    // Create network fault mock
    let _mock_client = NetworkFaultMockClient::new(vec![true, false]);
    
    // Create sync worker
    let queue = SledQueueStorage::new(&db)?;
    let sync_queue = SyncQueue::new(
        Box::new(queue),
        Arc::new(TimestampConflictResolver::new()),
        Box::new(ExponentialBackoff::default())
    );
    let _worker = SyncWorker::new(Arc::new(sync_queue), std::time::Duration::from_secs(60));
    
    // Create operation
    let _operation = SyncOperation::SetCurrency {
        user_id: Uuid::new_v4(),
        currency: Currency::USD,
        priority: OperationPriority::High,
        attempts: 0,
        scheduled_at: SystemTime::now(),
    };
    
    Ok(())
}