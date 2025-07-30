//! Dashboard domain entities for the BI & Analytics module

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use thiserror::Error;
use super::report::Report;

/// Error types for dashboard operations
#[derive(Error, Debug)]
pub enum DashboardError {
    #[error("Invalid dashboard data: {0}")]
    InvalidData(String),
    
    #[error("Dashboard not found: {0}")]
    NotFound(String),
    
    #[error("Report error: {0}")]
    ReportError(String),
}

/// Grid position for dashboard reports
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GridPosition {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// Dashboard report association
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DashboardReport {
    pub id: Uuid,
    pub dashboard_id: Uuid,
    pub report_id: Uuid,
    pub position: GridPosition,
}

/// Main dashboard entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dashboard {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub layout: HashMap<String, serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub owner_id: Uuid,
}

impl Dashboard {
    /// Create a new dashboard
    pub fn new(
        name: String,
        owner_id: Uuid,
        description: Option<String>,
        layout: HashMap<String, serde_json::Value>,
    ) -> Result<Self, DashboardError> {
        if name.is_empty() {
            return Err(DashboardError::InvalidData("Dashboard name cannot be empty".to_string()));
        }
        
        let now = Utc::now();
        
        Ok(Self {
            id: Uuid::new_v4(),
            name,
            description,
            layout,
            created_at: now,
            updated_at: now,
            owner_id,
        })
    }
    
    /// Update dashboard information
    pub fn update_info(
        &mut self,
        name: Option<String>,
        description: Option<String>,
        layout: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<(), DashboardError> {
        if let Some(name) = name {
            if name.is_empty() {
                return Err(DashboardError::InvalidData("Dashboard name cannot be empty".to_string()));
            }
            self.name = name;
        }
        
        if let Some(description) = description {
            self.description = Some(description);
        }
        
        if let Some(layout) = layout {
            self.layout = layout;
        }
        
        self.updated_at = Utc::now();
        Ok(())
    }
    
    /// Validate the dashboard
    pub fn validate(&self) -> Result<(), DashboardError> {
        if self.name.is_empty() {
            return Err(DashboardError::InvalidData("Dashboard name cannot be empty".to_string()));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_create_dashboard() {
        let owner_id = Uuid::new_v4();
        let layout = HashMap::new();
        
        let dashboard = Dashboard::new(
            "Sales Dashboard".to_string(),
            owner_id,
            Some("Monthly sales dashboard".to_string()),
            layout,
        ).unwrap();
        
        assert_eq!(dashboard.name, "Sales Dashboard");
        assert_eq!(dashboard.owner_id, owner_id);
    }
    
    #[test]
    fn test_update_dashboard_info() {
        let owner_id = Uuid::new_v4();
        let layout = HashMap::new();
        
        let mut dashboard = Dashboard::new(
            "Sales Dashboard".to_string(),
            owner_id,
            Some("Monthly sales dashboard".to_string()),
            layout,
        ).unwrap();
        
        let mut new_layout = HashMap::new();
        new_layout.insert("grid_size".to_string(), serde_json::Value::String("12x12".to_string()));
        
        dashboard.update_info(
            Some("Updated Sales Dashboard".to_string()),
            Some("Updated description".to_string()),
            Some(new_layout.clone()),
        ).unwrap();
        
        assert_eq!(dashboard.name, "Updated Sales Dashboard");
        assert_eq!(dashboard.description, Some("Updated description".to_string()));
        assert_eq!(dashboard.layout, new_layout);
    }
    
    #[test]
    fn test_dashboard_validation() {
        let owner_id = Uuid::new_v4();
        let layout = HashMap::new();
        
        let dashboard = Dashboard::new(
            "Sales Dashboard".to_string(),
            owner_id,
            Some("Monthly sales dashboard".to_string()),
            layout,
        ).unwrap();
        
        assert!(dashboard.validate().is_ok());
    }
}