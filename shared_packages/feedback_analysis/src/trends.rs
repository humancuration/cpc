//! Time-series analysis for feedback trends

use chrono::{DateTime, Utc, NaiveDate};
use feedback_core::FeedbackError;

/// Result of trend analysis
#[derive(Debug, Clone)]
pub struct TrendResult {
    pub periods: Vec<String>,
    pub averages: Vec<f32>,
    pub counts: Vec<usize>,
}

/// Time period for trend analysis
#[derive(Debug, Clone)]
pub enum TimePeriod {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl TrendResult {
    /// Create a new trend result
    pub fn new() -> Self {
        Self {
            periods: Vec::new(),
            averages: Vec::new(),
            counts: Vec::new(),
        }
    }
    
    /// Add a data point to the trend
    pub fn add_point(&mut self, period: String, average: f32, count: usize) {
        self.periods.push(period);
        self.averages.push(average);
        self.counts.push(count);
    }
    
    /// Get the number of data points
    pub fn len(&self) -> usize {
        self.periods.len()
    }
    
    /// Check if there are no data points
    pub fn is_empty(&self) -> bool {
        self.periods.is_empty()
    }
}

impl Default for TrendResult {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trend_result() {
        let mut trend = TrendResult::new();
        
        assert!(trend.is_empty());
        
        trend.add_point("2023-01".to_string(), 4.5, 10);
        trend.add_point("2023-02".to_string(), 4.2, 15);
        
        assert_eq!(trend.len(), 2);
        assert_eq!(trend.periods[0], "2023-01");
        assert_eq!(trend.averages[1], 4.2);
        assert_eq!(trend.counts[0], 10);
    }
}