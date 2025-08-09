//! Tests for statistical error handling

use cpc_statistics_core::StatisticalError;

#[test]
fn test_error_user_messages() {
    // Test InsufficientData error message
    let error = StatisticalError::InsufficientData(5, 10);
    let message = error.user_message();
    assert!(message.contains("Analysis requires 10 data points (only 5 available)"));
    assert!(message.contains("Collect more data points"));
    
    // Test NonNormalDistribution error message
    let error = StatisticalError::NonNormalDistribution;
    let message = error.user_message();
    assert!(message.contains("does not follow a normal distribution"));
    assert!(message.contains("Check for data entry errors"));
    
    // Test OutlierContamination error message
    let error = StatisticalError::OutlierContamination(12.5);
    let message = error.user_message();
    assert!(message.contains("contaminated by outliers (12.50%"));
    assert!(message.contains("Review data for entry errors"));
}

#[test]
fn test_error_methodology_sources() {
    // Test methodology source for different errors
    let insufficient_data = StatisticalError::InsufficientData(5, 10);
    assert!(insufficient_data.methodology_source().contains("Sample Size Requirements"));
    
    let non_normal = StatisticalError::NonNormalDistribution;
    assert!(non_normal.methodology_source().contains("Distribution Testing"));
    
    let outlier = StatisticalError::OutlierContamination(10.0);
    assert!(outlier.methodology_source().contains("Outlier Detection"));
}