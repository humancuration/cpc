//! Authentication Attempt Monitoring
//!
//! This module provides functionality for monitoring authentication attempts,
//! detecting suspicious patterns, and calculating risk scores for HIPAA compliance.
//!
//! Note: This implementation now uses database-backed pattern detection
//! instead of in-memory storage to support distributed environments.
//! See AUDIT_LOGGING_IMPLEMENTATION.md for details.

use crate::domain::audit_log::{AccessAttemptType, AuditLog};
use crate::infrastructure::database::audit_log_repository::{AuditLogRepository, AuthenticationAttemptRecord};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tracing::warn;

/// Configuration for the attempt monitor
#[derive(Debug, Clone)]
pub struct AttemptMonitorConfig {
    /// Maximum number of attempts to track per user
    pub max_attempts_per_user: usize,
    
    /// Time window for detecting rapid attempts (in seconds)
    pub rapid_attempt_window_secs: u64,
    
    /// Threshold for marking attempts as rapid
    pub rapid_attempt_threshold: usize,
    
    /// Risk score increment for rapid attempts
    pub rapid_attempt_risk_increment: u8,
    
    /// Risk score increment for multiple failed attempts
    pub failed_attempt_risk_increment: u8,
    
    /// Risk score increment for geographic anomalies (if implemented)
    pub geographic_anomaly_risk_increment: u8,
}

impl Default for AttemptMonitorConfig {
    fn default() -> Self {
        Self {
            max_attempts_per_user: 100,
            rapid_attempt_window_secs: 60, // 1 minute
            rapid_attempt_threshold: 5,
            rapid_attempt_risk_increment: 20,
            failed_attempt_risk_increment: 15,
            geographic_anomaly_risk_increment: 25,
        }
    }
}

/// Record of an authentication attempt for pattern detection
#[derive(Debug, Clone)]
pub struct AuthenticationAttemptRecord {
    pub user_id: Option<Uuid>,
    pub ip_address: Option<IpAddr>,
    pub device_fingerprint: Option<String>,
    pub attempt_type: AccessAttemptType,
    pub timestamp: DateTime<Utc>,
    pub correlation_id: Uuid,
}

/// Security action recommended based on detected patterns
#[derive(Debug, Clone)]
pub enum SecurityAction {
    /// No action needed
    None,
    
    /// Log warning about suspicious activity
    Warn,
    
    /// Temporarily lock account
    TemporaryLock,
    
    /// Notify security team
    NotifySecurity,
    
    /// Require additional verification
    RequireAdditionalVerification,
}

/// Monitors authentication attempts and detects suspicious patterns
pub struct AttemptMonitor {
    /// Repository for accessing audit logs
    audit_log_repository: Arc<dyn AuditLogRepository>,
    
    /// Configuration for the monitor
    config: AttemptMonitorConfig,
}

impl AttemptMonitor {
    /// Create a new attempt monitor with audit log repository
    pub fn new(audit_log_repository: Arc<dyn AuditLogRepository>) -> Self {
        Self::with_config(audit_log_repository, AttemptMonitorConfig::default())
    }
    
    /// Create a new attempt monitor with audit log repository and custom configuration
    pub fn with_config(audit_log_repository: Arc<dyn AuditLogRepository>, config: AttemptMonitorConfig) -> Self {
        Self {
            audit_log_repository,
            config,
        }
    }
    
    /// Record an authentication attempt and analyze patterns
    pub async fn record_attempt(
        &self,
        user_id: Option<Uuid>,
        ip_address: Option<IpAddr>,
        device_fingerprint: Option<String>,
        attempt_type: AccessAttemptType,
        correlation_id: Uuid,
    ) -> (SecurityAction, u8) {
        // Calculate risk score based on patterns
        let risk_score = self.calculate_risk_score(
            user_id,
            ip_address,
            device_fingerprint.clone(),
            &attempt_type,
        ).await;
        
        // Analyze patterns and return security action needed along with risk score
        let action = self.detect_suspicious_patterns(
            user_id,
            ip_address,
            device_fingerprint,
            &attempt_type,
            risk_score
        ).await;
        
        (action, risk_score)
    }
    
    /// Detect suspicious patterns in authentication attempts
    async fn detect_suspicious_patterns(
        &self,
        user_id: Option<Uuid>,
        ip_address: Option<IpAddr>,
        device_fingerprint: Option<String>,
        attempt_type: &AccessAttemptType,
        risk_score: u8,
    ) -> SecurityAction {
        // Determine security action based on risk score
        match risk_score {
            0..=30 => SecurityAction::None,
            31..=60 => SecurityAction::Warn,
            61..=80 => SecurityAction::RequireAdditionalVerification,
            81..=100 => SecurityAction::TemporaryLock,
            _ => {
                warn!("High risk authentication attempt detected for user {:?} from IP {:?}", user_id, ip_address);
                SecurityAction::NotifySecurity
            }
        }
    }
    
