//! Dataset domain entities for the BI & Analytics module

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use thiserror::Error;

/// Error types for dataset operations
#[derive(Error, Debug)]
pub enum DatasetError {
    #[error("Invalid dataset data: {0}")]
    InvalidData(String),
    
    #[error("Dataset not found: {0}")]
    NotFound(String),
    
    #[error("Data source error: {0}")]
    DataSourceError(String),
}

/// Field definition for dataset schema
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldDefinition {
    pub name: String,
    pub data_type: DataType,
    pub is_nullable: bool,
    pub description: Option<String>,
}

/// Data types supported in datasets
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataType {
    String,
    Integer,
    Float,
    Boolean,
    DateTime,
    // Add more types as needed
}

/// Data point in a dataset
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataPoint {
    pub timestamp: DateTime<Utc>,
    pub values: HashMap<String, serde_json::Value>,
    pub metadata: HashMap<String, String>,
}

/// Data source types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataSource {
    Crm,
    Finance,
    Calendar,
    Messenger,
    Custom(String),
}

/// Main dataset entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dataset {
    pub id: Uuid,
    pub name: String,
    pub source: DataSource,
    pub fields: Vec<FieldDefinition>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub owner_id: Uuid,
}

impl Dataset {
    /// Create a new dataset
    pub fn new(
        name: String,
        source: DataSource,
        fields: Vec<FieldDefinition>,
        owner_id: Uuid,
        description: Option<String>,
    ) -> Result<Self, DatasetError> {
        if name.is_empty() {
            return Err(DatasetError::InvalidData("Dataset name cannot be empty".to_string()));
        }
        
        if fields.is_empty() {
            return Err(DatasetError::InvalidData("Dataset must have at least one field".to_string()));
        }
        
        let now = Utc::now();
        
        Ok(Self {
            id: Uuid::new_v4(),
            name,
            source,
            fields,
            description,
            created_at: now,
            updated_at: now,
            owner_id,
        })
    }
    
    /// Update dataset information
    pub fn update_info(
        &mut self,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<(), DatasetError> {
        if let Some(name) = name {
            if name.is_empty() {
                return Err(DatasetError::InvalidData("Dataset name cannot be empty".to_string()));
            }
            self.name = name;
        }
        
        if let Some(description) = description {
            self.description = Some(description);
        }
        
        self.updated_at = Utc::now();
        Ok(())
    }
    
    /// Add a field to the dataset
    pub fn add_field(&mut self, field: FieldDefinition) {
        self.fields.push(field);
        self.updated_at = Utc::now();
    }
    
    /// Remove a field from the dataset
    pub fn remove_field(&mut self, field_name: &str) {
        self.fields.retain(|f| f.name != field_name);
        self.updated_at = Utc::now();
    }
    
    /// Validate the dataset
    pub fn validate(&self) -> Result<(), DatasetError> {
        if self.name.is_empty() {
            return Err(DatasetError::InvalidData("Dataset name cannot be empty".to_string()));
        }
        
        if self.fields.is_empty() {
            return Err(DatasetError::InvalidData("Dataset must have at least one field".to_string()));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_dataset() {
        let owner_id = Uuid::new_v4();
        let fields = vec![
            FieldDefinition {
                name: "revenue".to_string(),
                data_type: DataType::Float,
                is_nullable: false,
                description: Some("Monthly revenue".to_string()),
            }
        ];
        
        let dataset = Dataset::new(
            "Sales Data".to_string(),
            DataSource::Finance,
            fields,
            owner_id,
            Some("Monthly sales data".to_string()),
        ).unwrap();
        
        assert_eq!(dataset.name, "Sales Data");
        assert_eq!(dataset.source, DataSource::Finance);
        assert_eq!(dataset.owner_id, owner_id);
    }
    
    #[test]
    fn test_update_dataset_info() {
        let owner_id = Uuid::new_v4();
        let fields = vec![
            FieldDefinition {
                name: "revenue".to_string(),
                data_type: DataType::Float,
                is_nullable: false,
                description: Some("Monthly revenue".to_string()),
            }
        ];
        
        let mut dataset = Dataset::new(
            "Sales Data".to_string(),
            DataSource::Finance,
            fields,
            owner_id,
            Some("Monthly sales data".to_string()),
        ).unwrap();
        
        dataset.update_info(
            Some("Updated Sales Data".to_string()),
            Some("Updated description".to_string()),
        ).unwrap();
        
        assert_eq!(dataset.name, "Updated Sales Data");
        assert_eq!(dataset.description, Some("Updated description".to_string()));
    }
    
    #[test]
    fn test_add_and_remove_field() {
        let owner_id = Uuid::new_v4();
        let fields = vec![];
        
        let mut dataset = Dataset::new(
            "Test Data".to_string(),
            DataSource::Custom("test".to_string()),
            fields,
            owner_id,
            None,
        ).unwrap();
        
        let new_field = FieldDefinition {
            name: "test_field".to_string(),
            data_type: DataType::String,
            is_nullable: true,
            description: None,
        };
        
        dataset.add_field(new_field.clone());
        assert_eq!(dataset.fields.len(), 1);
        assert_eq!(dataset.fields[0], new_field);
        
        dataset.remove_field("test_field");
        assert_eq!(dataset.fields.len(), 0);
    }
    
    #[test]
    fn test_dataset_validation() {
        let owner_id = Uuid::new_v4();
        let fields = vec![
            FieldDefinition {
                name: "revenue".to_string(),
                data_type: DataType::Float,
                is_nullable: false,
                description: Some("Monthly revenue".to_string()),
            }
        ];
        
        let dataset = Dataset::new(
            "Sales Data".to_string(),
            DataSource::Finance,
            fields,
            owner_id,
            Some("Monthly sales data".to_string()),
        ).unwrap();
        
        assert!(dataset.validate().is_ok());
    }
}