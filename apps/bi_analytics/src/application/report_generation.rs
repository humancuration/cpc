//! Report generation service for the BI & Analytics module

use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::{
    report::{Report, VisualizationType},
    dataset::{Dataset, DataPoint},
};
use thiserror::Error;

/// Error types for report generation operations
#[derive(Error, Debug)]
pub enum ReportGenerationError {
    #[error("Report error: {0}")]
    ReportError(String),
    
    #[error("Dataset error: {0}")]
    DatasetError(String),
    
    #[error("Data query error: {0}")]
    DataQueryError(String),
    
    #[error("Visualization error: {0}")]
    VisualizationError(String),
}

/// Report generation service
pub struct ReportGenerationService<R: ReportRepository, D: DataRepository> {
    report_repository: R,
    data_repository: D,
}

impl<R: ReportRepository, D: DataRepository> ReportGenerationService<R, D> {
    /// Create a new report generation service
    pub fn new(report_repository: R, data_repository: D) -> Self {
        Self {
            report_repository,
            data_repository,
        }
    }
    
    /// Create a new report
    pub async fn create_report(
        &self,
        dataset_id: Uuid,
        name: String,
        query: String,
        visualization_type: VisualizationType,
        owner_id: Uuid,
        description: Option<String>,
    ) -> Result<Report, ReportGenerationError> {
        let report = Report::new(
            dataset_id,
            name,
            query,
            visualization_type,
            owner_id,
            description,
        ).map_err(|e| ReportGenerationError::ReportError(e.to_string()))?;
        
        self.report_repository.save_report(&report)
            .await
            .map_err(|e| ReportGenerationError::ReportError(e.to_string()))?;
        
        Ok(report)
    }
    
    /// Generate report data
    pub async fn generate_report_data(
        &self,
        report_id: Uuid,
    ) -> Result<ReportData, ReportGenerationError> {
        // Get the report
        let report = self.report_repository.get_report(report_id)
            .await
            .map_err(|e| ReportGenerationError::ReportError(e.to_string()))?;
        
        // Get the dataset
        let dataset = self.data_repository.get_dataset(report.dataset_id)
            .await
            .map_err(|e| ReportGenerationError::DatasetError(e.to_string()))?;
        
        // Get data points
        let data_points = self.data_repository.get_data_points(report.dataset_id)
            .await
            .map_err(|e| ReportGenerationError::DataQueryError(e.to_string()))?;
        
        // Apply query/filter to data points
        let filtered_data = self.apply_query(&data_points, &report.query)?;
        
        // Format data for visualization
        let formatted_data = self.format_data_for_visualization(&filtered_data, &dataset)?;
        
        Ok(ReportData {
            report_id,
            data: formatted_data,
            visualization_type: report.visualization_type,
        })
    }
    
    /// Apply query/filter to data points
    fn apply_query(
        &self,
        data_points: &[DataPoint],
        query: &str,
    ) -> Result<Vec<DataPoint>, ReportGenerationError> {
        // This is a simplified implementation
        // In a real system, this would parse and execute SQL-like queries
        
        // For now, we'll just return all data points if query is "SELECT *"
        if query.trim().to_uppercase() == "SELECT *" {
            Ok(data_points.to_vec())
        } else {
            // In a real implementation, we would parse the query and filter data accordingly
            // For now, we'll just return all data points
            Ok(data_points.to_vec())
        }
    }
    
    /// Format data for visualization
    fn format_data_for_visualization(
        &self,
        data_points: &[DataPoint],
        dataset: &Dataset,
    ) -> Result<serde_json::Value, ReportGenerationError> {
        // Convert data points to a format suitable for visualization
        let mut formatted_data = Vec::new();
        
        for data_point in data_points {
            let mut formatted_point = serde_json::Map::new();
            
            // Add timestamp
            formatted_point.insert(
                "timestamp".to_string(),
                serde_json::Value::String(data_point.timestamp.to_rfc3339())
            );
            
            // Add data values
            for field in &dataset.fields {
                if let Some(value) = data_point.values.get(&field.name) {
                    formatted_point.insert(field.name.clone(), value.clone());
                }
            }
            
            formatted_data.push(serde_json::Value::Object(formatted_point));
        }
        
        Ok(serde_json::Value::Array(formatted_data))
    }
    
    /// Update report information
    pub async fn update_report(
        &self,
        report_id: Uuid,
        name: Option<String>,
        description: Option<String>,
        query: Option<String>,
        visualization_type: Option<VisualizationType>,
    ) -> Result<Report, ReportGenerationError> {
        let mut report = self.report_repository.get_report(report_id)
            .await
            .map_err(|e| ReportGenerationError::ReportError(e.to_string()))?;
        
        report.update_info(name, description, query, visualization_type)
            .map_err(|e| ReportGenerationError::ReportError(e.to_string()))?;
        
        self.report_repository.save_report(&report)
            .await
            .map_err(|e| ReportGenerationError::ReportError(e.to_string()))?;
        
        Ok(report)
    }
}

