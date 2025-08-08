//! Financial Impact Reporting
//!
//! Generate reports and visualizations for financial impact data.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use rust_decimal::Decimal;
use crate::{FinancialAnalytics, FinancialImpactError, FinancialImpactAnalytics};

/// Financial impact report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialImpactReport {
    pub generated_at: DateTime<Utc>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub analytics: FinancialImpactAnalytics,
    pub summary: ReportSummary,
    pub recommendations: Vec<Recommendation>,
    pub visualizations: Vec<VisualizationData>,
}

/// Report summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSummary {
    pub total_financial_impact: Decimal,
    pub impact_trend: ImpactTrend,
    pub key_insights: Vec<String>,
    pub community_engagement_score: Decimal,
}

/// Impact trend indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactTrend {
    Increasing(Decimal),   // Percentage increase
    Decreasing(Decimal),   // Percentage decrease
    Stable,
}

/// Recommendation for financial impact improvement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: String,
    pub title: String,
    pub description: String,
    pub priority: RecommendationPriority,
    pub impact_estimate: Decimal,
    pub implementation_effort: ImplementationEffort,
}

/// Recommendation priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationPriority {
    High,
    Medium,
    Low,
}

/// Implementation effort estimation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationEffort {
    Low,
    Medium,
    High,
}

/// Visualization data for reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationData {
    pub id: String,
    pub title: String,
    pub data: serde_json::Value,
    pub chart_type: ChartType,
    pub description: String,
}

/// Chart types for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChartType {
    Bar,
    Line,
    Pie,
    Scatter,
    Area,
    Heatmap,
}

/// Financial report generator
pub struct FinancialReportGenerator {
    analytics: FinancialAnalytics,
}

impl FinancialReportGenerator {
    /// Create a new financial report generator
    pub fn new(analytics: FinancialAnalytics) -> Self {
        Self { analytics }
    }

