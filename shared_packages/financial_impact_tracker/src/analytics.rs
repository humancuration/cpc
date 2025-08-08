//! Financial Impact Analytics
//!
//! Advanced analytics for measuring and visualizing financial impact metrics.

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use chrono::{DateTime, Utc, Duration};
use rust_decimal::Decimal;
use cpc_financial_core::{FinancialCategory, FinancialMetric};
use crate::{FinancialImpactTracker, FinancialImpactError};

/// Financial impact analytics data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialImpactAnalytics {
    pub total_impact: Decimal,
    pub category_breakdown: Vec<CategoryImpact>,
    pub time_series: Vec<TimeSeriesPoint>,
    pub top_contributors: Vec<ContributorImpact>,
    pub roi_metrics: Vec<ROIMetric>,
    pub sustainability_metrics: SustainabilityMetrics,
}

/// Category-based impact breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryImpact {
    pub category: FinancialCategory,
    pub total_amount: Decimal,
    pub impact_score: Decimal,
    pub weighted_impact: Decimal,
    pub transaction_count: i64,
}

/// Time series data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesPoint {
    pub timestamp: DateTime<Utc>,
    pub amount: Decimal,
    pub impact_score: Decimal,
    pub category: FinancialCategory,
}

/// Top contributor impact data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributorImpact {
    pub contributor_id: String,
    pub name: String,
    pub total_contributions: Decimal,
    pub impact_score: Decimal,
    pub categories: Vec<FinancialCategory>,
}

/// Return on Investment metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ROIMetric {
    pub investment_category: FinancialCategory,
    pub return_category: FinancialCategory,
    pub investment_amount: Decimal,
    pub return_amount: Decimal,
    pub roi_percentage: Decimal,
    pub time_period: Duration,
}

/// Sustainability metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SustainabilityMetrics {
    pub monthly_recurring_revenue: Decimal,
    pub donation_stability_index: Decimal,
    pub community_investment_ratio: Decimal,
    pub financial_health_score: Decimal,
}

/// Financial impact analytics engine
pub struct FinancialAnalytics {
    tracker: FinancialImpactTracker,
}

impl FinancialAnalytics {
    /// Create a new financial analytics engine
    pub fn new(tracker: FinancialImpactTracker) -> Self {
        Self { tracker }
    }