/// Report data structure for visualization
pub struct ReportData {
    pub report_id: Uuid,
    pub data: serde_json::Value,
    pub visualization_type: VisualizationType,
}

/// Repository trait for report storage
#[async_trait]
pub trait ReportRepository: Send + Sync {
    /// Save a report
    async fn save_report(&self, report: &Report) -> Result<(), ReportGenerationError>;
    
    /// Get a report by ID
    async fn get_report(&self, id: Uuid) -> Result<Report, ReportGenerationError>;
    
    /// Get reports by owner
    async fn get_reports_by_owner(&self, owner_id: Uuid) -> Result<Vec<Report>, ReportGenerationError>;
}

/// Repository trait for data access
#[async_trait]
pub trait DataRepository: Send + Sync {
    /// Get a dataset by ID
    async fn get_dataset(&self, id: Uuid) -> Result<Dataset, ReportGenerationError>;
    
    /// Get data points for a dataset
    async fn get_data_points(&self, dataset_id: Uuid) -> Result<Vec<DataPoint>, ReportGenerationError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        dataset::{DataSource, FieldDefinition, DataType},
        report::VisualizationType,
    };
    use chrono::{DateTime, Utc};
    use std::collections::HashMap;
    
    // Mock report repository for testing
    struct MockReportRepository;
    
    #[async_trait]
    impl ReportRepository for MockReportRepository {
        async fn save_report(&self, _report: &Report) -> Result<(), ReportGenerationError> {
            Ok(())
        }
        
        async fn get_report(&self, _id: Uuid) -> Result<Report, ReportGenerationError> {
            Ok(Report::new(
                Uuid::new_v4(),
                "Test Report".to_string(),
                "SELECT *".to_string(),
                VisualizationType::BarChart,
                Uuid::new_v4(),
                Some("Test report".to_string()),
            ).unwrap())
        }
        
        async fn get_reports_by_owner(&self, _owner_id: Uuid) -> Result<Vec<Report>, ReportGenerationError> {
            Ok(vec![])
        }
    }
    
    // Mock data repository for testing
    struct MockDataRepository;
    
    #[async_trait]
    impl DataRepository for MockDataRepository {
        async fn get_dataset(&self, _id: Uuid) -> Result<Dataset, ReportGenerationError> {
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
        
        async fn get_data_points(&self, _dataset_id: Uuid) -> Result<Vec<DataPoint>, ReportGenerationError> {
            let mut values = HashMap::new();
            values.insert("revenue".to_string(), serde_json::Value::Number(serde_json::Number::from(1000)));
            
            let data_point = DataPoint {
                timestamp: Utc::now(),
                values,
                metadata: HashMap::new(),
            };
            
            Ok(vec![data_point])
        }
    }
    
    #[tokio::test]
    async fn test_create_report() {
        let report_repository = MockReportRepository;
        let data_repository = MockDataRepository;
        let service = ReportGenerationService::new(report_repository, data_repository);
        
        let report = service.create_report(
            Uuid::new_v4(),
            "Test Report".to_string(),
            "SELECT *".to_string(),
            VisualizationType::BarChart,
            Uuid::new_v4(),
            Some("Test report".to_string()),
        ).await.unwrap();
        
        assert_eq!(report.name, "Test Report");
        assert_eq!(report.visualization_type, VisualizationType::BarChart);
    }
    
    #[tokio::test]
    async fn test_generate_report_data() {
        let report_repository = MockReportRepository;
        let data_repository = MockDataRepository;
        let service = ReportGenerationService::new(report_repository, data_repository);
        
        let report_id = Uuid::new_v4();
        let report_data = service.generate_report_data(report_id).await.unwrap();
        
        assert_eq!(report_data.report_id, report_id);
        assert_eq!(report_data.visualization_type, VisualizationType::BarChart);
        assert!(report_data.data.is_array());
    }
    
    #[tokio::test]
    async fn test_update_report() {
        let report_repository = MockReportRepository;
        let data_repository = MockDataRepository;
        let service = ReportGenerationService::new(report_repository, data_repository);
        
        let report_id = Uuid::new_v4();
        let updated_report = service.update_report(
            report_id,
            Some("Updated Report".to_string()),
            Some("Updated description".to_string()),
            Some("SELECT revenue FROM data".to_string()),
            Some(VisualizationType::LineChart),
        ).await.unwrap();
        
        assert_eq!(updated_report.name, "Updated Report");
        assert_eq!(updated_report.description, Some("Updated description".to_string()));
        assert_eq!(updated_report.query, "SELECT revenue FROM data");
        assert_eq!(updated_report.visualization_type, VisualizationType::LineChart);
    }
}