//! Report domain entities for the BI & Analytics module

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use thiserror::Error;
use super::dataset::Dataset;

/// Error types for report operations
#[derive(Error, Debug)]
pub enum ReportError {
    #[error("Invalid report data: {0}")]
    InvalidData(String),
    
    #[error("Report not found: {0}")]
    NotFound(String),
    
    #[error("Dataset error: {0}")]
    DatasetError(String),
}

/// Visualization types for reports
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VisualizationType {
    Table,
    BarChart,
    LineChart,
    PieChart,
    ScatterPlot,
    Heatmap,
    AreaChart,
}

/// Main report entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Report {
    pub id: Uuid,
    pub dataset_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub query: String,
    pub visualization_type: VisualizationType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub owner_id: Uuid,
}

impl Report {
    /// Create a new report
    pub fn new(
        dataset_id: Uuid,
        name: String,
        query: String,
        visualization_type: VisualizationType,
        owner_id: Uuid,
        description: Option<String>,
    ) -> Result<Self, ReportError> {
        if name.is_empty() {
            return Err(ReportError::InvalidData("Report name cannot be empty".to_string()));
        }
        
        if query.is_empty() {
            return Err(ReportError::InvalidData("Report query cannot be empty".to_string()));
        }
        
        let now = Utc::now();
        
        Ok(Self {
            id: Uuid::new_v4(),
            dataset_id,
            name,
            description,
            query,
            visualization_type,
            created_at: now,
            updated_at: now,
            owner_id,
        })
    }
    
    /// Update report information
    pub fn update_info(
        &mut self,
        name: Option<String>,
        description: Option<String>,
        query: Option<String>,
        visualization_type: Option<VisualizationType>,
    ) -> Result<(), ReportError> {
        if let Some(name) = name {
            if name.is_empty() {
                return Err(ReportError::InvalidData("Report name cannot be empty".to_string()));
            }
            self.name = name;
        }
        
        if let Some(description) = description {
            self.description = Some(description);
        }
        
        if let Some(query) = query {
            if query.is_empty() {
                return Err(ReportError::InvalidData("Report query cannot be empty".to_string()));
            }
            self.query = query;
        }
        
        if let Some(visualization_type) = visualization_type {
            self.visualization_type = visualization_type;
        }
        
        self.updated_at = Utc::now();
        Ok(())
    }
    
    /// Validate the report
    pub fn validate(&self) -> Result<(), ReportError> {
        if self.name.is_empty() {
            return Err(ReportError::InvalidData("Report name cannot be empty".to_string()));
        }
        
        if self.query.is_empty() {
            return Err(ReportError::InvalidData("Report query cannot be empty".to_string()));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_report() {
        let dataset_id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        
        let report = Report::new(
            dataset_id,
            "Sales Report".to_string(),
            "SELECT * FROM sales_data".to_string(),
            VisualizationType::BarChart,
            owner_id,
            Some("Monthly sales report".to_string()),
        ).unwrap();
        
        assert_eq!(report.name, "Sales Report");
        assert_eq!(report.dataset_id, dataset_id);
        assert_eq!(report.visualization_type, VisualizationType::BarChart);
        assert_eq!(report.owner_id, owner_id);
    }
    
    #[test]
    fn test_update_report_info() {
        let dataset_id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        
        let mut report = Report::new(
            dataset_id,
            "Sales Report".to_string(),
            "SELECT * FROM sales_data".to_string(),
            VisualizationType::BarChart,
            owner_id,
            Some("Monthly sales report".to_string()),
        ).unwrap();
        
        report.update_info(
            Some("Updated Sales Report".to_string()),
            Some("Updated description".to_string()),
            Some("SELECT * FROM updated_sales_data".to_string()),
            Some(VisualizationType::LineChart),
        ).unwrap();
        
        assert_eq!(report.name, "Updated Sales Report");
        assert_eq!(report.description, Some("Updated description".to_string()));
        assert_eq!(report.query, "SELECT * FROM updated_sales_data");
        assert_eq!(report.visualization_type, VisualizationType::LineChart);
    }
    
    #[test]
    fn test_report_validation() {
        let dataset_id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        
        let report = Report::new(
            dataset_id,
            "Sales Report".to_string(),
            "SELECT * FROM sales_data".to_string(),
            VisualizationType::BarChart,
            owner_id,
            Some("Monthly sales report".to_string()),
        ).unwrap();
        
        assert!(report.validate().is_ok());
    }
}