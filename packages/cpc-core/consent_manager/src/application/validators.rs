//! Validation functions for consent operations.

use crate::domain::{
    consent::{DataSharingLevel, Domain},
    errors::ConsentError,
};

/// Validate that a consent level is valid for a domain
pub fn validate_consent_level(domain: &Domain, level: &DataSharingLevel) -> Result<(), ConsentError> {
    // All domains currently accept all levels
    // In the future, we might have domain-specific restrictions
    Ok(())
}

/// Validate user ID format
pub fn validate_user_id(user_id: &str) -> Result<(), ConsentError> {
    if user_id.is_empty() {
        return Err(ConsentError::ValidationError("User ID cannot be empty".to_string()));
    }
    
    if user_id.len() > 100 {
        return Err(ConsentError::ValidationError("User ID too long".to_string()));
    }
    
    Ok(())
}

/// Validate domain
pub fn validate_domain(domain: &Domain) -> Result<(), ConsentError> {
    // All domains are currently valid
    Ok(())
}