    /// Generate comprehensive financial impact analytics
    pub async fn generate_impact_analytics(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<FinancialImpactAnalytics, FinancialImpactError> {
        let total_impact = self.tracker.calculate_total_impact(start_time, end_time, None).await?;
        
        let category_breakdown = self.calculate_category_breakdown(start_time, end_time).await?;
        let time_series = self.generate_time_series(start_time, end_time).await?;
        let top_contributors = self.calculate_top_contributors(start_time, end_time).await?;
        let roi_metrics = self.calculate_roi_metrics(start_time, end_time).await?;
        let sustainability_metrics = self.calculate_sustainability_metrics(start_time, end_time).await?;

        Ok(FinancialImpactAnalytics {
            total_impact,
            category_breakdown,
            time_series,
            top_contributors,
            roi_metrics,
            sustainability_metrics,
        })
    }

    /// Calculate impact breakdown by category
    async fn calculate_category_breakdown(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<CategoryImpact>, FinancialImpactError> {
        let breakdown = sqlx::query_as::<_, CategoryImpact>(
            r#"
            SELECT 
                category as "category: FinancialCategory",
                SUM(amount) as total_amount,
                AVG(impact_score) as impact_score,
                SUM(amount * impact_score) as weighted_impact,
                COUNT(*) as transaction_count
            FROM financial_impact_records
            WHERE timestamp >= $1 AND timestamp <= $2
            GROUP BY category
            ORDER BY weighted_impact DESC
            "#
        )
        .bind(start_time)
        .bind(end_time)
        .fetch_all(&self.tracker.db_pool)
        .await?;

        Ok(breakdown)
    }

    /// Generate time series data for financial impact
    async fn generate_time_series(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<TimeSeriesPoint>, FinancialImpactError> {
        let time_series = sqlx::query_as::<_, TimeSeriesPoint>(
            r#"
            SELECT 
                DATE_TRUNC('day', timestamp) as timestamp,
                SUM(amount) as amount,
                AVG(impact_score) as impact_score,
                category as "category: FinancialCategory"
            FROM financial_impact_records
            WHERE timestamp >= $1 AND timestamp <= $2
            GROUP BY DATE_TRUNC('day', timestamp), category
            ORDER BY timestamp
            "#
        )
        .bind(start_time)
        .bind(end_time)
        .fetch_all(&self.tracker.db_pool)
        .await?;

        Ok(time_series)
    }

    /// Calculate top financial contributors
    async fn calculate_top_contributors(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<ContributorImpact>, FinancialImpactError> {
        let contributors = sqlx::query_as::<_, ContributorImpact>(
            r#"
            SELECT 
                metadata->>'contributor_id' as contributor_id,
                metadata->>'contributor_name' as name,
                SUM(amount) as total_contributions,
                AVG(impact_score) as impact_score,
                ARRAY_AGG(DISTINCT category) as categories
            FROM financial_impact_records
            WHERE timestamp >= $1 AND timestamp <= $2 
                AND metadata->>'contributor_id' IS NOT NULL
            GROUP BY metadata->>'contributor_id', metadata->>'contributor_name'
            ORDER BY total_contributions DESC
            LIMIT 20
            "#
        )
        .bind(start_time)
        .bind(end_time)
        .fetch_all(&self.tracker.db_pool)
        .await?;

        Ok(contributors)
    }

    /// Calculate ROI metrics between investment and return categories
    async fn calculate_roi_metrics(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<ROIMetric>, FinancialImpactError> {
        // This is a simplified ROI calculation - in practice, this would be more complex
        let roi_metrics = sqlx::query_as::<_, ROIMetric>(
            r#"
            WITH investments AS (
                SELECT 
                    category as "investment_category: FinancialCategory",
                    SUM(amount) as investment_amount
                FROM financial_impact_records
                WHERE timestamp >= $1 AND timestamp <= $2
                    AND category IN ('Infrastructure', 'Education', 'CommunityDevelopment')
                GROUP BY category
            ),
            returns AS (
                SELECT 
                    category as "return_category: FinancialCategory",
                    SUM(amount) as return_amount
                FROM financial_impact_records
                WHERE timestamp >= $1 AND timestamp <= $2
                    AND category IN ('Revenue', 'Grants', 'Donations')
                GROUP BY category
            )
            SELECT 
                i.investment_category,
                r.return_category,
                i.investment_amount,
                r.return_amount,
                CASE 
                    WHEN i.investment_amount > 0 
                    THEN ((r.return_amount - i.investment_amount) / i.investment_amount) * 100
                    ELSE 0
                END as roi_percentage,
                $3 as "time_period: Duration"
            FROM investments i
            CROSS JOIN returns r
            WHERE i.investment_category != r.return_category
            "#
        )
        .bind(start_time)
        .bind(end_time)
        .bind(end_time.signed_duration_since(start_time))
        .fetch_all(&self.tracker.db_pool)
        .await?;

        Ok(roi_metrics)
    }

    /// Calculate sustainability metrics
    async fn calculate_sustainability_metrics(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<SustainabilityMetrics, FinancialImpactError> {
        let metrics = sqlx::query_as::<_, SustainabilityMetrics>(
            r#"
            WITH monthly_data AS (
                SELECT 
                    DATE_TRUNC('month', timestamp) as month,
                    SUM(CASE WHEN category IN ('Donations', 'Grants') THEN amount ELSE 0 END) as income,
                    SUM(CASE WHEN category IN ('Expenses', 'Infrastructure') THEN amount ELSE 0 END) as expenses
                FROM financial_impact_records
                WHERE timestamp >= $1 AND timestamp <= $2
                GROUP BY DATE_TRUNC('month', timestamp)
            ),
            recurring_revenue AS (
                SELECT AVG(income) as monthly_recurring_revenue
                FROM monthly_data
            ),
            stability AS (
                SELECT 
                    STDDEV(income) / AVG(income) as donation_stability_index
                FROM monthly_data
                WHERE income > 0
            ),
            investment_ratio AS (
                SELECT 
                    SUM(CASE WHEN category = 'CommunityInvestment' THEN amount ELSE 0 END) /
                    SUM(CASE WHEN category IN ('Donations', 'Grants') THEN amount ELSE 0 END) 
                    as community_investment_ratio
                FROM financial_impact_records
                WHERE timestamp >= $1 AND timestamp <= $2
            ),
            health_score AS (
                SELECT 
                    (AVG(CASE WHEN impact_score > 0.7 THEN 1.0 ELSE 0.0 END) * 0.4 +
                     AVG(CASE WHEN category IN ('Donations', 'Grants') THEN 1.0 ELSE 0.0 END) * 0.3 +
                     AVG(CASE WHEN category = 'CommunityInvestment' THEN 1.0 ELSE 0.0 END) * 0.3)
                    as financial_health_score
                FROM financial_impact_records
                WHERE timestamp >= $1 AND timestamp <= $2
            )
            SELECT 
                COALESCE(rr.monthly_recurring_revenue, 0) as monthly_recurring_revenue,
                COALESCE(s.donation_stability_index, 0) as donation_stability_index,
                COALESCE(ir.community_investment_ratio, 0) as community_investment_ratio,
                COALESCE(hs.financial_health_score, 0) as financial_health_score
            FROM recurring_revenue rr
            CROSS JOIN stability s
            CROSS JOIN investment_ratio ir
            CROSS JOIN health_score hs
            "#
        )
        .bind(start_time)
        .bind(end_time)
        .fetch_one(&self.tracker.db_pool)
        .await?;

        Ok(metrics)
    }

    /// Generate comparison analytics between two time periods
    pub async fn generate_period_comparison(
        &self,
        period1_start: DateTime<Utc>,
        period1_end: DateTime<Utc>,
        period2_start: DateTime<Utc>,
        period2_end: DateTime<Utc>,
    ) -> Result<(FinancialImpactAnalytics, FinancialImpactAnalytics), FinancialImpactError> {
        let analytics1 = self.generate_impact_analytics(period1_start, period1_end).await?;
        let analytics2 = self.generate_impact_analytics(period2_start, period2_end).await?;
        
        Ok((analytics1, analytics2))
    }
}