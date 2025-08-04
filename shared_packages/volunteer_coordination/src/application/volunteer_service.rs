//! Application layer service implementation for Volunteer Coordination
//! Composes repositories and optional reputation verification port.

use std::sync::Arc;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::models::*;
use crate::domain::repository::*;
use crate::domain::service::*;

use crate::application::reputation_port::{ReputationPort, ReputationError};

pub struct VolunteerServiceImpl {
    pub opportunities: Arc<dyn OpportunityRepository>,
    pub applications: Arc<dyn ApplicationRepository>,
    pub contributions: Arc<dyn ContributionRepository>,
    // Optional async reputation port for verification
    pub reputation: Option<Arc<dyn ReputationPort + Send + Sync>>,
}

impl VolunteerServiceImpl {
    pub fn new(
        opportunities: Arc<dyn OpportunityRepository>,
        applications: Arc<dyn ApplicationRepository>,
        contributions: Arc<dyn ContributionRepository>,
        reputation: Option<Arc<dyn ReputationPort + Send + Sync>>,
    ) -> Self {
        Self { opportunities, applications, contributions, reputation }
    }
}

#[async_trait::async_trait]
impl VolunteerOpportunityService for VolunteerServiceImpl {
    async fn create_opportunity(
        &self,
        org_id: Uuid,
        created_by: Uuid,
        title: String,
        description: String,
        tags: Vec<String>,
        location: Option<String>,
        starts_at: Option<DateTime<Utc>>,
        ends_at: Option<DateTime<Utc>>,
    ) -> Result<VolunteerOpportunity, VolunteerServiceError> {
        let now = Utc::now();
        let opp = VolunteerOpportunity {
            id: OpportunityId(Uuid::new_v4()),
            org_id,
            created_by,
            title,
            description,
            tags,
            status: OpportunityStatus::Draft,
            location,
            starts_at,
            ends_at,
            created_at: now,
            updated_at: now,
        };
        if let Err(e) = opp.validate() {
            return Err(VolunteerServiceError::ValidationError(e));
        }
        self.opportunities.insert(&opp).await.map_err(|e| VolunteerServiceError::RepositoryError(e.to_string()))?;
        Ok(opp)
    }

    async fn publish_opportunity(&self, opportunity_id: OpportunityId, _user_id: Uuid) -> Result<VolunteerOpportunity, VolunteerServiceError> {
        let current = self.opportunities.get(opportunity_id).await.map_err(|e| VolunteerServiceError::RepositoryError(e.to_string()))?;
        let mut opp = current.ok_or(VolunteerServiceError::NotFound)?;
        if opp.status == OpportunityStatus::Published {
            return Ok(opp);
        }
        opp.status = OpportunityStatus::Published;
        opp.updated_at = Utc::now();
        self.opportunities.update_status(opportunity_id, OpportunityStatus::Published).await.map_err(|e| VolunteerServiceError::RepositoryError(e.to_string()))?;
        Ok(opp)
    }

    async fn close_opportunity(&self, opportunity_id: OpportunityId, _user_id: Uuid) -> Result<VolunteerOpportunity, VolunteerServiceError> {
        let current = self.opportunities.get(opportunity_id).await.map_err(|e| VolunteerServiceError::RepositoryError(e.to_string()))?;
        let mut opp = current.ok_or(VolunteerServiceError::NotFound)?;
        if opp.status == OpportunityStatus::Closed {
            return Ok(opp);
        }
        opp.status = OpportunityStatus::Closed;
        opp.updated_at = Utc::now();
        self.opportunities.update_status(opportunity_id, OpportunityStatus::Closed).await.map_err(|e| VolunteerServiceError::RepositoryError(e.to_string()))?;
        Ok(opp)
    }

    async fn get_opportunity(&self, opportunity_id: OpportunityId) -> Result<Option<VolunteerOpportunity>, VolunteerServiceError> {
        self.opportunities.get(opportunity_id).await.map_err(|e| VolunteerServiceError::RepositoryError(e.to_string()))
    }
}

