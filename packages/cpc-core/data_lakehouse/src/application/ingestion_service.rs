//! Ingestion service for managing data ingestion jobs

use crate::domain::models::{
    IngestionJob, DataSource, JobSchedule, DataError, JobExecutionResult, StorageFormat
};
use uuid::Uuid;
use std::sync::Arc;
use async_trait::async_trait;

/// Repository trait for ingestion jobs
#[async_trait]
pub trait IngestionJobRepository: Send + Sync {
    async fn save(&self, job: &IngestionJob) -> Result<(), DataError>;
    async fn get(&self, id: Uuid) -> Result<IngestionJob, DataError>;
    async fn list(&self, status: Option<crate::domain::models::JobStatus>) -> Result<Vec<IngestionJob>, DataError>;
    async fn update(&self, job: &IngestionJob) -> Result<(), DataError>;
    async fn log_execution(&self, job_id: Uuid, result: &JobExecutionResult) -> Result<(), DataError>;
}

/// CDC Manager trait
#[async_trait]
pub trait CDCManager: Send + Sync {
    async fn validate_source(&self, source: &DataSource) -> Result<(), DataError>;
    async fn ingest_postgres(&self, job: &IngestionJob) -> Result<crate::infrastructure::storage::DataFrame, DataError>;
    async fn ingest_sled(&self, job: &IngestionJob) -> Result<crate::infrastructure::storage::DataFrame, DataError>;
}

/// Storage manager trait
#[async_trait]
pub trait StorageManager: Send + Sync {
    async fn apply_transformations(
        &self,
        data: crate::infrastructure::storage::DataFrame,
        logic: &str
    ) -> Result<crate::infrastructure::storage::DataFrame, DataError>;
    
    async fn store(
        &self,
        asset_id: Uuid,
        data: crate::infrastructure::storage::DataFrame,
        format: StorageFormat,
    ) -> Result<JobExecutionResult, DataError>;
}

/// Ingestion service implementation
pub struct IngestionService {
    repository: Arc<dyn IngestionJobRepository>,
    cdc_manager: Arc<dyn CDCManager>,
    storage_manager: Arc<dyn StorageManager>,
}

impl IngestionService {
    pub fn new(
        repository: Arc<dyn IngestionJobRepository>,
        cdc_manager: Arc<dyn CDCManager>,
        storage_manager: Arc<dyn StorageManager>,
    ) -> Self {
        Self {
            repository,
            cdc_manager,
            storage_manager,
        }
    }

    pub async fn create_job(
        &self,
        name: String,
        source: DataSource,
        target_asset_id: Uuid,
        schedule: Option<JobSchedule>,
        transformation_logic: Option<String>,
    ) -> Result<IngestionJob, DataError> {
        let job = IngestionJob::new(
            name, source, target_asset_id, schedule, transformation_logic
        );
        
        // Validate connection to source
        self.cdc_manager.validate_source(&job.source).await?;
        
        self.repository.save(&job).await?;
        Ok(job)
    }

    pub async fn run_job(&self, job_id: Uuid) -> Result<JobExecutionResult, DataError> {
        let job = self.repository.get(job_id).await?;
        
        // Execute CDC or batch ingestion
        let data = match &job.source {
            DataSource::PostgreSQLTable {..} => {
                self.cdc_manager.ingest_postgres(&job).await?
            }
            DataSource::SledCollection {..} => {
                self.cdc_manager.ingest_sled(&job).await?
            }
            // Other source types...
            _ => {
                return Err(DataError::JobExecutionError("Unsupported data source".to_string()));
            }
        };
        
        // Apply transformations
        let transformed = if let Some(logic) = &job.transformation_logic {
            self.storage_manager.apply_transformations(data, logic).await?
        } else {
            data
        };
        
        // Store in appropriate format
        let result = self.storage_manager.store(
            job.target_asset_id,
            transformed,
            job.storage_format.clone()
        ).await?;
        
        // Log execution for monitoring
        self.repository.log_execution(job_id, &result).await?;
        
        Ok(result)
    }
}