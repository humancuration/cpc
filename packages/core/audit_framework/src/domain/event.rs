//! AuditEvent structure
//! 
//! This module defines the core audit event structure used throughout the system.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::fmt;

/// Audit action type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditAction {
    /// Create a new resource
    Create,
    /// Read an existing resource
    Read,
    /// Update an existing resource
    Update,
    /// Delete an existing resource
    Delete,
    /// Export data
    Export,
    /// Import data
    Import,
    /// Login attempt
    Login,
    /// Logout
    Logout,
}

impl fmt::Display for AuditAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuditAction::Create => write!(f, "Create"),
            AuditAction::Read => write!(f, "Read"),
            AuditAction::Update => write!(f, "Update"),
            AuditAction::Delete => write!(f, "Delete"),
            AuditAction::Export => write!(f, "Export"),
            AuditAction::Import => write!(f, "Import"),
            AuditAction::Login => write!(f, "Login"),
            AuditAction::Logout => write!(f, "Logout"),
        }
    }
}

/// Purpose code for audit events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PurposeCode {
    /// User viewing their own data
    UserView,
    /// Provider access (e.g., healthcare provider)
    ProviderAccess,
    /// Research access (anonymized)
    Research,
    /// Data synchronization
    DataSync,
    /// Administrative access
    Admin,
    /// System maintenance
    Maintenance,
    /// Security monitoring
    SecurityMonitoring,
    /// Fraud detection
    FraudDetection,
}

impl fmt::Display for PurposeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PurposeCode::UserView => write!(f, "UserView"),
            PurposeCode::ProviderAccess => write!(f, "ProviderAccess"),
            PurposeCode::Research => write!(f, "Research"),
            PurposeCode::DataSync => write!(f, "DataSync"),
            PurposeCode::Admin => write!(f, "Admin"),
            PurposeCode::Maintenance => write!(f, "Maintenance"),
            PurposeCode::SecurityMonitoring => write!(f, "SecurityMonitoring"),
            PurposeCode::FraudDetection => write!(f, "FraudDetection"),
        }
    }
}

/// Standardized audit structure (extending consent_manager's implementation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Unique identifier for the audit event
    pub event_id: Uuid,
    
    /// User ID (None for anonymized research)
    pub user_id: Option<String>,
    
    /// Domain that the event belongs to (e.g., "health", "finance")
    pub domain: String,
    
    /// Action that was performed
    pub action: AuditAction,
    
    /// Target resource identifier
    pub target: String,
    
    /// Purpose of the action
    pub purpose: PurposeCode,
    
    /// Timestamp when the event occurred
    pub timestamp: DateTime<Utc>,
    
    /// Additional metadata
    pub metadata: serde_json::Value,
}

impl AuditEvent {
    /// Create a new audit event
    pub fn new(
        user_id: Option<String>,
        domain: String,
        action: AuditAction,
        target: String,
        purpose: PurposeCode,
        metadata: serde_json::Value,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            user_id,
            domain,
            action,
            target,
            purpose,
            timestamp: Utc::now(),
            metadata,
        }
    }
    
    /// Create a new read audit event
    pub fn new_read(
        user_id: Option<String>,
        domain: String,
        target: String,
        purpose: PurposeCode,
        metadata: serde_json::Value,
    ) -> Self {
        Self::new(user_id, domain, AuditAction::Read, target, purpose, metadata)
    }
    
    /// Create a new create audit event
    pub fn new_create(
        user_id: Option<String>,
        domain: String,
        target: String,
        purpose: PurposeCode,
        metadata: serde_json::Value,
    ) -> Self {
        Self::new(user_id, domain, AuditAction::Create, target, purpose, metadata)
    }
}

impl fmt::Display for AuditEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AuditEvent({}, {}, {}, {})",
            self.domain, self.action, self.target, self.event_id
        )
    }
}