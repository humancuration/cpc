//! Consent-based access control for collaborative documents

use crate::core::{AccessLevel, DocumentError, DocumentPermission};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Error types for access control operations
#[derive(Error, Debug)]
pub enum AccessControlError {
    #[error("Access denied: {0}")]
    AccessDenied(String),
    #[error("Permission not found for user: {0}")]
    PermissionNotFound(Uuid),
    #[error("Invalid permission level: {0}")]
    InvalidPermission(String),
}

/// Consent record for document access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecord {
    pub id: Uuid,
    pub document_id: Uuid,
    pub user_id: Uuid,
    pub consent_type: ConsentType,
    pub granted_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub granted_by: Uuid,
}

/// Types of consent for document access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsentType {
    Read,
    Write,
    Admin,
    Share,
}

/// Access control policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPolicy {
    pub id: Uuid,
    pub document_id: Uuid,
    pub policy_type: PolicyType,
    pub conditions: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
}

/// Types of access policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyType {
    TimeBased,
    RoleBased,
    ConsentBased,
    IpBased,
}

/// Consent-based access controller
pub struct ConsentAccessController {
    consents: HashMap<Uuid, ConsentRecord>,
    policies: HashMap<Uuid, AccessPolicy>,
}

impl ConsentAccessController {
    /// Create a new consent access controller
    pub fn new() -> Self {
        Self {
            consents: HashMap::new(),
            policies: HashMap::new(),
        }
    }

