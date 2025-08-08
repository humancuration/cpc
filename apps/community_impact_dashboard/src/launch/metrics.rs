//! Launch impact measurement
//!
//! This module provides functionality for measuring the impact of the dashboard launch
//! itself, including adoption rates, validation participation, and community engagement.

use tracing::info;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Launch metrics collector and analyzer
pub struct LaunchMetrics {
    metrics: HashMap<String, LaunchMetric>,
    events: Vec<LaunchEvent>,
}

impl LaunchMetrics {
    /// Create a new launch metrics collector
    pub fn new() -> Self {
        Self {
            metrics: HashMap::new(),
            events: Vec::new(),
        }
    }
    
    /// Record a launch metric
    pub fn record_metric(&mut self, name: &str, value: f64, category: MetricCategory) {
        let metric = LaunchMetric {
            name: name.to_string(),
            value,
            category,
            recorded_at: Utc::now(),
        };
        
        self.metrics.insert(name.to_string(), metric);
        info!("Recorded launch metric: {} = {}", name, value);
    }
    
    /// Record a launch event
    pub fn record_event(&mut self, event_type: LaunchEventType, description: &str, user_id: Option<String>) {
        let event = LaunchEvent {
            id: Uuid::new_v4(),
            event_type,
            description: description.to_string(),
            user_id,
            timestamp: Utc::now(),
        };
        
        self.events.push(event);
        info!("Recorded launch event: {:?} - {}", event_type, description);
    }
    
    /// Get a specific metric by name
    pub fn get_metric(&self, name: &str) -> Option<&LaunchMetric> {
        self.metrics.get(name)
    }
    
    /// Get all metrics in a specific category
    pub fn get_metrics_by_category(&self, category: MetricCategory) -> Vec<&LaunchMetric> {
        self.metrics.values()
            .filter(|metric| metric.category == category)
            .collect()
    }
    
    /// Calculate adoption rate
    pub fn calculate_adoption_rate(&self, total_community_members: f64) -> Option<f64> {
        self.metrics.get("active_users")
            .map(|metric| (metric.value / total_community_members) * 100.0)
    }
    
    /// Calculate community validation participation rate
    pub fn calculate_validation_participation_rate(&self, total_community_members: f64) -> Option<f64> {
        self.metrics.get("validation_participants")
            .map(|metric| (metric.value / total_community_members) * 100.0)
    }
    
    /// Get recent events
    pub fn get_recent_events(&self, hours: i64) -> Vec<&LaunchEvent> {
        let cutoff = Utc::now() - chrono::Duration::hours(hours);
        self.events.iter()
            .filter(|event| event.timestamp > cutoff)
            .collect()
    }
    
    /// Generate a launch impact report
    pub fn generate_report(&self, total_community_members: f64) -> LaunchImpactReport {
        let adoption_rate = self.calculate_adoption_rate(total_community_members);
        let validation_rate = self.calculate_validation_participation_rate(total_community_members);
        
        let total_events = self.events.len();
        let validation_events = self.events.iter()
            .filter(|event| matches!(event.event_type, LaunchEventType::CommunityValidation))
            .count();
        
        LaunchImpactReport {
            generated_at: Utc::now(),
            adoption_rate,
            validation_participation_rate: validation_rate,
            total_events,
            validation_events,
            metrics_summary: self.get_metrics_summary(),
        }
    }
    
    /// Get a summary of all metrics
    fn get_metrics_summary(&self) -> HashMap<MetricCategory, MetricSummary> {
        let mut summary: HashMap<MetricCategory, MetricSummary> = HashMap::new();
        
        for metric in self.metrics.values() {
            let entry = summary.entry(metric.category).or_insert_with(|| MetricSummary {
                count: 0,
                total_value: 0.0,
                average_value: 0.0,
            });
            
            entry.count += 1;
            entry.total_value += metric.value;
        }
        
        // Calculate averages
        for summary_entry in summary.values_mut() {
            if summary_entry.count > 0 {
                summary_entry.average_value = summary_entry.total_value / summary_entry.count as f64;
            }
        }
        
        summary
    }
}

impl Default for LaunchMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Launch metric structure
#[derive(Debug, Clone)]
pub struct LaunchMetric {
    /// Name of the metric
    pub name: String,
    
    /// Metric value
    pub value: f64,
    
    /// Category of the metric
    pub category: MetricCategory,
    
