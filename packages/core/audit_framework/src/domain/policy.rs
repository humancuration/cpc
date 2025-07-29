//! Retention/compliance rules
//! 
//! This module defines compliance policies and regulations.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Regulation types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Regulation {
    /// Health Insurance Portability and Accountability Act
    Hipaa,
    /// General Data Protection Regulation
    Gdpr,
    /// California Consumer Privacy Act
    Ccpa,
    /// Payment Card Industry Data Security Standard
    PciDss,
    /// Sarbanes-Oxley Act
    Sox,
    /// International Financial Reporting Standards
    Ifrs,
}

impl fmt::Display for Regulation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Regulation::Hipaa => write!(f, "HIPAA"),
            Regulation::Gdpr => write!(f, "GDPR"),
            Regulation::Ccpa => write!(f, "CCPA"),
            Regulation::PciDss => write!(f, "PCI DSS"),
            Regulation::Sox => write!(f, "SOX"),
            Regulation::Ifrs => write!(f, "IFRS"),
        }
    }
}

/// Retention policy for audit events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Regulation this policy applies to
    pub regulation: Regulation,
    
    /// Minimum retention period in days
    pub min_retention_days: i32,
    
    /// Maximum retention period in days (None for indefinite)
    pub max_retention_days: Option<i32>,
    
    /// Whether encryption is required
    pub encryption_required: bool,
    
    /// Whether anonymization is allowed
    pub anonymization_allowed: bool,
}

impl RetentionPolicy {
    /// Create a new retention policy
    pub fn new(
        regulation: Regulation,
        min_retention_days: i32,
        max_retention_days: Option<i32>,
        encryption_required: bool,
        anonymization_allowed: bool,
    ) -> Self {
        Self {
            regulation,
            min_retention_days,
            max_retention_days,
            encryption_required,
            anonymization_allowed,
        }
    }
}

/// Compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// Whether the system is compliant
    pub success: bool,
    
    /// Regulation this report applies to
    pub regulation: Regulation,
    
    /// Details of the compliance check
    pub details: String,
    
    /// Timestamp of the report
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ComplianceReport {
    /// Create a new compliance report
    pub fn new(success: bool, regulation: Regulation, details: String) -> Self {
        Self {
            success,
            regulation,
            details,
            timestamp: chrono::Utc::now(),
        }
    }
}