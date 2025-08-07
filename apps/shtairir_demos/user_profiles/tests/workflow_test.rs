//! Tests for the user profile workflow

use anyhow::Result;
use shtairir_demos_user_profiles::{workflow, validation};
use shtairir_registry::Registry;

#[tokio::test]
async fn test_workflow_execution() -> Result<()> {
    // Create registry and load modules
    let registry = Registry::load(&["../".into()])?;
    
    // Execute the workflow with a small dataset
    let (summary, profiles) = workflow::execute_workflow(&registry, 5).await?;
    
    // Check that we got a summary and profiles
    assert!(!summary.is_empty());
    assert!(summary.contains("Processed"));
    assert!(summary.contains("valid profiles"));
    
    // Check that we got some processed profiles
    assert!(!profiles.is_empty());
    
    // Check that profiles have display names
    for profile in &profiles {
        assert!(profile.display_name.is_some());
        assert!(!profile.display_name.as_ref().unwrap().is_empty());
    }
    
    Ok(())
}

#[test]
fn test_profile_validation() {
    // Test valid profile
    let valid_profile = workflow::UserProfile {
        id: "user-1".to_string(),
        name: "Alice Johnson".to_string(),
        email: "alice@example.com".to_string(),
        age: 25,
        display_name: None,
    };
    
    let result = validation::validate_profile(&valid_profile);
    assert!(result.is_valid);
    assert!(result.errors.is_empty());
    
    // Test invalid profile (invalid email)
    let invalid_profile = workflow::UserProfile {
        id: "user-2".to_string(),
        name: "Bob Smith".to_string(),
        email: "invalid-email".to_string(),
        age: 30,
        display_name: None,
    };
    
    let result = validation::validate_profile(&invalid_profile);
    assert!(!result.is_valid);
    assert!(!result.errors.is_empty());
    assert!(result.errors.contains(&"Invalid email format".to_string()));
}

#[test]
fn test_name_normalization() {
    assert_eq!(validation::normalize_name(" alice johnson "), "Alice Johnson");
    assert_eq!(validation::normalize_name("BOB SMITH"), "Bob Smith");
    assert_eq!(validation::normalize_name(""), "");
}

#[test]
fn test_mock_profile_generation() {
    let count = 3;
    let profiles = workflow::generate_mock_profiles(count);
    
    assert_eq!(profiles.len(), count);
    
    for profile in profiles {
        assert!(!profile.id.is_empty());
        assert!(!profile.name.is_empty());
        assert!(profile.email.contains("@"));
        assert!(profile.age >= 18 && profile.age <= 80);
    }
}