    /// When the metric was recorded
    pub recorded_at: DateTime<Utc>,
}

/// Categories of launch metrics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetricCategory {
    /// Adoption metrics
    Adoption,
    
    /// Engagement metrics
    Engagement,
    
    /// Validation metrics
    Validation,
    
    /// Understanding metrics
    Understanding,
    
    /// Technical performance metrics
    Performance,
}

/// Launch event structure
#[derive(Debug, Clone)]
pub struct LaunchEvent {
    /// Unique identifier for the event
    pub id: Uuid,
    
    /// Type of event
    pub event_type: LaunchEventType,
    
    /// Description of the event
    pub description: String,
    
    /// User associated with the event (if applicable)
    pub user_id: Option<String>,
    
    /// When the event occurred
    pub timestamp: DateTime<Utc>,
}

/// Types of launch events
#[derive(Debug, Clone)]
pub enum LaunchEventType {
    /// Dashboard access
    DashboardAccess,
    
    /// Community validation session
    CommunityValidation,
    
    /// Feedback submission
    FeedbackSubmission,
    
    /// Story contribution
    StoryContribution,
    
    /// Onboarding completion
    OnboardingCompletion,
    
    /// Technical issue
    TechnicalIssue,
}

/// Launch impact report
#[derive(Debug, Clone)]
pub struct LaunchImpactReport {
    /// When the report was generated
    pub generated_at: DateTime<Utc>,
    
    /// Adoption rate percentage
    pub adoption_rate: Option<f64>,
    
    /// Community validation participation rate percentage
    pub validation_participation_rate: Option<f64>,
    
    /// Total number of events recorded
    pub total_events: usize,
    
    /// Number of validation events
    pub validation_events: usize,
    
    /// Summary of metrics by category
    pub metrics_summary: HashMap<MetricCategory, MetricSummary>,
}

/// Summary of metrics in a category
#[derive(Debug, Clone)]
pub struct MetricSummary {
    /// Number of metrics in this category
    pub count: usize,
    
    /// Total value of all metrics in this category
    pub total_value: f64,
    
    /// Average value of metrics in this category
    pub average_value: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_record_metric() {
        let mut metrics = LaunchMetrics::new();
        metrics.record_metric("active_users", 45.0, MetricCategory::Adoption);
        
        assert!(metrics.get_metric("active_users").is_some());
        assert_eq!(metrics.get_metric("active_users").unwrap().value, 45.0);
    }
    
    #[test]
    fn test_record_event() {
        let mut metrics = LaunchMetrics::new();
        metrics.record_event(LaunchEventType::DashboardAccess, "User accessed dashboard", Some("user123".to_string()));
        
        assert_eq!(metrics.events.len(), 1);
        assert_eq!(metrics.events[0].event_type, LaunchEventType::DashboardAccess);
    }
    
    #[test]
    fn test_calculate_adoption_rate() {
        let mut metrics = LaunchMetrics::new();
        metrics.record_metric("active_users", 50.0, MetricCategory::Adoption);
        
        let adoption_rate = metrics.calculate_adoption_rate(100.0);
        assert_eq!(adoption_rate, Some(50.0));
    }
    
    #[test]
    fn test_metrics_by_category() {
        let mut metrics = LaunchMetrics::new();
        metrics.record_metric("active_users", 45.0, MetricCategory::Adoption);
        metrics.record_metric("engagement_score", 8.5, MetricCategory::Engagement);
        metrics.record_metric("validation_count", 12.0, MetricCategory::Validation);
        
        let adoption_metrics = metrics.get_metrics_by_category(MetricCategory::Adoption);
        assert_eq!(adoption_metrics.len(), 1);
        assert_eq!(adoption_metrics[0].name, "active_users");
    }
    
    #[test]
    fn test_generate_report() {
        let mut metrics = LaunchMetrics::new();
        metrics.record_metric("active_users", 50.0, MetricCategory::Adoption);
        metrics.record_metric("validation_participants", 30.0, MetricCategory::Validation);
        
        metrics.record_event(LaunchEventType::DashboardAccess, "User accessed dashboard", Some("user123".to_string()));
        metrics.record_event(LaunchEventType::CommunityValidation, "Validation session completed", Some("user456".to_string()));
        
        let report = metrics.generate_report(100.0);
        assert!(report.adoption_rate.is_some());
        assert_eq!(report.total_events, 2);
        assert_eq!(report.validation_events, 1);
    }
}