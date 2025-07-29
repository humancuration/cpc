//! Deal domain entities for the CRM module
//!
//! This module contains the core business entities for managing deals,
//! including deal information, notes, and pipeline associations.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::domain::primitives::{DealId, ContactId, PipelineId, StageId, UserId, CrmPrimitiveError};
use thiserror::Error;

/// Error types for deal operations
#[derive(Error, Debug)]
pub enum DealError {
    #[error("Invalid deal data: {0}")]
    InvalidData(String),
    
    #[error("Invalid date: {0}")]
    InvalidDate(String),
    
    #[error("Primitive error: {0}")]
    PrimitiveError(#[from] CrmPrimitiveError),
}

/// A note associated with a deal
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DealNote {
    /// Content of the note
    pub content: String,
    
    /// ID of the user who created the note
    pub created_by: UserId,
    
    /// Timestamp when the note was created
    pub created_at: DateTime<Utc>,
    
    /// Indicates if this note is visible to the contact (for platform-native deals)
    pub is_shared_with_contact: bool,
}

impl DealNote {
    /// Create a new deal note
    pub fn new(
        content: String,
        created_by: UserId,
        is_shared_with_contact: bool,
    ) -> Result<Self, DealError> {
        if content.is_empty() {
            return Err(DealError::InvalidData("Deal note content cannot be empty".to_string()));
        }
        
        Ok(Self {
            content,
            created_by,
            created_at: Utc::now(),
            is_shared_with_contact,
        })
    }
    
    /// Update note content
    pub fn update_content(&mut self, content: String) -> Result<(), DealError> {
        if content.is_empty() {
            return Err(DealError::InvalidData("Deal note content cannot be empty".to_string()));
        }
        
        self.content = content;
        Ok(())
    }
    
    /// Validate the note
    pub fn validate(&self) -> Result<(), DealError> {
        if self.content.is_empty() {
            return Err(DealError::InvalidData("Deal note content cannot be empty".to_string()));
        }
        
        Ok(())
    }
}

/// Main deal entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Deal {
    /// Unique identifier for the deal
    pub id: DealId,
    
    /// ID of the contact associated with this deal
    pub contact_id: ContactId,
    
    /// ID of the pipeline this deal belongs to
    pub pipeline_id: PipelineId,
    
    /// ID of the current stage in the pipeline
    pub current_stage: StageId,
    
    /// Title of the deal
    pub title: String,
    
    /// Value of the deal
    pub value: crate::domain::primitives::MonetaryAmount,
    
    /// Expected close date
    pub expected_close_date: Option<DateTime<Utc>>,
    
    /// Notes associated with the deal
    pub notes: Vec<DealNote>,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    
    /// Indicates if the customer is platform-native
    pub is_platform_deal: bool,
    
    /// ID of the user who owns this deal
    pub owner_id: UserId,
    
    /// Custom fields for the deal
    pub custom_fields: std::collections::HashMap<String, String>,
}

impl Deal {
    /// Create a new deal
    pub fn new(
        contact_id: ContactId,
        pipeline_id: PipelineId,
        current_stage: StageId,
        title: String,
        value: crate::domain::primitives::MonetaryAmount,
        expected_close_date: Option<DateTime<Utc>>,
        is_platform_deal: bool,
        owner_id: UserId,
    ) -> Result<Self, DealError> {
        // Validate title
        if title.is_empty() {
            return Err(DealError::InvalidData("Deal title cannot be empty".to_string()));
        }
        
        // Validate expected close date if provided
        if let Some(date) = expected_close_date {
            if date < Utc::now() {
                return Err(DealError::InvalidDate("Expected close date cannot be in the past".to_string()));
            }
        }
        
        let now = Utc::now();
        
        Ok(Self {
            id: DealId::new(),
            contact_id,
            pipeline_id,
            current_stage,
            title,
            value,
            expected_close_date,
            notes: Vec::new(),
            created_at: now,
            updated_at: now,
            is_platform_deal,
            owner_id,
            custom_fields: std::collections::HashMap::new(),
        })
    }
    