#[async_trait::async_trait]
impl VolunteerApplicationService for VolunteerServiceImpl {
    async fn submit_application(
        &self,
        opportunity_id: OpportunityId,
        applicant_id: Uuid,
        motivation: Option<String>,
    ) -> Result<VolunteerApplication, VolunteerServiceError> {
        // Ensure opportunity exists and is open
        let opp = self.opportunities.get(opportunity_id).await.map_err(|e| VolunteerServiceError::RepositoryError(e.to_string()))?;
        let opp = opp.ok_or(VolunteerServiceError::NotFound)?;
        if opp.status != OpportunityStatus::Published {
            return Err(VolunteerServiceError::Conflict("Opportunity not open for applications".into()));
        }

        let app = VolunteerApplication {
            id: ApplicationId(Uuid::new_v4()),
            opportunity_id,
            applicant_id,
            motivation,
            status: ApplicationStatus::Submitted,
            submitted_at: Utc::now(),
            decided_at: None,
            reviewer_id: None,
        };
        if let Err(e) = app.validate() {
            return Err(VolunteerServiceError::ValidationError(e));
        }
        self.applications.insert(&app).await.map_err(|e| VolunteerServiceError::RepositoryError(e.to_string()))?;
        Ok(app)
    }

    async fn review_application(
        &self,
        application_id: ApplicationId,
        reviewer_id: Uuid,
        status: ApplicationStatus,
    ) -> Result<VolunteerApplication, VolunteerServiceError> {
        let cur = self.applications.get(application_id).await.map_err(|e| VolunteerServiceError::RepositoryError(e.to_string()))?;
        let mut app = cur.ok_or(VolunteerServiceError::NotFound)?;
        app.status = status;
        app.reviewer_id = Some(reviewer_id);
        app.decided_at = Some(Utc::now());
        self.applications.update_status(application_id, status, app.reviewer_id, app.decided_at).await.map_err(|e| VolunteerServiceError::RepositoryError(e.to_string()))?;
        Ok(app)
    }

    async fn get_application(&self, application_id: ApplicationId) -> Result<Option<VolunteerApplication>, VolunteerServiceError> {
        self.applications.get(application_id).await.map_err(|e| VolunteerServiceError::RepositoryError(e.to_string()))
    }
}

#[async_trait::async_trait]
impl VolunteerContributionService for VolunteerServiceImpl {
    async fn log_contribution(
        &self,
        opportunity_id: OpportunityId,
        contributor_id: Uuid,
        kind: ContributionKind,
        amount: f32,
        notes: Option<String>,
        occurred_at: DateTime<Utc>,
    ) -> Result<VolunteerContribution, VolunteerServiceError> {
        // Ensure opportunity exists
        let _ = self.opportunities.get(opportunity_id).await.map_err(|e| VolunteerServiceError::RepositoryError(e.to_string()))?
            .ok_or(VolunteerServiceError::NotFound)?;

        let c = VolunteerContribution {
            id: ContributionId(Uuid::new_v4()),
            opportunity_id,
            contributor_id,
            kind,
            amount,
            notes,
            occurred_at,
            created_at: Utc::now(),
            verified: false,
            verification_ref: None,
        };
        if let Err(e) = c.validate() {
            return Err(VolunteerServiceError::ValidationError(e));
        }
        self.contributions.insert(&c).await.map_err(|e| VolunteerServiceError::RepositoryError(e.to_string()))?;
        Ok(c)
    }

    async fn verify_contribution(
        &self,
        contribution_id: ContributionId,
        _verifier_id: Uuid,
        verification_ref: Option<Uuid>,
    ) -> Result<VolunteerContribution, VolunteerServiceError> {
        let cur = self.contributions.get(contribution_id).await.map_err(|e| VolunteerServiceError::RepositoryError(e.to_string()))?;
        let mut c = cur.ok_or(VolunteerServiceError::NotFound)?;

        // Best-effort verification using async reputation port if available.
        let mut verified = true;
        if let Some(rep) = &self.reputation {
            // Map domain types to port call
            let amount_hours = if c.kind == ContributionKind::Hours { Some(c.amount) } else { None };
            match rep.verify_contribution(c.id, c.kind, amount_hours, c.contributor_id).await {
                Ok(v) => { verified = v; }
                Err(_e) => {
                    // On error from reputation service, default to false to be conservative.
                    // TODO: consider policy ADR to decide fallback behavior.
                    verified = false;
                }
            }
        }
        // TODO: Emit future ContributionVerified event when introduced in SocialEvent set (see ADR 0008).

        self.contributions.verify(contribution_id, verified, verification_ref).await.map_err(|e| VolunteerServiceError::RepositoryError(e.to_string()))?;
        c.verified = verified;
        c.verification_ref = verification_ref;
        Ok(c)
    }

    async fn get_contribution(&self, contribution_id: ContributionId) -> Result<Option<VolunteerContribution>, VolunteerServiceError> {
        self.contributions.get(contribution_id).await.map_err(|e| VolunteerServiceError::RepositoryError(e.to_string()))
    }
}