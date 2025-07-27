//! Pipeline domain entities for the CRM module
//!
//! This module contains the core business entities for managing sales pipelines,
//! including pipeline stages and progression rules.

use serde::{Deserialize, Serialize};
use crate::domain::primitives::{PipelineId, StageId, UserId, CrmPrimitiveError};
use thiserror::Error;
use std::collections::HashMap;

/// Error types for pipeline operations
#[derive(Error, Debug)]
pub enum PipelineError {
    #[error("Invalid pipeline data: {0}")]
    InvalidData(String),
    
    #[error("Invalid stage data: {0}")]
    InvalidStage(String),
    
    #[error("Stage not found: {0}")]
    StageNotFound(StageId),
    
    #[error("Invalid probability: {0}")]
    InvalidProbability(u8),
    
    #[error("Primitive error: {0}")]
    PrimitiveError(#[from] CrmPrimitiveError),
}

/// A stage in a sales pipeline
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PipelineStage {
    /// Unique identifier for the stage
    pub id: StageId,
    
    /// Name of the stage
    pub name: String,
    
    /// Probability of closing a deal at this stage (0-100%)
    pub probability: u8,
    
    /// Estimated value of deals at this stage
    pub estimated_value: Option<crate::domain::primitives::MonetaryAmount>,
    
    /// Position in the pipeline (0-based index)
    pub position: u8,
}

impl PipelineStage {
    /// Create a new pipeline stage
    pub fn new(
        name: String,
        probability: u8,
        estimated_value: Option<crate::domain::primitives::MonetaryAmount>,
        position: u8,
    ) -> Result<Self, PipelineError> {
        // Validate probability
        if probability > 100 {
            return Err(PipelineError::InvalidProbability(probability));
        }
        
        // Validate name
        if name.is_empty() {
            return Err(PipelineError::InvalidStage("Stage name cannot be empty".to_string()));
        }
        
        Ok(Self {
            id: StageId::new(),
            name,
            probability,
            estimated_value,
            position,
        })
    }
    
    /// Update stage information
    pub fn update_info(
        &mut self,
        name: Option<String>,
        probability: Option<u8>,
        estimated_value: Option<crate::domain::primitives::MonetaryAmount>,
        position: Option<u8>,
    ) -> Result<(), PipelineError> {
        if let Some(name) = name {
            if name.is_empty() {
                return Err(PipelineError::InvalidStage("Stage name cannot be empty".to_string()));
            }
            self.name = name;
        }
        
        if let Some(probability) = probability {
            if probability > 100 {
                return Err(PipelineError::InvalidProbability(probability));
            }
            self.probability = probability;
        }
        
        if let Some(estimated_value) = estimated_value {
            self.estimated_value = Some(estimated_value);
        }
        
        if let Some(position) = position {
            self.position = position;
        }
        
        Ok(())
    }
    
    /// Validate the stage
    pub fn validate(&self) -> Result<(), PipelineError> {
        if self.name.is_empty() {
            return Err(PipelineError::InvalidStage("Stage name cannot be empty".to_string()));
        }
        
        if self.probability > 100 {
            return Err(PipelineError::InvalidProbability(self.probability));
        }
        
        Ok(())
    }
}

/// Main pipeline entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pipeline {
    /// Unique identifier for the pipeline
    pub id: PipelineId,
    
    /// Name of the pipeline
    pub name: String,
    
    /// Stages in the pipeline
    pub stages: Vec<PipelineStage>,
    
    /// ID of the user who owns this pipeline
    pub owner_id: UserId,
    
    /// Indicates if this pipeline is shared with other users
    pub is_shared: bool,
    
    /// Users this pipeline is shared with
    pub shared_with: Vec<UserId>,
    
    /// Custom fields for the pipeline
    pub custom_fields: HashMap<String, String>,
}

impl Pipeline {
    /// Create a new pipeline
    pub fn new(
        name: String,
        owner_id: UserId,
        is_shared: bool,
        shared_with: Vec<UserId>,
    ) -> Result<Self, PipelineError> {
        // Validate name
        if name.is_empty() {
            return Err(PipelineError::InvalidData("Pipeline name cannot be empty".to_string()));
        }
        
        Ok(Self {
            id: PipelineId::new(),
            name,
            stages: Vec::new(),
            owner_id,
            is_shared,
            shared_with,
            custom_fields: HashMap::new(),
        })
    }
    
    /// Add a stage to the pipeline
    pub fn add_stage(&mut self, mut stage: PipelineStage) -> Result<(), PipelineError> {
        // Validate stage
        stage.validate()?;
        
        // Set position if not already set
        if stage.position == 0 && !self.stages.is_empty() {
            stage.position = (self.stages.len()) as u8;
        }
        
        self.stages.push(stage);
        
        // Sort stages by position
        self.stages.sort_by_key(|stage| stage.position);
        
        Ok(())
    }
    
