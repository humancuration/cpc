use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

// Aggregate Root
#[derive(Debug, Serialize, Deserialize)]
pub struct SupplyChain {
    pub product_id: Uuid,
    pub stages: Vec<ProductionStage>,
    pub connections: Vec<StageConnection>,
    pub cooperative_impact: CooperativeImpactSummary,
    pub timeline_range: (DateTime<Utc>, DateTime<Utc>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductionStage {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub location: String, // Could be a more complex type later
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    // Other relevant metadata
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StageConnection {
    pub from_stage_id: Uuid,
    pub to_stage_id: Uuid,
    pub relationship_type: String, // e.g., "PART_OF", "TRANSFORMED_INTO"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CooperativeImpactSummary {
    pub workers_benefited: u32,
    pub coops_involved: u32,
    pub ethical_sourcing_score: Decimal, // A calculated score
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ProductSummary {
    pub id: Uuid,
    pub name: String,
}

// Data Transfer Objects for updating a supply chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSupplyChainData {
    pub product_id: Uuid,
    pub stages: Vec<ProductionStageData>,
    pub connections: Vec<StageConnectionData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionStageData {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub location: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageConnectionData {
    pub from_stage_id: Uuid,
    pub to_stage_id: Uuid,
    pub relationship_type: String,
}