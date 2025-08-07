//! Statistical significance testing and result interpretation
//!
//! This module provides tools for determining statistical significance
//! and presenting results in cooperative values-aligned plain language.

use crate::error::StatisticalError;
use statrs::distribution::Normal;
use statrs::statistics::Statistics;

/// Statistical significance levels with cooperative-aligned color coding
#[derive(Debug, Clone, PartialEq)]
pub enum SignificanceLevel {
    /// Highly significant (p < 0.01) - Green
    HighlySignificant,
    /// Moderately significant (0.01 ≤ p < 0.05) - Yellow
    ModeratelySignificant,
    /// Not significant (p ≥ 0.05) - Red
    NotSignificant,
}

impl SignificanceLevel {
    /// Get the color associated with this significance level
    pub fn color(&self) -> &'static str {
        match self {
            SignificanceLevel::HighlySignificant => "green",
            SignificanceLevel::ModeratelySignificant => "yellow",
            SignificanceLevel::NotSignificant => "red",
        }
    }
    
    /// Get a plain-language description of this significance level
    pub fn description(&self) -> &'static str {
        match self {
            SignificanceLevel::HighlySignificant => "strong evidence",
            SignificanceLevel::ModeratelySignificant => "moderate evidence",
            SignificanceLevel::NotSignificant => "insufficient evidence",
        }
    }
}

/// Result of a significance test
#[derive(Debug, Clone)]
pub struct SignificanceResult {
    /// The p-value from the test
    pub p_value: f64,
    /// The significance level classification
    pub level: SignificanceLevel,
    /// The test statistic
    pub statistic: f64,
    /// Degrees of freedom (if applicable)
    pub degrees_of_freedom: Option<f64>,
}

impl SignificanceResult {
    /// Create a new significance result
    pub fn new(
        p_value: f64,
        statistic: f64,
        degrees_of_freedom: Option<f64>,
    ) -> Self {
        let level = if p_value < 0.01 {
            SignificanceLevel::HighlySignificant
        } else if p_value < 0.05 {
            SignificanceLevel::ModeratelySignificant
        } else {
            SignificanceLevel::NotSignificant
        };
        
        Self {
            p_value,
            level,
            statistic,
            degrees_of_freedom,
        }
    }
    
    /// Generate a plain-language explanation of the significance result
    pub fn explanation(&self, context: &str) -> String {
        let evidence_strength = self.level.description();
        let significance_desc = match self.level {
            SignificanceLevel::HighlySignificant => "statistically highly significant",
            SignificanceLevel::ModeratelySignificant => "statistically moderately significant",
            SignificanceLevel::NotSignificant => "not statistically significant",
        };
        
        format!(
            "There is {} for {} (p = {:.4}). This result is {}.",
            evidence_strength,
            context,
            self.p_value,
            significance_desc
        )
    }
}

/// Statistical significance tester
pub struct SignificanceTester;

impl SignificanceTester {
    /// Perform a one-sample t-test
    pub fn one_sample_t_test(
        data: &[f64],
        hypothesized_mean: f64,
    ) -> Result<SignificanceResult, StatisticalError> {
        if data.len() < 2 {
            return Err(StatisticalError::InsufficientData(data.len(), 2));
        }
        
        let sample_mean = data.mean();
        let sample_std = data.std_dev();
        let n = data.len() as f64;
        
        // Calculate t-statistic
        let t_stat = (sample_mean - hypothesized_mean) / (sample_std / n.sqrt());
        
        // Calculate p-value (two-tailed test)
        let df = n - 1.0;
        // For simplicity, we'll approximate with normal distribution for large samples
        let p_value = if n > 30.0 {
            let normal = Normal::new(0.0, 1.0)
                .map_err(|e| StatisticalError::StatrsError(e))?;
            2.0 * (1.0 - normal.cdf(t_stat.abs()))
        } else {
            // For small samples, this would use t-distribution
            // This is a simplified approximation
            let normal = Normal::new(0.0, 1.0)
                .map_err(|e| StatisticalError::StatrsError(e))?;
            2.0 * (1.0 - normal.cdf(t_stat.abs()))
        };
        
        Ok(SignificanceResult::new(
            p_value,
            t_stat,
            Some(df),
        ))
    }
    
