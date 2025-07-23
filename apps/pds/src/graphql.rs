use async_graphql::*;
use uuid::Uuid;
use crate::impact::{ImpactRepository, ImpactReportResponse, GenerateImpactReportResponse};

/// GraphQL schema for impact reporting
pub struct ImpactSchema;

/// Impact report query root
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get impact report for a user
    async fn impact_report(&self, ctx: &Context<'_>, user_id: String) -> Result<ImpactReport> {
        let repo = ctx.data_unchecked::<ImpactRepository>();
        
        // Parse user_id string to UUID
        let user_uuid = Uuid::parse_str(&user_id)
            .map_err(|_| Error::new("Invalid user ID format"))?;
        
        // For development, return mock data
        let response = crate::impact::get_mock_impact_report(&user_id).await;
        
        Ok(ImpactReport {
            user_id: response.user_id,
            total_impact: response.total_impact,
            breakdown: response.breakdown.into_iter().map(|b| ImpactBreakdown {
                category: b.category,
                amount: b.amount,
                item_name: b.item_name,
                contribution: b.contribution,
                impact_score: b.impact_score,
            }).collect(),
            distribution: response.distribution.into_iter().map(|d| ImpactDistribution {
                category: d.category,
                weight: d.weight,
            }).collect(),
            timeline: response.timeline.into_iter().map(|t| ImpactTimeline {
                date: t.date,
                description: t.description,
                impact_value: t.impact_value,
                timestamp: t.timestamp,
                score: t.score,
            }).collect(),
        })
    }
}

/// Impact report mutation root
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Generate impact report for a user
    async fn generate_impact_report(&self, ctx: &Context<'_>, user_id: String) -> Result<GenerateImpactReportPayload> {
        let repo = ctx.data_unchecked::<ImpactRepository>();
        
        // Parse user_id string to UUID
        let user_uuid = Uuid::parse_str(&user_id)
            .map_err(|_| Error::new("Invalid user ID format"))?;
        
        // For now, return success response
        Ok(GenerateImpactReportPayload {
            success: true,
            message: "Impact report generated successfully".to_string(),
            report_id: user_id,
        })
    }
}

/// Impact report subscription root
pub struct SubscriptionRoot;

#[Object]
impl SubscriptionRoot {
    /// Subscribe to impact report updates for a user
    async fn impact_report_updates(&self, user_id: String) -> impl Stream<Item = ImpactReport> {
        use futures::stream;
        
        // For now, return a single mock update
        let report = ImpactReport {
            user_id: user_id.clone(),
            total_impact: 85.5,
            breakdown: vec![
                ImpactBreakdown {
                    category: "Environmental".to_string(),
                    amount: 45.2,
                    item_name: "Carbon Footprint Reduction".to_string(),
                    contribution: 52.8,
                    impact_score: 8.5,
                },
                ImpactBreakdown {
                    category: "Social".to_string(),
                    amount: 25.3,
                    item_name: "Community Engagement".to_string(),
                    contribution: 29.6,
                    impact_score: 7.2,
                },
                ImpactBreakdown {
                    category: "Economic".to_string(),
                    amount: 15.0,
                    item_name: "Local Economic Support".to_string(),
                    contribution: 17.6,
                    impact_score: 6.8,
                },
            ],
            distribution: vec![
                ImpactDistribution {
                    category: "Environmental".to_string(),
                    weight: 0.528,
                },
                ImpactDistribution {
                    category: "Social".to_string(),
                    weight: 0.296,
                },
                ImpactDistribution {
                    category: "Economic".to_string(),
                    weight: 0.176,
                },
            ],
            timeline: vec![
                ImpactTimeline {
                    date: "2024-01-15".to_string(),
                    description: "Started using eco-friendly products".to_string(),
                    impact_value: 15.2,
                    timestamp: 1705276800000,
                    score: 7.5,
                },
            ],
        };
        
        stream::once(async move { report })
    }
}

/// GraphQL types matching frontend expectations

#[derive(SimpleObject, Clone, Debug)]
pub struct ImpactReport {
    pub user_id: String,
    pub total_impact: f64,
    pub breakdown: Vec<ImpactBreakdown>,
    pub distribution: Vec<ImpactDistribution>,
    pub timeline: Vec<ImpactTimeline>,
}

#[derive(SimpleObject, Clone, Debug)]
pub struct ImpactBreakdown {
    pub category: String,
    pub amount: f64,
    pub item_name: String,
    pub contribution: f64,
    pub impact_score: f64,
}

#[derive(SimpleObject, Clone, Debug)]
pub struct ImpactDistribution {
    pub category: String,
    pub weight: f64,
}

#[derive(SimpleObject, Clone, Debug)]
pub struct ImpactTimeline {
    pub date: String,
    pub description: String,
    pub impact_value: f64,
    pub timestamp: u64,
    pub score: f64,
}

#[derive(SimpleObject, Clone, Debug)]
pub struct GenerateImpactReportPayload {
    pub success: bool,
    pub message: String,
    pub report_id: String,
}

/// Create GraphQL schema
pub type Schema = async_graphql::Schema<QueryRoot, MutationRoot, SubscriptionRoot>;