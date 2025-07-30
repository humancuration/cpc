//! Pipeline service for the CRM module
//!
//! This module contains the application service for managing sales pipelines,
//! including stage progression logic and deal pipeline association.

use crate::domain::pipeline::{Pipeline, PipelineStage, PipelineError};
use crate::domain::primitives::{PipelineId, StageId, UserId};
use crate::domain::deal::Deal;
use thiserror::Error;
use std::collections::HashMap;

/// Error types for pipeline service operations
#[derive(Error, Debug)]
pub enum PipelineServiceError {
    #[error("Pipeline error: {0}")]
    PipelineError(#[from] PipelineError),
    
    #[error("Pipeline not found: {0}")]
    PipelineNotFound(PipelineId),
    
    #[error("Stage not found: {0}")]
    StageNotFound(StageId),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

/// Service for managing pipelines
pub struct PipelineService;

impl PipelineService {
    /// Create a new pipeline
    pub fn create_pipeline(
        name: String,
        owner_id: UserId,
        is_shared: bool,
        shared_with: Vec<UserId>,
    ) -> Result<Pipeline, PipelineServiceError> {
        let pipeline = Pipeline::new(name, owner_id, is_shared, shared_with)?;
        Ok(pipeline)
    }
    
    /// Add a stage to a pipeline
    pub fn add_stage_to_pipeline(
        pipeline: &mut Pipeline,
        stage: PipelineStage,
    ) -> Result<(), PipelineServiceError> {
        pipeline.add_stage(stage)?;
        Ok(())
    }
    
    /// Remove a stage from a pipeline
    pub fn remove_stage_from_pipeline(
        pipeline: &mut Pipeline,
        stage_id: &StageId,
    ) -> Result<(), PipelineServiceError> {
        pipeline.remove_stage(stage_id)?;
        Ok(())
    }
    
    /// Update a stage in a pipeline
    pub fn update_stage_in_pipeline(
        pipeline: &mut Pipeline,
        stage_id: &StageId,
        stage: PipelineStage,
    ) -> Result<(), PipelineServiceError> {
        pipeline.update_stage(stage_id, stage)?;
        Ok(())
    }
    
    /// Get a stage from a pipeline
    pub fn get_stage_from_pipeline(
        pipeline: &Pipeline,
        stage_id: &StageId,
    ) -> Option<&PipelineStage> {
        pipeline.get_stage(stage_id)
    }
    
    /// Get the next stage in a pipeline
    pub fn get_next_stage(
        pipeline: &Pipeline,
        current_stage_id: &StageId,
    ) -> Option<&PipelineStage> {
        pipeline.get_next_stage(current_stage_id)
    }
    
    /// Get the previous stage in a pipeline
    pub fn get_previous_stage(
        pipeline: &Pipeline,
        current_stage_id: &StageId,
    ) -> Option<&PipelineStage> {
        pipeline.get_previous_stage(current_stage_id)
    }
    
    /// Update pipeline information
    pub fn update_pipeline_info(
        pipeline: &mut Pipeline,
        name: Option<String>,
        is_shared: Option<bool>,
        shared_with: Option<Vec<UserId>>,
    ) -> Result<(), PipelineServiceError> {
        pipeline.update_info(name, is_shared, shared_with)?;
        Ok(())
    }
    
    /// Validate a pipeline
    pub fn validate_pipeline(pipeline: &Pipeline) -> Result<(), PipelineServiceError> {
        pipeline.validate()?;
        Ok(())
    }
    
    /// Add custom fields to a pipeline
    pub fn add_custom_fields_to_pipeline(
        pipeline: &mut Pipeline,
        fields: HashMap<String, String>,
    ) {
        pipeline.add_custom_fields(fields);
    }
    
    /// Remove custom fields from a pipeline
    pub fn remove_custom_fields_from_pipeline(
        pipeline: &mut Pipeline,
        keys: Vec<String>,
    ) {
        pipeline.remove_custom_fields(keys);
    }
    
    /// Move a deal to the next stage in the pipeline
    pub fn move_deal_to_next_stage(
        pipeline: &Pipeline,
        deal: &mut Deal,
    ) -> Result<bool, PipelineServiceError> {
        let current_stage_id = &deal.current_stage;
        
        // Get the next stage
        if let Some(next_stage) = Self::get_next_stage(pipeline, current_stage_id) {
            // Move the deal to the next stage
            deal.move_to_stage(next_stage.id.clone());
            Ok(true)
        } else {
            // Already at the final stage
            Ok(false)
        }
    }
    
    /// Move a deal to the previous stage in the pipeline
    pub fn move_deal_to_previous_stage(
        pipeline: &Pipeline,
        deal: &mut Deal,
    ) -> Result<bool, PipelineServiceError> {
        let current_stage_id = &deal.current_stage;
        
        // Get the previous stage
        if let Some(prev_stage) = Self::get_previous_stage(pipeline, current_stage_id) {
            // Move the deal to the previous stage
            deal.move_to_stage(prev_stage.id.clone());
            Ok(true)
        } else {
            // Already at the first stage
            Ok(false)
        }
    }
    
    /// Move a deal to a specific stage in the pipeline
    pub fn move_deal_to_stage(
        pipeline: &Pipeline,
        deal: &mut Deal,
        target_stage_id: &StageId,
    ) -> Result<(), PipelineServiceError> {
        // Verify that the target stage exists in the pipeline
        if pipeline.get_stage(target_stage_id).is_none() {
            return Err(PipelineServiceError::StageNotFound(target_stage_id.clone()));
        }
        
        // Move the deal to the target stage
        deal.move_to_stage(target_stage_id.clone());
        Ok(())
    }
    
    /// Calculate the conversion rate between stages
    pub fn calculate_stage_conversion_rate(
        pipeline: &Pipeline,
        deals: &[Deal],
        from_stage_id: &StageId,
        to_stage_id: &StageId,
    ) -> f64 {
        let from_stage_deals: Vec<&Deal> = deals.iter()
            .filter(|deal| &deal.current_stage == from_stage_id || 
                          deal.current_stage == *to_stage_id)
            .collect();
        
        if from_stage_deals.is_empty() {
            return 0.0;
        }
        
        let converted_deals: Vec<&Deal> = from_stage_deals.iter()
            .filter(|deal| &deal.current_stage == to_stage_id)
            .count();
        
        converted_deals as f64 / from_stage_deals.len() as f64
    }
    
    /// Get pipeline statistics
    pub fn get_pipeline_statistics(
        pipeline: &Pipeline,
        deals: &[Deal],
    ) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        
        // Count deals in each stage
        for stage in &pipeline.stages {
            let count = deals.iter()
                .filter(|deal| deal.pipeline_id == pipeline.id && 
                               deal.current_stage == stage.id)
                .count();
            
            stats.insert(stage.name.clone(), count);
        }
        
        stats
    }
    
    /// Calculate the estimated revenue for the pipeline
    pub fn calculate_estimated_revenue(
        pipeline: &Pipeline,
        deals: &[Deal],
    ) -> crate::domain::primitives::MonetaryAmount {
        use cpc_core::finance::domain::primitives::{Money, Currency};
        use rust_decimal::Decimal;
        
        let mut total = Money::zero(Currency::USD); // Default currency
        
        for deal in deals {
            if deal.pipeline_id == pipeline.id {
                // Add the deal value multiplied by the stage probability
                if let Some(stage) = pipeline.get_stage(&deal.current_stage) {
                    let weighted_value = deal.calculate_weighted_value(stage.probability);
                    total = total.add(&weighted_value).unwrap_or(total);
                } else {
                    // If stage not found, just add the deal value
                    total = total.add(&deal.value).unwrap_or(total);
                }
            }
        }
        
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use crate::domain::primitives::DealId;
    use cpc_core::finance::domain::primitives::{Currency, Money};
    use rust_decimal::Decimal;

    #[test]
    fn test_create_pipeline() {
        let owner_id = UserId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let shared_with = vec![UserId::from_str("111e8400-e29b-41d4-a716-446655440000").unwrap()];
        
        let pipeline = PipelineService::create_pipeline(
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
        
        PipelineService::add_stage_to_pipeline(&mut pipeline, stage1).unwrap();
        PipelineService::add_stage_to_pipeline(&mut pipeline, stage2).unwrap();
        
        assert_eq!(pipeline.stages.len(), 2);
        
        let first_stage_id = pipeline.stages[0].id.clone();
        PipelineService::remove_stage_from_pipeline(&mut pipeline, &first_stage_id).unwrap();
        
        assert_eq!(pipeline.stages.len(), 1);
        assert_eq!(pipeline.stages[0].name, "Qualification");
    }

    #[test]
    fn test_move_deal_through_pipeline() {
        let mut pipeline = create_test_pipeline();
        let mut deal = create_test_deal();
        
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
        
        PipelineService::add_stage_to_pipeline(&mut pipeline, stage1).unwrap();
        PipelineService::add_stage_to_pipeline(&mut pipeline, stage2).unwrap();
        
        // Set deal to stage 1
        deal.move_to_stage(stage1_id.clone());
        
        // Move to next stage
        let moved = PipelineService::move_deal_to_next_stage(&pipeline, &mut deal).unwrap();
        assert!(moved);
        assert_eq!(deal.current_stage, stage2_id);
        
        // Try to move beyond last stage
        let moved = PipelineService::move_deal_to_next_stage(&pipeline, &mut deal).unwrap();
        assert!(!moved);
        assert_eq!(deal.current_stage, stage2_id);
    }

    #[test]
    fn test_pipeline_statistics() {
        let mut pipeline = create_test_pipeline();
        let deals = create_test_deals();
        
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
        
        PipelineService::add_stage_to_pipeline(&mut pipeline, stage1).unwrap();
        PipelineService::add_stage_to_pipeline(&mut pipeline, stage2).unwrap();
        
        let stats = PipelineService::get_pipeline_statistics(&pipeline, &deals);
        assert_eq!(stats.get("Stage 1"), Some(&1));
        assert_eq!(stats.get("Stage 2"), Some(&1));
    }

    #[test]
    fn test_estimated_revenue() {
        let mut pipeline = create_test_pipeline();
        let deals = create_test_deals_with_values();
        
        let stage1 = PipelineStage::new(
            "Stage 1".to_string(),
            20, // 20% probability
            None,
            0,
        ).unwrap();
        
        let stage2 = PipelineStage::new(
            "Stage 2".to_string(),
            50, // 50% probability
            None,
            1,
        ).unwrap();
        
        PipelineService::add_stage_to_pipeline(&mut pipeline, stage1).unwrap();
        PipelineService::add_stage_to_pipeline(&mut pipeline, stage2).unwrap();
        
        let revenue = PipelineService::calculate_estimated_revenue(&pipeline, &deals);
        
        // Expected: ($1000 * 0.2) + ($2000 * 0.5) = $200 + $1000 = $1200
        let expected = Money::new(Decimal::new(120000, 2), Currency::USD);
        assert_eq!(revenue, expected);
    }

    fn create_test_pipeline() -> Pipeline {
        let owner_id = UserId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        
        PipelineService::create_pipeline(
            "Test Pipeline".to_string(),
            owner_id,
            false,
            Vec::new(),
        ).unwrap()
    }
    
    fn create_test_deal() -> Deal {
        let contact_id = crate::domain::primitives::ContactId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let pipeline_id = PipelineId::from_str("111e8400-e29b-41d4-a716-446655440000").unwrap();
        let stage_id = StageId::from_str("222e8400-e29b-41d4-a716-446655440000").unwrap();
        let owner_id = UserId::from_str("333e8400-e29b-41d4-a716-446655440000").unwrap();
        let value = Money::new(Decimal::new(50000, 2), Currency::USD); // $500.00
        
        Deal::new(
            contact_id,
            pipeline_id,
            stage_id,
            "Test Deal".to_string(),
            value,
            None,
            false,
            owner_id,
        ).unwrap()
    }
    
    fn create_test_deals() -> Vec<Deal> {
        let contact_id = crate::domain::primitives::ContactId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let pipeline_id = PipelineId::from_str("111e8400-e29b-41d4-a716-446655440000").unwrap();
        let owner_id = UserId::from_str("333e8400-e29b-41d4-a716-446655440000").unwrap();
        let value = Money::new(Decimal::new(50000, 2), Currency::USD); // $500.00
        
        let stage1_id = StageId::from_str("222e8400-e29b-41d4-a716-446655440000").unwrap();
        let stage2_id = StageId::from_str("444e8400-e29b-41d4-a716-446655440000").unwrap();
        
        vec![
            Deal::new(
                contact_id.clone(),
                pipeline_id.clone(),
                stage1_id,
                "Deal 1".to_string(),
                value.clone(),
                None,
                false,
                owner_id.clone(),
            ).unwrap(),
            Deal::new(
                contact_id.clone(),
                pipeline_id.clone(),
                stage2_id,
                "Deal 2".to_string(),
                value.clone(),
                None,
                false,
                owner_id.clone(),
            ).unwrap(),
        ]
    }
    
    fn create_test_deals_with_values() -> Vec<Deal> {
        let contact_id = crate::domain::primitives::ContactId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let pipeline_id = PipelineId::from_str("111e8400-e29b-41d4-a716-446655440000").unwrap();
        let owner_id = UserId::from_str("333e8400-e29b-41d4-a716-446655440000").unwrap();
        
        let value1 = Money::new(Decimal::new(100000, 2), Currency::USD); // $1000.00
        let value2 = Money::new(Decimal::new(200000, 2), Currency::USD); // $2000.00
        
        let stage1_id = StageId::from_str("222e8400-e29b-41d4-a716-446655440000").unwrap();
        let stage2_id = StageId::from_str("444e8400-e29b-41d4-a716-446655440000").unwrap();
        
        vec![
            Deal::new(
                contact_id.clone(),
                pipeline_id.clone(),
                stage1_id,
                "Deal 1".to_string(),
                value1,
                None,
                false,
                owner_id.clone(),
            ).unwrap(),
            Deal::new(
                contact_id.clone(),
                pipeline_id.clone(),
                stage2_id,
                "Deal 2".to_string(),
                value2,
                None,
                false,
                owner_id.clone(),
            ).unwrap(),
        ]
    }
}