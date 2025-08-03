//! Rating distribution calculations for feedback analysis

use std::collections::HashMap;
use feedback_core::FeedbackError;

/// Rating distribution for a specific metric
#[derive(Debug, Clone)]
pub struct RatingDistribution {
    pub metric: String,
    pub values: HashMap<u8, u32>, // 0-100 scale
}

impl RatingDistribution {
    /// Create a new rating distribution
    pub fn new(metric: String) -> Self {
        Self {
            metric,
            values: HashMap::new(),
        }
    }
    
    /// Add a rating to the distribution
    pub fn add_rating(&mut self, rating: f32) -> Result<(), FeedbackError> {
        if rating < 0.0 || rating > 1.0 {
            return Err(FeedbackError::Validation(
                "Rating must be between 0.0 and 1.0".to_string()
            ));
        }
        
        // Convert 0.0-1.0 scale to 0-100 scale
        let scaled_value = (rating * 100.0).round() as u8;
        *self.values.entry(scaled_value).or_insert(0) += 1;
        
        Ok(())
    }
    
    /// Get the count for a specific rating value
    pub fn get_count(&self, value: u8) -> u32 {
        self.values.get(&value).copied().unwrap_or(0)
    }
    
    /// Get all rating values sorted
    pub fn sorted_values(&self) -> Vec<(u8, u32)> {
        let mut values: Vec<(u8, u32)> = self.values.iter().map(|(&k, &v)| (k, v)).collect();
        values.sort_by_key(|&(k, _)| k);
        values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rating_distribution() {
        let mut dist = RatingDistribution::new("quality".to_string());
        
        // Add some ratings
        dist.add_rating(0.8).unwrap(); // 80
        dist.add_rating(0.8).unwrap(); // 80
        dist.add_rating(0.9).unwrap(); // 90
        dist.add_rating(0.7).unwrap(); // 70
        
        assert_eq!(dist.get_count(80), 2);
        assert_eq!(dist.get_count(90), 1);
        assert_eq!(dist.get_count(70), 1);
        
        let sorted = dist.sorted_values();
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0], (70, 1));
        assert_eq!(sorted[1], (80, 2));
        assert_eq!(sorted[2], (90, 1));
    }
    
    #[test]
    fn test_rating_distribution_invalid_rating() {
        let mut dist = RatingDistribution::new("quality".to_string());
        
        // Try to add an invalid rating
        let result = dist.add_rating(1.5);
        assert!(result.is_err());
    }
}