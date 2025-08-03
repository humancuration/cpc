//! Filtering system for reviews
//!
//! This module provides a flexible filtering system for querying reviews
//! based on various criteria including entity, metrics, attributes, and demographics.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::models::{RatingMethod, Entity};

/// Filters for querying reviews
///
/// This struct allows for flexible filtering of reviews based on multiple criteria.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReviewFilters {
    /// Filter by specific entity ID
    pub entity_id: Option<Uuid>,
    
    /// Filter by entity type (e.g., "product", "service")
    pub entity_type: Option<String>,
    
    /// Filter by specific metrics and their values
    pub metrics: Vec<MetricFilter>,
    
    /// Filter by specific attributes
    pub attributes: Vec<AttributeFilter>,
    
    /// Filter by demographic information
    pub demographics: Option<DemographicFilter>,
    
    /// Filter by date range
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    
    /// Limit the number of results
    pub limit: Option<u32>,
    
    /// Offset for pagination
    pub offset: Option<u32>,
}

impl ReviewFilters {
    /// Create a new, empty filter set
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the entity ID filter
    pub fn with_entity_id(mut self, entity_id: Uuid) -> Self {
        self.entity_id = Some(entity_id);
        self
    }
    
    /// Set the entity type filter
    pub fn with_entity_type(mut self, entity_type: impl Into<String>) -> Self {
        self.entity_type = Some(entity_type.into());
        self
    }
    
    /// Add a metric filter
    pub fn with_metric(mut self, metric: MetricFilter) -> Self {
        self.metrics.push(metric);
        self
    }
    
    /// Add an attribute filter
    pub fn with_attribute(mut self, attribute: AttributeFilter) -> Self {
        self.attributes.push(attribute);
        self
    }
    
    /// Set the demographic filter
    pub fn with_demographics(mut self, demographics: DemographicFilter) -> Self {
        self.demographics = Some(demographics);
        self
    }
    
    /// Set the date range filter
    pub fn with_date_range(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.date_range = Some((start, end));
        self
    }
    
    /// Set the limit for results
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
    
    /// Set the offset for pagination
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }
}

/// Filter for rating metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricFilter {
    /// The metric name to filter by
    pub metric: String,
    
    /// The comparison operator
    pub operator: ComparisonOperator,
    
    /// The value to compare against
    pub value: f32,
    
    /// Optional method filter
    pub method: Option<RatingMethod>,
}

/// Comparison operators for filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    /// Equal to
    Equal,
    
    /// Not equal to
    NotEqual,
    
    /// Greater than
    GreaterThan,
    
    /// Less than
    LessThan,
    
    /// Greater than or equal to
    GreaterThanOrEqual,
    
    /// Less than or equal to
    LessThanOrEqual,
}

/// Filter for attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeFilter {
    /// The attribute key to filter by
    pub key: String,
    
    /// The attribute value to filter by
    pub value: String,
    
    /// Whether to match exactly or partially
    pub exact_match: bool,
}

/// Filter for demographic information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemographicFilter {
    /// Age group filter
    pub age_group: Option<String>,
    
    /// Gender filter
    pub gender: Option<String>,
    
    /// Location filter
    pub location: Option<String>,
    
    /// Occupation filter
    pub occupation: Option<String>,
}

/// Visitor pattern for filter traversal
///
/// This trait allows for implementing different behaviors when processing filters.
pub trait FilterVisitor {
    /// Visit a metric filter
    fn visit_metric(&mut self, filter: &MetricFilter);
    
    /// Visit an attribute filter
    fn visit_attribute(&mut self, filter: &AttributeFilter);
    
    /// Visit a demographic filter
    fn visit_demographic(&mut self, filter: &DemographicFilter);
}

/// A visitor that counts the number of filters
pub struct FilterCounter {
    pub count: usize,
}

impl FilterCounter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl FilterVisitor for FilterCounter {
    fn visit_metric(&mut self, _filter: &MetricFilter) {
        self.count += 1;
    }
    
    fn visit_attribute(&mut self, _filter: &AttributeFilter) {
        self.count += 1;
    }
    
    fn visit_demographic(&mut self, _filter: &DemographicFilter) {
        self.count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_filters_builder_pattern() {
        let filters = ReviewFilters::new()
            .with_entity_type("product")
            .with_limit(10)
            .with_offset(0);
        
        assert_eq!(filters.entity_type, Some("product".to_string()));
        assert_eq!(filters.limit, Some(10));
        assert_eq!(filters.offset, Some(0));
    }
    
    #[test]
    fn test_metric_filter() {
        let filter = MetricFilter {
            metric: "quality".to_string(),
            operator: ComparisonOperator::GreaterThan,
            value: 0.8,
            method: Some(RatingMethod::UserReported),
        };
        
        assert_eq!(filter.metric, "quality");
        assert_eq!(filter.value, 0.8);
    }
    
    #[test]
    fn test_filter_counter() {
        let mut counter = FilterCounter::new();
        let metric_filter = MetricFilter {
            metric: "quality".to_string(),
            operator: ComparisonOperator::GreaterThan,
            value: 0.8,
            method: None,
        };
        
        let attribute_filter = AttributeFilter {
            key: "color".to_string(),
            value: "blue".to_string(),
            exact_match: true,
        };
        
        counter.visit_metric(&metric_filter);
        counter.visit_attribute(&attribute_filter);
        
        assert_eq!(counter.count, 2);
    }
}