//! Backoff strategies for retrying failed operations
//!
//! This module implements various backoff strategies for retrying failed sync operations.
//! The default implementation uses exponential backoff with jitter to avoid overwhelming
//! services with repeated rapid retries.

use std::time::Duration;
use crate::storage::SyncOperation;

/// Trait for backoff strategies
pub trait BackoffStrategy {
    /// Calculate the next delay for retrying an operation
    fn next_delay(&self, operation: &SyncOperation) -> Duration;
}

/// Exponential backoff with jitter
pub struct ExponentialBackoff {
    base_delay: Duration,
    max_delay: Duration,
    jitter_factor: f64,
}

impl ExponentialBackoff {
    /// Create a new ExponentialBackoff instance
    pub fn new(base_delay: Duration, max_delay: Duration, jitter_factor: f64) -> Self {
        Self {
            base_delay,
            max_delay,
            jitter_factor,
        }
    }
    
    /// Create a default ExponentialBackoff instance
    pub fn default() -> Self {
        Self {
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(300), // 5 minutes
            jitter_factor: 0.25,
        }
    }
}

impl Default for ExponentialBackoff {
    fn default() -> Self {
        Self::default()
    }
}

impl BackoffStrategy for ExponentialBackoff {
    fn next_delay(&self, operation: &SyncOperation) -> Duration {
        let attempts = operation.attempts() as f64;
        let base = self.base_delay.as_millis() as f64;
        let max = self.max_delay.as_millis() as f64;
        
        // Exponential growth with jitter
        let delay = base * 2f64.powf(attempts);
        let jitter = rand::random::<f64>() * self.jitter_factor * delay;
        
        // Ensure we don't exceed max_delay
        let total_delay = (delay + jitter).min(max) as u64;
        
        Duration::from_millis(total_delay)
    }
}

/// Linear backoff strategy
pub struct LinearBackoff {
    base_delay: Duration,
    increment: Duration,
    max_delay: Duration,
}

impl LinearBackoff {
    /// Create a new LinearBackoff instance
    pub fn new(base_delay: Duration, increment: Duration, max_delay: Duration) -> Self {
        Self {
            base_delay,
            increment,
            max_delay,
        }
    }
}

impl BackoffStrategy for LinearBackoff {
    fn next_delay(&self, operation: &SyncOperation) -> Duration {
        let attempts = operation.attempts() as u32;
        let base = self.base_delay.as_millis() as u64;
        let increment = self.increment.as_millis() as u64;
        let max = self.max_delay.as_millis() as u64;
        
        let delay = base + (increment * attempts as u64);
        let clamped_delay = delay.min(max);
        
        Duration::from_millis(clamped_delay)
    }
}

/// Constant backoff strategy
pub struct ConstantBackoff {
    delay: Duration,
}

impl ConstantBackoff {
    /// Create a new ConstantBackoff instance
    pub fn new(delay: Duration) -> Self {
        Self { delay }
    }
}

impl BackoffStrategy for ConstantBackoff {
    fn next_delay(&self, _operation: &SyncOperation) -> Duration {
        self.delay
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;
    use uuid::Uuid;
    use packages::domains::finance::domain::primitives::Currency;
    use crate::storage::OperationPriority;

    #[test]
    fn test_exponential_backoff() {
        let backoff = ExponentialBackoff::default();
        
        let operation = SyncOperation::SetCurrency {
            user_id: Uuid::new_v4(),
            currency: Currency::USD,
            priority: OperationPriority::High,
            attempts: 0,
            scheduled_at: SystemTime::now(),
        };
        
        let delay = backoff.next_delay(&operation);
        assert_eq!(delay, Duration::from_millis(100)); // Base delay
        
        let operation = SyncOperation::SetCurrency {
            user_id: Uuid::new_v4(),
            currency: Currency::USD,
            priority: OperationPriority::High,
            attempts: 1,
            scheduled_at: SystemTime::now(),
        };
        
        let delay = backoff.next_delay(&operation);
        // Should be 200ms ± jitter
        assert!(delay >= Duration::from_millis(150));
        assert!(delay <= Duration::from_millis(250));
    }
    
    #[test]
    fn test_linear_backoff() {
        let backoff = LinearBackoff::new(
            Duration::from_millis(100),
            Duration::from_millis(50),
            Duration::from_secs(1)
        );
        
        let operation = SyncOperation::SetCurrency {
            user_id: Uuid::new_v4(),
            currency: Currency::USD,
            priority: OperationPriority::High,
            attempts: 0,
            scheduled_at: SystemTime::now(),
        };
        
        let delay = backoff.next_delay(&operation);
        assert_eq!(delay, Duration::from_millis(100)); // Base delay
        
        let operation = SyncOperation::SetCurrency {
            user_id: Uuid::new_v4(),
            currency: Currency::USD,
            priority: OperationPriority::High,
            attempts: 1,
            scheduled_at: SystemTime::now(),
        };
        
        let delay = backoff.next_delay(&operation);
        assert_eq!(delay, Duration::from_millis(150)); // Base + increment
    }
    
    #[test]
    fn test_constant_backoff() {
        let backoff = ConstantBackoff::new(Duration::from_millis(500));
        
        let operation = SyncOperation::SetCurrency {
            user_id: Uuid::new_v4(),
            currency: Currency::USD,
            priority: OperationPriority::High,
            attempts: 0,
            scheduled_at: SystemTime::now(),
        };
        
        let delay = backoff.next_delay(&operation);
        assert_eq!(delay, Duration::from_millis(500));
        
        let operation = SyncOperation::SetCurrency {
            user_id: Uuid::new_v4(),
            currency: Currency::USD,
            priority: OperationPriority::High,
            attempts: 5,
            scheduled_at: SystemTime::now(),
        };
        
        let delay = backoff.next_delay(&operation);
        assert_eq!(delay, Duration::from_millis(500)); // Always the same
    }
    
    #[test]
    fn test_exponential_backoff_max_delay() {
        let backoff = ExponentialBackoff::new(
            Duration::from_millis(100),
            Duration::from_millis(500),
            0.0
        );
        
        let operation = SyncOperation::SetCurrency {
            user_id: Uuid::new_v4(),
            currency: Currency::USD,
            priority: OperationPriority::High,
            attempts: 10, // High attempt count
            scheduled_at: SystemTime::now(),
        };
        
        let delay = backoff.next_delay(&operation);
        assert_eq!(delay, Duration::from_millis(500)); // Should be clamped to max
    }
}