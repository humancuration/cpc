//! Confidence interval calculations
//!
//! This module provides various methods for calculating confidence intervals
//! that align with cooperative values and platform requirements.

use crate::error::StatisticalError;
use statrs::distribution::{Normal, StudentsT};
use statrs::statistics::Statistics;
use ndarray::Array1;
use polars::prelude::*;

/// Methods for calculating confidence intervals
#[derive(Debug, Clone, PartialEq)]
pub enum ConfidenceMethod {
    /// Bootstrapping method for small datasets (<1000 samples)
    Bootstrap,
    /// Parametric method for known distributions
    Parametric,
    /// Bayesian approach for fundraising impact analysis
    Bayesian,
}

/// Represents a confidence interval with lower and upper bounds
#[derive(Debug, Clone)]
pub struct ConfidenceInterval {
    /// Lower bound of the confidence interval
    pub lower: f64,
    /// Upper bound of the confidence interval
    pub upper: f64,
    /// Confidence level (e.g., 0.95 for 95% confidence)
    pub confidence_level: f64,
    /// Method used to calculate the interval
    pub method: ConfidenceMethod,
    /// Sample size used
    pub sample_size: usize,
}

impl ConfidenceInterval {
    /// Create a new confidence interval
    pub fn new(
        lower: f64,
        upper: f64,
        confidence_level: f64,
        method: ConfidenceMethod,
        sample_size: usize,
    ) -> Self {
        Self {
            lower,
            upper,
            confidence_level,
            method,
            sample_size,
        }
    }
    
    /// Calculate the width of the confidence interval
    pub fn width(&self) -> f64 {
        self.upper - self.lower
    }
    
    /// Check if the confidence interval contains zero
    pub fn contains_zero(&self) -> bool {
        self.lower <= 0.0 && self.upper >= 0.0
    }
    
    /// Generate a plain-language explanation of the confidence interval
    pub fn explanation(&self, context: &str) -> String {
        format!(
            "Based on available data{}, there's a {:.0}% probability that the true {} is between {:.2} and {:.2}.",
            if self.sample_size < 30 { " (small sample size)" } else { "" },
            self.confidence_level * 100.0,
            context,
            self.lower,
            self.upper
        )
    }
}

/// Confidence interval calculator
pub struct ConfidenceCalculator;

impl ConfidenceCalculator {
    /// Calculate confidence interval using bootstrap method
    pub fn bootstrap_interval(
        data: &[f64],
        confidence_level: f64,
        n_bootstrap: usize,
    ) -> Result<ConfidenceInterval, StatisticalError> {
        if data.len() < 10 {
            return Err(StatisticalError::InsufficientData(data.len(), 10));
        }
        
        // Bootstrap sampling
        let mut bootstrap_means = Vec::with_capacity(n_bootstrap);
        let mut rng = rand::thread_rng();
        
        for _ in 0..n_bootstrap {
            let bootstrap_sample: Vec<f64> = (0..data.len())
                .map(|_| data[rand::Rng::gen_range(&mut rng, 0..data.len())])
                .collect();
            
            let mean = bootstrap_sample.mean();
            bootstrap_means.push(mean);
        }
        
        // Sort bootstrap means
        bootstrap_means.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        // Calculate percentiles
        let lower_percentile = (1.0 - confidence_level) / 2.0;
        let upper_percentile = 1.0 - lower_percentile;
        
        let lower_index = (lower_percentile * n_bootstrap as f64) as usize;
        let upper_index = (upper_percentile * n_bootstrap as f64) as usize;
        
        let lower_bound = bootstrap_means[lower_index];
        let upper_bound = bootstrap_means[upper_index];
        
        Ok(ConfidenceInterval::new(
            lower_bound,
            upper_bound,
            confidence_level,
            ConfidenceMethod::Bootstrap,
            data.len(),
        ))
    }
    
    /// Calculate confidence interval using parametric method
    pub fn parametric_interval(
        data: &[f64],
        confidence_level: f64,
    ) -> Result<ConfidenceInterval, StatisticalError> {
        if data.len() < 2 {
            return Err(StatisticalError::InsufficientData(data.len(), 2));
        }
        
        let mean = data.mean();
        let std_dev = data.std_dev();
        let n = data.len() as f64;
        
        // For small samples, use t-distribution
        let (lower_bound, upper_bound) = if n < 30.0 {
            let df = n - 1.0;
            let t_dist = StudentsT::new(0.0, 1.0, df)
                .map_err(|e| StatisticalError::StatrsError(e))?;
            
            let alpha = 1.0 - confidence_level;
            let t_critical = t_dist.inverse_cdf(1.0 - alpha / 2.0);
            
            let margin_of_error = t_critical * (std_dev / n.sqrt());
            (mean - margin_of_error, mean + margin_of_error)
        } else {
            // For large samples, use normal distribution
            let normal = Normal::new(0.0, 1.0)
                .map_err(|e| StatisticalError::StatrsError(e))?;
            
            let alpha = 1.0 - confidence_level;
            let z_critical = normal.inverse_cdf(1.0 - alpha / 2.0);
            
            let margin_of_error = z_critical * (std_dev / n.sqrt());
            (mean - margin_of_error, mean + margin_of_error)
        };
        
        Ok(ConfidenceInterval::new(
            lower_bound,
            upper_bound,
            confidence_level,
            ConfidenceMethod::Parametric,
            data.len(),
        ))
    }
    
