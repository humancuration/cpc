//! Advanced reporting service for the advanced CRM module
//!
//! This module contains the application service for generating sales reports.

use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Error types for reporting operations
#[derive(Debug, thiserror::Error)]
pub enum ReportingServiceError {
    #[error("Data access error: {0}")]
    DataAccessError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Export error: {0}")]
    ExportError(String),
}

/// Filter criteria for sales reports
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReportFilters {
    pub date_range: Option<DateRange>,
    pub user_ids: Option<Vec<Uuid>>,
    pub team_ids: Option<Vec<Uuid>>,
    pub status_filters: Option<Vec<DealStatus>>,
    pub min_amount: Option<i64>, // in cents
    pub max_amount: Option<i64>, // in cents
}

/// Date range filter
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DateRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

/// Status of a sales deal
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DealStatus {
    Prospecting,
    Qualified,
    Proposal,
    Negotiation,
    Won,
    Lost,
}

/// Sales report representation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SalesReport {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub filters: ReportFilters,
    pub data: ReportData,
    pub generated_at: DateTime<Utc>,
    pub generated_by: Uuid,
}

/// Report data structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReportData {
    pub summary: ReportSummary,
    pub deals: Vec<DealData>,
    pub trends: Vec<TrendData>,
    pub team_performance: Vec<TeamPerformanceData>,
}

/// Summary statistics for a report
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReportSummary {
    pub total_deals: u32,
    pub total_value: i64, // in cents
    pub average_deal_size: i64, // in cents
    pub win_rate: f32,
    pub conversion_rate: f32,
}

/// Deal data for reporting
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DealData {
    pub id: Uuid,
    pub name: String,
    pub value: i64, // in cents
    pub status: DealStatus,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
}

/// Trend data for reporting
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrendData {
    pub period: String,
    pub deal_count: u32,
    pub total_value: i64, // in cents
    pub win_rate: f32,
}

/// Team performance data for reporting
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TeamPerformanceData {
    pub team_id: Uuid,
    pub team_name: String,
    pub member_count: u32,
    pub total_deals: u32,
    pub total_value: i64, // in cents
    pub average_deal_size: i64, // in cents
    pub win_rate: f32,
}

/// Pipeline analysis data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PipelineAnalysis {
    pub total_pipeline_value: i64, // in cents
    pub pipeline_by_stage: Vec<StageData>,
    pub aging_analysis: Vec<AgingData>,
    pub forecast: ForecastData,
}

/// Stage data for pipeline analysis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StageData {
    pub stage: DealStatus,
    pub deal_count: u32,
    pub total_value: i64, // in cents
    pub average_age: f32, // in days
}

/// Aging data for pipeline analysis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgingData {
    pub days_range: String,
    pub deal_count: u32,
    pub total_value: i64, // in cents
}

/// Forecast data for pipeline analysis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForecastData {
    pub probability: f32,
    pub expected_value: i64, // in cents
    pub confidence_interval: (i64, i64), // in cents
}

/// Dashboard data for real-time visualization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DashboardData {
    pub kpi_data: Vec<KpiData>,
    pub recent_activity: Vec<ActivityData>,
    pub alerts: Vec<ReportAlert>,
}

/// KPI data for dashboard
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KpiData {
    pub name: String,
    pub value: f64,
    pub trend: f32, // positive for upward trend, negative for downward
    pub unit: String,
}

/// Activity data for dashboard
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActivityData {
    pub id: Uuid,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub user_id: Uuid,
    pub activity_type: ActivityType,
}

/// Types of activities for dashboard
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActivityType {
    DealWon,
    DealLost,
    NewLead,
    MeetingScheduled,
    ProposalSent,
}

/// Alert for reporting/dashboard
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReportAlert {
    pub id: Uuid,
    pub message: String,
    pub severity: AlertSeverity,
    pub created_at: DateTime<Utc>,
}

/// Severity levels for alerts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[async_trait]
pub trait ReportRepository {
    async fn save_report_definition(&self, report: &SalesReport) -> Result<(), ReportingServiceError>;
    async fn get_report_definition(&self, id: Uuid) -> Result<Option<SalesReport>, ReportingServiceError>;
    async fn get_report_definitions(&self, user_id: Uuid) -> Result<Vec<SalesReport>, ReportingServiceError>;
    async fn save_report_instance(&self, report: &SalesReport) -> Result<(), ReportingServiceError>;
    async fn get_report_instance(&self, id: Uuid) -> Result<Option<SalesReport>, ReportingServiceError>;
}

#[async_trait]
pub trait CrmDataAccess {
    async fn get_deals(&self, filters: &ReportFilters) -> Result<Vec<DealData>, ReportingServiceError>;
    async fn get_pipeline_data(&self) -> Result<Vec<DealData>, ReportingServiceError>;
    async fn get_team_data(&self, team_ids: Option<Vec<Uuid>>) -> Result<Vec<TeamPerformanceData>, ReportingServiceError>;
}