    /// Generate a comprehensive financial impact report
    pub async fn generate_report(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<FinancialImpactReport, FinancialImpactError> {
        let analytics = self.analytics.generate_impact_analytics(start_time, end_time).await?;
        let summary = self.generate_summary(&analytics, start_time, end_time).await?;
        let recommendations = self.generate_recommendations(&analytics).await?;
        let visualizations = self.generate_visualizations(&analytics).await?;

        Ok(FinancialImpactReport {
            generated_at: Utc::now(),
            period_start: start_time,
            period_end: end_time,
            analytics,
            summary,
            recommendations,
            visualizations,
        })
    }

    /// Generate report summary
    async fn generate_summary(
        &self,
        analytics: &FinancialImpactAnalytics,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<ReportSummary, FinancialImpactError> {
        // Calculate trend by comparing with previous period
        let previous_start = start_time - Duration::days(30);
        let previous_end = start_time;
        let previous_analytics = self.analytics.generate_impact_analytics(previous_start, previous_end).await?;
        
        let trend = if previous_analytics.total_impact > Decimal::ZERO {
            let change = (analytics.total_impact - previous_analytics.total_impact) / previous_analytics.total_impact * Decimal::from(100);
            if change > Decimal::from(5) {
                ImpactTrend::Increasing(change)
            } else if change < Decimal::from(-5) {
                ImpactTrend::Decreasing(change.abs())
            } else {
                ImpactTrend::Stable
            }
        } else {
            ImpactTrend::Stable
        };

        let key_insights = self.generate_key_insights(analytics).await?;
        let engagement_score = self.calculate_engagement_score(analytics).await?;

        Ok(ReportSummary {
            total_financial_impact: analytics.total_impact,
            impact_trend: trend,
            key_insights,
            community_engagement_score: engagement_score,
        })
    }

    /// Generate key insights from analytics
    async fn generate_key_insights(
        &self,
        analytics: &FinancialImpactAnalytics,
    ) -> Result<Vec<String>, FinancialImpactError> {
        let mut insights = Vec::new();

        // Top category insight
        if let Some(top_category) = analytics.category_breakdown.first() {
            insights.push(format!(
                "Highest impact category: {} with {} weighted impact",
                top_category.category, top_category.weighted_impact
            ));
        }

        // Top contributor insight
        if let Some(top_contributor) = analytics.top_contributors.first() {
            insights.push(format!(
                "Top contributor: {} with {} total contributions",
                top_contributor.name, top_contributor.total_contributions
            ));
        }

        // Sustainability insight
        if analytics.sustainability_metrics.financial_health_score > Decimal::from(0.8) {
            insights.push("Financial health is strong with high sustainability metrics".to_string());
        } else if analytics.sustainability_metrics.financial_health_score < Decimal::from(0.5) {
            insights.push("Financial health needs attention - consider diversifying revenue sources".to_string());
        }

        // ROI insight
        if let Some(best_roi) = analytics.roi_metrics.iter().max_by(|a, b| a.roi_percentage.cmp(&b.roi_percentage)) {
            if best_roi.roi_percentage > Decimal::from(50) {
                insights.push(format!(
                    "Best ROI category: {} with {}% return",
                    best_roi.investment_category, best_roi.roi_percentage
                ));
            }
        }

        Ok(insights)
    }

    /// Calculate community engagement score based on financial activity
    async fn calculate_engagement_score(
        &self,
        analytics: &FinancialImpactAnalytics,
    ) -> Result<Decimal, FinancialImpactError> {
        // Simple engagement score calculation based on:
        // - Number of contributors (30% weight)
        // - Diversity of categories (30% weight)
        // - Sustainability metrics (40% weight)
        
        let contributor_score = if analytics.top_contributors.len() > 10 {
            Decimal::from(100)
        } else {
            Decimal::from(analytics.top_contributors.len() * 10)
        };

        let category_diversity_score = if analytics.category_breakdown.len() > 5 {
            Decimal::from(100)
        } else {
            Decimal::from(analytics.category_breakdown.len() * 20)
        };

        let sustainability_score = analytics.sustainability_metrics.financial_health_score * Decimal::from(100);

        let engagement_score = (contributor_score * Decimal::from(30) +
                               category_diversity_score * Decimal::from(30) +
                               sustainability_score * Decimal::from(40)) / Decimal::from(100);

        Ok(engagement_score)
    }

    /// Generate recommendations for improving financial impact
    async fn generate_recommendations(
        &self,
        analytics: &FinancialImpactAnalytics,
    ) -> Result<Vec<Recommendation>, FinancialImpactError> {
        let mut recommendations = Vec::new();

        // Recommendation based on contributor diversity
        if analytics.top_contributors.len() < 5 {
            recommendations.push(Recommendation {
                id: "diversify_contributors".to_string(),
                title: "Diversify Financial Contributors".to_string(),
                description: "Currently relying on a small number of contributors. Focus on expanding the donor base.".to_string(),
                priority: RecommendationPriority::High,
                impact_estimate: Decimal::from(30), // Estimated 30% improvement potential
                implementation_effort: ImplementationEffort::Medium,
            });
        }

        // Recommendation based on category diversity
        if analytics.category_breakdown.len() < 3 {
            recommendations.push(Recommendation {
                id: "expand_categories".to_string(),
                title: "Expand Financial Categories".to_string(),
                description: "Limited financial activity categories. Explore new funding and investment opportunities.".to_string(),
                priority: RecommendationPriority::Medium,
                impact_estimate: Decimal::from(25), // Estimated 25% improvement potential
                implementation_effort: ImplementationEffort::High,
            });
        }

        // Recommendation based on sustainability
        if analytics.sustainability_metrics.donation_stability_index > Decimal::from(0.5) {
            recommendations.push(Recommendation {
                id: "improve_stability".to_string(),
                title: "Improve Donation Stability".to_string(),
                description: "High variance in donations. Implement recurring donation programs.".to_string(),
                priority: RecommendationPriority::High,
                impact_estimate: Decimal::from(40), // Estimated 40% improvement potential
                implementation_effort: ImplementationEffort::Low,
            });
        }

        // Recommendation based on ROI
        if let Some(lowest_roi) = analytics.roi_metrics.iter().min_by(|a, b| a.roi_percentage.cmp(&b.roi_percentage)) {
            if lowest_roi.roi_percentage < Decimal::from(10) {
                recommendations.push(Recommendation {
                    id: "optimize_investments".to_string(),
                    title: "Optimize Low-Return Investments".to_string(),
                    description: format!("Category {} has low ROI. Consider reallocating resources.", lowest_roi.investment_category),
                    priority: RecommendationPriority::Medium,
                    impact_estimate: Decimal::from(20), // Estimated 20% improvement potential
                    implementation_effort: ImplementationEffort::Medium,
                });
            }
        }

        Ok(recommendations)
    }

    /// Generate visualization data for the report
    async fn generate_visualizations(
        &self,
        analytics: &FinancialImpactAnalytics,
    ) -> Result<Vec<VisualizationData>, FinancialImpactError> {
        let mut visualizations = Vec::new();

        // Category breakdown pie chart
        visualizations.push(VisualizationData {
            id: "category_breakdown".to_string(),
            title: "Financial Impact by Category".to_string(),
            data: serde_json::to_value(&analytics.category_breakdown)?,
            chart_type: ChartType::Pie,
            description: "Distribution of financial impact across different categories".to_string(),
        });

        // Time series line chart
        visualizations.push(VisualizationData {
            id: "time_series".to_string(),
            title: "Financial Impact Over Time".to_string(),
            data: serde_json::to_value(&analytics.time_series)?,
            chart_type: ChartType::Line,
            description: "Financial impact trends over the reporting period".to_string(),
        });

        // Top contributors bar chart
        visualizations.push(VisualizationData {
            id: "top_contributors".to_string(),
            title: "Top Financial Contributors".to_string(),
            data: serde_json::to_value(&analytics.top_contributors)?,
            chart_type: ChartType::Bar,
            description: "Leading contributors to financial impact".to_string(),
        });

        // Sustainability metrics heatmap
        visualizations.push(VisualizationData {
            id: "sustainability_metrics".to_string(),
            title: "Financial Sustainability Metrics".to_string(),
            data: serde_json::to_value(&analytics.sustainability_metrics)?,
            chart_type: ChartType::Heatmap,
            description: "Key sustainability indicators for financial health".to_string(),
        });

        Ok(visualizations)
    }

    /// Generate a summary report for quick overview
    pub async fn generate_summary_report(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<ReportSummary, FinancialImpactError> {
        let analytics = self.analytics.generate_impact_analytics(start_time, end_time).await?;
        self.generate_summary(&analytics, start_time, end_time).await
    }

    /// Export report in different formats
    pub async fn export_report(
        &self,
        report: &FinancialImpactReport,
        format: ExportFormat,
    ) -> Result<Vec<u8>, FinancialImpactError> {
        match format {
            ExportFormat::Json => {
                Ok(serde_json::to_vec_pretty(report)?)
            },
            ExportFormat::Csv => {
                // Simplified CSV export - in practice this would be more detailed
                let mut csv_data = String::new();
                csv_data.push_str("Metric,Value\n");
                csv_data.push_str(&format!("Total Impact,{}\n", report.analytics.total_impact));
                csv_data.push_str(&format!("Community Engagement Score,{}\n", report.summary.community_engagement_score));
                Ok(csv_data.into_bytes())
            },
        }
    }
}

/// Export formats for reports
#[derive(Debug, Clone)]
pub enum ExportFormat {
    Json,
    Csv,
}