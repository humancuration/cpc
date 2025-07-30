//! # CPC Consent Management
//!
//! A unified consent management system for the CPC ecosystem that provides
//! centralized control over data sharing preferences across all applications.

pub mod middleware;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Consent levels for data sharing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConsentLevel {
    /// No data sharing allowed
    None,
    /// Minimal data sharing for core functionality
    Minimal,
    /// Standard data sharing for enhanced features
    Standard,
    /// Full data sharing for all purposes
    Full,
}

impl ConsentLevel {
    /// Returns the priority level of the consent (higher number means more permissive)
    pub fn priority(&self) -> u8 {
        match self {
            ConsentLevel::None => 0,
            ConsentLevel::Minimal => 1,
            ConsentLevel::Standard => 2,
            ConsentLevel::Full => 3,
        }
    }
}

/// Application domains that require consent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub enum Domain {
    /// Allat application
    Allat,
    /// Yapper application
    Yapper,
    /// Presence application
    Presence,
    /// SocialGraph application
    SocialGraph,
    /// Financial data sharing
    FinancialData,
    /// Health data sharing
    HealthData,
    /// Calendar data sharing
    CalendarData,
    /// CRM data sharing
    CrmData,
    /// SCM data sharing
    ScmData,
    /// Document data sharing
    DocumentData,
    /// Website builder data sharing
    WebsiteData,
    /// Recruitment data sharing
    RecruitmentData,
    /// Data lakehouse sharing
    DataLakehouse,
    /// Forecasting data sharing
    ForecastingData,
}

/// User consent profile for a specific domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentProfile {
    /// Unique user identifier
    pub user_id: String,
    /// Application domain
    pub domain: Domain,
    /// Current consent level
    pub level: ConsentLevel,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl ConsentProfile {
    /// Create a new consent profile
    pub fn new(user_id: String, domain: Domain, level: ConsentLevel) -> Self {
        let now = chrono::Utc::now();
        Self {
            user_id,
            domain,
            level,
            created_at: now,
            updated_at: now,
        }
    }

    /// Get the current consent level
    pub fn get_level(&self) -> &ConsentLevel {
        &self.level
    }

    /// Set a new consent level
    pub fn set_level(&mut self, level: ConsentLevel) {
        self.level = level;
        self.updated_at = chrono::Utc::now();
    }

    /// Check if consent level allows a specific level of access
    pub fn allows(&self, requested_level: ConsentLevel) -> bool {
        self.level.priority() >= requested_level.priority()
    }
}

/// Consent service for managing user consent profiles
pub struct ConsentService {
    /// Map of user IDs to their consent profiles
    user_consents: Arc<RwLock<HashMap<String, HashMap<Domain, ConsentProfile>>>>,
}

impl ConsentService {
    /// Create a new consent service
    pub fn new() -> Self {
        Self {
            user_consents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get a user's consent profile for a specific domain
    pub fn get_consent(&self, user_id: &str, domain: &Domain) -> Option<ConsentProfile> {
        let consents = self.user_consents.read().unwrap();
        consents.get(user_id)?.get(domain).cloned()
    }

    /// Set a user's consent profile for a specific domain
    pub fn set_consent(&self, user_id: String, domain: Domain, level: ConsentLevel) {
        let mut consents = self.user_consents.write().unwrap();
        let user_profiles = consents.entry(user_id).or_insert_with(HashMap::new);
        let profile = user_profiles.entry(domain.clone()).or_insert_with(|| {
            ConsentProfile::new(user_id.clone(), domain, ConsentLevel::None)
        });
        profile.set_level(level);
    }

    /// Check if a user's consent allows a specific level of access for a domain
    pub fn allows(&self, user_id: &str, domain: &Domain, requested_level: ConsentLevel) -> bool {
        if let Some(profile) = self.get_consent(user_id, domain) {
            profile.allows(requested_level)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consent_service() {
        let consent_service = ConsentService::new();
        
        // Test setting and getting consent
        consent_service.set_consent("user1".to_string(), Domain::Allat, ConsentLevel::Standard);
        
        let consent = consent_service.get_consent("user1", &Domain::Allat);
        assert!(consent.is_some());
        assert_eq!(consent.unwrap().get_level(), &ConsentLevel::Standard);
        
        // Test consent checking
        assert!(consent_service.allows("user1", &Domain::Allat, ConsentLevel::Minimal));
        assert!(consent_service.allows("user1", &Domain::Allat, ConsentLevel::Standard));
        assert!(!consent_service.allows("user1", &Domain::Allat, ConsentLevel::Full));
        
        // Test non-existent consent
        assert!(!consent_service.allows("user1", &Domain::Yapper, ConsentLevel::Minimal));
    }
}