pub struct AdvancedReportingService {
    report_repository: Arc<dyn ReportRepository>,
    crm_data_access: Arc<dyn CrmDataAccess>,
}

impl AdvancedReportingService {
    pub fn new(
        report_repository: Arc<dyn ReportRepository>,
        crm_data_access: Arc<dyn CrmDataAccess>,
    ) -> Self {
        Self {
            report_repository,
            crm_data_access,
        }
    }

    /// Generates comprehensive sales reports
    pub async fn generate_sales_report(
        &self,
        name: String,
        description: Option<String>,
        filters: ReportFilters,
        generated_by: Uuid,
    ) -> Result<SalesReport, ReportingServiceError> {
        // Validate filters
        if let Some(date_range) = &filters.date_range {
            if date_range.start > date_range.end {
                return Err(ReportingServiceError::ValidationError(
                    "Start date must be before end date".to_string()
                ));
            }
        }

        // Get deal data based on filters
        let deals = self.crm_data_access.get_deals(&filters).await?;

        // Calculate summary statistics
        let summary = self.calculate_summary(&deals);

        // Calculate trends (simplified - in a real implementation this would be more complex)
        let trends = self.calculate_trends(&deals);

        // Get team performance data
        let team_performance = self.crm_data_access.get_team_data(filters.team_ids.clone()).await?;

        let report_data = ReportData {
            summary,
            deals,
            trends,
            team_performance,
        };

        let report = SalesReport {
            id: Uuid::new_v4(),
            name,
            description,
            filters,
            data: report_data,
            generated_at: Utc::now(),
            generated_by,
        };

        // Save the report instance
        self.report_repository.save_report_instance(&report).await?;

        Ok(report)
    }

    /// Provides deep analysis of sales pipeline health
    pub async fn get_pipeline_analysis(&self) -> Result<PipelineAnalysis, ReportingServiceError> {
        // Get all pipeline data
        let deals = self.crm_data_access.get_pipeline_data().await?;

        // Calculate total pipeline value
        let total_pipeline_value: i64 = deals.iter().map(|d| d.value).sum();

        // Group deals by stage
        let mut pipeline_by_stage = Vec::new();
        let stages = [
            DealStatus::Prospecting,
            DealStatus::Qualified,
            DealStatus::Proposal,
            DealStatus::Negotiation,
            DealStatus::Won,
            DealStatus::Lost,
        ];

        for stage in &stages {
            let stage_deals: Vec<&DealData> = deals.iter().filter(|d| &d.status == stage).collect();
            let deal_count = stage_deals.len() as u32;
            let total_value: i64 = stage_deals.iter().map(|d| d.value).sum();
            
            // Calculate average age (simplified)
            let average_age = if !stage_deals.is_empty() {
                let total_age: f32 = stage_deals.iter().map(|d| {
                    Utc::now().signed_duration_since(d.created_at).num_days() as f32
                }).sum();
                total_age / deal_count as f32
            } else {
                0.0
            };

            pipeline_by_stage.push(StageData {
                stage: stage.clone(),
                deal_count,
                total_value,
                average_age,
            });
        }

        // Calculate aging analysis (simplified)
        let aging_analysis = self.calculate_aging_analysis(&deals);

        // Calculate forecast (simplified)
        let forecast = self.calculate_forecast(&deals);

        Ok(PipelineAnalysis {
            total_pipeline_value,
            pipeline_by_stage,
            aging_analysis,
            forecast,
        })
    }

