//! Sales pipeline service with calendar integration
//!
//! This service manages the sales pipeline stages and integrates with the calendar module
//! to create visual timeline events for key milestones.

use crate::domain::{
    Opportunity, SalesStage, SalesPipelineError, OpportunityRepository, 
    StageTransition, TransitionHistory
};
use async_trait::async_trait;
use uuid::Uuid;
use std::sync::Arc;
use tracing::{info, error, instrument};
use chrono::Utc;

/// Service for managing sales pipeline stages
pub struct SalesPipelineService {
    opportunity_repo: Arc<dyn OpportunityRepository>,
    calendar_integration: Arc<dyn CalendarEventRegistrar>,
}

impl SalesPipelineService {
    pub fn new(
        opportunity_repo: Arc<dyn OpportunityRepository>,
        calendar_integration: Arc<dyn CalendarEventRegistrar>,
    ) -> Self {
        Self {
            opportunity_repo,
            calendar_integration,
        }
    }

    /// Move an opportunity to the next stage
    #[instrument(skip(self))]
    pub async fn advance_stage(
        &self,
        opportunity_id: Uuid,
        user_id: Uuid,
    ) -> Result<Opportunity, SalesPipelineError> {
        let mut opportunity = self.opportunity_repo
            .find_by_id(opportunity_id)
            .await?
            .ok_or(SalesPipelineError::OpportunityNotFound(opportunity_id))?;

        // Record the transition
        let previous_stage = opportunity.stage;
        opportunity.advance_stage();
        opportunity.transition_history.push(TransitionHistory {
            from_stage: previous_stage,
            to_stage: opportunity.stage,
            timestamp: Utc::now(),
            user_id,
        });

        // Save the updated opportunity
        self.opportunity_repo.save(&opportunity).await?;

        // Register with calendar if this is a significant milestone
        if let Err(e) = self.register_pipeline_milestone(user_id, opportunity_id, opportunity.stage).await {
            // Log error but don't fail the operation
            eprintln!("Failed to register pipeline milestone with calendar: {}", e);
        }

        Ok(opportunity)
    }

    /// Register a pipeline milestone with the calendar
    async fn register_pipeline_milestone(
        &self,
        user_id: Uuid,
        opportunity_id: Uuid,
        stage: SalesStage,
    ) -> Result<(), SalesPipelineError> {
        // Only register key milestones, not every small transition
        match stage {
            SalesStage::Qualified | 
            SalesStage::DemoScheduled | 
            SalesStage::ProposalSent | 
            SalesStage::ClosedWon => {
                self.calendar_integration.register_sales_pipeline_event(
                    user_id,
                    opportunity_id,
                    stage,
                ).map_err(|e| SalesPipelineError::CalendarIntegrationError(e.to_string()))?;
            }
            _ => {}
        }
        Ok(())
    }
}

/// Trait for registering events with the calendar
pub trait CalendarEventRegistrar: Send + Sync {
    fn register_sales_pipeline_event(
        &self,
        user_id: Uuid,
        opportunity_id: Uuid,
        stage: SalesStage,
    ) -> Result<(), String>;
}