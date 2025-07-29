//! Consent domain entities and logic.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::domain::errors::ConsentError;

/// Levels of data sharing consent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataSharingLevel {
    /// No data sharing allowed
    None,
    /// Minimal data sharing for core functionality
    Minimal,
    /// Standard data sharing for enhanced features
    Standard,
    /// Full data sharing for all purposes
    Full,
}

impl DataSharingLevel {
    /// Returns the priority level of the consent (higher number means more permissive)
    pub fn priority(&self) -> u8 {
        match self {
            DataSharingLevel::None => 0,
            DataSharingLevel::Minimal => 1,
            DataSharingLevel::Standard => 2,
            DataSharingLevel::Full => 3,
        }
    }
}

/// Application domains that require consent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub enum Domain {
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
    pub level: DataSharingLevel,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl ConsentProfile {
    /// Create a new consent profile
    pub fn new(user_id: String, domain: Domain, level: DataSharingLevel) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            domain,
            level,
            created_at: now,
            updated_at: now,
        }
    }

    /// Get the current consent level
    pub fn get_level(&self) -> &DataSharingLevel {
        &self.level
    }

    /// Set a new consent level
    pub fn set_level(&mut self, level: DataSharingLevel) -> Result<(), ConsentError> {
        // Validate that the level is different
        if self.level == level {
            return Err(ConsentError::NoChange);
        }
        
        self.level = level;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Check if consent level allows a specific level of access
    pub fn allows(&self, requested_level: DataSharingLevel) -> bool {
        self.level.priority() >= requested_level.priority()
    }
}