    /// Exports reports to CSV format
    pub async fn export_to_csv(&self, report: &SalesReport) -> Result<Vec<u8>, ReportingServiceError> {
        let mut csv_data = String::new();

        // Add header
        csv_data.push_str("Deal ID,Deal Name,Value,Status,Owner,Created Date,Closed Date\n");

        // Add data rows
        for deal in &report.data.deals {
            let closed_date = deal.closed_at.as_ref()
                .map(|d| d.to_rfc3339())
                .unwrap_or_else(|| "N/A".to_string());

            csv_data.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                deal.id,
                deal.name,
                deal.value,
                format_deal_status(&deal.status),
                deal.owner_id,
                deal.created_at.to_rfc3339(),
                closed_date
            ));
        }

        Ok(csv_data.into_bytes())
    }

    /// Supplies data for Bevy visualizations
    pub async fn get_realtime_dashboard_data(&self) -> Result<DashboardData, ReportingServiceError> {
        // In a real implementation, this would fetch real-time data
        // For now, we'll return sample data

        let kpi_data = vec![
            KpiData {
                name: "Total Pipeline Value".to_string(),
                value: 1250000.0,
                trend: 5.2,
                unit: "USD".to_string(),
            },
            KpiData {
                name: "Conversion Rate".to_string(),
                value: 28.5,
                trend: 2.1,
                unit: "%".to_string(),
            },
            KpiData {
                name: "Avg Deal Size".to_string(),
                value: 25000.0,
                trend: -1.3,
                unit: "USD".to_string(),
            },
        ];

        let recent_activity = vec![
            ActivityData {
                id: Uuid::new_v4(),
                description: "Deal won with ABC Corp".to_string(),
                timestamp: Utc::now() - chrono::Duration::hours(2),
                user_id: Uuid::new_v4(),
                activity_type: ActivityType::DealWon,
            },
            ActivityData {
                id: Uuid::new_v4(),
                description: "Proposal sent to XYZ Inc".to_string(),
                timestamp: Utc::now() - chrono::Duration::hours(5),
                user_id: Uuid::new_v4(),
                activity_type: ActivityType::ProposalSent,
            },
        ];

        let alerts = vec![
            ReportAlert {
                id: Uuid::new_v4(),
                message: "High-value deal at risk".to_string(),
                severity: AlertSeverity::High,
                created_at: Utc::now() - chrono::Duration::hours(1),
            },
        ];

        Ok(DashboardData {
            kpi_data,
            recent_activity,
            alerts,
        })
    }

    fn calculate_summary(&self, deals: &[DealData]) -> ReportSummary {
        let total_deals = deals.len() as u32;
        let total_value: i64 = deals.iter().map(|d| d.value).sum();
        let average_deal_size = if total_deals > 0 {
            total_value / total_deals as i64
        } else {
            0
        };

        // Calculate win rate (deals with status Won / total deals)
        let won_deals = deals.iter().filter(|d| d.status == DealStatus::Won).count() as u32;
        let win_rate = if total_deals > 0 {
            won_deals as f32 / total_deals as f32
        } else {
            0.0
        };

        // Calculate conversion rate (simplified)
        let conversion_rate = win_rate; // In a real implementation, this would be more complex

        ReportSummary {
            total_deals,
            total_value,
            average_deal_size,
            win_rate,
            conversion_rate,
        }
    }

    fn calculate_trends(&self, deals: &[DealData]) -> Vec<TrendData> {
        // Simplified trend calculation - in a real implementation this would be more complex
        // Group deals by month and calculate metrics
        let mut trends = Vec::new();

        // For now, we'll just create a single trend entry
        if !deals.is_empty() {
            let total_value: i64 = deals.iter().map(|d| d.value).sum();
            let won_deals = deals.iter().filter(|d| d.status == DealStatus::Won).count() as u32;
            let win_rate = if !deals.is_empty() {
                won_deals as f32 / deals.len() as f32
            } else {
                0.0
            };

            trends.push(TrendData {
                period: "Current".to_string(),
                deal_count: deals.len() as u32,
                total_value,
                win_rate,
            });
        }

        trends
    }

    fn calculate_aging_analysis(&self, deals: &[DealData]) -> Vec<AgingData> {
        let mut aging_analysis = Vec::new();

        // Categorize deals by age
        let mut age_categories = std::collections::HashMap::new();
        age_categories.insert("0-30 days", (0, 0i64));
        age_categories.insert("31-60 days", (0, 0i64));
        age_categories.insert("61-90 days", (0, 0i64));
        age_categories.insert("90+ days", (0, 0i64));

        for deal in deals {
            let age_days = Utc::now().signed_duration_since(deal.created_at).num_days();
            let (count, value) = match age_days {
                0..=30 => age_categories.get_mut("0-30 days").unwrap(),
                31..=60 => age_categories.get_mut("31-60 days").unwrap(),
                61..=90 => age_categories.get_mut("61-90 days").unwrap(),
                _ => age_categories.get_mut("90+ days").unwrap(),
            };
            *count += 1;
            *value += deal.value;
        }

        for (range, (count, value)) in age_categories {
            aging_analysis.push(AgingData {
                days_range: range.to_string(),
                deal_count: count,
                total_value: value,
            });
        }

        aging_analysis
    }

    fn calculate_forecast(&self, deals: &[DealData]) -> ForecastData {
        // Simplified forecast calculation
        let total_value: i64 = deals.iter().map(|d| d.value).sum();
        let probability = 0.7; // 70% probability (simplified)
        let expected_value = (total_value as f64 * probability) as i64;

        // Confidence interval (simplified)
        let lower_bound = (expected_value as f64 * 0.8) as i64;
        let upper_bound = (expected_value as f64 * 1.2) as i64;

        ForecastData {
            probability: probability as f32,
            expected_value,
            confidence_interval: (lower_bound, upper_bound),
        }
    }
}

/// Helper function to format deal status for CSV export
fn format_deal_status(status: &DealStatus) -> String {
    match status {
        DealStatus::Prospecting => "Prospecting",
        DealStatus::Qualified => "Qualified",
        DealStatus::Proposal => "Proposal",
        DealStatus::Negotiation => "Negotiation",
        DealStatus::Won => "Won",
        DealStatus::Lost => "Lost",
    }.to_string()
}