    /// Update deal information
    pub fn update_info(
        &mut self,
        title: Option<String>,
        value: Option<crate::domain::primitives::MonetaryAmount>,
        expected_close_date: Option<DateTime<Utc>>,
        current_stage: Option<StageId>,
    ) -> Result<(), DealError> {
        if let Some(title) = title {
            if title.is_empty() {
                return Err(DealError::InvalidData("Deal title cannot be empty".to_string()));
            }
            self.title = title;
        }
        
        if let Some(value) = value {
            self.value = value;
        }
        
        if let Some(expected_close_date) = expected_close_date {
            if expected_close_date < Utc::now() {
                return Err(DealError::InvalidDate("Expected close date cannot be in the past".to_string()));
            }
            self.expected_close_date = Some(expected_close_date);
        }
        
        if let Some(current_stage) = current_stage {
            self.current_stage = current_stage;
        }
        
        self.updated_at = Utc::now();
        Ok(())
    }
    
    /// Add a note to the deal
    pub fn add_note(&mut self, note: DealNote) -> Result<(), DealError> {
        note.validate()?;
        self.notes.push(note);
        self.updated_at = Utc::now();
        Ok(())
    }
    
    /// Remove a note from the deal
    pub fn remove_note(&mut self, index: usize) -> Result<(), DealError> {
        if index >= self.notes.len() {
            return Err(DealError::InvalidData("Note index out of bounds".to_string()));
        }
        
        self.notes.remove(index);
        self.updated_at = Utc::now();
        Ok(())
    }
    
    /// Update a note in the deal
    pub fn update_note(&mut self, index: usize, content: String) -> Result<(), DealError> {
        if index >= self.notes.len() {
            return Err(DealError::InvalidData("Note index out of bounds".to_string()));
        }
        
        self.notes[index].update_content(content)?;
        self.updated_at = Utc::now();
        Ok(())
    }
    
    /// Add custom fields to the deal
    pub fn add_custom_fields(&mut self, fields: std::collections::HashMap<String, String>) {
        for (key, value) in fields {
            self.custom_fields.insert(key, value);
        }
        self.updated_at = Utc::now();
    }
    
    /// Remove custom fields from the deal
    pub fn remove_custom_fields(&mut self, keys: Vec<String>) {
        for key in keys {
            self.custom_fields.remove(&key);
        }
        self.updated_at = Utc::now();
    }
    
    /// Move the deal to a new stage
    pub fn move_to_stage(&mut self, new_stage_id: StageId) {
        self.current_stage = new_stage_id;
        self.updated_at = Utc::now();
    }
    
    /// Calculate the weighted value of the deal based on the current stage probability
    pub fn calculate_weighted_value(&self, stage_probability: u8) -> crate::domain::primitives::MonetaryAmount {
        use rust_decimal::Decimal;
        
        // Convert probability percentage to decimal (e.g., 50% -> 0.5)
        let probability_decimal = Decimal::from(stage_probability) / Decimal::from(100u8);
        
        // Multiply the deal value by the probability
        self.value.multiply(probability_decimal)
    }
    
    /// Validate the deal
    pub fn validate(&self) -> Result<(), DealError> {
        if self.title.is_empty() {
            return Err(DealError::InvalidData("Deal title cannot be empty".to_string()));
        }
        
        // Validate expected close date if provided
        if let Some(date) = self.expected_close_date {
            if date < self.created_at {
                return Err(DealError::InvalidDate("Expected close date cannot be before creation date".to_string()));
            }
        }
        
        // Validate all notes
        for note in &self.notes {
            note.validate()?;
        }
        
        Ok(())
    }
    