    /// Remove a stage from the pipeline
    pub fn remove_stage(&mut self, stage_id: &StageId) -> Result<(), PipelineError> {
        let index = self.stages.iter().position(|stage| &stage.id == stage_id)
            .ok_or_else(|| PipelineError::StageNotFound(stage_id.clone()))?;
        
        self.stages.remove(index);
        
        // Update positions of remaining stages
        for (i, stage) in self.stages.iter_mut().enumerate() {
            stage.position = i as u8;
        }
        
        Ok(())
    }
    
    /// Update a stage in the pipeline
    pub fn update_stage(&mut self, stage_id: &StageId, stage: PipelineStage) -> Result<(), PipelineError> {
        // Validate stage
        stage.validate()?;
        
        let index = self.stages.iter().position(|s| &s.id == stage_id)
            .ok_or_else(|| PipelineError::StageNotFound(stage_id.clone()))?;
        
        self.stages[index] = stage;
        
        // Sort stages by position
        self.stages.sort_by_key(|stage| stage.position);
        
        Ok(())
    }
    
    /// Get a stage by ID
    pub fn get_stage(&self, stage_id: &StageId) -> Option<&PipelineStage> {
        self.stages.iter().find(|stage| &stage.id == stage_id)
    }
    
    /// Get a stage by position
    pub fn get_stage_by_position(&self, position: u8) -> Option<&PipelineStage> {
        self.stages.iter().find(|stage| stage.position == position)
    }
    
    /// Get the next stage in the pipeline
    pub fn get_next_stage(&self, current_stage_id: &StageId) -> Option<&PipelineStage> {
        let current_stage = self.get_stage(current_stage_id)?;
        let next_position = current_stage.position + 1;
        self.get_stage_by_position(next_position)
    }
    
    /// Get the previous stage in the pipeline
    pub fn get_previous_stage(&self, current_stage_id: &StageId) -> Option<&PipelineStage> {
        let current_stage = self.get_stage(current_stage_id)?;
        if current_stage.position > 0 {
            let prev_position = current_stage.position - 1;
            self.get_stage_by_position(prev_position)
        } else {
            None
        }
    }
    
    /// Update pipeline information
    pub fn update_info(
        &mut self,
        name: Option<String>,
        is_shared: Option<bool>,
        shared_with: Option<Vec<UserId>>,
    ) -> Result<(), PipelineError> {
        if let Some(name) = name {
            if name.is_empty() {
                return Err(PipelineError::InvalidData("Pipeline name cannot be empty".to_string()));
            }
            self.name = name;
        }
        
        if let Some(is_shared) = is_shared {
            self.is_shared = is_shared;
        }
        
        if let Some(shared_with) = shared_with {
            self.shared_with = shared_with;
        }
        
        Ok(())
    }
    
    /// Add custom fields to the pipeline
    pub fn add_custom_fields(&mut self, fields: HashMap<String, String>) {
        for (key, value) in fields {
            self.custom_fields.insert(key, value);
        }
    }
    
    /// Remove custom fields from the pipeline
    pub fn remove_custom_fields(&mut self, keys: Vec<String>) {
        for key in keys {
            self.custom_fields.remove(&key);
        }
    }
    
    /// Validate the pipeline
    pub fn validate(&self) -> Result<(), PipelineError> {
        if self.name.is_empty() {
            return Err(PipelineError::InvalidData("Pipeline name cannot be empty".to_string()));
        }
        
        // Validate all stages
        for stage in &self.stages {
            stage.validate()?;
        }
        
        // Check for duplicate positions
        let mut positions = std::collections::HashSet::new();
        for stage in &self.stages {
            if !positions.insert(stage.position) {
                return Err(PipelineError::InvalidStage(
                    format!("Duplicate position {} in pipeline stages", stage.position)
                ));
            }
        }
        
        Ok(())
    }
    
