//! Status domain models for the invoicing module
//!
//! This module contains the core business entities for payment status tracking.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Payment status with additional states
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentStatus {
    Draft,
    Sent,
    Viewed,
    Paid,
    Overdue,
    Partial,
    PaymentFailed,
    Pending,
}

/// Status transition record
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatusTransition {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub from_status: PaymentStatus,
    pub to_status: PaymentStatus,
    pub transition_reason: Option<String>,
    pub transitioned_by: Uuid, // User ID who made the transition
    pub timestamp: DateTime<Utc>,
}

/// Manual status override record
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatusOverride {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub overridden_status: PaymentStatus,
    pub override_reason: String,
    pub overridden_by: Uuid, // User ID who made the override
    pub timestamp: DateTime<Utc>,
}

/// Error types for status operations
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum StatusError {
    #[error("Invalid status transition: {0} -> {1}")]
    InvalidTransition(PaymentStatus, PaymentStatus),
    #[error("Status override requires reason")]
    OverrideRequiresReason,
    #[error("Data access error: {0}")]
    DataAccessError(String),
}

impl PaymentStatus {
    /// Check if a transition between statuses is valid
    pub fn can_transition_to(&self, target: &PaymentStatus) -> bool {
        match (self, target) {
            // Draft can transition to most states
            (PaymentStatus::Draft, _) => true,
            
            // Sent can transition to Viewed, Paid, Overdue, PaymentFailed, Pending
            (PaymentStatus::Sent, PaymentStatus::Viewed) => true,
            (PaymentStatus::Sent, PaymentStatus::Paid) => true,
            (PaymentStatus::Sent, PaymentStatus::Overdue) => true,
            (PaymentStatus::Sent, PaymentStatus::PaymentFailed) => true,
            (PaymentStatus::Sent, PaymentStatus::Pending) => true,
            (PaymentStatus::Sent, _) => false,
            
            // Viewed can transition to Paid, Overdue, PaymentFailed, Pending
            (PaymentStatus::Viewed, PaymentStatus::Paid) => true,
            (PaymentStatus::Viewed, PaymentStatus::Overdue) => true,
            (PaymentStatus::Viewed, PaymentStatus::PaymentFailed) => true,
            (PaymentStatus::Viewed, PaymentStatus::Pending) => true,
            (PaymentStatus::Viewed, _) => false,
            
            // Paid cannot transition to any other state
            (PaymentStatus::Paid, _) => false,
            
            // Overdue can transition to Paid, PaymentFailed, Pending
            (PaymentStatus::Overdue, PaymentStatus::Paid) => true,
            (PaymentStatus::Overdue, PaymentStatus::PaymentFailed) => true,
            (PaymentStatus::Overdue, PaymentStatus::Pending) => true,
            (PaymentStatus::Overdue, _) => false,
            
            // Partial can transition to Paid, Overdue, PaymentFailed, Pending
            (PaymentStatus::Partial, PaymentStatus::Paid) => true,
            (PaymentStatus::Partial, PaymentStatus::Overdue) => true,
            (PaymentStatus::Partial, PaymentStatus::PaymentFailed) => true,
            (PaymentStatus::Partial, PaymentStatus::Pending) => true,
            (PaymentStatus::Partial, _) => false,
            
            // PaymentFailed can transition to Pending, Paid (if re-attempted)
            (PaymentStatus::PaymentFailed, PaymentStatus::Pending) => true,
            (PaymentStatus::PaymentFailed, PaymentStatus::Paid) => true,
            (PaymentStatus::PaymentFailed, _) => false,
            
            // Pending can transition to Paid, PaymentFailed
            (PaymentStatus::Pending, PaymentStatus::Paid) => true,
            (PaymentStatus::Pending, PaymentStatus::PaymentFailed) => true,
            (PaymentStatus::Pending, _) => false,
        }
    }

