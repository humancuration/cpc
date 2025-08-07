//! Validation utilities for user profiles

use regex::Regex;
use tracing::debug;

/// Represents validation results
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
}

/// Validate a user profile
pub fn validate_profile(profile: &super::UserProfile) -> ValidationResult {
    let mut errors = Vec::new();
    
    // Validate name
    if profile.name.trim().is_empty() {
        errors.push("Name cannot be empty".to_string());
    }
    
    // Validate email format
    if !is_valid_email(&profile.email) {
        errors.push("Invalid email format".to_string());
    }
    
    // Validate age (COPPA compliance)
    if profile.age < 13 {
        errors.push("Age must be at least 13".to_string());
    }
    
    ValidationResult {
        is_valid: errors.is_empty(),
        errors,
    }
}

/// Check if an email has valid format
fn is_valid_email(email: &str) -> bool {
    // Simple email validation - in production, use a proper email validation library
    let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    email_regex.is_match(email)
}

/// Normalize a name (trim and proper capitalization)
pub fn normalize_name(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    
    // Simple capitalization - first letter of each word uppercase, rest lowercase
    let mut result = String::new();
    let mut capitalize_next = true;
    
    for ch in trimmed.chars() {
        if ch.is_whitespace() {
            result.push(ch);
            capitalize_next = true;
        } else if capitalize_next {
            result.push(ch.to_uppercase().next().unwrap_or(ch));
            capitalize_next = false;
        } else {
            result.push(ch.to_lowercase().next().unwrap_or(ch));
        }
    }
    
    result
}

/// Trim whitespace from email
pub fn trim_email(email: &str) -> String {
    email.trim().to_string()
}

/// Create a display name from a user profile
pub fn create_display_name(profile: &super::UserProfile) -> String {
    let name_parts: Vec<&str> = profile.name.split_whitespace().collect();
    let first_name = if !name_parts.is_empty() {
        name_parts[0]
    } else {
        &profile.name
    };
    
    format!("{} ({})", first_name, profile.id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::UserProfile;
    
    #[test]
    fn test_valid_profile() {
        let profile = UserProfile {
            id: "user-1".to_string(),
            name: "Alice Johnson".to_string(),
            email: "alice@example.com".to_string(),
            age: 25,
            display_name: None,
        };
        
        let result = validate_profile(&profile);
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
    }
    
    #[test]
    fn test_invalid_email() {
        let profile = UserProfile {
            id: "user-1".to_string(),
            name: "Alice Johnson".to_string(),
            email: "invalid-email".to_string(),
            age: 25,
            display_name: None,
        };
        
        let result = validate_profile(&profile);
        assert!(!result.is_valid);
        assert!(result.errors.contains(&"Invalid email format".to_string()));
    }
    
    #[test]
    fn test_underage_user() {
        let profile = UserProfile {
            id: "user-1".to_string(),
            name: "Alice Johnson".to_string(),
            email: "alice@example.com".to_string(),
            age: 10,
            display_name: None,
        };
        
        let result = validate_profile(&profile);
        assert!(!result.is_valid);
        assert!(result.errors.contains(&"Age must be at least 13".to_string()));
    }
    
    #[test]
    fn test_normalize_name() {
        assert_eq!(normalize_name(" alice johnson "), "Alice Johnson");
        assert_eq!(normalize_name("BOB SMITH"), "Bob Smith");
        assert_eq!(normalize_name(""), "");
    }
    
    #[test]
    fn test_trim_email() {
        assert_eq!(trim_email(" alice@example.com "), "alice@example.com");
        assert_eq!(trim_email("bob@test.org"), "bob@test.org");
    }
    
    #[test]
    fn test_create_display_name() {
        let profile = UserProfile {
            id: "user-123".to_string(),
            name: "Alice Johnson".to_string(),
            email: "alice@example.com".to_string(),
            age: 25,
            display_name: None,
        };
        
        assert_eq!(create_display_name(&profile), "Alice (user-123)");
    }
}