use async_graphql::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use async_graphql::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::services::impact::{ImpactService, OrganizationImpactReport as ServiceOrgReport};
use crate::services::impact::DiversityMetrics as ServiceDiversityMetrics;
use futures_util::stream::{self, Stream};
use cpc_core::business::impact::{ImpactCalculator, CalculationError, ImpactDistribution};
use std::sync::Arc;

// Removed - now using domain model ImpactDistribution from cpc-core

/// Breakdown of impact contributions
#[derive(SimpleObject, Clone, Debug)]
pub struct ImpactBreakdown {
    pub category: String,
    pub amount: f64,
    pub item_name: String,
    pub contribution: f64,
    pub impact_score: f64,
}

/// Timeline point for impact visualization
#[derive(SimpleObject, Clone, Debug)]
pub struct ImpactTimelinePoint {
    pub date: String,
    pub description: String,
    pub impact_value: f64,
    pub timestamp: i64,
    pub score: f64,
}

/// Diversity metrics for impact reporting
#[derive(SimpleObject, Clone, Debug)]
pub struct DiversityMetrics {
    pub gender_diversity: f64,
    pub ethnic_diversity: f64,
}

/// User-level impact report
#[derive(SimpleObject, Clone, Debug)]
pub struct ImpactReport {
    pub user_id: Uuid,
    pub total_impact: f64,
    pub breakdown: Vec<ImpactBreakdown>,
    pub distribution: Vec<ImpactDistributionObject>,
    pub timeline: Vec<ImpactTimelinePoint>,
    pub generated_at: DateTime<Utc>,
    pub degradation_threshold: f64,
}

/// Organization-level impact report for business intelligence
#[derive(SimpleObject, Clone, Debug)]
pub struct OrganizationImpactReport {
    pub organization_id: Uuid,
    pub year: i32,
    pub carbon_footprint: f64,        // Metric tons CO2 equivalent
    pub community_investment: f64,    // USD
    pub diversity_metrics: DiversityMetrics,
    pub supply_chain_score: f64,      // 0-100 score
}

impl From<CalculationError> for Error {
    fn from(err: CalculationError) -> Self {
        match err {
            CalculationError::InsufficientData => Error::new("IMPACT_DATA_MISSING"),
            CalculationError::InvalidDistribution => Error::new("INVALID_DISTRIBUTION"),
            CalculationError::UserNotFound => Error::new("USER_NOT_FOUND"),
        }
    }
}

#[derive(Default)]
pub struct ImpactQuery;

#[Object]
impl ImpactQuery {
    /// Impact report implementation: docs/architecture/impact-service.md
    async fn get_impact_report(&self, ctx: &Context<'_>, user_id: ID) -> Result<Option<ImpactReport>> {
        // In production, this would call impact_service.get_user_impact_report()
        // For now, return sample data that matches our UI requirements
        let feature_flags = ctx.data::<FeatureFlags>()?;
        let impact_calculator = ctx.data::<Arc<dyn ImpactCalculator>>()?;

        let distribution = if feature_flags.impact_real_data_enabled {
            impact_calculator.calculate(&user_id).await?
        } else {
            vec![
                ImpactDistribution { category: "Community".into(), weight: 0.45 },
                ImpactDistribution { category: "Environment".into(), weight: 0.30 },
                ImpactDistribution { category: "Workers".into(), weight: 0.25 },
            ]
        };

        Ok(Some(ImpactReport {
            user_id: user_id.parse().map_err(|_| Error::new("Invalid user ID"))?,
            total_impact: 100.0,
            breakdown: vec![],
            distribution: distribution.into_iter().map(ImpactDistributionObject).collect(),
            timeline: vec![],
            generated_at: Utc::now(),
            degradation_threshold: feature_flags.ui_degradation_threshold,
        }))
    }

    /// Get organization impact report for a specific year
    async fn get_organization_impact_report(&self, ctx: &Context<'_>, org_id: Uuid, year: i32) -> Result<Option<OrganizationImpactReport>> {
        let impact_service = ctx.data::<ImpactService>()?;
        let report = impact_service.get_organization_impact_report(org_id, year)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        
        Ok(report.map(|r| OrganizationImpactReport {
            organization_id: r.organization_id,
            year: r.year,
            carbon_footprint: r.carbon_footprint,
            community_investment: r.community_investment,
            diversity_metrics: DiversityMetrics {
                gender_diversity: r.gender_diversity,
                ethnic_diversity: r.ethnic_diversity,
            },
            supply_chain_score: r.supply_chain_score,
        }))
    }
}

#[derive(Default)]
pub struct ImpactMutation;

#[Object]
impl ImpactMutation {
    /// Trigger recalculation of impact report for an organization
    async fn recalculate_organization_impact(
        &self,
        ctx: &Context<'_>,
        org_id: Uuid,
        year: i32
    ) -> Result<Uuid> {
        let impact_service = ctx.data::<ImpactService>()?;
        let job_id = impact_service.generate_organization_impact_report(org_id, year)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(job_id)
    }
}

#[derive(Default)]
pub struct ImpactSubscription;

#[Subscription]
impl ImpactSubscription {
    /// Subscribe to progress updates for an impact calculation job
    async fn impact_calculation_progress(
        &self,
        ctx: &Context<'_>,
        job_id: Uuid,
    ) -> Result<impl Stream<Item = String>> {
        // For now, return a mock stream. We'll implement proper progress in a future task.
        Ok(stream::once(async move {
            "Calculation complete".to_string()
        }))
    }
}