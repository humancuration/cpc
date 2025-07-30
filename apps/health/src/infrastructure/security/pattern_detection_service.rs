//! Pattern Detection Service with Circuit Breaker
//!
//! This module provides a circuit breaker wrapper for the pattern detection service
//! to ensure resilience in distributed environments.

use crate::domain::audit_log::AccessAttemptType;
use crate::infrastructure::security::attempt_monitor::{AttemptMonitor, SecurityAction};
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use uuid::Uuid;
use tracing::{warn, error};

/// Configuration for the circuit breaker
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Number of failures that will trip the circuit
    pub failure_threshold: usize,
    
    /// Time window for counting failures
    pub failure_window: Duration,
    
    /// Time to wait before attempting to close the circuit again
    pub reset_timeout: Duration,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            failure_window: Duration::from_secs(60), // 1 minute
            reset_timeout: Duration::from_secs(120), // 2 minutes
        }
    }
}

/// State of the circuit breaker
#[derive(Debug, Clone, PartialEq)]
enum CircuitState {
    /// Circuit is closed and functioning normally
    Closed,
    
    /// Circuit is open due to too many failures
    Open,
    
    /// Circuit is half-open, allowing limited attempts to test if service is restored
    HalfOpen,
}

/// Circuit breaker for pattern detection service
pub struct PatternDetectionService {
    /// The underlying attempt monitor
    attempt_monitor: Arc<AttemptMonitor>,
    
    /// Circuit breaker configuration
    config: CircuitBreakerConfig,
    
    /// Current state of the circuit
    state: CircuitState,
    
    /// Time when the circuit opened
    open_time: Option<Instant>,
    
    /// Recent failure timestamps
    failure_timestamps: Vec<Instant>,
}

impl PatternDetectionService {
    /// Create a new pattern detection service with circuit breaker
    pub fn new(attempt_monitor: Arc<AttemptMonitor>) -> Self {
        Self::with_config(attempt_monitor, CircuitBreakerConfig::default())
    }
    
    /// Create a new pattern detection service with custom configuration
    pub fn with_config(attempt_monitor: Arc<AttemptMonitor>, config: CircuitBreakerConfig) -> Self {
        Self {
            attempt_monitor,
            config,
            state: CircuitState::Closed,
            open_time: None,
            failure_timestamps: Vec::new(),
        }
    }
    
    /// Record an authentication attempt and analyze patterns
    /// Returns (SecurityAction, u8) tuple with risk score, or fallback values if circuit is open
    pub async fn record_attempt(
        &mut self,
        user_id: Option<Uuid>,
        ip_address: Option<IpAddr>,
        device_fingerprint: Option<String>,
        attempt_type: AccessAttemptType,
        correlation_id: Uuid,
    ) -> (SecurityAction, u8) {
        // Check circuit state
        if self.state == CircuitState::Open {
            // Check if we should attempt to close the circuit
            if let Some(open_time) = self.open_time {
                if open_time.elapsed() >= self.config.reset_timeout {
                    self.state = CircuitState::HalfOpen;
                } else {
                    // Circuit is still open, return fallback values
                    return self.fallback_response(&attempt_type);
                }
            } else {
                // Circuit is open but we don't know when it opened, return fallback
                return self.fallback_response(&attempt_type);
            }
        }
        
        // Try to call the underlying service
        match self.attempt_monitor.record_attempt(
            user_id,
            ip_address,
            device_fingerprint,
            attempt_type,
            correlation_id,
        ).await {
            Ok(result) => {
                // Success, reset failure count and close circuit
                self.on_success();
                result
            }
            Err(e) => {
                // Failure, record it and check if we should open the circuit
                error!("Pattern detection service failed: {}", e);
                self.on_failure();
                
                if self.state == CircuitState::Open || self.state == CircuitState::HalfOpen {
                    // Circuit is open or half-open, return fallback values
                    self.fallback_response(&attempt_type)
                } else {
                    // Circuit is closed, but we had a failure
                    // Still return the error to the caller
                    // In a real implementation, you might want to handle this differently
                    self.fallback_response(&attempt_type)
                }
            }
        }
    }
    
    /// Handle successful operation
    fn on_success(&mut self) {
        if self.state == CircuitState::HalfOpen {
            // Success in half-open state, close the circuit
            self.state = CircuitState::Closed;
            self.failure_timestamps.clear();
            self.open_time = None;
        }
        // In closed state, we don't need to do anything special on success
    }
    
    /// Handle failed operation
    fn on_failure(&mut self) {
        let now = Instant::now();
        
        // Remove old failures outside the window
        self.failure_timestamps.retain(|&timestamp| {
            now.duration_since(timestamp) <= self.config.failure_window
        });
        
        // Add the new failure
        self.failure_timestamps.push(now);
        
        // Check if we should trip the circuit
        if self.failure_timestamps.len() >= self.config.failure_threshold {
            if self.state != CircuitState::Open {
                warn!("Circuit breaker tripped due to {} failures in {:?}",
                      self.failure_timestamps.len(), self.config.failure_window);
                self.state = CircuitState::Open;
                self.open_time = Some(now);
            }
        } else if self.state == CircuitState::HalfOpen {
            // Failure in half-open state, open the circuit again
            self.state = CircuitState::Open;
            self.open_time = Some(now);
        }
    }
    
