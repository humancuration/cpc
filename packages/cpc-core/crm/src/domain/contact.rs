//! Contact domain entities for the CRM module
//!
//! This module contains the core business entities for managing contacts,
//! including both platform-native and external contacts with consent settings.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::domain::primitives::{ContactId, UserId, Email, Phone, CrmPrimitiveError};
use thiserror::Error;

/// Error types for contact operations
#[derive(Error, Debug)]
pub enum ContactError {
    #[error("Invalid contact data: {0}")]
    InvalidData(String),
    
    #[error("Invalid consent settings: {0}")]
    InvalidConsent(String),
    
    #[error("Primitive error: {0}")]
    PrimitiveError(#[from] CrmPrimitiveError),
}

/// Type of contact
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContactType {
    /// Platform-native contact with consent settings
    PlatformNative(UserId, ConsentSettings),
    
    /// External contact with traditional contact data
    External(ExternalContactData),
}

/// Data sharing level for consent settings (legacy)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DataSharingLevel {
    /// No sharing allowed
    None,
    
    /// View-only access
    ViewOnly,
    
    /// Editable access
    Editable,
}

/// Consent settings for platform-native contacts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConsentSettings {
    /// Sharing level for profile data
    pub share_profile: DataSharingLevel,
    
    /// Sharing level for interaction history
    pub share_interaction_history: DataSharingLevel,
    
    /// Sharing level for preferences
    pub share_preferences: DataSharingLevel,
    
    /// Custom field sharing levels
    pub custom_fields: HashMap<String, DataSharingLevel>,
}

impl ConsentSettings {
    /// Create new consent settings with default values (no sharing)
    pub fn new() -> Self {
        Self {
            share_profile: DataSharingLevel::None,
            share_interaction_history: DataSharingLevel::None,
            share_preferences: DataSharingLevel::None,
            custom_fields: HashMap::new(),
        }
    }
    
    /// Create consent settings with specific values
    pub fn with_levels(
        profile: DataSharingLevel,
        interaction_history: DataSharingLevel,
        preferences: DataSharingLevel,
    ) -> Self {
        Self {
            share_profile: profile,
            share_interaction_history: interaction_history,
            share_preferences: preferences,
            custom_fields: HashMap::new(),
        }
    }
    
    /// Create ConsentSettings from the new DataSharingLevel
    pub fn from_new_level(level: &consent_manager::domain::consent::DataSharingLevel) -> Self {
        let legacy_level = match level {
            consent_manager::domain::consent::DataSharingLevel::None => DataSharingLevel::None,
            consent_manager::domain::consent::DataSharingLevel::Minimal => DataSharingLevel::ViewOnly,
            consent_manager::domain::consent::DataSharingLevel::Standard => DataSharingLevel::Editable,
            consent_manager::domain::consent::DataSharingLevel::Full => DataSharingLevel::Editable,
        };
        
        Self {
            share_profile: legacy_level.clone(),
            share_interaction_history: legacy_level.clone(),
            share_preferences: legacy_level,
            custom_fields: HashMap::new(),
        }
    }
    
    /// Validate consent settings
    pub fn validate(&self) -> Result<(), ContactError> {
        // Currently no specific validation rules, but this is where they would go
        Ok(())
    }
}

impl Default for ConsentSettings {
    fn default() -> Self {
        Self::new()
    }
}

/// External contact data for non-platform contacts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExternalContactData {
    /// Email address
    pub email: Option<Email>,
    
    /// Phone number
    pub phone: Option<Phone>,
    
    /// Address
    pub address: Option<String>,
    
    /// Social media handles
    pub social_media: HashMap<String, String>,
}

impl ExternalContactData {
    /// Create new external contact data
    pub fn new(
        email: Option<Email>,
        phone: Option<Phone>,
        address: Option<String>,
        social_media: HashMap<String, String>,
    ) -> Self {
        Self {
            email,
            phone,
            address,
            social_media,
        }
    }
    
