//! Mock client for simulating network faults during testing
//!
//! This module provides a mock client that can simulate various network conditions
//! for testing the resilience of our sync queue implementation.

use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use uuid::Uuid;
use packages::domains::finance::domain::primitives::Currency;
use crate::queue::UserPreferencesClient;

/// Record of an operation attempt
#[derive(Debug, Clone)]
pub struct OperationRecord {
    pub timestamp: SystemTime,
    pub user_id: Uuid,
    pub currency: Currency,
    pub success: bool,
}

/// Mock client that simulates various network conditions
pub struct NetworkFaultMockClient {
    /// Pattern of successes/failures (true = success, false = failure)
    failure_pattern: Vec<bool>,
    current_index: std::sync::atomic::AtomicUsize,
    /// Timestamp of last successful operation
    last_success: Mutex<Option<SystemTime>>,
    /// Record of all operations attempted
    operation_history: Mutex<Vec<OperationRecord>>,
}

impl NetworkFaultMockClient {
    /// Create a new NetworkFaultMockClient with a specific failure pattern
    pub fn new(failure_pattern: Vec<bool>) -> Self {
        Self {
            failure_pattern,
            current_index: std::sync::atomic::AtomicUsize::new(0),
            last_success: Mutex::new(None),
            operation_history: Mutex::new(Vec::new()),
        }
    }
    
    /// Create a client that fails randomly with specified probability
    pub fn with_random_failure_rate(failure_probability: f64) -> Self {
        // Generate a pattern of 100 operations with random failures
        let mut pattern = Vec::with_capacity(100);
        for _ in 0..100 {
            pattern.push(rand::random::<f64>() > failure_probability);
        }
        Self::new(pattern)
    }
    
    /// Create a client that fails for specified duration then succeeds
    pub fn with_temporary_failure(duration: Duration) -> Self {
        // Create pattern with initial failures followed by successes
        // For simplicity, we'll create 50 failures then 50 successes
        let mut pattern = vec![false; 50];
        pattern.extend(vec![true; 50]);
        Self::new(pattern)
    }
    
    /// Get the operation history
    pub fn get_operation_history(&self) -> Vec<OperationRecord> {
        self.operation_history.lock().unwrap().clone()
    }
    
    /// Get the timestamp of the last successful operation
    pub fn last_success_time(&self) -> Option<SystemTime> {
        *self.last_success.lock().unwrap()
    }
}

#[async_trait]
impl UserPreferencesClient for NetworkFaultMockClient {
    async fn set_preferred_currency(&self, user_id: Uuid, currency: Currency) -> Result<(), String> {
        let index = self.current_index.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let pattern_index = index % self.failure_pattern.len();
        let success = self.failure_pattern[pattern_index];
        
        // Record operation
        self.operation_history.lock().unwrap().push(OperationRecord {
            timestamp: SystemTime::now(),
            user_id,
            currency: currency.clone(),
            success,
        });
        
        if success {
            *self.last_success.lock().unwrap() = Some(SystemTime::now());
            Ok(())
        } else {
            Err("Network error".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_network_fault_mock_client() {
        let client = NetworkFaultMockClient::new(vec![true, false, true]);
        
        let user_id = Uuid::new_v4();
        
        // First call should succeed
        let result = client.set_preferred_currency(user_id, Currency::USD).await;
        assert!(result.is_ok());
        
        // Second call should fail
        let result = client.set_preferred_currency(user_id, Currency::EUR).await;
        assert!(result.is_err());
        
        // Third call should succeed
        let result = client.set_preferred_currency(user_id, Currency::GBP).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_network_fault_mock_client_with_random_failures() {
        let client = NetworkFaultMockClient::with_random_failure_rate(0.3); // 30% failure rate
        
        let user_id = Uuid::new_v4();
        
        // Make several calls and check that some succeed and some fail
        let mut successes = 0;
        let mut failures = 0;
        
        for _ in 0..20 {
            let result = client.set_preferred_currency(user_id, Currency::USD).await;
            if result.is_ok() {
                successes += 1;
            } else {
                failures += 1;
            }
            sleep(Duration::from_millis(10)).await; // Small delay between calls
        }
        
        // We should have both successes and failures
        assert!(successes > 0);
        assert!(failures > 0);
    }
    
    #[tokio::test]
    async fn test_network_fault_mock_client_operation_history() {
        let client = NetworkFaultMockClient::new(vec![true, false]);
        
        let user_id = Uuid::new_v4();
        
        // Make a few calls
        let _ = client.set_preferred_currency(user_id, Currency::USD).await;
        let _ = client.set_preferred_currency(user_id, Currency::EUR).await;
        
        // Check operation history
        let history = client.get_operation_history();
        assert_eq!(history.len(), 2);
        
        // First operation should be successful
        assert!(history[0].success);
        assert_eq!(history[0].currency, Currency::USD);
        
        // Second operation should fail
        assert!(!history[1].success);
        assert_eq!(history[1].currency, Currency::EUR);
    }
}