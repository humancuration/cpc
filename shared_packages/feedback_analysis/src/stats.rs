//! Common statistical functions for feedback analysis

use feedback_core::FeedbackError;

/// Calculate correlation between two variables
pub fn calculate_correlation(data: &[(f32, f32)]) -> Result<Option<f32>, FeedbackError> {
    if data.len() < 2 {
        return Ok(None);
    }
    
    // Calculate means
    let (sum1, sum2): (f32, f32) = data.iter().fold((0.0, 0.0), |(s1, s2), &(v1, v2)| (s1 + v1, s2 + v2));
    let mean1 = sum1 / data.len() as f32;
    let mean2 = sum2 / data.len() as f32;
    
    // Calculate correlation coefficient
    let mut numerator = 0.0;
    let mut denom1 = 0.0;
    let mut denom2 = 0.0;
    
    for &(v1, v2) in data {
        let diff1 = v1 - mean1;
        let diff2 = v2 - mean2;
        numerator += diff1 * diff2;
        denom1 += diff1 * diff1;
        denom2 += diff2 * diff2;
    }
    
    if denom1 == 0.0 || denom2 == 0.0 {
        return Ok(None);
    }
    
    let correlation = numerator / (denom1 * denom2).sqrt();
    
    // Ensure the correlation is within valid range [-1, 1]
    let correlation = correlation.max(-1.0).min(1.0);
    
    Ok(Some(correlation))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_correlation() {
        let data = vec![(1.0, 2.0), (2.0, 4.0), (3.0, 6.0), (4.0, 8.0)];
        let result = calculate_correlation(&data).unwrap();
        assert_eq!(result, Some(1.0)); // Perfect positive correlation
    }

    #[test]
    fn test_calculate_correlation_negative() {
        let data = vec![(1.0, 8.0), (2.0, 6.0), (3.0, 4.0), (4.0, 2.0)];
        let result = calculate_correlation(&data).unwrap();
        assert_eq!(result, Some(-1.0)); // Perfect negative correlation
    }

    #[test]
    fn test_calculate_correlation_insufficient_data() {
        let data = vec![(1.0, 2.0)];
        let result = calculate_correlation(&data).unwrap();
        assert_eq!(result, None); // Not enough data
    }

    #[test]
    fn test_calculate_correlation_no_variance() {
        let data = vec![(1.0, 1.0), (1.0, 1.0), (1.0, 1.0)];
        let result = calculate_correlation(&data).unwrap();
        assert_eq!(result, None); // No variance in data
    }
}