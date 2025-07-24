//! BI toolkit data models

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Core impact report structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactReport {
    pub user_id: Uuid,
    pub generated_at: DateTime<Utc>,
    pub overall_score: f64,
    pub ethical_distribution: HashMap<ImpactCategory, f64>,
    pub timeline: Vec<ImpactTimelinePoint>,
    pub breakdown: Vec<ImpactBreakdownItem>,
    pub signature: String,
}

/// Impact timeline data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactTimelinePoint {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub category: ImpactCategory,
}

/// Individual impact breakdown item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactBreakdownItem {
    pub item_id: Uuid,
    pub name: String,
    pub category: ImpactCategory,
    pub value: f64,
    pub ethical_score: f64,
}

/// Impact category enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImpactCategory {
    Environmental,
    Social,
    Economic,
}

/// BI service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BIConfig {
    pub data_sources: Vec<DataSourceConfig>,
    pub processing_timeout: u64,
    pub cache_duration: u64,
}

/// Data source configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceConfig {
    pub source_type: DataSourceType,
    pub endpoint: String,
    pub credentials: Option<HashMap<String, String>>,
}

/// Data source types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataSourceType {
    Database,
    Grpc,
    Api,
    File,
}

/// BI query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BIQueryParams {
    pub user_id: Uuid,
    pub date_range: Option<DateRange>,
    pub categories: Option<Vec<ImpactCategory>>,
    pub aggregation_level: Option<AggregationLevel>,
}

/// Date range for queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

/// Data aggregation levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AggregationLevel {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

/// Processing status for long-running jobs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingStatus {
    pub job_id: Uuid,
    pub status: JobStatus,
    pub progress: f64,
    pub message: Option<String>,
    pub estimated_completion: Option<DateTime<Utc>>,
}

/// Job status enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}