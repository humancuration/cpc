use serde::{Deserialize, Serialize};
use uuid::Uuid;
use cpc_core::impact::{ImpactReport, MetricTonnes, DiversityStats};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactDashboardData {
    pub report: ImpactReport,
    pub carbon_footprint: CarbonFootprintData,
    pub community_investment: CommunityInvestmentData,
    pub diversity_metrics: DiversityMetricsData,
    pub supply_chain_score: SupplyChainData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbonFootprintData {
    pub total_emissions: f64,
    pub total_sequestered: f64,
    pub net_footprint: f64,
    pub trend: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityInvestmentData {
    pub total_amount: f64,
    pub beneficiaries: u32,
    pub trend: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiversityMetricsData {
    pub gender_diversity: f64,
    pub ethnic_diversity: f64,
    pub pay_equity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyChainData {
    pub ethics_score: f64,
    pub supplier_count: u32,
    pub local_suppliers_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactReportRequest {
    pub org_id: Uuid,
    pub year: i32,
}