    /// Calculate risk score for an authentication attempt
    pub async fn calculate_risk_score(
        &self,
        user_id: Option<Uuid>,
        ip_address: Option<IpAddr>,
        device_fingerprint: Option<String>,
        attempt_type: &AccessAttemptType,
    ) -> u8 {
        let mut risk_score = 0u8;
        
        // Base risk score based on attempt type
        match attempt_type {
            AccessAttemptType::Success => risk_score = 0,
            AccessAttemptType::FailedDualAuth => risk_score = risk_score.saturating_add(20),
            AccessAttemptType::InvalidCredentials => risk_score = risk_score.saturating_add(15),
            AccessAttemptType::AccountLocked => risk_score = risk_score.saturating_add(25),
        }
        
        // Additional risk factors based on patterns from database
        if let Some(uid) = user_id {
            // Get recent attempts for this user (last 5 minutes)
            match self.audit_log_repository.get_recent_authentication_attempts(
                Some(uid),
                None,
                300, // 5 minutes
            ).await {
                Ok(attempts) => {
                    let recent_failed_attempts: usize = attempts
                        .iter()
                        .filter(|a| !matches!(a.attempt_type, AccessAttemptType::Success))
                        .count();
                    
                    // Increase risk score for multiple recent failed attempts
                    risk_score = risk_score.saturating_add(
                        (recent_failed_attempts as u8) * self.config.failed_attempt_risk_increment
                    );
                }
                Err(e) => {
                    warn!("Failed to get recent authentication attempts: {}", e);
                    // Fallback: use base risk score only
                }
            }
        }
        
        // Check for rapid succession of attempts from same IP
        if let Some(ip) = ip_address {
            match self.audit_log_repository.get_recent_authentication_attempts(
                None,
                Some(ip),
                self.config.rapid_attempt_window_secs,
            ).await {
                Ok(attempts) => {
                    // Check for rapid succession of attempts
                    if attempts.len() >= self.config.rapid_attempt_threshold {
                        risk_score = risk_score.saturating_add(self.config.rapid_attempt_risk_increment);
                    }
                    
                    // Check for multiple failed attempts from same IP
                    let failed_attempts: usize = attempts
                        .iter()
                        .filter(|a| !matches!(a.attempt_type, AccessAttemptType::Success))
                        .count();
                    
                    if failed_attempts > 5 {
                        risk_score = risk_score.saturating_add(
                            (failed_attempts as u8) * self.config.failed_attempt_risk_increment / 2
                        );
                    }
                }
                Err(e) => {
                    warn!("Failed to get recent authentication attempts by IP: {}", e);
                    // Fallback: use base risk score only
                }
            }
        }
        
        // Cap risk score at 100
        risk_score.min(100)
    }
    
    /// Calculate risk score for an authentication attempt

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod tests {
        use super::*;
        use std::net::Ipv4Addr;
        
        // Mock implementation of AuditLogRepository for testing
        struct MockAuditLogRepository;
        
        #[async_trait::async_trait]
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
            ) -> Result<Vec<AuthenticationAttemptRecord>, crate::infrastructure::database::audit_log_repository::AuditLogRepositoryError> {
                Ok(Vec::new())
            }
            
            async fn find_by_user_id(&self, user_id: Uuid, requester_id: Uuid, is_admin: bool, has_dual_auth: bool) -> Result<Vec<AuditLog>, crate::infrastructure::database::audit_log_repository::AuditLogRepositoryError> {
                unimplemented!()
            }
            
            async fn find_by_data_type(&self, data_type: &str, requester_id: Uuid, is_admin: bool, has_dual_auth: bool) -> Result<Vec<AuditLog>, crate::infrastructure::database::audit_log_repository::AuditLogRepositoryError> {
                unimplemented!()
            }
            
            async fn find_by_time_range(
                &self,
                start: DateTime<Utc>,
                end: DateTime<Utc>,
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
        async fn test_record_successful_attempt() {
            let monitor = AttemptMonitor::new(Arc::new(MockAuditLogRepository));
            let user_id = Some(Uuid::new_v4());
            let ip_address = Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
            let correlation_id = Uuid::new_v4();
            
            let (action, risk_score) = monitor.record_attempt(
                user_id,
                ip_address,
                Some("device123".to_string()),
                AccessAttemptType::Success,
                correlation_id,
            ).await;
            
            assert_eq!(action, SecurityAction::None);
            assert_eq!(risk_score, 0);
        }
        
        #[tokio::test]
        async fn test_record_failed_attempt() {
            let monitor = AttemptMonitor::new(Arc::new(MockAuditLogRepository));
            let user_id = Some(Uuid::new_v4());
            let ip_address = Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
            let correlation_id = Uuid::new_v4();
            
            let (action, risk_score) = monitor.record_attempt(
                user_id,
                ip_address,
                Some("device123".to_string()),
                AccessAttemptType::FailedDualAuth,
                correlation_id,
            ).await;
            
            assert_eq!(action, SecurityAction::None);
            assert_eq!(risk_score, 20);
        }
        
        #[tokio::test]
        async fn test_calculate_risk_score() {
            let monitor = AttemptMonitor::new(Arc::new(MockAuditLogRepository));
            let user_id = Some(Uuid::new_v4());
            let ip_address = Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
            
            let risk_score = monitor.calculate_risk_score(
                user_id,
                ip_address,
                Some("device123".to_string()),
                &AccessAttemptType::Success,
            ).await;
            
            assert_eq!(risk_score, 0);
            
            let risk_score = monitor.calculate_risk_score(
                user_id,
                ip_address,
                Some("device123".to_string()),
                &AccessAttemptType::FailedDualAuth,
            ).await;
            
            assert_eq!(risk_score, 20);
        }