    /// Check if the deal is overdue (past expected close date)
    pub fn is_overdue(&self) -> bool {
        if let Some(expected_close_date) = self.expected_close_date {
            Utc::now() > expected_close_date
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use cpc_core::finance::domain::primitives::{Currency, Money};
    use rust_decimal::Decimal;

    #[test]
    fn test_create_deal() {
        let contact_id = ContactId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let pipeline_id = PipelineId::from_str("111e8400-e29b-41d4-a716-446655440000").unwrap();
        let stage_id = StageId::from_str("222e8400-e29b-41d4-a716-446655440000").unwrap();
        let owner_id = UserId::from_str("333e8400-e29b-41d4-a716-446655440000").unwrap();
        let value = Money::new(Decimal::new(50000, 2), Currency::USD); // $500.00
        let expected_close_date = Some(Utc::now() + chrono::Duration::days(30));
        
        let deal = Deal::new(
            contact_id.clone(),
            pipeline_id.clone(),
            stage_id.clone(),
            "New Website Project".to_string(),
            value.clone(),
            expected_close_date,
            true,
            owner_id.clone(),
        ).unwrap();
        
        assert_eq!(deal.contact_id, contact_id);
        assert_eq!(deal.pipeline_id, pipeline_id);
        assert_eq!(deal.current_stage, stage_id);
        assert_eq!(deal.title, "New Website Project");
        assert_eq!(deal.value, value);
        assert_eq!(deal.is_platform_deal, true);
        assert_eq!(deal.owner_id, owner_id);
    }

    #[test]
    fn test_create_deal_note() {
        let user_id = UserId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        
        let note = DealNote::new(
            "Discussed project requirements with client".to_string(),
            user_id.clone(),
            true,
        ).unwrap();
        
        assert_eq!(note.content, "Discussed project requirements with client");
        assert_eq!(note.created_by, user_id);
        assert!(note.is_shared_with_contact);
    }

    #[test]
    fn test_add_and_remove_notes() {
        let mut deal = create_test_deal();
        let user_id = UserId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        
        let note1 = DealNote::new(
            "First note".to_string(),
            user_id.clone(),
            false,
        ).unwrap();
        
        let note2 = DealNote::new(
            "Second note".to_string(),
            user_id.clone(),
            true,
        ).unwrap();
        
        deal.add_note(note1).unwrap();
        deal.add_note(note2).unwrap();
        
        assert_eq!(deal.notes.len(), 2);
        
        deal.remove_note(0).unwrap();
        assert_eq!(deal.notes.len(), 1);
        assert_eq!(deal.notes[0].content, "Second note");
    }

    #[test]
    fn test_update_deal_info() {
        let mut deal = create_test_deal();
        let new_value = Money::new(Decimal::new(75000, 2), Currency::USD); // $750.00
        let new_date = Some(Utc::now() + chrono::Duration::days(45));
        
        let original_updated = deal.updated_at;
        
        deal.update_info(
            Some("Updated Deal Title".to_string()),
            Some(new_value.clone()),
            new_date,
            None,
        ).unwrap();
        
        assert_eq!(deal.title, "Updated Deal Title");
        assert_eq!(deal.value, new_value);
        assert_eq!(deal.expected_close_date, new_date);
        assert!(deal.updated_at > original_updated);
    }

    #[test]
    fn test_deal_validation() {
        let deal = create_test_deal();
        assert!(deal.validate().is_ok());
        
        // Test invalid deal with empty title
        let invalid_deal = Deal {
            title: "".to_string(),
            ..create_test_deal()
        };
        assert!(invalid_deal.validate().is_err());
    }

    #[test]
    fn test_note_validation() {
        let user_id = UserId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        
        let note = DealNote::new(
            "Valid note".to_string(),
            user_id,
            false,
        ).unwrap();
        assert!(note.validate().is_ok());
        
        // Test invalid note with empty content
        let invalid_note = DealNote {
            content: "".to_string(),
            ..note
        };
        assert!(invalid_note.validate().is_err());
    }

    #[test]
    fn test_calculate_weighted_value() {
        let deal = create_test_deal();
        let probability = 75u8; // 75% probability
        
        let weighted_value = deal.calculate_weighted_value(probability);
        
        // Expected: $500.00 * 0.75 = $375.00
        let expected_value = Money::new(Decimal::new(37500, 2), Currency::USD);
        assert_eq!(weighted_value, expected_value);
    }

    #[test]
    fn test_is_overdue() {
        let past_date = Some(Utc::now() - chrono::Duration::days(1));
        let mut deal = create_test_deal();
        deal.expected_close_date = past_date;
        
        assert!(deal.is_overdue());
        
        let future_date = Some(Utc::now() + chrono::Duration::days(1));
        deal.expected_close_date = future_date;
        
        assert!(!deal.is_overdue());
    }

    fn create_test_deal() -> Deal {
        let contact_id = ContactId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let pipeline_id = PipelineId::from_str("111e8400-e29b-41d4-a716-446655440000").unwrap();
        let stage_id = StageId::from_str("222e8400-e29b-41d4-a716-446655440000").unwrap();
        let owner_id = UserId::from_str("333e8400-e29b-41d4-a716-446655440000").unwrap();
        let value = Money::new(Decimal::new(50000, 2), Currency::USD); // $500.00
        let expected_close_date = Some(Utc::now() + chrono::Duration::days(30));
        
        Deal::new(
            contact_id,
            pipeline_id,
            stage_id,
            "Test Deal".to_string(),
            value,
            expected_close_date,
            false,
            owner_id,
        ).unwrap()
    }
}