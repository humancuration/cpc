//! Data models for the Cause Management system
//!
//! This module defines the core data structures used for managing causes
//! within the CPC platform.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::fmt;

/// Cause structure for donations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cause {
    /// Unique identifier for the cause
    pub id: Uuid,
    
    /// Name of the cause
    pub name: String,
    
    /// Description of the cause
    pub description: String,
    
    /// URL to the cause's image
    pub image_url: Option<String>,
    
    /// Total donations received
    pub total_donations: Decimal,
    
    /// Timestamp when the cause was created
    pub created_at: DateTime<Utc>,
    
    /// Timestamp when the cause was last updated
    pub updated_at: DateTime<Utc>,
}

impl Cause {
    /// Create a new cause
    pub fn new(
        name: String,
        description: String,
        image_url: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            image_url,
            total_donations: Decimal::new(0, 0),
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Update the cause information
    pub fn update(
        &mut self,
        name: Option<String>,
        description: Option<String>,
        image_url: Option<String>,
    ) {
        if let Some(name) = name {
            self.name = name;
        }
        if let Some(description) = description {
            self.description = description;
        }
        if let Some(image_url) = image_url {
            self.image_url = Some(image_url);
        }
        self.updated_at = Utc::now();
    }
    
    /// Add donation amount to the cause
    pub fn add_donation(&mut self, amount: Decimal) {
        self.total_donations += amount;
        self.updated_at = Utc::now();
    }
}

/// Cause creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCauseRequest {
    /// Name of the cause
    pub name: String,
    
    /// Description of the cause
    pub description: String,
    
    /// URL to the cause's image
    pub image_url: Option<String>,
}

/// Cause update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCauseRequest {
    /// Unique identifier for the cause
    pub cause_id: Uuid,
    
    /// Name of the cause (optional)
    pub name: Option<String>,
    
    /// Description of the cause (optional)
    pub description: Option<String>,
    
    /// URL to the cause's image (optional)
    pub image_url: Option<String>,
}

/// Cause listing request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCausesRequest {
    /// Maximum number of causes to return
    pub limit: Option<i32>,
    
    /// Offset for pagination
    pub offset: Option<i32>,
}

/// Cause listing response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCausesResponse {
    /// List of causes
    pub causes: Vec<Cause>,
    
    /// Total number of causes
    pub total_count: i32,
}

/// Cause management error types
#[derive(Debug, thiserror::Error)]
pub enum CauseError {
    /// Cause not found
    #[error("Cause not found: {0}")]
    CauseNotFound(Uuid),
    
    /// Database error
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    /// General error
    #[error("Cause management error: {0}")]
    General(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    
    #[test]
    fn test_cause_creation() {
        let name = "Test Cause".to_string();
        let description = "Test Description".to_string();
        let image_url = Some("http://example.com/image.jpg".to_string());
        
        let cause = Cause::new(name.clone(), description.clone(), image_url.clone());
        
        assert!(!cause.id.is_nil());
        assert_eq!(cause.name, name);
        assert_eq!(cause.description, description);
        assert_eq!(cause.image_url, image_url);
        assert_eq!(cause.total_donations, dec!(0.0));
    }
    
    #[test]
    fn test_cause_update() {
        let mut cause = Cause::new(
            "Test Cause".to_string(),
            "Test Description".to_string(),
            Some("http://example.com/image.jpg".to_string()),
        );
        
        let updated_name = "Updated Cause".to_string();
        let updated_description = "Updated Description".to_string();
        let updated_image_url = "http://example.com/updated_image.jpg".to_string();
        
        cause.update(
            Some(updated_name.clone()),
            Some(updated_description.clone()),
            Some(updated_image_url.clone()),
        );
        
        assert_eq!(cause.name, updated_name);
        assert_eq!(cause.description, updated_description);
        assert_eq!(cause.image_url, Some(updated_image_url));
    }
    
    #[test]
    fn test_add_donation() {
        let mut cause = Cause::new(
            "Test Cause".to_string(),
            "Test Description".to_string(),
            None,
        );
        
        let donation_amount = dec!(100.50);
        cause.add_donation(donation_amount);
        
        assert_eq!(cause.total_donations, donation_amount);
    }
}