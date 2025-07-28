//! Payment audit logger for the invoicing module
//!
//! This module contains the functionality for logging all payment status changes for audit purposes.

use crate::domain::status::{StatusTransition, StatusOverride};
use crate::domain::payment::Invoice;
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Error types for audit logging operations
#[derive(Debug, thiserror::Error)]
pub enum AuditError {
    #[error("Data access error: {0}")]
    DataAccessError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Encryption error: {0}")]
    EncryptionError(String),
}

/// Audit log entry for payment status changes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentAuditLog {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub log_type: AuditLogType,
    pub details: serde_json::Value,
    pub user_id: Option<Uuid>, // None for automated changes
    pub timestamp: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

/// Types of audit log entries
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditLogType {
    StatusTransition,
    StatusOverride,
    ManualUpdate,
    AutomatedUpdate,
    PaymentProcessed,
    PaymentFailed,
}

/// Audit log details for status transitions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatusTransitionDetails {
    pub from_status: String,
    pub to_status: String,
    pub reason: Option<String>,
}

/// Audit log details for status overrides
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatusOverrideDetails {
    pub overridden_status: String,
    pub override_reason: String,
}

#[async_trait]
pub trait AuditRepository {
    async fn save_log(&self, log: &PaymentAuditLog) -> Result<(), AuditError>;
    async fn get_logs_for_invoice(&self, invoice_id: Uuid) -> Result<Vec<PaymentAuditLog>, AuditError>;
    async fn get_logs_for_user(&self, user_id: Uuid) -> Result<Vec<PaymentAuditLog>, AuditError>;
    async fn get_logs_in_time_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<PaymentAuditLog>, AuditError>;
}

/// Payment audit logger
pub struct PaymentAuditLogger {
    audit_repository: Box<dyn AuditRepository>,
    encryption_enabled: bool,
}

impl PaymentAuditLogger {
    pub fn new(audit_repository: Box<dyn AuditRepository>, encryption_enabled: bool) -> Self {
        Self {
            audit_repository,
            encryption_enabled,
        }
    }

