//! Audit trail entities for consent changes.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::domain::consent::{Domain, DataSharingLevel};

/// Actions that can be performed on consent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConsentAction {
    /// Consent level was granted/updated
    Granted,
    /// Consent was revoked
    Revoked,
    /// Consent level was modified
    Modified,
}

/// Actor performing consent operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Actor {
    /// Human user
    User(String), // user_id
    /// System service
    Service(String), // service_name
    /// Administrator
    Admin(String), // admin_id
}

/// Audit event for consent changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Unique event identifier
    pub id: String,
    /// User affected by the change
    pub user_id: String,
    /// Domain affected by the change
    pub domain: Domain,
    /// Action performed
    pub action: ConsentAction,
    /// Previous consent level (if any)
    pub previous_level: Option<DataSharingLevel>,
    /// New consent level
    pub new_level: DataSharingLevel,
    /// Actor who performed the action
    pub actor: Actor,
    /// Timestamp of the event
    pub timestamp: DateTime<Utc>,
}

impl AuditEvent {
    /// Create a new audit event
    pub fn new(
        user_id: String,
        domain: Domain,
        action: ConsentAction,
        previous_level: Option<DataSharingLevel>,
        new_level: DataSharingLevel,
        actor: Actor,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            domain,
            action,
            previous_level,
            new_level,
            actor,
            timestamp: Utc::now(),
        }
    }
}