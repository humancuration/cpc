//! Service layer for opportunity management.

use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use super::{
    models::{VolunteerOpportunity, OpportunityApplication, ApplicationStatus},
    repository::{OpportunityRepository, OpportunityRepositoryError, ListOpportunitiesFilters},
};

use crate::skill_management::repository::{SkillRepository, SkillRepositoryError};
use cause_management::proto::cause_service_client::CauseServiceClient;
use cause_management::proto::GetCauseRequest;
use tonic::transport::Channel;

use chrono::{DateTime, Utc};

#[derive(Error, Debug)]
pub enum OpportunityServiceError {
    #[error("Opportunity not found")]
    NotFound,
    #[error("Cause not found")]
    CauseNotFound,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("An internal error occurred: {0}")]
    Internal(String),
    #[error("Cause service communication error: {0}")]
    CauseServiceError(String),
}

impl From<OpportunityRepositoryError> for OpportunityServiceError {
    fn from(err: OpportunityRepositoryError) -> Self {
        match err {
            OpportunityRepositoryError::NotFound => OpportunityServiceError::NotFound,
            OpportunityRepositoryError::DatabaseError(e) => OpportunityServiceError::Internal(e.to_string()),
            OpportunityRepositoryError::ApplicationNotFound => OpportunityServiceError::NotFound,
            _ => OpportunityServiceError::Internal("An unexpected error occurred".to_string()),
        }
    }
}

pub struct UpdateOpportunityData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub required_skill_ids: Option<Vec<Uuid>>,
    pub estimated_hours: Option<i32>,
    pub deadline: Option<Option<DateTime<Utc>>>, // Option of Option to allow setting deadline to NULL
}

/// The service for managing volunteer opportunities.
pub struct OpportunityService {
    opp_repo: Arc<dyn OpportunityRepository>,
    skill_repo: Arc<dyn SkillRepository>,
    cause_client: CauseServiceClient<Channel>,
}

impl OpportunityService {
    /// Creates a new `OpportunityService`.
    pub fn new(
        opp_repo: Arc<dyn OpportunityRepository>,
        skill_repo: Arc<dyn SkillRepository>,
        cause_client: CauseServiceClient<Channel>,
    ) -> Self {
        Self { opp_repo, skill_repo, cause_client }
    }

    /// Creates a new volunteer opportunity.
    pub async fn create_opportunity(
        &mut self,
        cause_id: Uuid,
        title: String,
        description: String,
        required_skill_ids: Vec<Uuid>,
        estimated_hours: Option<i32>,
        deadline: Option<DateTime<Utc>>,
        created_by: Uuid,
    ) -> Result<VolunteerOpportunity, OpportunityServiceError> {
        // 1. Validate input
        if title.is_empty() {
            return Err(OpportunityServiceError::InvalidInput("Title cannot be empty".to_string()));
        }

        // 2. Validate cause exists by calling the CauseService
        let request = tonic::Request::new(GetCauseRequest {
            cause_id: cause_id.to_string(),
        });
        
        self.cause_client.get_cause(request).await.map_err(|e| {
            OpportunityServiceError::CauseServiceError(e.to_string())
        })?;

        // 3. Validate that all skill IDs exist
        for skill_id in &required_skill_ids {
            self.skill_repo.find_skill_by_id(*skill_id).await
                .map_err(|e| OpportunityServiceError::Internal(e.to_string()))?
                .ok_or_else(|| OpportunityServiceError::InvalidInput(format!("Skill with ID {} not found", skill_id)))?;
        }

        // 4. Create the opportunity
        let opportunity = VolunteerOpportunity::new(
            cause_id,
            title,
            description,
            required_skill_ids,
            estimated_hours,
            deadline,
            created_by,
        );

        self.opp_repo.create_opportunity(&opportunity).await?;

        Ok(opportunity)
    }

    /// Gets a single volunteer opportunity by its ID.
    pub async fn get_opportunity(
        &self,
        opportunity_id: Uuid,
    ) -> Result<VolunteerOpportunity, OpportunityServiceError> {
        self.opp_repo
            .find_opportunity_by_id(opportunity_id)
            .await?
            .ok_or(OpportunityServiceError::NotFound)
    }

    /// Lists volunteer opportunities based on filter criteria.
    pub async fn list_opportunities(
        &self,
        cause_id: Option<Uuid>,
        skill_id: Option<Uuid>,
        only_open: bool,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VolunteerOpportunity>, i64), OpportunityServiceError> {
        let filters = ListOpportunitiesFilters {
            cause_id,
            skill_id,
            only_open,
            limit,
            offset,
        };

        self.opp_repo
            .list_opportunities(&filters)
            .await
            .map_err(|e| e.into())
    }