    /// Calculate confidence interval using Bayesian approach
    pub fn bayesian_interval(
        data: &[f64],
        confidence_level: f64,
        prior_mean: f64,
        prior_std: f64,
    ) -> Result<ConfidenceInterval, StatisticalError> {
        if data.len() < 5 {
            return Err(StatisticalError::InsufficientData(data.len(), 5));
        }
        
        let sample_mean = data.mean();
        let sample_std = data.std_dev();
        let n = data.len() as f64;
        
        // Bayesian update using conjugate priors
        let posterior_precision = 1.0 / (prior_std * prior_std) + n / (sample_std * sample_std);
        let posterior_std = (1.0 / posterior_precision).sqrt();
        
        let posterior_mean = (
            prior_mean / (prior_std * prior_std) + 
            n * sample_mean / (sample_std * sample_std)
        ) / posterior_precision;
        
        // Calculate credible interval
        let normal = Normal::new(0.0, 1.0)
            .map_err(|e| StatisticalError::StatrsError(e))?;
        
        let alpha = 1.0 - confidence_level;
        let z_critical = normal.inverse_cdf(1.0 - alpha / 2.0);
        
        let margin_of_error = z_critical * posterior_std;
        let lower_bound = posterior_mean - margin_of_error;
        let upper_bound = posterior_mean + margin_of_error;
        
        Ok(ConfidenceInterval::new(
            lower_bound,
            upper_bound,
            confidence_level,
            ConfidenceMethod::Bayesian,
            data.len(),
        ))
    }
    
    /// Calculate effect size (Cohen's d)
    pub fn effect_size(
        group1: &[f64],
        group2: &[f64],
    ) -> Result<f64, StatisticalError> {
        if group1.len() < 2 || group2.len() < 2 {
            return Err(StatisticalError::InsufficientData(
                group1.len().min(group2.len()),
                2,
            ));
        }
        
        let mean1 = group1.mean();
        let mean2 = group2.mean();
        let std1 = group1.std_dev();
        let std2 = group2.std_dev();
        
        let pooled_std = ((std1 * std1 + std2 * std2) / 2.0).sqrt();
        
        if pooled_std == 0.0 {
            return Ok(0.0);
        }
        
        Ok((mean1 - mean2) / pooled_std)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_confidence_interval_creation() {
        let ci = ConfidenceInterval::new(
            10.0,
            20.0,
            0.95,
            ConfidenceMethod::Parametric,
            100,
        );
        
        assert_eq!(ci.lower, 10.0);
        assert_eq!(ci.upper, 20.0);
        assert_eq!(ci.confidence_level, 0.95);
        assert_eq!(ci.method, ConfidenceMethod::Parametric);
        assert_eq!(ci.sample_size, 100);
        assert_eq!(ci.width(), 10.0);
    }
    
    #[test]
    fn test_confidence_interval_contains_zero() {
        let ci_positive = ConfidenceInterval::new(5.0, 15.0, 0.95, ConfidenceMethod::Parametric, 50);
        let ci_negative = ConfidenceInterval::new(-15.0, -5.0, 0.95, ConfidenceMethod::Parametric, 50);
        let ci_zero = ConfidenceInterval::new(-5.0, 5.0, 0.95, ConfidenceMethod::Parametric, 50);
        
        assert!(!ci_positive.contains_zero());
        assert!(!ci_negative.contains_zero());
        assert!(ci_zero.contains_zero());
    }
    
    #[test]
    fn test_parametric_interval() {
        let data: Vec<f64> = (0..100).map(|i| 50.0 + (i as f64 - 50.0) * 0.1).collect();
        let ci = ConfidenceCalculator::parametric_interval(&data, 0.95).unwrap();
        
        assert!(ci.lower < ci.upper);
        assert!(ci.confidence_level > 0.9);
        assert_eq!(ci.method, ConfidenceMethod::Parametric);
    }
    
    #[test]
    fn test_effect_size() {
        let group1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let group2 = vec![2.0, 3.0, 4.0, 5.0, 6.0];
        
        let effect_size = ConfidenceCalculator::effect_size(&group1, &group2).unwrap();
        assert!(effect_size.is_finite());
    }
}