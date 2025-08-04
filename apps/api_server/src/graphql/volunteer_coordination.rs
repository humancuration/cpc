#![allow(clippy::unused_async)]
//! GraphQL for Volunteer Coordination (ADR 0008 Volunteer Coordination)
//! Follows the same style as collaborative_workspace resolvers.
//! Exposes types, inputs, queries, and mutations for opportunities,
//! applications, and contributions, wired to volunteer_coordination services.

use async_graphql::{Context, Enum, InputObject, Object, Result, SimpleObject, ID};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use shared_packages::volunteer_coordination::domain::models as vc_models;
use shared_packages::volunteer_coordination::domain::service::{
    ApplicationStatus as DomainApplicationStatus, ContributionKind as DomainContributionKind,
    OpportunityId, ApplicationId, ContributionId,
    VolunteerApplicationService, VolunteerContributionService, VolunteerOpportunityService,
    VolunteerServiceError,
};
use shared_packages::volunteer_coordination::application::volunteer_service::VolunteerServiceImpl;

// Helper: current authenticated user id from context (pattern like collaborative_workspace)
fn current_user_id(ctx: &Context<'_>) -> Result<Uuid> {
    let user_id = ctx
        .data::<Uuid>()
        .map_err(|_| "Unauthorized: missing user id in context")?;
    Ok(*user_id)
}

fn map_service_err(e: VolunteerServiceError) -> async_graphql::Error {
    async_graphql::Error::new(e.to_string())
}