    /// Log a status transition
    pub async fn log_status_transition(
        &self,
        transition: &StatusTransition,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<(), AuditError> {
        let details = StatusTransitionDetails {
            from_status: format!("{:?}", transition.from_status),
            to_status: format!("{:?}", transition.to_status),
            reason: transition.transition_reason.clone(),
        };

        let log = PaymentAuditLog {
            id: Uuid::new_v4(),
            invoice_id: transition.invoice_id,
            log_type: AuditLogType::StatusTransition,
            details: serde_json::to_value(details).map_err(|e| AuditError::SerializationError(e.to_string()))?,
            user_id: if transition.transitioned_by.is_nil() { 
                None // System/user automated transition
            } else { 
                Some(transition.transitioned_by) 
            },
            timestamp: transition.timestamp,
            ip_address,
            user_agent,
        };

        self.save_audit_log(&log).await
    }

    /// Log a status override
    pub async fn log_status_override(
        &self,
        override_record: &StatusOverride,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<(), AuditError> {
        let details = StatusOverrideDetails {
            overridden_status: format!("{:?}", override_record.overridden_status),
            override_reason: override_record.override_reason.clone(),
        };

        let log = PaymentAuditLog {
            id: Uuid::new_v4(),
            invoice_id: override_record.invoice_id,
            log_type: AuditLogType::StatusOverride,
            details: serde_json::to_value(details).map_err(|e| AuditError::SerializationError(e.to_string()))?,
            user_id: Some(override_record.overridden_by),
            timestamp: override_record.timestamp,
            ip_address,
            user_agent,
        };

        self.save_audit_log(&log).await
    }

    /// Log a manual invoice update
    pub async fn log_manual_update(
        &self,
        invoice: &Invoice,
        user_id: Uuid,
        changes: serde_json::Value,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<(), AuditError> {
        let log = PaymentAuditLog {
            id: Uuid::new_v4(),
            invoice_id: invoice.id,
            log_type: AuditLogType::ManualUpdate,
            details: changes,
            user_id: Some(user_id),
            timestamp: Utc::now(),
            ip_address,
            user_agent,
        };

        self.save_audit_log(&log).await
    }

    /// Log an automated update
    pub async fn log_automated_update(
        &self,
        invoice: &Invoice,
        update_type: String,
        details: serde_json::Value,
    ) -> Result<(), AuditError> {
        let log = PaymentAuditLog {
            id: Uuid::new_v4(),
            invoice_id: invoice.id,
            log_type: AuditLogType::AutomatedUpdate,
            details,
            user_id: None, // Automated process
            timestamp: Utc::now(),
            ip_address: None,
            user_agent: None,
        };

        self.save_audit_log(&log).await
    }

    /// Log a successful payment
    pub async fn log_payment_processed(
        &self,
        invoice: &Invoice,
        payment_provider: &str,
        payment_intent_id: &str,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<(), AuditError> {
        let details = serde_json::json!({
            "payment_provider": payment_provider,
            "payment_intent_id": payment_intent_id,
            "amount": invoice.total_amount.to_string(),
        });

        let log = PaymentAuditLog {
            id: Uuid::new_v4(),
            invoice_id: invoice.id,
            log_type: AuditLogType::PaymentProcessed,
            details,
            user_id: None, // Payment processed by system/customer
            timestamp: Utc::now(),
            ip_address,
            user_agent,
        };

        self.save_audit_log(&log).await
    }

    /// Log a failed payment
    pub async fn log_payment_failed(
        &self,
        invoice: &Invoice,
        payment_provider: &str,
        payment_intent_id: &str,
        error_message: &str,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<(), AuditError> {
        let details = serde_json::json!({
            "payment_provider": payment_provider,
            "payment_intent_id": payment_intent_id,
            "error_message": error_message,
            "amount": invoice.total_amount.to_string(),
        });

        let log = PaymentAuditLog {
            id: Uuid::new_v4(),
            invoice_id: invoice.id,
            log_type: AuditLogType::PaymentFailed,
            details,
            user_id: None, // Payment failed by system/customer
            timestamp: Utc::now(),
            ip_address,
            user_agent,
        };

        self.save_audit_log(&log).await
    }

    /// Save an audit log entry, with optional encryption
    async fn save_audit_log(&self, log: &PaymentAuditLog) -> Result<(), AuditError> {
        let log_to_save = if self.encryption_enabled {
            // In a real implementation, we would encrypt sensitive fields
            // For now, we'll just save the log as-is
            log.clone()
        } else {
            log.clone()
        };

        self.audit_repository.save_log(&log_to_save).await
    }

    /// Get audit logs for an invoice
    pub async fn get_logs_for_invoice(&self, invoice_id: Uuid) -> Result<Vec<PaymentAuditLog>, AuditError> {
        self.audit_repository.get_logs_for_invoice(invoice_id).await
    }

    /// Get audit logs for a user
    pub async fn get_logs_for_user(&self, user_id: Uuid) -> Result<Vec<PaymentAuditLog>, AuditError> {
        self.audit_repository.get_logs_for_user(user_id).await
    }

    /// Get audit logs in a time range
    pub async fn get_logs_in_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<PaymentAuditLog>, AuditError> {
        self.audit_repository.get_logs_in_time_range(start, end).await
    }
}

/// Implementation of AuditRepository using a database
pub struct DatabaseAuditRepository {
    // In a real implementation, this would contain database connection details
}

#[async_trait]
impl AuditRepository for DatabaseAuditRepository {
    async fn save_log(&self, _log: &PaymentAuditLog) -> Result<(), AuditError> {
        // In a real implementation, this would save to a database
        // For now, we'll just print to stdout as a mock
        println!("Saving audit log: {:?}", _log);
        Ok(())
    }

    async fn get_logs_for_invoice(&self, _invoice_id: Uuid) -> Result<Vec<PaymentAuditLog>, AuditError> {
        // In a real implementation, this would query the database
        Ok(vec![])
    }

    async fn get_logs_for_user(&self, _user_id: Uuid) -> Result<Vec<PaymentAuditLog>, AuditError> {
        // In a real implementation, this would query the database
        Ok(vec![])
    }

    async fn get_logs_in_time_range(&self, _start: DateTime<Utc>, _end: DateTime<Utc>) -> Result<Vec<PaymentAuditLog>, AuditError> {
        // In a real implementation, this would query the database
        Ok(vec![])
    }
}

/// Secure audit logger with encryption
pub struct SecureAuditLogger {
    audit_logger: PaymentAuditLogger,
    // In a real implementation, this would contain encryption keys/secrets
}

impl SecureAuditLogger {
    pub fn new(audit_logger: PaymentAuditLogger) -> Self {
        Self { audit_logger }
    }

    /// Log a status transition with encryption
    pub async fn log_status_transition(
        &self,
        transition: &StatusTransition,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<(), AuditError> {
        // In a real implementation, we would encrypt sensitive data before logging
        self.audit_logger.log_status_transition(transition, ip_address, user_agent).await
    }

    /// Log a status override with encryption
    pub async fn log_status_override(
        &self,
        override_record: &StatusOverride,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<(), AuditError> {
        // In a real implementation, we would encrypt sensitive data before logging
        self.audit_logger.log_status_override(override_record, ip_address, user_agent).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use chrono::Utc;
    use uuid::Uuid;

    struct MockAuditRepository;

    #[async_trait]
    impl AuditRepository for MockAuditRepository {
        async fn save_log(&self, _log: &PaymentAuditLog) -> Result<(), AuditError> {
            Ok(())
        }

        async fn get_logs_for_invoice(&self, _invoice_id: Uuid) -> Result<Vec<PaymentAuditLog>, AuditError> {
            Ok(vec![])
        }

        async fn get_logs_for_user(&self, _user_id: Uuid) -> Result<Vec<PaymentAuditLog>, AuditError> {
            Ok(vec![])
        }

        async fn get_logs_in_time_range(&self, _start: DateTime<Utc>, _end: DateTime<Utc>) -> Result<Vec<PaymentAuditLog>, AuditError> {
            Ok(vec![])
        }
    }

    #[tokio::test]
    async fn test_audit_logger_creation() {
        let audit_logger = PaymentAuditLogger::new(
            Box::new(MockAuditRepository),
            false, // encryption_enabled
        );

        assert!(!audit_logger.encryption_enabled);
    }

    #[tokio::test]
    async fn test_status_transition_logging() {
        let audit_logger = PaymentAuditLogger::new(
            Box::new(MockAuditRepository),
            false, // encryption_enabled
        );

        let transition = StatusTransition::new(
            Uuid::new_v4(),
            crate::domain::status::PaymentStatus::Draft,
            crate::domain::status::PaymentStatus::Sent,
            Some("Sending invoice".to_string()),
            Uuid::new_v4(),
        ).unwrap();

        let result = audit_logger.log_status_transition(
            &transition,
            Some("127.0.0.1".to_string()),
            Some("test-agent".to_string()),
        ).await;

        assert!(result.is_ok());
    }
}