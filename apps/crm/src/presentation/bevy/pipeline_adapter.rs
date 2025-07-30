//! Pipeline adapter for CRM visualization
//!
//! This module provides adapters to convert domain objects to visualization objects.

use crate::domain::pipeline::{Pipeline, PipelineStage};
use crate::domain::deal::Deal;
use crate::domain::contact::{Contact, ConsentSettings};
use std::collections::HashMap;
use crate::domain::primitives::{ContactId, PipelineId, StageId, UserId};
use cpc_core::finance::domain::primitives::{Currency, Money};
use rust_decimal::Decimal;
use std::str::FromStr;
use chrono::{Utc, Duration};

/// Visualization representation of a pipeline
#[derive(Debug, Clone)]
pub struct VizPipeline {
    /// Name of the pipeline
    pub name: String,
    
    /// Stages in the pipeline
    pub stages: Vec<VizStage>,
}

/// Visualization representation of a pipeline stage
#[derive(Debug, Clone)]
pub struct VizStage {
    /// Name of the stage
    pub name: String,
    
    /// Probability of closing a deal at this stage
    pub probability: u8,
    
    /// Position in the pipeline
    pub position: u8,
    
    /// Deals in this stage
    pub deals: Vec<VizDeal>,
}

/// Visualization representation of a deal
#[derive(Debug, Clone)]
pub struct VizDeal {
    /// Title of the deal
    pub title: String,
    
    /// Value of the deal
    pub value: crate::domain::primitives::MonetaryAmount,
    
    /// Indicates if the deal is overdue
    pub is_overdue: bool,
    
    /// Contact associated with the deal
    pub contact: Option<VizContact>,
}

/// Visualization representation of a contact
#[derive(Debug, Clone)]
pub struct VizContact {
    /// Name of the contact
    pub name: String,
    
    /// Company of the contact
    pub company: Option<String>,
}

/// Convert a domain pipeline to a visualization pipeline
pub fn convert_pipeline_to_viz(pipeline: &Pipeline, deals: &[Deal], contacts: &[Contact]) -> VizPipeline {
    // Create a map of contacts for quick lookup
    let contact_map: HashMap<_, _> = contacts.iter().map(|c| (c.id.clone(), c)).collect();
    
    // Group deals by stage
    let mut deals_by_stage: HashMap<_, Vec<_>> = HashMap::new();
    for deal in deals {
        deals_by_stage.entry(deal.current_stage.clone()).or_default().push(deal);
    }
    
    // Convert stages
    let stages = pipeline.stages.iter().map(|stage| {
        // Get deals for this stage
        let stage_deals = deals_by_stage.get(&stage.id).cloned().unwrap_or_default();
        
        // Convert deals to visualization deals
        let deals = stage_deals.iter().map(|deal| {
            // Get contact for this deal
            let contact = contact_map.get(&deal.contact_id).map(|c| VizContact {
                name: c.name.clone(),
                company: c.company.clone(),
            });
            
            VizDeal {
                title: deal.title.clone(),
                value: deal.value.clone(),
                is_overdue: deal.is_overdue(),
                contact,
            }
        }).collect();
        
        VizStage {
            name: stage.name.clone(),
            probability: stage.probability,
            position: stage.position,
            deals,
        }
    }).collect();
    
    VizPipeline {
        name: pipeline.name.clone(),
        stages,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::primitives::{PipelineId, StageId, ContactId, DealId, UserId};
    use cpc_core::finance::domain::primitives::{Currency, Money};
    use rust_decimal::Decimal;
    use std::str::FromStr;
    use chrono::{Utc, Duration};

    #[test]
    fn test_convert_pipeline_to_viz() {
        // Create a pipeline
        let owner_id = UserId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let mut pipeline = Pipeline::new(
            "Test Pipeline".to_string(),
            owner_id,
            false,
            Vec::new(),
        ).unwrap();
        
        // Add stages
        let stage1 = PipelineStage::new(
            "Prospecting".to_string(),
            20,
            None,
            0,
        ).unwrap();
        
        let stage2 = PipelineStage::new(
            "Qualification".to_string(),
            50,
            None,
            1,
        ).unwrap();
        
        pipeline.add_stage(stage1.clone()).unwrap();
        pipeline.add_stage(stage2.clone()).unwrap();
        
        // Create contacts
        let contact1 = create_test_contact("John Doe", "Acme Corp");
        let contact2 = create_test_contact("Jane Smith", "Beta Inc");
        
        // Create deals
        let deal1 = create_test_deal(
            "Deal 1".to_string(),
            contact1.id.clone(),
            pipeline.id.clone(),
            stage1.id.clone(),
            false,
        );
        
        let mut deal2 = create_test_deal(
            "Deal 2".to_string(),
            contact2.id.clone(),
            pipeline.id.clone(),
            stage2.id.clone(),
            false,
        );
        
        // Make deal2 overdue
        deal2.expected_close_date = Some(Utc::now() - Duration::days(1));
        
        let deals = vec![deal1, deal2];
        let contacts = vec![contact1, contact2];
        
        // Convert to visualization pipeline
        let viz_pipeline = convert_pipeline_to_viz(&pipeline, &deals, &contacts);
        
        assert_eq!(viz_pipeline.name, "Test Pipeline");
        assert_eq!(viz_pipeline.stages.len(), 2);
        
        // Check first stage
        let first_stage = &viz_pipeline.stages[0];
        assert_eq!(first_stage.name, "Prospecting");
        assert_eq!(first_stage.probability, 20);
        assert_eq!(first_stage.position, 0);
        assert_eq!(first_stage.deals.len(), 1);
        
        let first_deal = &first_stage.deals[0];
        assert_eq!(first_deal.title, "Deal 1");
        assert!(!first_deal.is_overdue);
        assert!(first_deal.contact.is_some());
        assert_eq!(first_deal.contact.as_ref().unwrap().name, "John Doe");
        assert_eq!(first_deal.contact.as_ref().unwrap().company, Some("Acme Corp".to_string()));
        
        // Check second stage
        let second_stage = &viz_pipeline.stages[1];
        assert_eq!(second_stage.name, "Qualification");
        assert_eq!(second_stage.probability, 50);
        assert_eq!(second_stage.position, 1);
        assert_eq!(second_stage.deals.len(), 1);
        
        let second_deal = &second_stage.deals[0];
        assert_eq!(second_deal.title, "Deal 2");
        assert!(second_deal.is_overdue);
        assert!(second_deal.contact.is_some());
        assert_eq!(second_deal.contact.as_ref().unwrap().name, "Jane Smith");
        assert_eq!(second_deal.contact.as_ref().unwrap().company, Some("Beta Inc".to_string()));
    }
    
    fn create_test_contact(name: &str, company: &str) -> Contact {
        let user_id = UserId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let consent = ConsentSettings::new();
        
        Contact::new_platform_native(
            user_id,
            name.to_string(),
            None,
            None,
            Some(company.to_string()),
            consent,
        ).unwrap()
    }
    
    fn create_test_deal(
        title: String,
        contact_id: ContactId,
        pipeline_id: PipelineId,
        stage_id: StageId,
        is_overdue: bool,
    ) -> Deal {
        let owner_id = UserId::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let value = Money::new(Decimal::new(50000, 2), Currency::USD); // $500.00
        let expected_close_date = if is_overdue {
            Some(Utc::now() - Duration::days(1))
        } else {
            Some(Utc::now() + Duration::days(30))
        };
        
        Deal::new(
            contact_id,
            pipeline_id,
            stage_id,
            title,
            value,
            expected_close_date,
            false,
            owner_id,
        ).unwrap()
    }
}