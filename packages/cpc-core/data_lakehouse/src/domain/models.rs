//! Core domain models for the data lakehouse

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Represents a data asset in the lakehouse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAsset {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub asset_type: DataAssetType,
    pub storage_format: StorageFormat,
    pub schema: JsonValue,  // JSON representation of schema
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: u64,
    pub lineage: DataLineage,  // Reference to lineage tracking
}

/// Types of data assets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataAssetType {
    Structured,
    SemiStructured,
    Unstructured,
    TimeSeries,
    Geospatial,
    Graph,
}

/// Storage formats for data assets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageFormat {
    PostgreSQL,
    SledEdge,
    WebMColumnar,  // WebM container with AV1 for structured data
}

/// Data lineage tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataLineage {
    pub sources: Vec<Uuid>,  // Source data asset IDs
    pub transformations: Vec<String>,  // Applied transformations
    pub created_by: Option<Uuid>,  // User or process that created this asset
}

impl DataLineage {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            transformations: Vec::new(),
            created_by: None,
        }
    }
}

impl Default for DataLineage {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents an ingestion job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestionJob {
    pub id: Uuid,
    pub name: String,
    pub source: DataSource,
    pub target_asset_id: Uuid,
    pub schedule: Option<JobSchedule>,
    pub transformation_logic: Option<String>,  // Rust code or SQL
    pub status: JobStatus,
    pub last_run: Option<DateTime<Utc>>,
    pub next_run: Option<DateTime<Utc>>,
    pub error_count: u32,
    pub max_errors: u32,
}

impl IngestionJob {
    pub fn new(
        name: String,
        source: DataSource,
        target_asset_id: Uuid,
        schedule: Option<JobSchedule>,
        transformation_logic: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            source,
            target_asset_id,
            schedule,
            transformation_logic,
            status: JobStatus::Pending,
            last_run: None,
            next_run: None,
            error_count: 0,
            max_errors: 10,
        }
    }
}

/// Data source types for ingestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSource {
    PostgreSQLTable { connection: String, table: String },
    SledCollection { node_id: Uuid, collection: String },
    ExternalAPI { url: String, auth: Option<ApiAuth> },
    FileDrop { path: String, pattern: String },
}

/// Authentication for external APIs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiAuth {
    BearerToken(String),
    BasicAuth { username: String, password: String },
    ApiKey { key: String, header: String },
}

/// Job scheduling options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobSchedule {
    Realtime,
    Interval(chrono::Duration),
    Cron(String),
}

/// Status of an ingestion job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobStatus {
    Pending,
    Running,
    Success,
    Failed,
    Cancelled,
}

/// Access purposes for audit logging
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccessPurpose {
    UserView,
    ProviderAccess,
    Research,
    DataSync,
    Admin,
    Analytics,
    MachineLearning,
}

/// Data actions for audit logging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataAction {
    Read,
    Write,
    Delete,
    SchemaChange,
    VersionRestore,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub asset_id: Uuid,
    pub purpose: AccessPurpose,
    pub action: DataAction,
    pub timestamp: DateTime<Utc>,
    pub source_ip: Option<String>,
    pub device_info: Option<String>,
    pub data_content: Option<String>,
}

impl AuditLog {
    pub fn new(
        user_id: Option<Uuid>,
        asset_id: Uuid,
        purpose: AccessPurpose,
        action: DataAction,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            asset_id,
            purpose,
            action,
            timestamp,
            source_ip: None,
            device_info: None,
            data_content: None,
        }
    }
}

/// Result of a job execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobExecutionResult {
    pub job_id: Uuid,
    pub success: bool,
    pub records_processed: u64,
    pub error_message: Option<String>,
    pub execution_time: chrono::Duration,
    pub output_asset_id: Option<Uuid>,
}

/// Data error types
#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Conversion error: {0}")]
    ConversionError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Job execution error: {0}")]
    JobExecutionError(String),
    
    #[error("Audit error: {0}")]
    AuditError(String),
}

/// Storage error types
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Conversion error: {0}")]
    ConversionError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Audit error types
#[derive(Debug, thiserror::Error)]
pub enum AuditError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}