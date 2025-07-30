//! # Consent Manager
//!
//! Manager for handling user consent preferences using the CPC consent system.

use cpc_consent::{ConsentService, Domain, ConsentLevel};
use uuid::Uuid;
use std::sync::Arc;

/// Yapper Consent Manager
///
/// This manager handles user consent preferences for the Yapper application
/// using the centralized CPC consent system.
pub struct YapperConsentManager {
    consent_service: Arc<ConsentService>,
    domain: Domain,
}

impl YapperConsentManager {
    /// Create a new Yapper Consent Manager
    ///
    /// # Arguments
    /// * `consent_service` - The centralized consent service
    ///
    /// # Returns
    /// * `Self` - The new consent manager
    pub fn new(consent_service: Arc<ConsentService>) -> Self {
        Self {
            consent_service,
            domain: Domain::Yapper,
        }
    }
    
    /// Check if a user has given consent for a specific level
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user
    /// * `level` - The consent level to check
    ///
    /// # Returns
    /// * `bool` - Whether the user has given consent
    pub fn check_consent(&self, user_id: Uuid, level: ConsentLevel) -> bool {
        self.consent_service.allows(&user_id.to_string(), &self.domain, level)
    }
    
    /// Set a user's consent level
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user
    /// * `level` - The consent level to set
    pub fn set_consent(&self, user_id: Uuid, level: ConsentLevel) {
        self.consent_service.set_consent(user_id.to_string(), self.domain.clone(), level);
    }
    
    /// Get a user's current consent level
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user
    ///
    /// # Returns
    /// * `Option<ConsentLevel>` - The user's consent level or None if not set
    pub fn get_consent(&self, user_id: Uuid) -> Option<ConsentLevel> {
        self.consent_service.get_consent(&user_id.to_string(), &self.domain)
            .map(|profile| profile.level.clone())
    }
}