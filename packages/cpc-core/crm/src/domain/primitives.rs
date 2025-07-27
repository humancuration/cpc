//! CRM primitives for the CPC platform
//!
//! This module provides basic types that can be used across the CRM module:
//! - ContactId: A unique identifier for contacts
//! - InteractionId: A unique identifier for interactions
//! - PipelineId: A unique identifier for pipelines
//! - StageId: A unique identifier for pipeline stages
//! - DealId: A unique identifier for deals
//! - UserId: A unique identifier for users
//! - Email: A validated email address
//! - Phone: A validated phone number
//! - MonetaryAmount: A monetary amount with currency (re-export from finance module)

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;
use thiserror::Error;

/// Error types for CRM primitives
#[derive(Error, Debug)]
pub enum CrmPrimitiveError {
    #[error("Invalid email format: {0}")]
    InvalidEmail(String),
    
    #[error("Invalid phone format: {0}")]
    InvalidPhone(String),
    
    #[error("Invalid UUID: {0}")]
    InvalidUuid(#[from] uuid::Error),
}

/// A unique identifier for contacts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ContactId(pub Uuid);

impl ContactId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl fmt::Display for ContactId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ContactId {
    type Err = CrmPrimitiveError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::parse_str(s)?;
        Ok(ContactId(uuid))
    }
}

/// A unique identifier for interactions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InteractionId(pub Uuid);

impl InteractionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl fmt::Display for InteractionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for InteractionId {
    type Err = CrmPrimitiveError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::parse_str(s)?;
        Ok(InteractionId(uuid))
    }
}

/// A unique identifier for pipelines
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PipelineId(pub Uuid);

impl PipelineId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl fmt::Display for PipelineId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PipelineId {
    type Err = CrmPrimitiveError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::parse_str(s)?;
        Ok(PipelineId(uuid))
    }
}

/// A unique identifier for pipeline stages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StageId(pub Uuid);

impl StageId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl fmt::Display for StageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for StageId {
    type Err = CrmPrimitiveError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::parse_str(s)?;
        Ok(StageId(uuid))
    }
}

/// A unique identifier for deals
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DealId(pub Uuid);

impl DealId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl fmt::Display for DealId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for DealId {
    type Err = CrmPrimitiveError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::parse_str(s)?;
        Ok(DealId(uuid))
    }
}

/// A unique identifier for users
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for UserId {
    type Err = CrmPrimitiveError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::parse_str(s)?;
        Ok(UserId(uuid))
    }
}

/// A validated email address
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Email(pub String);

impl Email {
    pub fn new(email: String) -> Result<Self, CrmPrimitiveError> {
        // Basic email validation - in a real implementation, you might want more robust validation
        if email.contains('@') && email.contains('.') {
            Ok(Self(email))
        } else {
            Err(CrmPrimitiveError::InvalidEmail(email))
        }
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A validated phone number
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Phone(pub String);

impl Phone {
    pub fn new(phone: String) -> Result<Self, CrmPrimitiveError> {
        // Basic phone validation - in a real implementation, you might want more robust validation
        // This is a very simple check, just ensuring it's not empty
        if !phone.is_empty() {
            Ok(Self(phone))
        } else {
            Err(CrmPrimitiveError::InvalidPhone(phone))
        }
    }
}

impl fmt::Display for Phone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A monetary amount with currency (re-export from finance module)
pub type MonetaryAmount = cpc_core::finance::domain::primitives::Money;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_id_creation() {
        let id1 = ContactId::new();
        let id2 = ContactId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_contact_id_from_uuid() {
        let uuid = Uuid::new_v4();
        let id = ContactId::from_uuid(uuid);
        assert_eq!(id.0, uuid);
    }

    #[test]
    fn test_contact_id_display() {
        let uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let id = ContactId::from_uuid(uuid);
        assert_eq!(format!("{}", id), "550e8400-e29b-41d4-a716-446655440000");
    }

    #[test]
    fn test_contact_id_from_str() {
        let id = ContactId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        assert_eq!(id.0.to_string(), "550e8400-e29b-41d4-a716-446655440000");
    }

    #[test]
    fn test_email_validation() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        assert_eq!(email.0, "test@example.com");
    }

    #[test]
    fn test_invalid_email() {
        let result = Email::new("invalid-email".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_phone_validation() {
        let phone = Phone::new("+1-555-123-4567".to_string()).unwrap();
        assert_eq!(phone.0, "+1-555-123-4567");
    }

    #[test]
    fn test_invalid_phone() {
        let result = Phone::new("".to_string());
        assert!(result.is_err());
    }
}