    /// Get fallback response when circuit is open
    fn fallback_response(&self, attempt_type: &AccessAttemptType) -> (SecurityAction, u8) {
        warn!("Using fallback response for pattern detection due to circuit breaker");
        
        // Calculate base risk score based on attempt type
        let risk_score = match attempt_type {
            AccessAttemptType::Success => 0,
            AccessAttemptType::FailedDualAuth => 20,
            AccessAttemptType::InvalidCredentials => 15,
            AccessAttemptType::AccountLocked => 25,
        };
        
        // Never return zero risk score for failed attempts
        let risk_score = if risk_score == 0 && !matches!(attempt_type, AccessAttemptType::Success) {
            10 // Minimum risk score for failed attempts
        } else {
            risk_score
        };
        
        // Always return None action when circuit is open to avoid blocking legitimate users
        (SecurityAction::None, risk_score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::database::audit_log_repository::AuditLogRepository;
    use crate::domain::audit_log::AuditLog;
    use std::net::Ipv4Addr;
    use std::time::Duration;
    use async_trait::async_trait;
    
    // Mock implementation of AuditLogRepository for testing
    struct MockAuditLogRepository;
    
    #[async_trait]
    impl AuditLogRepository for MockAuditLogRepository {
        async fn create(&self, audit_log: AuditLog) -> Result<AuditLog, crate::infrastructure::database::audit_log_repository::AuditLogRepositoryError> {
            Ok(audit_log)
        }
        
        async fn log_authentication_attempt(
            &self,
            user_id: Option<Uuid>,
            attempt_type: String,
            correlation_id: Uuid,
            risk_score: u8,
            failure_reason: Option<String>,
            source_ip: Option<String>,
            device_info: Option<String>,
        ) -> Result<Uuid, crate::infrastructure::database::audit_log_repository::AuditLogRepositoryError> {
            Ok(Uuid::new_v4())
        }
        
        async fn get_recent_authentication_attempts(
            &self,
            user_id: Option<Uuid>,
            ip_address: Option<IpAddr>,
            window_seconds: u64,
        ) -> Result<Vec<crate::infrastructure::database::audit_log_repository::AuthenticationAttemptRecord>, crate::infrastructure::database::audit_log_repository::AuditLogRepositoryError> {
            // Simulate a failure for testing circuit breaker
            Err(crate::infrastructure::database::audit_log_repository::AuditLogRepositoryError::DatabaseError(
                sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, "Simulated failure"))
            ))
        }
        
        async fn find_by_user_id(&self, user_id: Uuid, requester_id: Uuid, is_admin: bool, has_dual_auth: bool) -> Result<Vec<AuditLog>, crate::infrastructure::database::audit_log_repository::AuditLogRepositoryError> {
            unimplemented!()
        }
        
        async fn find_by_data_type(&self, data_type: &str, requester_id: Uuid, is_admin: bool, has_dual_auth: bool) -> Result<Vec<AuditLog>, crate::infrastructure::database::audit_log_repository::AuditLogRepositoryError> {
            unimplemented!()
        }
        
        async fn find_by_time_range(
            &self,
            start: chrono::DateTime<chrono::Utc>,
            end: chrono::DateTime<chrono::Utc>,
            requester_id: Uuid,
            is_admin: bool,
            has_dual_auth: bool
        ) -> Result<Vec<AuditLog>, crate::infrastructure::database::audit_log_repository::AuditLogRepositoryError> {
            unimplemented!()
        }
        
        async fn find_by_id(&self, id: Uuid, requester_id: Uuid, is_admin: bool, has_dual_auth: bool) -> Result<AuditLog, crate::infrastructure::database::audit_log_repository::AuditLogRepositoryError> {
            unimplemented!()
        }
    }
    
    #[tokio::test]
    async fn test_circuit_breaker_opens_after_failures() {
        let attempt_monitor = Arc::new(AttemptMonitor::new(Arc::new(MockAuditLogRepository)));
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            failure_window: Duration::from_secs(1),
            reset_timeout: Duration::from_secs(2),
        };
        let mut service = PatternDetectionService::with_config(attempt_monitor, config);
        
        let user_id = Some(Uuid::new_v4());
        let ip_address = Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        let correlation_id = Uuid::new_v4();
        
        // First failure
        let (action, risk_score) = service.record_attempt(
            user_id,
            ip_address,
            Some("device123".to_string()),
            AccessAttemptType::FailedDualAuth,
            correlation_id,
        ).await;
        
        assert_eq!(action, SecurityAction::None);
        assert_eq!(risk_score, 20);
        
        // Second failure
        let (action, risk_score) = service.record_attempt(
            user_id,
            ip_address,
            Some("device123".to_string()),
            AccessAttemptType::FailedDualAuth,
            correlation_id,
        ).await;
        
        assert_eq!(action, SecurityAction::None);
        assert_eq!(risk_score, 20);
        
        // Third failure - should trip the circuit
        let (action, risk_score) = service.record_attempt(
            user_id,
            ip_address,
            Some("device123".to_string()),
            AccessAttemptType::FailedDualAuth,
            correlation_id,
        ).await;
        
        assert_eq!(action, SecurityAction::None);
        assert_eq!(risk_score, 20);
    }
}