    /// Get a list of valid next statuses for this status
    pub fn valid_next_statuses(&self) -> Vec<PaymentStatus> {
        match self {
            PaymentStatus::Draft => vec![
                PaymentStatus::Sent,
                PaymentStatus::Viewed,
                PaymentStatus::Paid,
                PaymentStatus::Overdue,
                PaymentStatus::Partial,
                PaymentStatus::PaymentFailed,
                PaymentStatus::Pending,
            ],
            PaymentStatus::Sent => vec![
                PaymentStatus::Viewed,
                PaymentStatus::Paid,
                PaymentStatus::Overdue,
                PaymentStatus::PaymentFailed,
                PaymentStatus::Pending,
            ],
            PaymentStatus::Viewed => vec![
                PaymentStatus::Paid,
                PaymentStatus::Overdue,
                PaymentStatus::PaymentFailed,
                PaymentStatus::Pending,
            ],
            PaymentStatus::Paid => vec![], // No valid transitions from Paid
            PaymentStatus::Overdue => vec![
                PaymentStatus::Paid,
                PaymentStatus::PaymentFailed,
                PaymentStatus::Pending,
            ],
            PaymentStatus::Partial => vec![
                PaymentStatus::Paid,
                PaymentStatus::Overdue,
                PaymentStatus::PaymentFailed,
                PaymentStatus::Pending,
            ],
            PaymentStatus::PaymentFailed => vec![
                PaymentStatus::Pending,
                PaymentStatus::Paid,
            ],
            PaymentStatus::Pending => vec![
                PaymentStatus::Paid,
                PaymentStatus::PaymentFailed,
            ],
        }
    }
}

impl StatusTransition {
    /// Create a new status transition record
    pub fn new(
        invoice_id: Uuid,
        from_status: PaymentStatus,
        to_status: PaymentStatus,
        transition_reason: Option<String>,
        transitioned_by: Uuid,
    ) -> Result<Self, StatusError> {
        if !from_status.can_transition_to(&to_status) {
            return Err(StatusError::InvalidTransition(from_status, to_status));
        }

        Ok(Self {
            id: Uuid::new_v4(),
            invoice_id,
            from_status,
            to_status,
            transition_reason,
            transitioned_by,
            timestamp: Utc::now(),
        })
    }
}

impl StatusOverride {
    /// Create a new status override record
    pub fn new(
        invoice_id: Uuid,
        overridden_status: PaymentStatus,
        override_reason: String,
        overridden_by: Uuid,
    ) -> Result<Self, StatusError> {
        if override_reason.trim().is_empty() {
            return Err(StatusError::OverrideRequiresReason);
        }

        Ok(Self {
            id: Uuid::new_v4(),
            invoice_id,
            overridden_status,
            override_reason,
            overridden_by,
            timestamp: Utc::now(),
        })
    }
}

/// Status workflow configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatusWorkflowConfig {
    pub id: Uuid,
    pub user_id: Uuid,
    pub auto_overdue_enabled: bool,
    pub overdue_days: i32, // Days after due date to mark as overdue
    pub auto_cancel_enabled: bool,
    pub cancel_days: i32, // Days after overdue to auto-cancel
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl StatusWorkflowConfig {
    /// Create a new status workflow configuration
    pub fn new(
        user_id: Uuid,
        auto_overdue_enabled: bool,
        overdue_days: i32,
        auto_cancel_enabled: bool,
        cancel_days: i32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            auto_overdue_enabled,
            overdue_days,
            auto_cancel_enabled,
            cancel_days,
            created_at: now,
            updated_at: now,
        }
    }

    /// Update the configuration
    pub fn update(
        &mut self,
        auto_overdue_enabled: bool,
        overdue_days: i32,
        auto_cancel_enabled: bool,
        cancel_days: i32,
    ) {
        self.auto_overdue_enabled = auto_overdue_enabled;
        self.overdue_days = overdue_days;
        self.auto_cancel_enabled = auto_cancel_enabled;
        self.cancel_days = cancel_days;
        self.updated_at = Utc::now();
    }

    /// Check if an invoice should be marked as overdue
    pub fn should_mark_overdue(&self, due_date: DateTime<Utc>) -> bool {
        if !self.auto_overdue_enabled {
            return false;
        }

        let overdue_date = due_date + chrono::Duration::days(self.overdue_days as i64);
        Utc::now() >= overdue_date
    }

    /// Check if an overdue invoice should be cancelled
    pub fn should_cancel_overdue(&self, overdue_date: DateTime<Utc>) -> bool {
        if !self.auto_cancel_enabled {
            return false;
        }

        let cancel_date = overdue_date + chrono::Duration::days(self.cancel_days as i64);
        Utc::now() >= cancel_date
    }
}