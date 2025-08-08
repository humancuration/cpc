//! UI Components for Finance Admin Dashboard
//!
//! Reusable UI components for displaying financial impact metrics and analytics.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone)]
pub struct MetricCardProps {
    pub title: String,
    pub value: String,
    pub change: Option<f64>,
    pub change_direction: Option<ChangeDirection>,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ChangeDirection {
    Up,
    Down,
    Neutral,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EngagementChartProps {
    pub data: Vec<EngagementDataPoint>,
    pub title: String,
    pub y_axis_label: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EngagementDataPoint {
    pub timestamp: DateTime<Utc>,
    pub views: u64,
    pub interactions: u64,
    pub avg_time: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CorrelationChartProps {
    pub data: Vec<CorrelationDataPoint>,
    pub title: String,
    pub x_axis_label: String,
    pub y_axis_label: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CorrelationDataPoint {
    pub visualization_usage: f64,
    pub financial_participation: f64,
    pub user_count: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FeedbackSummaryProps {
    pub total_feedback: u64,
    pub helpful_percentage: f64,
    pub avg_rating: f64,
    pub sentiment_score: f64,
    pub recent_feedback: Vec<FeedbackItem>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FeedbackItem {
    pub user_id: String,
    pub component_id: String,
    pub rating: u32,
    pub comment: String,
    pub timestamp: DateTime<Utc>,
    pub sentiment: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ImprovementRecommendationProps {
    pub recommendations: Vec<RecommendationItem>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RecommendationItem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub priority: RecommendationPriority,
    pub expected_impact: f64,
    pub implementation_effort: ImplementationEffort,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum RecommendationPriority {
    High,
    Medium,
    Low,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ImplementationEffort {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ABRoadmapProps {
    pub active_tests: Vec<ABTestItem>,
    pub completed_tests: Vec<ABTestItem>,
    pub upcoming_tests: Vec<ABTestItem>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ABTestItem {
    pub id: String,
    pub name: String,
    pub component_id: String,
    pub status: ABTestStatus,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub results: Option<ABTestResults>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ABTestStatus {
    Planning,
    Running,
    Completed,
    Cancelled,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ABTestResults {
    pub winning_variant: Option<String>,
    pub statistical_significance: f64,
    pub improvement_percentage: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CrossSystemImpactProps {
    pub financial_participation_rate: f64,
    pub learning_completion_rate: f64,
    pub volunteer_completion_rate: f64,
    pub correlations: CrossSystemCorrelations,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CrossSystemCorrelations {
    pub learning_financial: f64,
    pub volunteer_financial: f64,
    pub cause_financial: f64,
    pub community_engagement: f64,
}

// Component implementations would go here in a real frontend framework
// For now, we're just defining the data structures that would be used
// by frontend components to render the dashboard

impl MetricCardProps {
    pub fn new(title: String, value: String, description: String) -> Self {
        Self {
            title,
            value,
            change: None,
            change_direction: None,
            description,
        }
    }
    
    pub fn with_change(mut self, change: f64, direction: ChangeDirection) -> Self {
        self.change = Some(change);
        self.change_direction = Some(direction);
        self
    }
}

impl EngagementChartProps {
    pub fn new(title: String, y_axis_label: String) -> Self {
        Self {
            data: Vec::new(),
            title,
            y_axis_label,
        }
    }
    
    pub fn add_data_point(&mut self, point: EngagementDataPoint) {
        self.data.push(point);
    }
}

impl CorrelationChartProps {
    pub fn new(title: String, x_axis_label: String, y_axis_label: String) -> Self {
        Self {
            data: Vec::new(),
            title,
            x_axis_label,
            y_axis_label,
        }
    }
    
    pub fn add_data_point(&mut self, point: CorrelationDataPoint) {
        self.data.push(point);
    }
}