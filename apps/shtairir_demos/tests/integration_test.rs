//! Integration tests for all Shtairir demo applications

use std::process::Command;

#[test]
fn test_data_processing_demo_compiles() {
    let output = Command::new("cargo")
        .args(&["check", "-p", "shtairir_demos_data_processing"])
        .output()
        .expect("Failed to execute cargo check");
    
    assert!(output.status.success(), "Data processing demo failed to compile: {}", 
            String::from_utf8_lossy(&output.stderr));
}

#[test]
fn test_user_profiles_demo_compiles() {
    let output = Command::new("cargo")
        .args(&["check", "-p", "shtairir_demos_user_profiles"])
        .output()
        .expect("Failed to execute cargo check");
    
    assert!(output.status.success(), "User profiles demo failed to compile: {}", 
            String::from_utf8_lossy(&output.stderr));
}

#[test]
fn test_ml_features_demo_compiles() {
    let output = Command::new("cargo")
        .args(&["check", "-p", "shtairir_demos_ml_features"])
        .output()
        .expect("Failed to execute cargo check");
    
    assert!(output.status.success(), "ML features demo failed to compile: {}", 
            String::from_utf8_lossy(&output.stderr));
}