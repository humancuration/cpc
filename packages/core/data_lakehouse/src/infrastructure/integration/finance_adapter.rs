//! Finance module integration adapter

use crate::domain::models::{DataAsset, DataAssetType, StorageFormat, DataSource, IngestionJob, JobSchedule};
use crate::infrastructure::cdc::postgres_cdc::PostgresCDCManager;
use uuid::Uuid;

/// Adapter for integrating finance module data into the lakehouse
pub struct FinanceAdapter {
    cdc_manager: PostgresCDCManager,
}

impl FinanceAdapter {
    pub fn new(cdc_manager: PostgresCDCManager) -> Self {
        Self { cdc_manager }
    }

    /// Build CDC connection to finance database
    pub async fn create_finance_ingestion_job(
        &self,
        name: String,
        finance_db_connection: String,
        table_name: String,
    ) -> Result<IngestionJob, Box<dyn std::error::Error>> {
        let data_source = DataSource::PostgreSQLTable {
            connection: finance_db_connection,
            table: table_name,
        };

        // Validate the data source
        self.cdc_manager.validate_source(&data_source).await?;

        let job = IngestionJob::new(
            name,
            data_source,
            Uuid::new_v4(), // Target asset ID would be created by the service
            Some(JobSchedule::Interval(chrono::Duration::hours(1))), // Hourly sync
            Some("SELECT * FROM finance_table".to_string()), // Simple transformation
        );

        Ok(job)
    }

    /// Implement financial transformation pipeline
    pub fn transform_financial_data(&self, asset: &mut DataAsset) {
        // Add finance-specific tags
        asset.tags.extend(vec![
            "finance".to_string(),
            "transactional".to_string(),
        ]);

        // Apply financial data transformations
        // In a real implementation, this would include:
        // - Currency normalization
        // - Account categorization
        // - Compliance tagging
        // - Data quality checks
    }
}