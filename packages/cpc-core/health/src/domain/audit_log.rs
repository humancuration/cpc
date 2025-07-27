//! Audit Log Domain Model
//!
//! This module contains the domain model for audit logs, which are required for HIPAA compliance.
//! Audit logs track all access to health data, including who accessed what data, when, and why.

use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Represents an audit log entry for tracking health data access
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditLog {
    /// Unique identifier for the audit log entry
    pub id: Uuid,
    
    /// User who accessed the data (None for research/anonymized access)
    pub user_id: Option<Uuid>,
    
    /// Timestamp when the data was accessed
    pub accessed_at: DateTime<Utc>,
    
    /// Type of data that was accessed (e.g., "VitalSign", "HealthCondition")
    pub data_type: String,
    
    /// ID of the specific data record that was accessed
    pub data_id: Uuid,
    
    /// Type of access (e.g., "Read", "Write", "Delete")
    pub access_type: String,
    
    /// Purpose of access (e.g., "UserView", "Research", "DataSync")
    pub purpose: String,
    
    /// IP address of the requester
    pub source_ip: Option<IpAddr>,
    
    /// Device information of the requester
    pub device_info: Option<String>,
    
    /// Timestamp when the audit log entry was created
    pub created_at: DateTime<Utc>,
    
    /// Type of access attempt (Success, FailedDualAuth, InvalidCredentials, AccountLocked)
    pub attempt_type: String,
    
    /// Correlation ID to group related access attempts
    pub attempt_correlation_id: Uuid,
    
    /// Risk score for security monitoring (0-100)
    pub risk_score: u8,
    
    /// Specific reason for failure (if applicable)
    pub failure_reason: Option<String>,
}
impl AuditLog {
    /// Create a new audit log entry
    pub fn new(
        user_id: Option<Uuid>,
        data_type: &str,
        data_id: Uuid,
        access_type: &str,
        purpose: &str,
        source_ip: Option<IpAddr>,
        device_info: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            accessed_at: now,
            data_type: data_type.to_string(),
            data_id,
            access_type: access_type.to_string(),
            purpose: purpose.to_string(),
            source_ip,
            device_info,
            created_at: now,
            attempt_type: "Success".to_string(), // Default to Success for backward compatibility
            attempt_correlation_id: Uuid::new_v4(), // Generate a new correlation ID
            risk_score: 0, // Default to 0 risk
            failure_reason: None, // No failure reason by default
        }
    }
    
    /// Create a new authentication attempt log entry
    pub fn new_auth_attempt(
        user_id: Option<Uuid>,
        attempt_type: AccessAttemptType,
        correlation_id: Uuid,
        risk_score: u8,
        failure_reason: Option<String>,
        source_ip: Option<IpAddr>,
        device_info: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            accessed_at: now,
            data_type: "AuthenticationAttempt".to_string(),
            data_id: Uuid::nil(), // No specific data ID for authentication attempts
            access_type: "Authentication".to_string(),
            purpose: "SecurityMonitoring".to_string(),
            source_ip,
            device_info,
            created_at: now,
            attempt_type: attempt_type.as_str().to_string(),
            attempt_correlation_id: correlation_id,
            risk_score,
            failure_reason,
        }
    }
    
    /// Validate the audit log entry
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.data_type.is_empty() {
            return Err(ValidationError::InvalidDataType);
        }
        
        if self.access_type.is_empty() {
            return Err(ValidationError::InvalidAccessType);
        }
        
        if self.purpose.is_empty() {
            return Err(ValidationError::InvalidPurpose);
        }
        
        // Validate risk score is within range
        if self.risk_score > 100 {
            return Err(ValidationError::InvalidRiskScore);
        }
        
        Ok(())
    }
}

/// Validation errors for audit logs
#[derive(Debug, Clone, thiserror::Error)]
pub enum ValidationError {
    #[error("Data type cannot be empty")]
    InvalidDataType,
    
    #[error("Access type cannot be empty")]
    InvalidAccessType,
    
    #[error("Purpose cannot be empty")]
    InvalidPurpose,
    
    #[error("Risk score must be between 0 and 100")]
    InvalidRiskScore,
}

/// Purpose codes for audit log entries
#[derive(Debug, Clone, PartialEq)]
pub enum AuditPurpose {
    /// User viewing their own data
    UserView,
    
    /// Healthcare provider access
    ProviderAccess,
    
    /// Research access (anonymized)
    Research,
    
    /// Data synchronization with wearables
    DataSync,
    
    /// Administrative access
    Admin,
    
    /// System maintenance
    Maintenance,
}

impl AuditPurpose {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditPurpose::UserView => "UserView",
            AuditPurpose::ProviderAccess => "ProviderAccess",
            AuditPurpose::Research => "Research",
            AuditPurpose::DataSync => "DataSync",
            AuditPurpose::Admin => "Admin",
            AuditPurpose::Maintenance => "Maintenance",
        }
    }
}

/// Types of access attempts for authentication logging
#[derive(Debug, Clone, PartialEq)]
pub enum AccessAttemptType {
    /// Successful authentication
    Success,
    
    /// Failed due to missing dual authentication
    FailedDualAuth,
    
    /// Failed due to invalid credentials
    InvalidCredentials,
    
    /// Failed due to account being locked
    AccountLocked,
}

impl AccessAttemptType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccessAttemptType::Success => "Success",
            AccessAttemptType::FailedDualAuth => "FailedDualAuth",
            AccessAttemptType::InvalidCredentials => "InvalidCredentials",
            AccessAttemptType::AccountLocked => "AccountLocked",
        }
    }
}

/// Access types for audit log entries
#[derive(Debug, Clone, PartialEq)]
pub enum AccessType {
    Read,
    Write,
    Delete,
    Export,
}

impl AccessType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccessType::Read => "Read",
            AccessType::Write => "Write",
            AccessType::Delete => "Delete",
            AccessType::Export => "Export",
        }
    }
}