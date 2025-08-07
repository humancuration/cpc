//! Integration tests that actually execute the Shtairir demo applications
//!
//! These tests verify that the examples work correctly in a real environment.

use std::process::Command;
use std::time::Duration;

#[test]
fn test_data_processing_demo_execution() {
    // This test runs the data processing demo with a small dataset
    // and verifies it completes successfully
    
    let output = Command::new("cargo")
        .args(&["run", "-p", "shtairir_demos_data_processing", "--", "--test-mode"])
        .output()
        .expect("Failed to execute data processing demo");
    
    // In a real implementation, we would check the output
    // For now, we just verify it compiles and runs without panic
    if !output.status.success() {
        eprintln!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    // We're just testing that it doesn't crash, not validating output
    // In a production environment, we would add more comprehensive checks
    assert!(output.status.success() || output.status.code() == Some(0));
}

#[test]
fn test_user_profiles_demo_execution() {
    // This test runs the user profiles demo with a small dataset
    // and verifies it completes successfully
    
    let output = Command::new("cargo")
        .args(&["run", "-p", "shtairir_demos_user_profiles", "--", "--test-mode"])
        .output()
        .expect("Failed to execute user profiles demo");
    
    // In a real implementation, we would check the output
    // For now, we just verify it compiles and runs without panic
    if !output.status.success() {
        eprintln!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    // We're just testing that it doesn't crash, not validating output
    assert!(output.status.success() || output.status.code() == Some(0));
}

#[test]
fn test_ml_features_demo_execution() {
    // This test runs the ML features demo with a small dataset
    // and verifies it completes successfully
    
    let output = Command::new("cargo")
        .args(&["run", "-p", "shtairir_demos_ml_features", "--", "--test-mode"])
        .output()
        .expect("Failed to execute ML features demo");
    
    // In a real implementation, we would check the output
    // For now, we just verify it compiles and runs without panic
    if !output.status.success() {
        eprintln!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    // We're just testing that it doesn't crash, not validating output
    assert!(output.status.success() || output.status.code() == Some(0));
}
