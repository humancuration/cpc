//! GraphQL types and resolvers for the BI toolkit

use async_graphql::*;
use uuid::Uuid;
use std::sync::Arc;

use crate::bi::BIService;
use crate::bi::models::*;

/// GraphQL type for ImpactReport
#[derive(SimpleObject, Clone, Debug)]
pub struct ImpactReport {
    pub user_id: String,
    pub total_impact: f64,
    pub breakdown: Vec<ImpactBreakdown>,
    pub distribution: Vec<ImpactDistribution>,
    pub timeline: Vec<ImpactTimelinePoint>,
    pub generated_at: String,
}

/// GraphQL type for ImpactBreakdown
#[derive(SimpleObject, Clone, Debug)]
pub struct ImpactBreakdown {
    pub category: String,
    pub amount: f64,
    pub item_name: String,
    pub contribution: f64,
    pub impact_score: f64,
}

/// GraphQL type for ImpactDistribution
#[derive(SimpleObject, Clone, Debug)]
pub struct ImpactDistribution {
    pub category: String,
    pub weight: f64,
}

/// GraphQL type for ImpactTimelinePoint
#[derive(SimpleObject, Clone, Debug)]
pub struct ImpactTimelinePoint {
    pub date: String,
    pub description: String,
    pub impact_value: f64,
    pub timestamp: u64,
    pub score: f64,
}

/// GraphQL type for ProcessingStatus
#[derive(SimpleObject, Clone, Debug)]
pub struct ProcessingStatus {
    pub job_id: String,
    pub status: String,
    pub progress: f64,
    pub message: Option<String>,
    pub estimated_completion: Option<String>,
}

/// BI queries
#[derive(Default)]
pub struct BIQuery;

#[Object]
impl BIQuery {
    /// Get impact report for a specific user
    async fn get_impact_report(&self, ctx: &Context<'_>, user_id: Uuid) -> Result<ImpactReport> {
        let bi_service = ctx.data_unchecked::<Arc<BIService>>();
        
        match bi_service.get_impact_report(user_id).await {
            Ok(report) => Ok(ImpactReport::from_domain(report)),
            Err(e) => Err(Error::new(e.to_string())),
        }
    }

    /// Get processing status for a job
    async fn get_processing_status(&self, ctx: &Context<'_>, job_id: Uuid) -> Result<ProcessingStatus> {
        let bi_service = ctx.data_unchecked::<Arc<BIService>>();
        
        match bi_service.get_processing_status(job_id).await {
            Ok(status) => Ok(ProcessingStatus::from_domain(status)),
            Err(e) => Err(Error::new(e.to_string())),
        }
    }
}

/// BI mutations
#[derive(Default)]
pub struct BIMutation;

#[Object]
impl BIMutation {
    /// Generate a new impact report for a user
    async fn generate_impact_report(&self, ctx: &Context<'_>, user_id: Uuid) -> Result<ProcessingStatus> {
        let bi_service = ctx.data_unchecked::<Arc<BIService>>();
        
        match bi_service.generate_impact_report(user_id).await {
            Ok(status) => Ok(ProcessingStatus::from_domain(status)),
            Err(e) => Err(Error::new(e.to_string())),
        }
    }
}

/// BI subscriptions
#[derive(Default)]
pub struct BISubscription;

#[Subscription]
impl BISubscription {
    /// Subscribe to impact report updates for a user
    async fn impact_report_updated(
        &self,
        ctx: &Context<'_>,
        user_id: Uuid,
    ) -> Result<impl futures_util::Stream<Item = ImpactReport>> {
        // TODO: Implement real subscription using channels
        let bi_service = ctx.data_unchecked::<Arc<BIService>>();
        
        match bi_service.get_impact_report(user_id).await {
            Ok(report) => {
                let graphql_report = ImpactReport::from_domain(report);
                Ok(futures_util::stream::once(async move { graphql_report }))
            }
            Err(e) => Err(Error::new(e.to_string())),
        }
    }
}

// Conversion implementations
impl ImpactReport {
    fn from_domain(domain: crate::bi::models::ImpactReport) -> Self {
        let total_impact = domain.overall_score;
        
        let breakdown = domain.breakdown.into_iter()
            .map(|item| ImpactBreakdown {
                category: format!("{:?}", item.category),
                amount: item.value,
                item_name: item.name,
                contribution: 0.0, // Will be calculated based on total
                impact_score: item.ethical_score * 10.0, // Scale to 0-10
            })
            .collect();

        let distribution = domain.ethical_distribution.into_iter()
            .map(|(category, weight)| ImpactDistribution {
                category: format!("{:?}", category),
                weight,
            })
            .collect();

        let timeline = domain.timeline.into_iter()
            .enumerate()
            .map(|(i, point)| ImpactTimelinePoint {
                date: point.timestamp.format("%Y-%m-%d").to_string(),
                description: format!("Impact in {:?} category", point.category),
                impact_value: point.value,
                timestamp: point.timestamp.timestamp_millis() as u64,
                score: 7.5 + (i as f64 * 0.5), // Simple progression
            })
            .collect();

        Self {
            user_id: domain.user_id.to_string(),
            total_impact,
            breakdown,
            distribution,
            timeline,
            generated_at: domain.generated_at.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        }
    }
}

impl ProcessingStatus {
    fn from_domain(domain: crate::bi::models::ProcessingStatus) -> Self {
        Self {
            job_id: domain.job_id.to_string(),
            status: format!("{:?}", domain.status).to_lowercase(),
            progress: domain.progress,
            message: domain.message,
            estimated_completion: domain.estimated_completion
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()),
        }
    }
}