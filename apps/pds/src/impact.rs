use std::sync::Arc;
use uuid::Uuid;
use anyhow::Result;
use cpc_core::{
    models::impact::{ImpactReport, ImpactBreakdownItem, ImpactTimelinePoint, ImpactCategory},
    services::impact::ImpactService,
    repositories::impact_repository::MockImpactRepository,
};
use chrono::{DateTime, Utc};

/// Repository for impact data operations
pub struct ImpactRepository {
    service: Arc<ImpactService>,
}

impl ImpactRepository {
    pub fn new() -> Self {
        let mock_repo = Arc::new(MockImpactRepository);
        let service = Arc::new(ImpactService::new(mock_repo));
        Self { service }
    }

    /// Fetch impact report for a user
    pub async fn get_impact_report(&self, user_id: Uuid) -> Result<ImpactReportResponse> {
        let report = self.service.get_user_impact_report(user_id).await?;
        
        // Convert to response format that matches frontend expectations
        let response = ImpactReportResponse {
            user_id: report.user_id.to_string(),
            total_impact: report.overall_score,
            breakdown: report.breakdown.into_iter().map(|item| ImpactBreakdownResponse {
                category: match item.category {
                    ImpactCategory::Environmental => "Environmental".to_string(),
                    ImpactCategory::Social => "Social".to_string(),
                    ImpactCategory::Economic => "Economic".to_string(),
                },
                amount: item.value,
                item_name: item.name,
                contribution: 0.0, // Will be calculated based on total
                impact_score: item.ethical_score,
            }).collect(),
            distribution: report.ethical_distribution.into_iter().map(|(category, weight)| ImpactDistributionResponse {
                category: match category {
                    ImpactCategory::Environmental => "Environmental".to_string(),
                    ImpactCategory::Social => "Social".to_string(),
                    ImpactCategory::Economic => "Economic".to_string(),
                },
                weight,
            }).collect(),
            timeline: report.timeline.into_iter().map(|point| ImpactTimelineResponse {
                date: point.timestamp.format("%Y-%m-%d").to_string(),
                description: format!("Impact in {} category", match point.category {
                    ImpactCategory::Environmental => "Environmental",
                    ImpactCategory::Social => "Social",
                    ImpactCategory::Economic => "Economic",
                }),
                impact_value: point.value,
                timestamp: point.timestamp.timestamp_millis() as u64,
                score: 7.5, // Default score for now
            }).collect(),
        };

        Ok(response)
    }

    /// Generate a new impact report for a user
    pub async fn generate_impact_report(&self, user_id: Uuid) -> Result<GenerateImpactReportResponse> {
        // For now, just fetch the existing report
        let _report = self.service.get_user_impact_report(user_id).await?;
        
        Ok(GenerateImpactReportResponse {
            success: true,
            message: "Impact report generated successfully".to_string(),
            report_id: user_id.to_string(),
        })
    }
}

/// Response structure for impact report queries
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImpactReportResponse {
    pub user_id: String,
    pub total_impact: f64,
    pub breakdown: Vec<ImpactBreakdownResponse>,
    pub distribution: Vec<ImpactDistributionResponse>,
    pub timeline: Vec<ImpactTimelineResponse>,
}

/// Individual breakdown item response
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImpactBreakdownResponse {
    pub category: String,
    pub amount: f64,
    pub item_name: String,
    pub contribution: f64,
    pub impact_score: f64,
}

/// Distribution data response
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImpactDistributionResponse {
    pub category: String,
    pub weight: f64,
}

/// Timeline point response
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImpactTimelineResponse {
    pub date: String,
    pub description: String,
    pub impact_value: f64,
    pub timestamp: u64,
    pub score: f64,
}

/// Response for impact report generation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GenerateImpactReportResponse {
    pub success: bool,
    pub message: String,
    pub report_id: String,
}

/// Mock data implementation for development
pub async fn get_mock_impact_report(user_id: &str) -> ImpactReportResponse {
    ImpactReportResponse {
        user_id: user_id.to_string(),
        total_impact: 85.5,
        breakdown: vec![
            ImpactBreakdownResponse {
                category: "Environmental".to_string(),
                amount: 45.2,
                item_name: "Carbon Footprint Reduction".to_string(),
                contribution: 52.8,
                impact_score: 8.5,
            },
            ImpactBreakdownResponse {
                category: "Social".to_string(),
                amount: 25.3,
                item_name: "Community Engagement".to_string(),
                contribution: 29.6,
                impact_score: 7.2,
            },
            ImpactBreakdownResponse {
                category: "Economic".to_string(),
                amount: 15.0,
                item_name: "Local Economic Support".to_string(),
                contribution: 17.6,
                impact_score: 6.8,
            },
        ],
        distribution: vec![
            ImpactDistributionResponse {
                category: "Environmental".to_string(),
                weight: 0.528,
            },
            ImpactDistributionResponse {
                category: "Social".to_string(),
                weight: 0.296,
            },
            ImpactDistributionResponse {
                category: "Economic".to_string(),
                weight: 0.176,
            },
        ],
        timeline: vec![
            ImpactTimelineResponse {
                date: "2024-01-15".to_string(),
                description: "Started using eco-friendly products".to_string(),
                impact_value: 15.2,
                timestamp: 1705276800000,
                score: 7.5,
            },
            ImpactTimelineResponse {
                date: "2024-03-20".to_string(),
                description: "Joined local sustainability group".to_string(),
                impact_value: 25.3,
                timestamp: 1710892800000,
                score: 8.2,
            },
            ImpactTimelineResponse {
                date: "2024-06-10".to_string(),
                description: "Implemented recycling program".to_string(),
                impact_value: 45.0,
                timestamp: 1717977600000,
                score: 9.1,
            },
        ],
    }
}