    /// Calculate the weighted value of all deals in this pipeline
    pub fn calculate_weighted_value(&self) -> Option<crate::domain::primitives::MonetaryAmount> {
        // This would require access to deals, which is outside the domain scope
        // In a real implementation, this would be handled by an application service
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use cpc_core::finance::domain::primitives::{Currency, Money};
    use rust_decimal::Decimal;

    #[test]
    fn test_create_pipeline_stage() {
        let value = Money::new(Decimal::new(10000, 2), Currency::USD); // $100.00
        
        let stage = PipelineStage::new(
            "Prospecting".to_string(),
            20,
            Some(value.clone()),
            0,
        ).unwrap();
        
        assert_eq!(stage.name, "Prospecting");
        assert_eq!(stage.probability, 20);
        assert_eq!(stage.estimated_value, Some(value));
        assert_eq!(stage.position, 0);
    }

    #[test]
    fn test_invalid_probability() {
        let result = PipelineStage::new(
            "Invalid Stage".to_string(),
            150, // Invalid probability > 100
            None,
            0,
        );
        
        assert!(result.is_err());
        match result.unwrap_err() {
            PipelineError::InvalidProbability(prob) => assert_eq!(prob, 150),
            _ => panic!("Expected InvalidProbability error"),
        }
    }

    #[test]
    fn test_create_pipeline() {
        let owner_id = UserId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let shared_with = vec![UserId::from_str("111e8400-e29b-41d4-a716-446655440000").unwrap()];
        
        let pipeline = Pipeline::new(
            "Sales Pipeline".to_string(),
            owner_id.clone(),
            true,
            shared_with.clone(),
        ).unwrap();
        
        assert_eq!(pipeline.name, "Sales Pipeline");
        assert_eq!(pipeline.owner_id, owner_id);
        assert!(pipeline.is_shared);
        assert_eq!(pipeline.shared_with, shared_with);
    }

    #[test]
    fn test_add_and_remove_stages() {
        let mut pipeline = create_test_pipeline();
        
        let stage1 = PipelineStage::new(
            "Prospecting".to_string(),
            10,
            None,
            0,
        ).unwrap();
        
        let stage2 = PipelineStage::new(
            "Qualification".to_string(),
            30,
            None,
            1,
        ).unwrap();
        
        pipeline.add_stage(stage1).unwrap();
        pipeline.add_stage(stage2).unwrap();
        
        assert_eq!(pipeline.stages.len(), 2);
        
        let first_stage_id = pipeline.stages[0].id.clone();
        pipeline.remove_stage(&first_stage_id).unwrap();
        
        assert_eq!(pipeline.stages.len(), 1);
        assert_eq!(pipeline.stages[0].name, "Qualification");
        assert_eq!(pipeline.stages[0].position, 0); // Position should be updated
    }

    #[test]
    fn test_update_stage() {
        let mut pipeline = create_test_pipeline();
        
        let mut stage = PipelineStage::new(
            "Initial Stage".to_string(),
            20,
            None,
            0,
        ).unwrap();
        
        let stage_id = stage.id.clone();
        pipeline.add_stage(stage).unwrap();
        
        let updated_stage = PipelineStage::new(
            "Updated Stage".to_string(),
            40,
            None,
            0,
        ).unwrap();
        
        pipeline.update_stage(&stage_id, updated_stage).unwrap();
        
        let retrieved_stage = pipeline.get_stage(&stage_id).unwrap();
        assert_eq!(retrieved_stage.name, "Updated Stage");
        assert_eq!(retrieved_stage.probability, 40);
    }

    #[test]
    fn test_stage_navigation() {
        let mut pipeline = create_test_pipeline();
        
        let stage1 = PipelineStage::new(
            "Stage 1".to_string(),
            20,
            None,
            0,
        ).unwrap();
        
        let stage1_id = stage1.id.clone();
        
        let stage2 = PipelineStage::new(
            "Stage 2".to_string(),
            50,
            None,
            1,
        ).unwrap();
        
        let stage2_id = stage2.id.clone();
        
        let stage3 = PipelineStage::new(
            "Stage 3".to_string(),
            80,
            None,
            2,
        ).unwrap();
        
        pipeline.add_stage(stage1).unwrap();
        pipeline.add_stage(stage2).unwrap();
        pipeline.add_stage(stage3).unwrap();
        
        let next_stage = pipeline.get_next_stage(&stage1_id).unwrap();
        assert_eq!(next_stage.id, stage2_id);
        
        let prev_stage = pipeline.get_previous_stage(&stage2_id).unwrap();
        assert_eq!(prev_stage.id, stage1_id);
        
        let no_prev_stage = pipeline.get_previous_stage(&stage1_id);
        assert!(no_prev_stage.is_none());
    }

    #[test]
    fn test_pipeline_validation() {
        let pipeline = create_test_pipeline();
        assert!(pipeline.validate().is_ok());
        
        // Test invalid pipeline with empty name
        let invalid_pipeline = Pipeline {
            name: "".to_string(),
            ..create_test_pipeline()
        };
        assert!(invalid_pipeline.validate().is_err());
    }

    #[test]
    fn test_stage_validation() {
        let stage = create_test_stage();
        assert!(stage.validate().is_ok());
        
        // Test invalid stage with empty name
        let invalid_stage = PipelineStage {
            name: "".to_string(),
            ..create_test_stage()
        };
        assert!(invalid_stage.validate().is_err());
    }

    fn create_test_pipeline() -> Pipeline {
        let owner_id = UserId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        
        Pipeline::new(
            "Test Pipeline".to_string(),
            owner_id,
            false,
            Vec::new(),
        ).unwrap()
    }
    
    fn create_test_stage() -> PipelineStage {
        PipelineStage::new(
            "Test Stage".to_string(),
            50,
            None,
            0,
        ).unwrap()
    }
}