    /// Validate external contact data
    pub fn validate(&self) -> Result<(), ContactError> {
        // Currently no specific validation rules, but this is where they would go
        Ok(())
    }
}

/// Main contact entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Contact {
    /// Unique identifier for the contact
    pub id: ContactId,
    
    /// Type of contact (platform-native or external)
    pub contact_type: ContactType,
    
    /// Contact name
    pub name: String,
    
    /// Primary email address
    pub primary_email: Option<Email>,
    
    /// Primary phone number
    pub primary_phone: Option<Phone>,
    
    /// Company name
    pub company: Option<String>,
    
    /// Tags associated with the contact
    pub tags: Vec<String>,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    
    /// Timestamp of last interaction
    pub last_interaction: Option<DateTime<Utc>>,
}

impl Contact {
    /// Create a new platform-native contact
    pub fn new_platform_native(
        user_id: UserId,
        name: String,
        primary_email: Option<Email>,
        primary_phone: Option<Phone>,
        company: Option<String>,
        consent_settings: ConsentSettings,
    ) -> Result<Self, ContactError> {
        // Validate consent settings
        consent_settings.validate()?;
        
        let now = Utc::now();
        
        Ok(Self {
            id: ContactId::new(),
            contact_type: ContactType::PlatformNative(user_id, consent_settings),
            name,
            primary_email,
            primary_phone,
            company,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            last_interaction: None,
        })
    }
    
    /// Create a new external contact
    pub fn new_external(
        name: String,
        primary_email: Option<Email>,
        primary_phone: Option<Phone>,
        company: Option<String>,
        external_data: ExternalContactData,
    ) -> Result<Self, ContactError> {
        // Validate external data
        external_data.validate()?;
        
        let now = Utc::now();
        
        Ok(Self {
            id: ContactId::new(),
            contact_type: ContactType::External(external_data),
            name,
            primary_email,
            primary_phone,
            company,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            last_interaction: None,
        })
    }
    
    /// Update contact information
    pub fn update_info(
        &mut self,
        name: Option<String>,
        primary_email: Option<Email>,
        primary_phone: Option<Phone>,
        company: Option<String>,
    ) -> Result<(), ContactError> {
        if let Some(name) = name {
            self.name = name;
        }
        
        if let Some(email) = primary_email {
            self.primary_email = Some(email);
        }
        
        if let Some(phone) = primary_phone {
            self.primary_phone = Some(phone);
        }
        
        if let Some(company) = company {
            self.company = Some(company);
        }
        
        self.updated_at = Utc::now();
        Ok(())
    }
    
    /// Add tags to the contact
    pub fn add_tags(&mut self, tags: Vec<String>) {
        for tag in tags {
            if !self.tags.contains(&tag) {
                self.tags.push(tag);
            }
        }
        self.updated_at = Utc::now();
    }
    
    /// Remove tags from the contact
    pub fn remove_tags(&mut self, tags: Vec<String>) {
        self.tags.retain(|tag| !tags.contains(tag));
        self.updated_at = Utc::now();
    }
    
    /// Check if this is a platform-native contact
    pub fn is_platform_native(&self) -> bool {
        matches!(self.contact_type, ContactType::PlatformNative(_, _))
    }
    
    /// Get the user ID if this is a platform-native contact
    pub fn get_user_id(&self) -> Option<&UserId> {
        match &self.contact_type {
            ContactType::PlatformNative(user_id, _) => Some(user_id),
            ContactType::External(_) => None,
        }
    }
    
    /// Validate the contact
    pub fn validate(&self) -> Result<(), ContactError> {
        if self.name.is_empty() {
            return Err(ContactError::InvalidData("Contact name cannot be empty".to_string()));
        }
        
        // Validate based on contact type
        match &self.contact_type {
            ContactType::PlatformNative(_, consent) => {
                consent.validate()?;
            }
            ContactType::External(external_data) => {
                external_data.validate()?;
            }
        }
        
        Ok(())
    }
    
