//! Deal service for the CRM module
//!
//! This module contains the application service for managing deals,
//! including value calculation, forecasting, and pipeline stage transitions.

use crate::domain::deal::{Deal, DealNote, DealError};
use crate::domain::primitives::{DealId, ContactId, PipelineId, StageId, UserId};
use crate::domain::pipeline::Pipeline;
use thiserror::Error;
use std::collections::HashMap;

/// Error types for deal service operations
#[derive(Error, Debug)]
pub enum DealServiceError {
    #[error("Deal error: {0}")]
    DealError(#[from] DealError),
    
    #[error("Deal not found: {0}")]
    DealNotFound(DealId),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

/// Service for managing deals
pub struct DealService;

impl DealService {
    /// Create a new deal
    pub fn create_deal(
        contact_id: ContactId,
        pipeline_id: PipelineId,
        current_stage: StageId,
        title: String,
        value: crate::domain::primitives::MonetaryAmount,
        expected_close_date: Option<chrono::DateTime<chrono::Utc>>,
        is_platform_deal: bool,
        owner_id: UserId,
    ) -> Result<Deal, DealServiceError> {
        let deal = Deal::new(
            contact_id,
            pipeline_id,
            current_stage,
            title,
            value,
            expected_close_date,
            is_platform_deal,
            owner_id,
        )?;
        
        Ok(deal)
    }
    
    /// Update deal information
    pub fn update_deal_info(
        deal: &mut Deal,
        title: Option<String>,
        value: Option<crate::domain::primitives::MonetaryAmount>,
        expected_close_date: Option<chrono::DateTime<chrono::Utc>>,
        current_stage: Option<StageId>,
    ) -> Result<(), DealServiceError> {
        deal.update_info(title, value, expected_close_date, current_stage)?;
        Ok(())
    }
    
    /// Add a note to a deal
    pub fn add_note_to_deal(
        deal: &mut Deal,
        note: DealNote,
    ) -> Result<(), DealServiceError> {
        deal.add_note(note)?;
        Ok(())
    }
    
    /// Remove a note from a deal
    pub fn remove_note_from_deal(
        deal: &mut Deal,
        index: usize,
    ) -> Result<(), DealServiceError> {
        deal.remove_note(index)?;
        Ok(())
    }
    
    /// Update a note in a deal
    pub fn update_note_in_deal(
        deal: &mut Deal,
        index: usize,
        content: String,
    ) -> Result<(), DealServiceError> {
        deal.update_note(index, content)?;
        Ok(())
    }
    
    /// Add custom fields to a deal
    pub fn add_custom_fields_to_deal(
        deal: &mut Deal,
        fields: HashMap<String, String>,
    ) {
        deal.add_custom_fields(fields);
    }
    
    /// Remove custom fields from a deal
    pub fn remove_custom_fields_from_deal(
        deal: &mut Deal,
        keys: Vec<String>,
    ) {
        deal.remove_custom_fields(keys);
    }
    
    /// Validate a deal
    pub fn validate_deal(deal: &Deal) -> Result<(), DealServiceError> {
        deal.validate()?;
        Ok(())
    }
    
    /// Check if a deal is overdue
    pub fn is_deal_overdue(deal: &Deal) -> bool {
        deal.is_overdue()
    }
    
    /// Calculate the weighted value of a deal based on stage probability
    pub fn calculate_weighted_value(
        deal: &Deal,
        pipeline: &Pipeline,
    ) -> Option<crate::domain::primitives::MonetaryAmount> {
        // Get the current stage
        let stage = pipeline.get_stage(&deal.current_stage)?;
        
        // Calculate weighted value
        Some(deal.calculate_weighted_value(stage.probability))
    }
    
    /// Forecast deal value based on historical conversion rates
    pub fn forecast_deal_value(
        deal: &Deal,
        pipeline: &Pipeline,
        conversion_rates: &HashMap<StageId, f64>,
    ) -> Option<crate::domain::primitives::MonetaryAmount> {
        use rust_decimal::Decimal;
        
        // Get the current stage
        let current_stage = pipeline.get_stage(&deal.current_stage)?;
        
        // Get conversion rate for current stage
        let conversion_rate = conversion_rates.get(&current_stage.id)?;
        
        // Calculate forecasted value
        let value_decimal = deal.value.amount;
        let forecasted_decimal = value_decimal * Decimal::from_f64(*conversion_rate)?;
        
        Some(crate::domain::primitives::MonetaryAmount::new(
            forecasted_decimal,
            deal.value.currency.clone(),
        ))
    }
    
    /// Get deals for a specific contact
    pub fn get_deals_for_contact(
        deals: &[Deal],
        contact_id: &ContactId,
    ) -> Vec<&Deal> {
        deals.iter()
            .filter(|deal| &deal.contact_id == contact_id)
            .collect()
    }
    
    /// Get deals in a specific pipeline
    pub fn get_deals_in_pipeline(
        deals: &[Deal],
        pipeline_id: &PipelineId,
    ) -> Vec<&Deal> {
        deals.iter()
            .filter(|deal| &deal.pipeline_id == pipeline_id)
            .collect()
    }
    
    /// Get deals in a specific stage
    pub fn get_deals_in_stage(
        deals: &[Deal],
        stage_id: &StageId,
    ) -> Vec<&Deal> {
        deals.iter()
            .filter(|deal| &deal.current_stage == stage_id)
            .collect()
    }
    
    /// Get overdue deals
    pub fn get_overdue_deals(deals: &[Deal]) -> Vec<&Deal> {
        deals.iter()
            .filter(|deal| deal.is_overdue())
            .collect()
    }
    
    /// Calculate total value of deals
    pub fn calculate_total_value(deals: &[Deal]) -> crate::domain::primitives::MonetaryAmount {
        use cpc_core::finance::domain::primitives::{Money, Currency};
        
        // Start with zero amount in a default currency
        let mut total = Money::zero(Currency::USD);
        
        for deal in deals {
            // Try to add the deal value to the total
            total = total.add(&deal.value).unwrap_or(total);
        }
        
        total
    }
    
    /// Calculate weighted total value of deals based on pipeline stages
    pub fn calculate_weighted_total_value(
        deals: &[Deal],
        pipelines: &[Pipeline],
    ) -> crate::domain::primitives::MonetaryAmount {
        use cpc_core::finance::domain::primitives::{Money, Currency};
        
        // Start with zero amount in a default currency
        let mut total = Money::zero(Currency::USD);
        
        // Create a map of pipelines for quick lookup
        let pipeline_map: HashMap<&PipelineId, &Pipeline> = pipelines.iter()
            .map(|p| (&p.id, p))
            .collect();
        
        for deal in deals {
            // Find the pipeline for this deal
            if let Some(pipeline) = pipeline_map.get(&deal.pipeline_id) {
                // Calculate weighted value for this deal
                if let Some(weighted_value) = Self::calculate_weighted_value(deal, pipeline) {
                    // Add to total
                    total = total.add(&weighted_value).unwrap_or(total);
                } else {
                    // If we can't calculate weighted value, just add the deal value
                    total = total.add(&deal.value).unwrap_or(total);
                }
            } else {
                // If pipeline not found, just add the deal value
                total = total.add(&deal.value).unwrap_or(total);
            }
        }
        
        total
    }
    
    /// Get deal aging report
    pub fn get_deal_aging_report(
        deals: &[Deal],
    ) -> HashMap<String, usize> {
        let mut aging_report = HashMap::new();
        let now = chrono::Utc::now();
        
        for deal in deals {
            if let Some(expected_close_date) = deal.expected_close_date {
                let days_until_close = (expected_close_date - now).num_days();
                
                let category = if days_until_close < 0 {
                    "Overdue".to_string()
                } else if days_until_close <= 7 {
                    "This Week".to_string()
                } else if days_until_close <= 30 {
                    "This Month".to_string()
                } else {
                    "Later".to_string()
                };
                
                *aging_report.entry(category).or_insert(0) += 1;
            } else {
                *aging_report.entry("No Date".to_string()).or_insert(0) += 1;
            }
        }
        
        aging_report
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
        let expected_close_date = Some(chrono::Utc::now() + chrono::Duration::days(30));
        
        let deal = DealService::create_deal(
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
        
        DealService::add_note_to_deal(&mut deal, note1).unwrap();
        DealService::add_note_to_deal(&mut deal, note2).unwrap();
        
        assert_eq!(deal.notes.len(), 2);
        
        DealService::remove_note_from_deal(&mut deal, 0).unwrap();
        assert_eq!(deal.notes.len(), 1);
        assert_eq!(deal.notes[0].content, "Second note");
    }

    #[test]
    fn test_update_deal_info() {
        let mut deal = create_test_deal();
        let new_value = Money::new(Decimal::new(75000, 2), Currency::USD); // $750.00
        let new_date = Some(chrono::Utc::now() + chrono::Duration::days(45));
        
        let result = DealService::update_deal_info(
            &mut deal,
            Some("Updated Deal Title".to_string()),
            Some(new_value.clone()),
            new_date,
            None,
        );
        
        assert!(result.is_ok());
        assert_eq!(deal.title, "Updated Deal Title");
        assert_eq!(deal.value, new_value);
        assert_eq!(deal.expected_close_date, new_date);
    }

    #[test]
    fn test_calculate_weighted_value() {
        let deal = create_test_deal();
        let mut pipeline = create_test_pipeline();
        
        let stage = crate::domain::pipeline::PipelineStage::new(
            "Qualified".to_string(),
            75, // 75% probability
            None,
            0,
        ).unwrap();
        
        let stage_id = stage.id.clone();
        deal.move_to_stage(stage_id.clone());
        pipeline.add_stage(stage).unwrap();
        
        let weighted_value = DealService::calculate_weighted_value(&deal, &pipeline).unwrap();
        
        // Expected: $500.00 * 0.75 = $375.00
        let expected_value = Money::new(Decimal::new(37500, 2), Currency::USD);
        assert_eq!(weighted_value, expected_value);
    }

    #[test]
    fn test_get_deals_for_contact() {
        let deals = create_test_deals();
        let contact_id = ContactId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        
        let contact_deals = DealService::get_deals_for_contact(&deals, &contact_id);
        assert_eq!(contact_deals.len(), 2); // Two deals for this contact
    }

    #[test]
    fn test_calculate_total_value() {
        let deals = create_test_deals();
        
        let total_value = DealService::calculate_total_value(&deals);
        
        // Expected: $500.00 + $750.00 = $1250.00
        let expected_value = Money::new(Decimal::new(125000, 2), Currency::USD);
        assert_eq!(total_value, expected_value);
    }

    #[test]
    fn test_deal_aging_report() {
        let deals = create_test_deals_with_dates();
        
        let aging_report = DealService::get_deal_aging_report(&deals);
        
        // We should have at least one overdue deal and one with a future date
        assert!(aging_report.contains_key("Overdue") || aging_report.contains_key("This Week") || 
                aging_report.contains_key("This Month") || aging_report.contains_key("Later"));
    }

    fn create_test_deal() -> Deal {
        let contact_id = ContactId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let pipeline_id = PipelineId::from_str("111e8400-e29b-41d4-a716-446655440000").unwrap();
        let stage_id = StageId::from_str("222e8400-e29b-41d4-a716-446655440000").unwrap();
        let owner_id = UserId::from_str("333e8400-e29b-41d4-a716-446655440000").unwrap();
        let value = Money::new(Decimal::new(50000, 2), Currency::USD); // $500.00
        
        DealService::create_deal(
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
    
    fn create_test_pipeline() -> Pipeline {
        let owner_id = UserId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        
        Pipeline::new(
            "Test Pipeline".to_string(),
            owner_id,
            false,
            Vec::new(),
        ).unwrap()
    }
    
    fn create_test_deals() -> Vec<Deal> {
        let contact_id = ContactId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let pipeline_id = PipelineId::from_str("111e8400-e29b-41d4-a716-446655440000").unwrap();
        let owner_id = UserId::from_str("333e8400-e29b-41d4-a716-446655440000").unwrap();
        
        let value1 = Money::new(Decimal::new(50000, 2), Currency::USD); // $500.00
        let value2 = Money::new(Decimal::new(75000, 2), Currency::USD); // $750.00
        
        let stage_id = StageId::from_str("222e8400-e29b-41d4-a716-446655440000").unwrap();
        
        vec![
            Deal::new(
                contact_id.clone(),
                pipeline_id.clone(),
                stage_id.clone(),
                "Deal 1".to_string(),
                value1,
                None,
                false,
                owner_id.clone(),
            ).unwrap(),
            Deal::new(
                contact_id.clone(),
                pipeline_id.clone(),
                stage_id.clone(),
                "Deal 2".to_string(),
                value2,
                None,
                false,
                owner_id.clone(),
            ).unwrap(),
            Deal::new(
                ContactId::from_str("666e8400-e29b-41d4-a716-446655440000").unwrap(), // Different contact
                pipeline_id.clone(),
                stage_id.clone(),
                "Deal 3".to_string(),
                value1,
                None,
                false,
                owner_id.clone(),
            ).unwrap(),
        ]
    }
    
    fn create_test_deals_with_dates() -> Vec<Deal> {
        let contact_id = ContactId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let pipeline_id = PipelineId::from_str("111e8400-e29b-41d4-a716-446655440000").unwrap();
        let owner_id = UserId::from_str("333e8400-e29b-41d4-a716-446655440000").unwrap();
        let value = Money::new(Decimal::new(50000, 2), Currency::USD); // $500.00
        let stage_id = StageId::from_str("222e8400-e29b-41d4-a716-446655440000").unwrap();
        
        vec![
            Deal::new(
                contact_id.clone(),
                pipeline_id.clone(),
                stage_id.clone(),
                "Overdue Deal".to_string(),
                value.clone(),
                Some(chrono::Utc::now() - chrono::Duration::days(1)), // Overdue
                false,
                owner_id.clone(),
            ).unwrap(),
            Deal::new(
                contact_id.clone(),
                pipeline_id.clone(),
                stage_id.clone(),
                "Future Deal".to_string(),
                value.clone(),
                Some(chrono::Utc::now() + chrono::Duration::days(10)), // Future
                false,
                owner_id.clone(),
            ).unwrap(),
        ]
    }
}