/* ===================== Enums ===================== */

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum OpportunityStatus {
    Draft,
    Published,
    Closed,
    Archived,
}
impl From<vc_models::OpportunityStatus> for OpportunityStatus {
    fn from(s: vc_models::OpportunityStatus) -> Self {
        match s {
            vc_models::OpportunityStatus::Draft => OpportunityStatus::Draft,
            vc_models::OpportunityStatus::Published => OpportunityStatus::Published,
            vc_models::OpportunityStatus::Closed => OpportunityStatus::Closed,
            vc_models::OpportunityStatus::Archived => OpportunityStatus::Archived,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ApplicationStatus {
    Submitted,
    UnderReview,
    Accepted,
    Rejected,
    Withdrawn,
}
impl From<vc_models::ApplicationStatus> for ApplicationStatus {
    fn from(s: vc_models::ApplicationStatus) -> Self {
        use vc_models::ApplicationStatus as A;
        match s {
            A::Submitted => ApplicationStatus::Submitted,
            A::UnderReview => ApplicationStatus::UnderReview,
            A::Accepted => ApplicationStatus::Accepted,
            A::Rejected => ApplicationStatus::Rejected,
            A::Withdrawn => ApplicationStatus::Withdrawn,
        }
    }
}
impl From<ApplicationStatus> for DomainApplicationStatus {
    fn from(s: ApplicationStatus) -> Self {
        match s {
            ApplicationStatus::Submitted => DomainApplicationStatus::Submitted,
            ApplicationStatus::UnderReview => DomainApplicationStatus::UnderReview,
            ApplicationStatus::Accepted => DomainApplicationStatus::Accepted,
            ApplicationStatus::Rejected => DomainApplicationStatus::Rejected,
            ApplicationStatus::Withdrawn => DomainApplicationStatus::Withdrawn,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ContributionKind {
    Hours,
    Deliverable,
    Donation,
    Other,
}
impl From<vc_models::ContributionKind> for ContributionKind {
    fn from(k: vc_models::ContributionKind) -> Self {
        use vc_models::ContributionKind as K;
        match k {
            K::Hours => ContributionKind::Hours,
            K::Deliverable => ContributionKind::Deliverable,
            K::Donation => ContributionKind::Donation,
            K::Other => ContributionKind::Other,
        }
    }
}
impl From<ContributionKind> for DomainContributionKind {
    fn from(k: ContributionKind) -> Self {
        match k {
            ContributionKind::Hours => DomainContributionKind::Hours,
            ContributionKind::Deliverable => DomainContributionKind::Deliverable,
            ContributionKind::Donation => DomainContributionKind::Donation,
            ContributionKind::Other => DomainContributionKind::Other,
        }
    }
}

/* ===================== Types ===================== */

#[derive(SimpleObject, Clone)]
pub struct VolunteerOpportunity {
    pub id: ID,
    pub org_id: ID,
    pub title: String,
    pub description: String,
    /// Tags/skills needed per ADR 0008
    pub skills_needed: Vec<String>,
    pub location: Option<String>,
    pub status: OpportunityStatus,
    pub created_by: ID,
    pub created_at: String,
    pub updated_at: String,
}
impl From<vc_models::VolunteerOpportunity> for VolunteerOpportunity {
    fn from(o: vc_models::VolunteerOpportunity) -> Self {
        Self {
            id: o.id.0.to_string().into(),
            org_id: o.org_id.to_string().into(),
            title: o.title,
            description: o.description,
            skills_needed: o.tags,
            location: o.location,
            status: o.status.into(),
            created_by: o.created_by.to_string().into(),
            created_at: o.created_at.to_rfc3339(),
            updated_at: o.updated_at.to_rfc3339(),
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct VolunteerApplication {
    pub id: ID,
    pub opportunity_id: ID,
    pub applicant_id: ID,
    pub message: Option<String>,
    pub status: ApplicationStatus,
    pub created_at: String,
    pub reviewed_by: Option<ID>,
    pub reviewed_at: Option<String>,
}
impl From<vc_models::VolunteerApplication> for VolunteerApplication {
    fn from(a: vc_models::VolunteerApplication) -> Self {
        Self {
            id: a.id.0.to_string().into(),
            opportunity_id: a.opportunity_id.0.to_string().into(),
            applicant_id: a.applicant_id.to_string().into(),
            message: a.motivation,
            status: a.status.into(),
            created_at: a.submitted_at.to_rfc3339(),
            reviewed_by: a.reviewer_id.map(|id| id.to_string().into()),
            reviewed_at: a.decided_at.map(|dt| dt.to_rfc3339()),
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct VolunteerContribution {
    pub id: ID,
    pub opportunity_id: ID,
    pub contributor_id: ID,
    pub kind: ContributionKind,
    pub hours: Option<f32>,
    pub notes: Option<String>,
    pub verified: bool,
    pub verified_by: Option<ID>,      // surfaced from verification_ref
    /// GraphQL alias for verified_by for client ergonomics
    #[graphql(name = "verificationRef")]
    pub verification_ref: Option<ID>,
    pub verified_at: Option<String>,   // not tracked in domain; None
    pub created_at: String,
}
impl From<vc_models::VolunteerContribution> for VolunteerContribution {
    fn from(c: vc_models::VolunteerContribution) -> Self {
        let hours = match c.kind {
            vc_models::ContributionKind::Hours => Some(c.amount),
            _ => None,
        };
        let vref = c.verification_ref.map(|id| id.to_string().into());
        Self {
            id: c.id.0.to_string().into(),
            opportunity_id: c.opportunity_id.0.to_string().into(),
            contributor_id: c.contributor_id.to_string().into(),
            kind: c.kind.into(),
            hours,
            notes: c.notes,
            verified: c.verified,
            verified_by: vref.clone(),
            verification_ref: vref,
            verified_at: None,
            created_at: c.created_at.to_rfc3339(),
        }
    }
}

/* ===================== Inputs ===================== */

#[derive(InputObject)]
pub struct CreateOpportunityInput {
    pub org_id: ID,
    pub title: String,
    pub description: String,
    pub skills_needed: Vec<String>,
    pub location: Option<String>,
    pub starts_at: Option<String>,
    pub ends_at: Option<String>,
}

#[derive(InputObject)]
pub struct SubmitApplicationInput {
    pub opportunity_id: ID,
    pub message: Option<String>,
}

#[derive(InputObject)]
pub struct ReviewApplicationInput {
    pub application_id: ID,
    pub status: ApplicationStatus, // approve/reject via Accepted/Rejected etc.
}

#[derive(InputObject)]
pub struct LogContributionInput {
    pub opportunity_id: ID,
    pub kind: ContributionKind,
    /// For hours kind, supply hours; for others, use a numeric quantity
    pub amount: f32,
    pub notes: Option<String>,
    /// ISO8601 timestamp of when it occurred
    pub occurred_at: String,
}

#[derive(InputObject)]
pub struct VerifyContributionInput {
    pub contribution_id: ID,
    /// If your domain requires a verification reference, pass it using this optional ID.
    pub verification_ref: Option<ID>,
}

/* ===================== Query Root ===================== */

#[derive(Default)]
pub struct VolunteerCoordinationQueries;

#[Object]
impl VolunteerCoordinationQueries {
    /// Get a single opportunity by ID
    async fn opportunity(&self, ctx: &Context<'_>, id: ID) -> Result<VolunteerOpportunity> {
        let _user = current_user_id(ctx)?;
        // Aligned: resolvers consistently fetch Arc<VolunteerServiceImpl> inserted by schema helper.
        let svc = ctx
            .data::<std::sync::Arc<VolunteerServiceImpl>>()
            .map_err(|_| "VolunteerServiceImpl not in context")?;
        let opp_id = Uuid::parse_str(id.as_str()).map_err(|_| "Invalid id")?;
        let found = svc
            .get_opportunity(OpportunityId(opp_id))
            .await
            .map_err(map_service_err)?
            .ok_or_else(|| async_graphql::Error::new("Not found"))?;
        Ok(found.into())
    }

    /// List opportunities by org, optional status filter
    async fn opportunities_by_org(
        &self,
        ctx: &Context<'_>,
        org_id: ID,
        status: Option<OpportunityStatus>,
    ) -> Result<Vec<VolunteerOpportunity>> {
        let _user = current_user_id(ctx)?;
        // NOTE: List operations are not in domain service yet; return empty for now to satisfy contract.
        let _ = (org_id, status);
        Ok(vec![])
    }

    /// List applications by opportunity, optional status
    async fn applications_by_opportunity(
        &self,
        ctx: &Context<'_>,
        opportunity_id: ID,
        status: Option<ApplicationStatus>,
    ) -> Result<Vec<VolunteerApplication>> {
        let _user = current_user_id(ctx)?;
        let _ = (opportunity_id, status);
        Ok(vec![])
    }

    /// List contributions by opportunity
    async fn contributions_by_opportunity(
        &self,
        ctx: &Context<'_>,
        opportunity_id: ID,
    ) -> Result<Vec<VolunteerContribution>> {
        let _user = current_user_id(ctx)?;
        let _ = opportunity_id;
        Ok(vec![])
    }

    /// List contributions by contributor
    async fn contributions_by_contributor(
        &self,
        ctx: &Context<'_>,
        contributor_id: ID,
    ) -> Result<Vec<VolunteerContribution>> {
        let _user = current_user_id(ctx)?;
        let _ = contributor_id;
        Ok(vec![])
    }
}

/* ===================== Mutation Root ===================== */

#[derive(Default)]
pub struct VolunteerCoordinationMutations;

#[Object]
impl VolunteerCoordinationMutations {
    /// Create a new volunteer opportunity (Draft status) - ADR 0008
    async fn create_opportunity(
        &self,
        ctx: &Context<'_>,
        input: CreateOpportunityInput,
    ) -> Result<VolunteerOpportunity> {
        let user_id = current_user_id(ctx)?;
        let org_id = Uuid::parse_str(input.org_id.as_str()).map_err(|_| "Invalid org id")?;

        // Parse optional datetimes
        let starts_at = parse_opt_datetime(&input.starts_at)?;
        let ends_at = parse_opt_datetime(&input.ends_at)?;

        // Aligned: resolvers consistently fetch Arc<VolunteerServiceImpl> inserted by schema helper.
        let svc = ctx
            .data::<std::sync::Arc<VolunteerServiceImpl>>()
            .map_err(|_| "VolunteerServiceImpl not in context")?;

        let opp = svc
            .create_opportunity(
                org_id,
                user_id,
                input.title,
                input.description,
                input.skills_needed,
                input.location,
                starts_at,
                ends_at,
            )
            .await
            .map_err(map_service_err)?;
        Ok(opp.into())
    }

    /// Publish an opportunity
    async fn publish_opportunity(&self, ctx: &Context<'_>, id: ID) -> Result<VolunteerOpportunity> {
        let user_id = current_user_id(ctx)?;
        let opp_id = OpportunityId(Uuid::parse_str(id.as_str()).map_err(|_| "Invalid id")?);
        // Aligned: resolvers consistently fetch Arc<VolunteerServiceImpl> inserted by schema helper.
        let svc = ctx
            .data::<std::sync::Arc<VolunteerServiceImpl>>()
            .map_err(|_| "VolunteerServiceImpl not in context")?;
        let opp = svc.publish_opportunity(opp_id, user_id).await.map_err(map_service_err)?;
        Ok(opp.into())
    }

    /// Close an opportunity
    async fn close_opportunity(&self, ctx: &Context<'_>, id: ID) -> Result<VolunteerOpportunity> {
        let user_id = current_user_id(ctx)?;
        let opp_id = OpportunityId(Uuid::parse_str(id.as_str()).map_err(|_| "Invalid id")?);
        // Aligned: resolvers consistently fetch Arc<VolunteerServiceImpl> inserted by schema helper.
        let svc = ctx
            .data::<std::sync::Arc<VolunteerServiceImpl>>()
            .map_err(|_| "VolunteerServiceImpl not in context")?;
        let opp = svc.close_opportunity(opp_id, user_id).await.map_err(map_service_err)?;
        Ok(opp.into())
    }

    /// Submit an application
    async fn submit_application(
        &self,
        ctx: &Context<'_>,
        input: SubmitApplicationInput,
    ) -> Result<VolunteerApplication> {
        let applicant_id = current_user_id(ctx)?;
        let opp_id = OpportunityId(Uuid::parse_str(input.opportunity_id.as_str()).map_err(|_| "Invalid opportunity id")?);
        // Aligned: resolvers consistently fetch Arc<VolunteerServiceImpl> inserted by schema helper.
        let svc = ctx
            .data::<std::sync::Arc<VolunteerServiceImpl>>()
            .map_err(|_| "VolunteerServiceImpl not in context")?;
        let app = svc
            .submit_application(opp_id, applicant_id, input.message)
            .await
            .map_err(map_service_err)?;
        Ok(app.into())
    }

    /// Review an application (approve/reject/etc.)
    async fn review_application(
        &self,
        ctx: &Context<'_>,
        input: ReviewApplicationInput,
    ) -> Result<VolunteerApplication> {
        let reviewer = current_user_id(ctx)?;
        let app_id = ApplicationId(Uuid::parse_str(input.application_id.as_str()).map_err(|_| "Invalid application id")?);
        let svc = ctx
            .data::<std::sync::Arc<VolunteerServiceImpl>>()
            .or_else(|_| ctx.data::<Box<VolunteerServiceImpl>>().map(|b| std::sync::Arc::from(b.clone())))
            .map_err(|_| "VolunteerServiceImpl not in context")?;
        let app = svc
            .review_application(app_id, reviewer, input.status.into())
            .await
            .map_err(map_service_err)?;
        Ok(app.into())
    }

    /// Log a contribution
    async fn log_contribution(
        &self,
        ctx: &Context<'_>,
        input: LogContributionInput,
    ) -> Result<VolunteerContribution> {
        let contributor = current_user_id(ctx)?;
        let opp_id = OpportunityId(Uuid::parse_str(input.opportunity_id.as_str()).map_err(|_| "Invalid opportunity id")?);
        let occurred_at = parse_datetime(&input.occurred_at)?;
        let svc = ctx
            .data::<std::sync::Arc<VolunteerServiceImpl>>()
            .or_else(|_| ctx.data::<Box<VolunteerServiceImpl>>().map(|b| std::sync::Arc::from(b.clone())))
            .map_err(|_| "VolunteerServiceImpl not in context")?;
        let c = svc
            .log_contribution(opp_id, contributor, input.kind.into(), input.amount, input.notes, occurred_at)
            .await
            .map_err(map_service_err)?;
        Ok(c.into())
    }

    /// Verify a contribution (boolean simplified via verification_ref presence)
    async fn verify_contribution(
        &self,
        ctx: &Context<'_>,
        input: VerifyContributionInput,
    ) -> Result<VolunteerContribution> {
        let verifier = current_user_id(ctx)?;
        let c_id = ContributionId(Uuid::parse_str(input.contribution_id.as_str()).map_err(|_| "Invalid contribution id")?);
        let verification_ref = input.verification_ref
            .and_then(|id| Uuid::parse_str(id.as_str()).ok());
        // Aligned: resolvers consistently fetch Arc<VolunteerServiceImpl> inserted by schema helper.
        let svc = ctx
            .data::<std::sync::Arc<VolunteerServiceImpl>>()
            .map_err(|_| "VolunteerServiceImpl not in context")?;
        let c = svc
            .verify_contribution(c_id, verifier, verification_ref)
            .await
            .map_err(map_service_err)?;
        Ok(c.into())
    }
}

/* ===================== Helpers ===================== */

fn parse_opt_datetime(v: &Option<String>) -> Result<Option<DateTime<Utc>>> {
    if let Some(s) = v {
        let dt = parse_datetime(s)?;
        Ok(Some(dt))
    } else {
        Ok(None)
    }
}

fn parse_datetime(s: &str) -> Result<DateTime<Utc>> {
    let dt = DateTime::parse_from_rfc3339(s)
        .map_err(|_| "Invalid datetime format (expected RFC3339)")?
        .with_timezone(&Utc);
    Ok(dt)
}