    /// Update the last interaction timestamp
    pub fn update_last_interaction(&mut self) {
        self.last_interaction = Some(Utc::now());
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_create_platform_native_contact() {
        let user_id = UserId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let consent = ConsentSettings::new();
        let email = Email::new("test@example.com".to_string()).unwrap();
        
        let contact = Contact::new_platform_native(
            user_id.clone(),
            "John Doe".to_string(),
            Some(email.clone()),
            None,
            Some("Acme Corp".to_string()),
            consent,
        ).unwrap();
        
        assert_eq!(contact.name, "John Doe");
        assert_eq!(contact.primary_email, Some(email));
        assert_eq!(contact.company, Some("Acme Corp".to_string()));
        assert!(contact.is_platform_native());
        assert_eq!(contact.get_user_id(), Some(&user_id));
    }

    #[test]
    fn test_create_external_contact() {
        let email = Email::new("external@example.com".to_string()).unwrap();
        let mut social_media = HashMap::new();
        social_media.insert("twitter".to_string(), "@external".to_string());
        
        let external_data = ExternalContactData::new(
            Some(email.clone()),
            None,
            Some("123 Main St".to_string()),
            social_media,
        );
        
        let contact = Contact::new_external(
            "Jane Smith".to_string(),
            Some(email.clone()),
            None,
            Some("External Corp".to_string()),
            external_data,
        ).unwrap();
        
        assert_eq!(contact.name, "Jane Smith");
        assert_eq!(contact.primary_email, Some(email));
        assert_eq!(contact.company, Some("External Corp".to_string()));
        assert!(!contact.is_platform_native());
        assert_eq!(contact.get_user_id(), None);
    }

    #[test]
    fn test_update_contact_info() {
        let mut contact = create_test_contact();
        let new_email = Email::new("new@example.com".to_string()).unwrap();
        
        let original_updated = contact.updated_at;
        
        contact.update_info(
            Some("New Name".to_string()),
            Some(new_email.clone()),
            None,
            Some("New Company".to_string()),
        ).unwrap();
        
        assert_eq!(contact.name, "New Name");
        assert_eq!(contact.primary_email, Some(new_email));
        assert_eq!(contact.company, Some("New Company".to_string()));
        assert!(contact.updated_at > original_updated);
    }

    #[test]
    fn test_add_and_remove_tags() {
        let mut contact = create_test_contact();
        
        contact.add_tags(vec!["customer".to_string(), "vip".to_string()]);
        assert_eq!(contact.tags.len(), 2);
        assert!(contact.tags.contains(&"customer".to_string()));
        assert!(contact.tags.contains(&"vip".to_string()));
        
        contact.remove_tags(vec!["customer".to_string()]);
        assert_eq!(contact.tags.len(), 1);
        assert!(contact.tags.contains(&"vip".to_string()));
        assert!(!contact.tags.contains(&"customer".to_string()));
    }

    #[test]
    fn test_update_last_interaction() {
        let mut contact = create_test_contact();
        let original_updated = contact.updated_at;
        
        contact.update_last_interaction();
        
        assert!(contact.last_interaction.is_some());
        assert!(contact.updated_at > original_updated);
    }

    #[test]
    fn test_contact_validation() {
        let contact = create_test_contact();
        assert!(contact.validate().is_ok());
        
        // Test invalid contact with empty name
        let invalid_contact = Contact {
            name: "".to_string(),
            ..create_test_contact()
        };
        assert!(invalid_contact.validate().is_err());
    }

    fn create_test_contact() -> Contact {
        let user_id = UserId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let consent = ConsentSettings::new();
        let email = Email::new("test@example.com".to_string()).unwrap();
        
        Contact::new_platform_native(
            user_id,
            "John Doe".to_string(),
            Some(email),
            None,
            Some("Acme Corp".to_string()),
            consent,
        ).unwrap()
    }
}