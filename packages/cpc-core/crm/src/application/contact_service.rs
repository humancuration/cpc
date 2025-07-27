//! Contact service for the CRM module
//!
//! This module contains the application service for managing contacts,
//! including consent management workflows and dual-mode handling.

use crate::domain::contact::{Contact, ContactType, ConsentSettings, ExternalContactData, ContactError};
use crate::domain::primitives::{ContactId, UserId, Email, Phone};
use std::collections::HashMap;
use thiserror::Error;

/// Error types for contact service operations
#[derive(Error, Debug)]
pub enum ContactServiceError {
    #[error("Contact error: {0}")]
    ContactError(#[from] ContactError),
    
    #[error("Contact not found: {0}")]
    ContactNotFound(ContactId),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

/// Service for managing contact consent workflows
pub struct ContactConsentService;

impl ContactConsentService {
    /// Create new consent settings with default values
    pub fn create_default_consent() -> ConsentSettings {
        ConsentSettings::new()
    }
    
    /// Update consent settings for a platform-native contact
    pub fn update_consent_settings(
        contact: &mut Contact,
        new_settings: ConsentSettings,
    ) -> Result<(), ContactServiceError> {
        // Validate that this is a platform-native contact
        if !contact.is_platform_native() {
            return Err(ContactServiceError::InvalidOperation(
                "Cannot update consent settings for external contacts".to_string()
            ));
        }
        
        // Validate new settings
        new_settings.validate()?;
        
        // Update consent settings
        match &mut contact.contact_type {
            ContactType::PlatformNative(_, ref mut consent_settings) => {
                *consent_settings = new_settings;
            }
            ContactType::External(_) => {
                return Err(ContactServiceError::InvalidOperation(
                    "Cannot update consent settings for external contacts".to_string()
                ));
            }
        }
        
        Ok(())
    }
    
    /// Get consent settings for a platform-native contact
    pub fn get_consent_settings(contact: &Contact) -> Option<&ConsentSettings> {
        match &contact.contact_type {
            ContactType::PlatformNative(_, consent_settings) => Some(consent_settings),
            ContactType::External(_) => None,
        }
    }
}

/// Service for managing contacts
pub struct ContactService;

impl ContactService {
    /// Create a new platform-native contact
    pub fn create_platform_native_contact(
        user_id: UserId,
        name: String,
        primary_email: Option<Email>,
        primary_phone: Option<Phone>,
        company: Option<String>,
        consent_settings: ConsentSettings,
    ) -> Result<Contact, ContactServiceError> {
        let contact = Contact::new_platform_native(
            user_id,
            name,
            primary_email,
            primary_phone,
            company,
            consent_settings,
        )?;
        
        Ok(contact)
    }
    
    /// Create a new external contact
    pub fn create_external_contact(
        name: String,
        primary_email: Option<Email>,
        primary_phone: Option<Phone>,
        company: Option<String>,
        external_data: ExternalContactData,
    ) -> Result<Contact, ContactServiceError> {
        let contact = Contact::new_external(
            name,
            primary_email,
            primary_phone,
            company,
            external_data,
        )?;
        
        Ok(contact)
    }
    
    /// Update contact information
    pub fn update_contact_info(
        contact: &mut Contact,
        name: Option<String>,
        primary_email: Option<Email>,
        primary_phone: Option<Phone>,
        company: Option<String>,
    ) -> Result<(), ContactServiceError> {
        contact.update_info(name, primary_email, primary_phone, company)?;
        Ok(())
    }
    
    /// Add tags to a contact
    pub fn add_tags_to_contact(contact: &mut Contact, tags: Vec<String>) {
        contact.add_tags(tags);
    }
    
    /// Remove tags from a contact
    pub fn remove_tags_from_contact(contact: &mut Contact, tags: Vec<String>) {
        contact.remove_tags(tags);
    }
    
    /// Update the last interaction timestamp for a contact
    pub fn update_last_interaction(contact: &mut Contact) {
        contact.update_last_interaction();
    }
    
    /// Validate a contact
    pub fn validate_contact(contact: &Contact) -> Result<(), ContactServiceError> {
        contact.validate()?;
        Ok(())
    }
    
    /// Check if a contact is platform-native
    pub fn is_platform_native(contact: &Contact) -> bool {
        contact.is_platform_native()
    }
    
    /// Get the user ID for a platform-native contact
    pub fn get_user_id(contact: &Contact) -> Option<&UserId> {
        contact.get_user_id()
    }
    
    /// Merge two contacts (useful when deduplicating)
    pub fn merge_contacts(
        primary: &mut Contact,
        secondary: &Contact,
    ) -> Result<(), ContactServiceError> {
        // Merge basic information (prefer primary values if they exist)
        if primary.name.is_empty() && !secondary.name.is_empty() {
            primary.name = secondary.name.clone();
        }
        
        if primary.primary_email.is_none() && secondary.primary_email.is_some() {
            primary.primary_email = secondary.primary_email.clone();
        }
        
        if primary.primary_phone.is_none() && secondary.primary_phone.is_some() {
            primary.primary_phone = secondary.primary_phone.clone();
        }
        
        if primary.company.is_none() && secondary.company.is_some() {
            primary.company = secondary.company.clone();
        }
        
        // Merge tags (avoid duplicates)
        for tag in &secondary.tags {
            if !primary.tags.contains(tag) {
                primary.tags.push(tag.clone());
            }
        }
        
        // Update timestamps
        if secondary.created_at < primary.created_at {
            primary.created_at = secondary.created_at;
        }
        
        if secondary.updated_at > primary.updated_at {
            primary.updated_at = secondary.updated_at;
        }
        
        if let Some(secondary_last) = secondary.last_interaction {
            if let Some(primary_last) = primary.last_interaction {
                if secondary_last > primary_last {
                    primary.last_interaction = Some(secondary_last);
                }
            } else {
                primary.last_interaction = Some(secondary_last);
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use crate::domain::contact::DataSharingLevel;

    #[test]
    fn test_create_platform_native_contact() {
        let user_id = UserId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let mut consent = ConsentSettings::new();
        consent.share_profile = DataSharingLevel::ViewOnly;
        
        let contact = ContactService::create_platform_native_contact(
            user_id.clone(),
            "John Doe".to_string(),
            Some(email.clone()),
            None,
            Some("Acme Corp".to_string()),
            consent.clone(),
        ).unwrap();
        
        assert_eq!(contact.name, "John Doe");
        assert_eq!(contact.primary_email, Some(email));
        assert_eq!(contact.company, Some("Acme Corp".to_string()));
        assert!(ContactService::is_platform_native(&contact));
        assert_eq!(ContactService::get_user_id(&contact), Some(&user_id));
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
        
        let contact = ContactService::create_external_contact(
            "Jane Smith".to_string(),
            Some(email.clone()),
            None,
            Some("External Corp".to_string()),
            external_data,
        ).unwrap();
        
        assert_eq!(contact.name, "Jane Smith");
        assert_eq!(contact.primary_email, Some(email));
        assert_eq!(contact.company, Some("External Corp".to_string()));
        assert!(!ContactService::is_platform_native(&contact));
        assert_eq!(ContactService::get_user_id(&contact), None);
    }

    #[test]
    fn test_update_consent_settings() {
        let user_id = UserId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let mut consent = ConsentSettings::new();
        consent.share_profile = DataSharingLevel::ViewOnly;
        
        let mut contact = ContactService::create_platform_native_contact(
            user_id,
            "John Doe".to_string(),
            None,
            None,
            None,
            consent,
        ).unwrap();
        
        let mut new_consent = ConsentSettings::new();
        new_consent.share_profile = DataSharingLevel::Editable;
        new_consent.share_interaction_history = DataSharingLevel::ViewOnly;
        
        let result = ContactConsentService::update_consent_settings(&mut contact, new_consent.clone());
        assert!(result.is_ok());
        
        let updated_consent = ContactConsentService::get_consent_settings(&contact).unwrap();
        assert_eq!(updated_consent.share_profile, DataSharingLevel::Editable);
        assert_eq!(updated_consent.share_interaction_history, DataSharingLevel::ViewOnly);
    }

    #[test]
    fn test_update_contact_info() {
        let mut contact = create_test_contact();
        let new_email = Email::new("new@example.com".to_string()).unwrap();
        
        let result = ContactService::update_contact_info(
            &mut contact,
            Some("New Name".to_string()),
            Some(new_email.clone()),
            None,
            Some("New Company".to_string()),
        );
        
        assert!(result.is_ok());
        assert_eq!(contact.name, "New Name");
        assert_eq!(contact.primary_email, Some(new_email));
        assert_eq!(contact.company, Some("New Company".to_string()));
    }

    #[test]
    fn test_add_and_remove_tags() {
        let mut contact = create_test_contact();
        
        ContactService::add_tags_to_contact(&mut contact, vec!["customer".to_string(), "vip".to_string()]);
        assert_eq!(contact.tags.len(), 2);
        assert!(contact.tags.contains(&"customer".to_string()));
        assert!(contact.tags.contains(&"vip".to_string()));
        
        ContactService::remove_tags_from_contact(&mut contact, vec!["customer".to_string()]);
        assert_eq!(contact.tags.len(), 1);
        assert!(contact.tags.contains(&"vip".to_string()));
        assert!(!contact.tags.contains(&"customer".to_string()));
    }

    #[test]
    fn test_merge_contacts() {
        let mut primary = create_test_contact();
        let mut secondary = create_test_contact();
        
        // Modify secondary contact
        secondary.name = "Secondary Name".to_string();
        secondary.tags.push("secondary".to_string());
        secondary.last_interaction = Some(chrono::Utc::now());
        
        let result = ContactService::merge_contacts(&mut primary, &secondary);
        assert!(result.is_ok());
        
        // Primary should keep its name but gain secondary's tags
        assert_eq!(primary.name, "John Doe"); // Should keep primary name
        assert!(primary.tags.contains(&"secondary".to_string()));
    }

    fn create_test_contact() -> Contact {
        let user_id = UserId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let consent = ConsentSettings::new();
        let email = Email::new("test@example.com".to_string()).unwrap();
        
        ContactService::create_platform_native_contact(
            user_id,
            "John Doe".to_string(),
            Some(email),
            None,
            Some("Acme Corp".to_string()),
            consent,
        ).unwrap()
    }
}