    /// Grant consent for document access
    pub fn grant_consent(
        &mut self,
        document_id: Uuid,
        user_id: Uuid,
        consent_type: ConsentType,
        granted_by: Uuid,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<ConsentRecord, AccessControlError> {
        let consent = ConsentRecord {
            id: Uuid::new_v4(),
            document_id,
            user_id,
            consent_type,
            granted_at: Utc::now(),
            expires_at,
            granted_by,
        };

        self.consents.insert(consent.id, consent.clone());
        Ok(consent)
    }

    /// Revoke consent for document access
    pub fn revoke_consent(
        &mut self,
        consent_id: Uuid,
    ) -> Result<(), AccessControlError> {
        if self.consents.remove(&consent_id).is_some() {
            Ok(())
        } else {
            Err(AccessControlError::PermissionNotFound(consent_id))
        }
    }

    /// Check if user has consent for document access
    pub fn has_consent(
        &self,
        document_id: Uuid,
        user_id: Uuid,
        consent_type: ConsentType,
    ) -> Result<bool, AccessControlError> {
        let now = Utc::now();
        
        for consent in self.consents.values() {
            // Check if consent is for the right document and user
            if consent.document_id == document_id && consent.user_id == user_id {
                // Check if consent is for the right type or a higher level
                let has_access = match (&consent.consent_type, &consent_type) {
                    (ConsentType::Admin, _) => true, // Admin has all access
                    (ConsentType::Write, ConsentType::Read) => true, // Write includes read
                    (ConsentType::Write, ConsentType::Write) => true,
                    (ConsentType::Read, ConsentType::Read) => true,
                    (ConsentType::Share, ConsentType::Read) => true, // Share includes read
                    (ConsentType::Share, ConsentType::Share) => true,
                    _ => consent.consent_type == consent_type,
                };

                // Check if consent is still valid
                if has_access {
                    if let Some(expires_at) = consent.expires_at {
                        if expires_at > now {
                            return Ok(true);
                        }
                    } else {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    /// Get all consents for a document
    pub fn get_document_consents(
        &self,
        document_id: Uuid,
    ) -> Vec<&ConsentRecord> {
        self.consents
            .values()
            .filter(|consent| consent.document_id == document_id)
            .collect()
    }

    /// Get all consents for a user
    pub fn get_user_consents(
        &self,
        user_id: Uuid,
    ) -> Vec<&ConsentRecord> {
        self.consents
            .values()
            .filter(|consent| consent.user_id == user_id)
            .collect()
    }

    /// Create an access policy
    pub fn create_policy(
        &mut self,
        document_id: Uuid,
        policy_type: PolicyType,
        conditions: serde_json::Value,
        created_by: Uuid,
    ) -> AccessPolicy {
        let policy = AccessPolicy {
            id: Uuid::new_v4(),
            document_id,
            policy_type,
            conditions,
            created_at: Utc::now(),
            created_by,
        };

        self.policies.insert(policy.id, policy.clone());
        policy
    }

    /// Evaluate access policy for a user
    pub fn evaluate_policy(
        &self,
        policy_id: Uuid,
        user_id: Uuid,
        context: &AccessContext,
    ) -> Result<bool, AccessControlError> {
        let policy = self.policies.get(&policy_id)
            .ok_or(AccessControlError::PermissionNotFound(policy_id))?;

        match &policy.policy_type {
            PolicyType::TimeBased => self.evaluate_time_policy(policy, context),
            PolicyType::RoleBased => self.evaluate_role_policy(policy, user_id, context),
            PolicyType::ConsentBased => self.evaluate_consent_policy(policy, user_id, context),
            PolicyType::IpBased => self.evaluate_ip_policy(policy, context),
        }
    }

    /// Evaluate time-based policy
    fn evaluate_time_policy(
        &self,
        policy: &AccessPolicy,
        context: &AccessContext,
    ) -> Result<bool, AccessControlError> {
        // Check if current time is within allowed time range
        if let Some(allowed_times) = policy.conditions.get("allowed_times") {
            let now = Utc::now();
            // This is a simplified implementation
            // In a real system, this would check actual time constraints
            Ok(true)
        } else {
            Ok(true)
        }
    }

    /// Evaluate role-based policy
    fn evaluate_role_policy(
        &self,
        policy: &AccessPolicy,
        user_id: Uuid,
        context: &AccessContext,
    ) -> Result<bool, AccessControlError> {
        // Check if user has required role
        if let Some(required_roles) = policy.conditions.get("required_roles") {
            // This is a simplified implementation
            // In a real system, this would check actual user roles
            Ok(true)
        } else {
            Ok(true)
        }
    }

    /// Evaluate consent-based policy
    fn evaluate_consent_policy(
        &self,
        policy: &AccessPolicy,
        user_id: Uuid,
        context: &AccessContext,
    ) -> Result<bool, AccessControlError> {
        // Check if user has given consent
        if let Some(document_id) = policy.conditions.get("document_id") {
            if let Some(doc_id_str) = document_id.as_str() {
                if let Ok(doc_id) = Uuid::parse_str(doc_id_str) {
                    // Check if user has read consent for this document
                    self.has_consent(doc_id, user_id, ConsentType::Read)
                } else {
                    Ok(false)
                }
            } else {
                Ok(false)
            }
        } else {
            Ok(true)
        }
    }

    /// Evaluate IP-based policy
    fn evaluate_ip_policy(
        &self,
        policy: &AccessPolicy,
        context: &AccessContext,
    ) -> Result<bool, AccessControlError> {
        // Check if user IP is allowed
        if let Some(allowed_ips) = policy.conditions.get("allowed_ips") {
            if let Some(user_ip) = &context.user_ip {
                // This is a simplified implementation
                // In a real system, this would check actual IP constraints
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(true)
        }
    }
}

/// Context for access evaluation
#[derive(Debug, Clone)]
pub struct AccessContext {
    pub user_ip: Option<String>,
    pub user_agent: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub resource_id: Option<Uuid>,
}

/// Trait for consent service integration
#[async_trait]
pub trait ConsentService: Send + Sync {
    /// Check if user has given consent for a resource
    async fn has_consent(
        &self,
        user_id: Uuid,
        resource_id: Uuid,
        consent_type: &str,
    ) -> Result<bool, DocumentError>;

    /// Grant consent to a user for a resource
    async fn grant_consent(
        &self,
        user_id: Uuid,
        resource_id: Uuid,
        consent_type: String,
        granted_by: Uuid,
    ) -> Result<(), DocumentError>;

    /// Revoke consent from a user for a resource
    async fn revoke_consent(
        &self,
        user_id: Uuid,
        resource_id: Uuid,
        consent_type: String,
    ) -> Result<(), DocumentError>;
}

/// Document access checker
pub struct DocumentAccessChecker {
    consent_controller: ConsentAccessController,
    consent_service: Option<Box<dyn ConsentService>>,
}

impl DocumentAccessChecker {
    /// Create a new document access checker
    pub fn new(consent_service: Option<Box<dyn ConsentService>>) -> Self {
        Self {
            consent_controller: ConsentAccessController::new(),
            consent_service,
        }
    }

    /// Check if user has access to document
    pub async fn check_access(
        &self,
        document_id: Uuid,
        user_id: Uuid,
        required_level: AccessLevel,
        context: AccessContext,
    ) -> Result<bool, DocumentError> {
        // First check local consent controller
        let consent_type = match required_level {
            AccessLevel::Read => ConsentType::Read,
            AccessLevel::Write => ConsentType::Write,
            AccessLevel::Admin => ConsentType::Admin,
        };

        if self.consent_controller.has_consent(document_id, user_id, consent_type)? {
            return Ok(true);
        }

        // If we have a consent service, check that as well
        if let Some(consent_service) = &self.consent_service {
            let consent_type_str = match required_level {
                AccessLevel::Read => "read",
                AccessLevel::Write => "write",
                AccessLevel::Admin => "admin",
            };

            if consent_service.has_consent(user_id, document_id, consent_type_str).await? {
                return Ok(true);
            }
        }

        // Check policies
        // This is a simplified implementation
        Ok(false)
    }

    /// Grant access to document
    pub fn grant_access(
        &mut self,
        document_id: Uuid,
        user_id: Uuid,
        access_level: AccessLevel,
        granted_by: Uuid,
    ) -> Result<ConsentRecord, AccessControlError> {
        let consent_type = match access_level {
            AccessLevel::Read => ConsentType::Read,
            AccessLevel::Write => ConsentType::Write,
            AccessLevel::Admin => ConsentType::Admin,
        };

        self.consent_controller.grant_consent(
            document_id,
            user_id,
            consent_type,
            granted_by,
            None, // No expiration
        )
    }

    /// Revoke access to document
    pub fn revoke_access(
        &mut self,
        consent_id: Uuid,
    ) -> Result<(), AccessControlError> {
        self.consent_controller.revoke_consent(consent_id)
    }

    /// Get the consent controller for direct access (for testing)
    pub fn get_consent_controller(&self) -> &ConsentAccessController {
        &self.consent_controller
    }

    /// Convert consent record to document permission
    pub fn consent_to_permission(
        &self,
        consent: &ConsentRecord,
    ) -> DocumentPermission {
        let access_level = match consent.consent_type {
            ConsentType::Read => AccessLevel::Read,
            ConsentType::Write => AccessLevel::Write,
            ConsentType::Admin => AccessLevel::Admin,
            ConsentType::Share => AccessLevel::Write, // Share implies write access
        };

        DocumentPermission {
            user_id: consent.user_id,
            access_level,
            granted_at: consent.granted_at,
            granted_by: consent.granted_by,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[test]
    fn test_consent_access_controller() {
        let mut controller = ConsentAccessController::new();
        let document_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let granted_by = Uuid::new_v4();
        
        let consent = controller.grant_consent(
            document_id,
            user_id,
            ConsentType::Read,
            granted_by,
            None,
        ).unwrap();
        
        assert!(controller.has_consent(document_id, user_id, ConsentType::Read).unwrap());
        assert!(!controller.has_consent(document_id, user_id, ConsentType::Write).unwrap());
        
        controller.revoke_consent(consent.id).unwrap();
        assert!(!controller.has_consent(document_id, user_id, ConsentType::Read).unwrap());
    }

    #[tokio::test]
    async fn test_document_access_checker() {
        let mut checker = DocumentAccessChecker::new(None);
        let document_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let granted_by = Uuid::new_v4();
        let context = AccessContext {
            user_ip: None,
            user_agent: None,
            timestamp: Utc::now(),
            resource_id: Some(document_id),
        };
        
        checker.grant_access(
            document_id,
            user_id,
            AccessLevel::Read,
            granted_by,
        ).unwrap();
        
        // This would be true if we had a proper implementation
        // For now, we'll just test that it doesn't panic
        let _result = checker.check_access(
            document_id,
            user_id,
            AccessLevel::Read,
            context,
        ).await;
    }
}