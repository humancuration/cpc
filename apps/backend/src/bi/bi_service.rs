//! Core Business Intelligence service
//! 
//! Provides the foundational BI functionality including data aggregation,
//! processing, and coordination with cpc-node workers.

use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

use crate::bi::models::*;
use crate::bi::impact_report::ImpactReportService;

/// Core BI service that coordinates all BI operations
#[derive(Debug)]
pub struct BIService {
    impact_service: Arc<ImpactReportService>,
    config: Arc<RwLock<BIConfig>>,
    // TODO: Add gRPC client for cpc-node integration
}

impl BIService {
    /// Create a new BI service instance
    pub fn new(config: BIConfig) -> Self {
        let impact_service = Arc::new(ImpactReportService::new());
        let config = Arc::new(RwLock::new(config));
        
        Self {
            impact_service,
            config,
        }
    }

    /// Get impact report for a specific user
    pub async fn get_impact_report(&self, user_id: Uuid) -> Result<ImpactReport> {
        self.impact_service.get_report(user_id).await
    }

    /// Generate a new impact report for a user
    pub async fn generate_impact_report(&self, user_id: Uuid) -> Result<ProcessingStatus> {
        self.impact_service.generate_report(user_id).await
    }

    /// Get processing status for a report generation job
    pub async fn get_processing_status(&self, job_id: Uuid) -> Result<ProcessingStatus> {
        self.impact_service.get_status(job_id).await
    }

    /// Update BI configuration
    pub async fn update_config(&self, new_config: BIConfig) -> Result<()> {
        let mut config = self.config.write().await;
        *config = new_config;
        Ok(())
    }

    /// Get current BI configuration
    pub async fn get_config(&self) -> BIConfig {
        self.config.read().await.clone()
    }

    /// Create router for BI service endpoints
    pub fn router(bi_service: Arc<BIService>) -> axum::Router {
        use axum::routing::{get, post};
        use crate::routes::impact;
        
        axum::Router::new()
            .route("/impact-report/:user_id", get(impact::get_impact_report))
            .route("/generate-report", post(impact::generate_impact_report))
            .route("/status/:job_id", get(impact::get_processing_status))
            .with_state(bi_service)
    }
}

/// Trait for data source adapters
#[async_trait]
pub trait DataSourceAdapter: Send + Sync {
    async fn fetch_data(&self, params: &BIQueryParams) -> Result<Vec<u8>>;
    async fn validate_connection(&self) -> Result<bool>;
}

/// Trait for data processors
#[async_trait]
pub trait DataProcessor: Send + Sync {
    async fn process_data(&self, raw_data: Vec<u8>) -> Result<ProcessedData>;
}

/// Processed data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedData {
    pub metadata: ProcessingMetadata,
    pub data: Vec<u8>,
}

/// Processing metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingMetadata {
    pub processed_at: DateTime<Utc>,
    pub processing_time_ms: u64,
    pub data_source: String,
    pub record_count: u64,
}