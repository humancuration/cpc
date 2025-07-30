//! Data ingestion service for the BI & Analytics module

use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::{
    dataset::{Dataset, DataPoint, DataSource, FieldDefinition},
    compliance::gdpr::AnonymizationLevel,
};
use thiserror::Error;

/// Error types for data ingestion operations
#[derive(Error, Debug)]
pub enum DataIngestionError {
    #[error("Dataset error: {0}")]
    DatasetError(String),
    
    #[error("Data source error: {0}")]
    DataSourceError(String),
    
    #[error("Compliance error: {0}")]
    ComplianceError(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
}

/// Data ingestion service
pub struct DataIngestionService<R: DataRepository, S: DataSourceProvider> {
    repository: R,
    source_provider: S,
}

impl<R: DataRepository, S: DataSourceProvider> DataIngestionService<R, S> {
    /// Create a new data ingestion service
    pub fn new(repository: R, source_provider: S) -> Self {
        Self {
            repository,
            source_provider,
        }
    }
    
    /// Ingest data from a source into a dataset
    pub async fn ingest_data(
        &self,
        dataset_id: Uuid,
        anonymization_level: AnonymizationLevel,
    ) -> Result<(), DataIngestionError> {
        // Get the dataset
        let dataset = self.repository.get_dataset(dataset_id)
            .await
            .map_err(|e| DataIngestionError::DatasetError(e.to_string()))?;
        
        // Get data from the source
        let raw_data = self.source_provider.fetch_data(&dataset.source)
            .await
            .map_err(|e| DataIngestionError::DataSourceError(e.to_string()))?;
        
        // Process and anonymize data based on compliance requirements
        let processed_data = self.process_data(raw_data, &anonymization_level)?;
        
        // Store the processed data
        self.repository.store_data_points(dataset_id, processed_data)
            .await
            .map_err(|e| DataIngestionError::StorageError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Process raw data and apply anonymization
    fn process_data(
        &self,
        raw_data: Vec<serde_json::Value>,
        anonymization_level: &AnonymizationLevel,
    ) -> Result<Vec<DataPoint>, DataIngestionError> {
        let mut processed_data = Vec::new();
        
        for raw_item in raw_data {
            let timestamp = if let Some(ts_str) = raw_item.get("timestamp") {
                if let Some(ts_str) = ts_str.as_str() {
                    DateTime::parse_from_rfc3339(ts_str)
                        .map_err(|e| DataIngestionError::DataSourceError(e.to_string()))?
                        .with_timezone(&Utc)
                } else {
                    Utc::now()
                }
            } else {
                Utc::now()
            };
            
            let mut values = std::collections::HashMap::new();
            let mut metadata = std::collections::HashMap::new();
            
            // Apply anonymization based on level
            match anonymization_level {
                AnonymizationLevel::None => {
                    // No anonymization, copy all fields
                    if let Some(obj) = raw_item.as_object() {
                        for (key, value) in obj {
                            if key != "timestamp" {
                                values.insert(key.clone(), value.clone());
                            }
                        }
                    }
                }
                AnonymizationLevel::Basic => {
                    // Basic anonymization - remove personal identifiers
                    if let Some(obj) = raw_item.as_object() {
                        for (key, value) in obj {
                            if key != "timestamp" && 
                               key != "name" && 
                               key != "email" && 
                               key != "phone" &&
                               key != "address" {
                                values.insert(key.clone(), value.clone());
                            } else if key == "name" || key == "email" || key == "phone" || key == "address" {
                                metadata.insert(format!("{}_anonymized", key), "true".to_string());
                            }
                        }
                    }
                }
                AnonymizationLevel::Strong => {
                    // Strong anonymization - aggregate data and remove identifiers
                    if let Some(obj) = raw_item.as_object() {
                        for (key, value) in obj {
                            // Only keep numeric and date fields for strong anonymization
                            if key != "timestamp" && 
                               (value.is_number() || value.is_boolean()) {
                                values.insert(key.clone(), value.clone());
                            } else if !value.is_number() && !value.is_boolean() {
                                metadata.insert(format!("{}_anonymized", key), "true".to_string());
                            }
                        }
                    }
                }
            }
            
            let data_point = DataPoint {
                timestamp,
                values,
                metadata,
            };
            
            processed_data.push(data_point);
        }
        
        Ok(processed_data)
    }
    
    /// Create a new dataset
    pub async fn create_dataset(
        &self,
        name: String,
        source: DataSource,
        fields: Vec<FieldDefinition>,
        owner_id: Uuid,
        description: Option<String>,
    ) -> Result<Dataset, DataIngestionError> {
        let dataset = Dataset::new(name, source, fields, owner_id, description)
            .map_err(|e| DataIngestionError::DatasetError(e.to_string()))?;
        
        self.repository.save_dataset(&dataset)
            .await
            .map_err(|e| DataIngestionError::StorageError(e.to_string()))?;
        
        Ok(dataset)
    }
}

/// Repository trait for data storage
#[async_trait]
pub trait DataRepository: Send + Sync {
    /// Save a dataset
    async fn save_dataset(&self, dataset: &Dataset) -> Result<(), DataIngestionError>;
    
    /// Get a dataset by ID
    async fn get_dataset(&self, id: Uuid) -> Result<Dataset, DataIngestionError>;
    
    /// Store data points for a dataset
    async fn store_data_points(&self, dataset_id: Uuid, data_points: Vec<DataPoint>) -> Result<(), DataIngestionError>;
    
    /// Get data points for a dataset
    async fn get_data_points(&self, dataset_id: Uuid) -> Result<Vec<DataPoint>, DataIngestionError>;
}

/// Data source provider trait
#[async_trait]
pub trait DataSourceProvider: Send + Sync {
    /// Fetch data from a source
    async fn fetch_data(&self, source: &DataSource) -> Result<Vec<serde_json::Value>, DataIngestionError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::dataset::{DataType, DataSource};
    use serde_json::json;
    use std::collections::HashMap;
    
    // Mock repository for testing
    struct MockRepository;
    
    #[async_trait]
    impl DataRepository for MockRepository {
        async fn save_dataset(&self, _dataset: &Dataset) -> Result<(), DataIngestionError> {
            Ok(())
        }
        
        async fn get_dataset(&self, _id: Uuid) -> Result<Dataset, DataIngestionError> {
            let fields = vec![
                FieldDefinition {
                    name: "revenue".to_string(),
                    data_type: DataType::Float,
                    is_nullable: false,
                    description: Some("Monthly revenue".to_string()),
                }
            ];
            
            Ok(Dataset::new(
                "Test Dataset".to_string(),
                DataSource::Finance,
                fields,
                Uuid::new_v4(),
                Some("Test dataset".to_string()),
            ).unwrap())
        }
        
        async fn store_data_points(&self, _dataset_id: Uuid, _data_points: Vec<DataPoint>) -> Result<(), DataIngestionError> {
            Ok(())
        }
        
        async fn get_data_points(&self, _dataset_id: Uuid) -> Result<Vec<DataPoint>, DataIngestionError> {
            Ok(vec![])
        }
    }
    
    // Mock data source provider for testing
    struct MockDataSourceProvider;
    
    #[async_trait]
    impl DataSourceProvider for MockDataSourceProvider {
        async fn fetch_data(&self, _source: &DataSource) -> Result<Vec<serde_json::Value>, DataIngestionError> {
            let data = vec![
                json!({
                    "timestamp": "2023-01-01T00:00:00Z",
                    "revenue": 1000.0,
                    "name": "John Doe",
                    "email": "john@example.com"
                }),
                json!({
                    "timestamp": "2023-01-02T00:00:00Z",
                    "revenue": 1500.0,
                    "name": "Jane Smith",
                    "email": "jane@example.com"
                })
            ];
            Ok(data)
        }
    }
    
    #[tokio::test]
    async fn test_create_dataset() {
        let repository = MockRepository;
        let source_provider = MockDataSourceProvider;
        let service = DataIngestionService::new(repository, source_provider);
        
        let fields = vec![
            FieldDefinition {
                name: "revenue".to_string(),
                data_type: DataType::Float,
                is_nullable: false,
                description: Some("Monthly revenue".to_string()),
            }
        ];
        
        let dataset = service.create_dataset(
            "Test Dataset".to_string(),
            DataSource::Finance,
            fields,
            Uuid::new_v4(),
            Some("Test dataset".to_string()),
        ).await.unwrap();
        
        assert_eq!(dataset.name, "Test Dataset");
        assert_eq!(dataset.source, DataSource::Finance);
    }
    
    #[tokio::test]
    async fn test_process_data_no_anonymization() {
        let repository = MockRepository;
        let source_provider = MockDataSourceProvider;
        let service = DataIngestionService::new(repository, source_provider);
        
        let raw_data = vec![
            json!({
                "timestamp": "2023-01-01T00:00:00Z",
                "revenue": 1000.0,
                "name": "John Doe"
            })
        ];
        
        let processed_data = service.process_data(raw_data, &AnonymizationLevel::None).unwrap();
        
        assert_eq!(processed_data.len(), 1);
        assert_eq!(processed_data[0].values.len(), 2); // revenue and name
        assert!(processed_data[0].values.contains_key("revenue"));
        assert!(processed_data[0].values.contains_key("name"));
    }
    
    #[tokio::test]
    async fn test_process_data_basic_anonymization() {
        let repository = MockRepository;
        let source_provider = MockDataSourceProvider;
        let service = DataIngestionService::new(repository, source_provider);
        
        let raw_data = vec![
            json!({
                "timestamp": "2023-01-01T00:00:00Z",
                "revenue": 1000.0,
                "name": "John Doe",
                "email": "john@example.com"
            })
        ];
        
        let processed_data = service.process_data(raw_data, &AnonymizationLevel::Basic).unwrap();
        
        assert_eq!(processed_data.len(), 1);
        assert_eq!(processed_data[0].values.len(), 1); // only revenue
        assert!(processed_data[0].values.contains_key("revenue"));
        assert!(!processed_data[0].values.contains_key("name"));
        assert!(!processed_data[0].values.contains_key("email"));
        assert!(processed_data[0].metadata.contains_key("name_anonymized"));
        assert!(processed_data[0].metadata.contains_key("email_anonymized"));
    }
}