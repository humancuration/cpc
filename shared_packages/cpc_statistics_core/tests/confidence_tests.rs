//! Tests for confidence interval calculations

use cpc_statistics_core::{ConfidenceCalculator, ConfidenceMethod};

#[test]
fn test_parametric_confidence_interval() {
    // Generate test data with known properties
    let data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    
    let ci = ConfidenceCalculator::parametric_interval(&data, 0.95).unwrap();
    
    assert!(ci.lower < ci.upper);
    assert_eq!(ci.confidence_level, 0.95);
    assert_eq!(ci.method, ConfidenceMethod::Parametric);
    assert_eq!(ci.sample_size, 10);
    
    // Test explanation generation
    let explanation = ci.explanation("test measurement");
    assert!(explanation.contains("95% probability"));
    assert!(explanation.contains("test measurement"));
}

#[test]
fn test_bootstrap_confidence_interval() {
    // Generate test data
    let data: Vec<f64> = (1..=100).map(|x| x as f64).collect();
    
    let ci = ConfidenceCalculator::bootstrap_interval(&data, 0.95, 1000).unwrap();
    
    assert!(ci.lower < ci.upper);
    assert_eq!(ci.confidence_level, 0.95);
    assert_eq!(ci.method, ConfidenceMethod::Bootstrap);
    assert_eq!(ci.sample_size, 100);
}

#[test]
fn test_effect_size_calculation() {
    let group1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let group2 = vec![3.0, 4.0, 5.0, 6.0, 7.0];
    
    let effect_size = ConfidenceCalculator::effect_size(&group1, &group2).unwrap();
    
    assert!(effect_size.is_finite());
    // Effect size should be negative since group2 has higher mean
    assert!(effect_size < 0.0);
}

#[test]
fn test_insufficient_data_error() {
    let small_data: Vec<f64> = vec![1.0];
    
    let result = ConfidenceCalculator::parametric_interval(&small_data, 0.95);
    assert!(result.is_err());
    
    match result.unwrap_err() {
        cpc_statistics_core::StatisticalError::InsufficientData(observed, required) => {
            assert_eq!(observed, 1);
            assert_eq!(required, 2);
        }
        _ => panic!("Expected InsufficientData error"),
    }
}