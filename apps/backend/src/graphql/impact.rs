use async_graphql::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::services::impact::{ImpactService, OrganizationImpactReport as ServiceOrgReport};
use crate::services::impact::DiversityMetrics as ServiceDiversityMetrics;
use futures_util::stream::{self, Stream};

/// Diversity metrics for impact reporting
#[derive(SimpleObject, Clone, Debug)]
pub struct DiversityMetrics {
    pub gender_diversity: f64,
    pub ethnic_diversity: f64,
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

#[derive(Default)]
pub struct ImpactQuery;

#[Object]
impl ImpactQuery {
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