    /// Perform a two-sample t-test
    pub fn two_sample_t_test(
        group1: &[f64],
        group2: &[f64],
    ) -> Result<SignificanceResult, StatisticalError> {
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
        let n1 = group1.len() as f64;
        let n2 = group2.len() as f64;
        
        // Calculate pooled standard error
        let pooled_se = ((std1 * std1 / n1) + (std2 * std2 / n2)).sqrt();
        
        // Calculate t-statistic
        let t_stat = (mean1 - mean2) / pooled_se;
        
        // Calculate degrees of freedom using Welch's formula
        let df = ((std1 * std1 / n1 + std2 * std2 / n2).powi(2)) /
            ((std1 * std1 / n1).powi(2) / (n1 - 1.0) + (std2 * std2 / n2).powi(2) / (n2 - 1.0));
        
        // Calculate p-value (two-tailed test)
        let normal = Normal::new(0.0, 1.0)
            .map_err(|e| StatisticalError::StatrsError(e))?;
        let p_value = 2.0 * (1.0 - normal.cdf(t_stat.abs()));
        
        Ok(SignificanceResult::new(
            p_value,
            t_stat,
            Some(df),
        ))
    }
    
    /// Perform a correlation test
    pub fn correlation_test(
        x: &[f64],
        y: &[f64],
    ) -> Result<SignificanceResult, StatisticalError> {
        if x.len() != y.len() {
            return Err(StatisticalError::InvalidInput(
                "Arrays must have the same length".to_string()
            ));
        }
        
        if x.len() < 3 {
            return Err(StatisticalError::InsufficientData(x.len(), 3));
        }
        
        // Calculate Pearson correlation coefficient
        let correlation = x.pearson_correlation(y);
        
        // Calculate t-statistic for correlation
        let n = x.len() as f64;
        let df = n - 2.0;
        let t_stat = correlation * ((n - 2.0) / (1.0 - correlation * correlation)).sqrt();
        
        // Calculate p-value (two-tailed test)
        let normal = Normal::new(0.0, 1.0)
            .map_err(|e| StatisticalError::StatrsError(e))?;
        let p_value = 2.0 * (1.0 - normal.cdf(t_stat.abs()));
        
        Ok(SignificanceResult::new(
            p_value,
            t_stat,
            Some(df),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_significance_level_color() {
        let highly = SignificanceLevel::HighlySignificant;
        let moderately = SignificanceLevel::ModeratelySignificant;
        let not = SignificanceLevel::NotSignificant;
        
        assert_eq!(highly.color(), "green");
        assert_eq!(moderately.color(), "yellow");
        assert_eq!(not.color(), "red");
    }
    
    #[test]
    fn test_significance_result_creation() {
        let result = SignificanceResult::new(0.005, 2.5, Some(20.0));
        assert_eq!(result.level, SignificanceLevel::HighlySignificant);
        
        let result = SignificanceResult::new(0.03, 2.0, Some(20.0));
        assert_eq!(result.level, SignificanceLevel::ModeratelySignificant);
        
        let result = SignificanceResult::new(0.1, 1.5, Some(20.0));
        assert_eq!(result.level, SignificanceLevel::NotSignificant);
    }
    
    #[test]
    fn test_one_sample_t_test() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = SignificanceTester::one_sample_t_test(&data, 3.0).unwrap();
        
        assert!(result.p_value > 0.0);
        assert!(result.p_value < 1.0);
    }
    
    #[test]
    fn test_two_sample_t_test() {
        let group1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let group2 = vec![2.0, 3.0, 4.0, 5.0, 6.0];
        let result = SignificanceTester::two_sample_t_test(&group1, &group2).unwrap();
        
        assert!(result.p_value > 0.0);
        assert!(result.p_value < 1.0);
    }
}