    /// Updates an existing volunteer opportunity.
    pub async fn update_opportunity(
        &mut self,
        opportunity_id: Uuid,
        data: UpdateOpportunityData,
    ) -> Result<VolunteerOpportunity, OpportunityServiceError> {
        // 1. Fetch the existing opportunity
        let mut opportunity = self
            .opp_repo
            .find_opportunity_by_id(opportunity_id)
            .await?
            .ok_or(OpportunityServiceError::NotFound)?;

        // 2. Update fields if they are provided
        if let Some(title) = data.title {
            if title.is_empty() {
                return Err(OpportunityServiceError::InvalidInput("Title cannot be empty".to_string()));
            }
            opportunity.title = title;
        }
        if let Some(description) = data.description {
            opportunity.description = description;
        }
        if let Some(estimated_hours) = data.estimated_hours {
            opportunity.estimated_hours = Some(estimated_hours);
        }
        if let Some(deadline) = data.deadline {
            opportunity.deadline = deadline;
        }
        if let Some(required_skill_ids) = data.required_skill_ids {
            // Validate that all skill IDs exist
            for skill_id in &required_skill_ids {
                self.skill_repo.find_skill_by_id(*skill_id).await
                    .map_err(|e| OpportunityServiceError::Internal(e.to_string()))?
                    .ok_or_else(|| OpportunityServiceError::InvalidInput(format!("Skill with ID {} not found", skill_id)))?;
            }
            opportunity.required_skills = required_skill_ids;
        }

        // 3. Persist the changes
        self.opp_repo.update_opportunity(&opportunity).await
            .map_err(|e| e.into())
    }
    /// Applies a user for a volunteer opportunity.
    pub async fn apply_for_opportunity(
        &self,
        user_id: Uuid,
        opportunity_id: Uuid,
    ) -> Result<OpportunityApplication, OpportunityServiceError> {
        // 1. Check if opportunity exists
        self.opp_repo
            .find_opportunity_by_id(opportunity_id)
            .await?
            .ok_or(OpportunityServiceError::NotFound)?;

        // 2. TODO: Check if user exists by calling a user service.
        // For now, we assume the user exists.

        // 3. TODO: Check if the user has already applied for this opportunity.
        // This would require a new repository method like `find_application_by_user_and_opportunity`.
        // For now, we allow multiple applications.

        // 4. Create the application
        self.opp_repo
            .create_application(user_id, opportunity_id)
            .await
            .map_err(|e| e.into())
    }

    /// Updates the status of an application.
    pub async fn update_application_status(
        &self,
        application_id: Uuid,
        new_status: ApplicationStatus,
        volunteer_hours: Option<rust_decimal::Decimal>,
    ) -> Result<OpportunityApplication, OpportunityServiceError> {
        // 1. Fetch the application
        let mut application = self
            .opp_repo
            .find_application_by_id(application_id)
            .await
            .map_err(|e| e.into())?
            .ok_or(OpportunityServiceError::NotFound)?;

        // 2. Update status
        application.status = new_status;

        // 3. Update volunteer hours based on status
        if application.status == ApplicationStatus::Completed {
            // As per the ADR, only update hours if they are explicitly provided.
            if let Some(hours) = volunteer_hours {
                application.volunteer_hours = Some(hours);
            }
        } else {
            // If the status is not 'Completed', clear any logged hours for data consistency.
            application.volunteer_hours = None;
        }

        // 4. Persist changes
        self.opp_repo
            .update_application(&application)
            .await
            .map_err(|e| e.into())
    }

    /// Deletes a volunteer opportunity.
    pub async fn delete_opportunity(
        &self,
        opportunity_id: Uuid,
    ) -> Result<(), OpportunityServiceError> {
        // 1. Ensure the opportunity exists before deleting
        self.opp_repo
            .find_opportunity_by_id(opportunity_id)
            .await?
            .ok_or(OpportunityServiceError::NotFound)?;

        // 2. Delete the opportunity
        self.opp_repo
            .delete_opportunity(opportunity_id)
            .await
            .map_err(|e| e.into())
    }

   /// Lists applications for a given user.
   pub async fn list_user_applications(
       &self,
       user_id: Uuid,
       status: Option<String>,
       limit: i64,
       offset: i64,
   ) -> Result<(Vec<OpportunityApplication>, i64), OpportunityServiceError> {
       // In a real application, you might want to validate the status string against the ApplicationStatus enum
       let filters = super::repository::ListUserApplicationsFilters {
           user_id,
           status,
           limit,
           offset,
       };

       self.opp_repo
           .list_user_applications(&filters)
           .await
           .map_err(|e| e.into())
   }
}