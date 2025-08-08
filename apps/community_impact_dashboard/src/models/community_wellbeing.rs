//! Community Wellbeing Model
//!
//! This module defines data structures that represent community wellbeing
//! indicators across all impact domains.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::impact_data::ImpactDomain;

/// Community Wellbeing Indicators
/// 
/// Comprehensive metrics showing community wellbeing across all domains
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityWellbeing {
    /// Unique identifier for this wellbeing snapshot
    pub id: Uuid,
    
    /// Timestamp when this wellbeing data was collected
    pub timestamp: DateTime<Utc>,
    
    /// Overall community wellbeing score (0.0 to 1.0)
    pub overall_score: f64,
    
    /// Wellbeing indicators by domain
    pub domain_indicators: DomainWellbeingIndicators,
    
    /// Collective progress toward cooperative goals
    pub cooperative_goals_progress: Vec<CooperativeGoalProgress>,
    
    /// Historical progress with timeline visualization
    pub historical_progress: Vec<WellbeingProgressPoint>,
    
    /// Comparative metrics showing community growth over time
    pub comparative_metrics: ComparativeMetrics,
}

/// Domain Wellbeing Indicators
/// 
/// Wellbeing indicators for each of the four impact domains
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainWellbeingIndicators {
    /// Learning domain wellbeing
    pub learning: LearningWellbeing,
    
    /// Volunteer domain wellbeing
    pub volunteer: VolunteerWellbeing,
    
    /// Financial domain wellbeing
    pub financial: FinancialWellbeing,
    
    /// Cause domain wellbeing
    pub cause: CauseWellbeing,
}

/// Learning Wellbeing Indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningWellbeing {
    /// Knowledge sharing rate in the community (0.0 to 1.0)
    pub knowledge_sharing_rate: f64,
    
    /// Skill development progress (0.0 to 1.0)
    pub skill_development_progress: f64,
    
    /// Educational equity index (0.0 to 1.0)
    pub educational_equity: f64,
    
    /// Community learning satisfaction (0.0 to 1.0)
    pub community_satisfaction: f64,
}

/// Volunteer Wellbeing Indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolunteerWellbeing {
    /// Volunteer participation rate (0.0 to 1.0)
    pub participation_rate: f64,
    
    /// Volunteer retention rate (0.0 to 1.0)
    pub retention_rate: f64,
    
    /// Volunteer satisfaction index (0.0 to 1.0)
    pub satisfaction_index: f64,
    
    /// Community service coverage (0.0 to 1.0)
    pub service_coverage: f64,
}

/// Financial Wellbeing Indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialWellbeing {
    /// Community financial health score (0.0 to 1.0)
    pub financial_health: f64,
    
    /// Resource distribution equity (0.0 to 1.0)
    pub resource_equity: f64,
    
    /// Financial sustainability index (0.0 to 1.0)
    pub sustainability_index: f64,
    
    /// Community economic participation (0.0 to 1.0)
    pub economic_participation: f64,
}

/// Cause Wellbeing Indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CauseWellbeing {
    /// Cause engagement rate (0.0 to 1.0)
    pub engagement_rate: f64,
    
    /// Social impact effectiveness (0.0 to 1.0)
    pub impact_effectiveness: f64,
    
    /// Community solidarity index (0.0 to 1.0)
    pub solidarity_index: f64,
    
    /// Justice advancement progress (0.0 to 1.0)
    pub justice_progress: f64,
}

/// Cooperative Goal Progress
/// 
/// Progress toward a specific cooperative goal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CooperativeGoalProgress {
    /// Goal ID
    pub id: Uuid,
    
    /// Goal title
    pub title: String,
    
    /// Goal description
    pub description: String,
    
    /// Current progress toward goal (0.0 to 1.0)
    pub progress: f64,
    
    /// Target completion date
    pub target_date: Option<DateTime<Utc>>,
    
    /// Cooperative values alignment
    pub values_alignment: Vec<String>,
}

/// Wellbeing Progress Point
/// 
/// A point in time showing historical wellbeing progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellbeingProgressPoint {
    /// Timestamp of this progress point
    pub timestamp: DateTime<Utc>,
    
    /// Overall wellbeing score at this point (0.0 to 1.0)
    pub overall_score: f64,
    
    /// Learning wellbeing at this point (0.0 to 1.0)
    pub learning_score: f64,
    
    /// Volunteer wellbeing at this point (0.0 to 1.0)
    pub volunteer_score: f64,
    
    /// Financial wellbeing at this point (0.0 to 1.0)
    pub financial_score: f64,
    
    /// Cause wellbeing at this point (0.0 to 1.0)
    pub cause_score: f64,
}

/// Comparative Metrics
/// 
/// Metrics comparing community growth over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparativeMetrics {
    /// Comparison period (e.g., "Last Month", "Last Quarter", "Last Year")
    pub period: String,
    
    /// Overall growth rate (percentage change)
    pub overall_growth: f64,
    
    /// Growth by domain
    pub domain_growth: DomainGrowthMetrics,
    
    /// Benchmark comparisons
    pub benchmarks: Vec<BenchmarkComparison>,
}

/// Domain Growth Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainGrowthMetrics {
    /// Learning domain growth rate (percentage change)
    pub learning_growth: f64,
    
    /// Volunteer domain growth rate (percentage change)
    pub volunteer_growth: f64,
    
    /// Financial domain growth rate (percentage change)
    pub financial_growth: f64,
    
    /// Cause domain growth rate (percentage change)
    pub cause_growth: f64,
}

/// Benchmark Comparison
/// 
/// Comparison against a benchmark or standard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkComparison {
    /// Benchmark name
    pub name: String,
    
    /// Benchmark value
    pub benchmark_value: f64,
    
    /// Community value
    pub community_value: f64,
    
    /// Difference between community and benchmark (-1.0 to 1.0)
    pub difference: f64,
    
    /// Whether community is performing above or below benchmark
    pub performance: PerformanceIndicator,
}

/// Performance Indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceIndicator {
    Above,
    Below,
    Meeting,
}

impl CommunityWellbeing {
    /// Create a new CommunityWellbeing instance
    pub fn new(
        overall_score: f64,
        domain_indicators: DomainWellbeingIndicators,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            overall_score,
            domain_indicators,
            cooperative_goals_progress: Vec::new(),
            historical_progress: Vec::new(),
            comparative_metrics: ComparativeMetrics {
                period: "Last Month".to_string(),
                overall_growth: 0.0,
                domain_growth: DomainGrowthMetrics {
                    learning_growth: 0.0,
                    volunteer_growth: 0.0,
                    financial_growth: 0.0,
                    cause_growth: 0.0,
                },
                benchmarks: Vec::new(),
            },
        }
    }
    
    /// Add cooperative goal progress
    pub fn add_cooperative_goal(mut self, goal: CooperativeGoalProgress) -> Self {
        self.cooperative_goals_progress.push(goal);
        self
    }
    
    /// Add historical progress point
    pub fn add_historical_progress(mut self, progress: WellbeingProgressPoint) -> Self {
        self.historical_progress.push(progress);
        self
    }
    
    /// Set comparative metrics
    pub fn with_comparative_metrics(mut self, metrics: ComparativeMetrics) -> Self {
        self.comparative_metrics = metrics;
        self
    }
}