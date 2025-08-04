#![allow(clippy::unused_async)]
//! PostgreSQL repository implementations for Volunteer Coordination
//!
//! Implements OpportunityRepository, ApplicationRepository, ContributionRepository
//! using SQLx. Emits volunteer events through SocialEventBus.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::models::{
    ApplicationId, ApplicationStatus, ContributionId, ContributionKind, OpportunityId,
    OpportunityStatus, VolunteerApplication, VolunteerContribution, VolunteerOpportunity,
};
use crate::domain::repository::{
    ApplicationRepository, ContributionRepository, OpportunityRepository, VolunteerRepositoryError,
};
use crate::infrastructure::event_bus_integration::{SocialEventBus, VolunteerEvents};

fn db_err(e: impl ToString) -> VolunteerRepositoryError {
    VolunteerRepositoryError::DatabaseError(e.to_string())
}

pub struct PostgresOpportunityRepository {
    pub pool: PgPool,
    pub event_bus: Option<SocialEventBus>,
}

impl PostgresOpportunityRepository {
    pub fn new(pool: PgPool, event_bus: Option<SocialEventBus>) -> Self {
        Self { pool, event_bus }
    }
}

#[async_trait]
impl OpportunityRepository for PostgresOpportunityRepository {
    async fn insert(&self, o: &VolunteerOpportunity) -> Result<(), VolunteerRepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO volunteer_opportunities
                (id, org_id, created_by, title, description, tags, status, location, starts_at, ends_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
            o.id.0,
            o.org_id,
            o.created_by,
            o.title,
            o.description,
            &o.tags[..],
            // store status as smallint/int
            match o.status {
                OpportunityStatus::Draft => 0i16,
                OpportunityStatus::Published => 1i16,
                OpportunityStatus::Closed => 2i16,
                OpportunityStatus::Archived => 3i16,
            },
            o.location,
            o.starts_at,
            o.ends_at,
            o.created_at,
            o.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(db_err)?;

        if matches!(o.status, OpportunityStatus::Published) {
            if let Some(bus) = &self.event_bus {
                let _ = bus
                    .publish_event(VolunteerEvents::opportunity_created(
                        o.id.0,
                        o.org_id,
                        o.created_by,
                    ))
                    .await;
            }
        }

        Ok(())
    }

    async fn update_status(
        &self,
        id: OpportunityId,
        status: OpportunityStatus,
    ) -> Result<(), VolunteerRepositoryError> {
        sqlx::query!(
            r#"
            UPDATE volunteer_opportunities
            SET status = $2, updated_at = NOW()
            WHERE id = $1
            "#,
            id.0,
            match status {
                OpportunityStatus::Draft => 0i16,
                OpportunityStatus::Published => 1i16,
                OpportunityStatus::Closed => 2i16,
                OpportunityStatus::Archived => 3i16,
            }
        )
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn get(
        &self,
        id: OpportunityId,
    ) -> Result<Option<VolunteerOpportunity>, VolunteerRepositoryError> {
        let row = sqlx::query!(
            r#"
            SELECT id, org_id, created_by, title, description, tags, status, location, starts_at, ends_at, created_at, updated_at
            FROM volunteer_opportunities
            WHERE id = $1
            "#,
            id.0
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(row.map(|r| VolunteerOpportunity {
            id: OpportunityId(r.id),
            org_id: r.org_id,
            created_by: r.created_by,
            title: r.title,
            description: r.description,
            tags: r.tags.unwrap_or_default(),
            status: match r.status {
                0 => OpportunityStatus::Draft,
                1 => OpportunityStatus::Published,
                2 => OpportunityStatus::Closed,
                _ => OpportunityStatus::Archived,
            },
            location: r.location,
            starts_at: r.starts_at.map(|d| DateTime::from(d)),
            ends_at: r.ends_at.map(|d| DateTime::from(d)),
            created_at: DateTime::from(r.created_at),
            updated_at: DateTime::from(r.updated_at),
        }))
    }
}

pub struct PostgresApplicationRepository {
    pub pool: PgPool,
    pub event_bus: Option<SocialEventBus>,
}

impl PostgresApplicationRepository {
    pub fn new(pool: PgPool, event_bus: Option<SocialEventBus>) -> Self {
        Self { pool, event_bus }
    }
}

#[async_trait]
impl ApplicationRepository for PostgresApplicationRepository {
    async fn insert(&self, a: &VolunteerApplication) -> Result<(), VolunteerRepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO volunteer_applications
                (id, opportunity_id, applicant_id, motivation, status, submitted_at, decided_at, reviewer_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            a.id.0,
            a.opportunity_id.0,
            a.applicant_id,
            a.motivation,
            match a.status {
                ApplicationStatus::Submitted => 0i16,
                ApplicationStatus::UnderReview => 1i16,
                ApplicationStatus::Accepted => 2i16,
                ApplicationStatus::Rejected => 3i16,
                ApplicationStatus::Withdrawn => 4i16,
            },
            a.submitted_at,
            a.decided_at,
            a.reviewer_id
        )
        .execute(&self.pool)
        .await
        .map_err(db_err)?;

        if let Some(bus) = &self.event_bus {
            let _ = bus
                .publish_event(VolunteerEvents::application_submitted(
                    a.id.0,
                    a.opportunity_id.0,
                    a.applicant_id,
                ))
                .await;
        }

        Ok(())
    }

    async fn update_status(
        &self,
        id: ApplicationId,
        status: ApplicationStatus,
        reviewer_id: Option<Uuid>,
        decided_at: Option<DateTime<Utc>>,
    ) -> Result<(), VolunteerRepositoryError> {
        sqlx::query!(
            r#"
            UPDATE volunteer_applications
            SET status = $2, reviewer_id = $3, decided_at = $4
            WHERE id = $1
            "#,
            id.0,
            match status {
                ApplicationStatus::Submitted => 0i16,
                ApplicationStatus::UnderReview => 1i16,
                ApplicationStatus::Accepted => 2i16,
                ApplicationStatus::Rejected => 3i16,
                ApplicationStatus::Withdrawn => 4i16,
            },
            reviewer_id,
            decided_at
        )
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn get(
        &self,
        id: ApplicationId,
    ) -> Result<Option<VolunteerApplication>, VolunteerRepositoryError> {
        let row = sqlx::query!(
            r#"
            SELECT id, opportunity_id, applicant_id, motivation, status, submitted_at, decided_at, reviewer_id
            FROM volunteer_applications
            WHERE id = $1
            "#,
            id.0
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(row.map(|r| VolunteerApplication {
            id: ApplicationId(r.id),
            opportunity_id: OpportunityId(r.opportunity_id),
            applicant_id: r.applicant_id,
            motivation: r.motivation,
            status: match r.status {
                0 => ApplicationStatus::Submitted,
                1 => ApplicationStatus::UnderReview,
                2 => ApplicationStatus::Accepted,
                3 => ApplicationStatus::Rejected,
                _ => ApplicationStatus::Withdrawn,
            },
            submitted_at: DateTime::from(r.submitted_at),
            decided_at: r.decided_at.map(|d| DateTime::from(d)),
            reviewer_id: r.reviewer_id,
        }))
    }
}

pub struct PostgresContributionRepository {
    pub pool: PgPool,
    pub event_bus: Option<SocialEventBus>,
}

impl PostgresContributionRepository {
    pub fn new(pool: PgPool, event_bus: Option<SocialEventBus>) -> Self {
        Self { pool, event_bus }
    }
}

#[async_trait]
impl ContributionRepository for PostgresContributionRepository {
    async fn insert(&self, c: &VolunteerContribution) -> Result<(), VolunteerRepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO volunteer_contributions
                (id, opportunity_id, contributor_id, kind, amount, notes, occurred_at, created_at, verified, verification_ref)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            c.id.0,
            c.opportunity_id.0,
            c.contributor_id,
            match c.kind {
                ContributionKind::Hours => 0i16,
                ContributionKind::Deliverable => 1i16,
                ContributionKind::Donation => 2i16,
                ContributionKind::Other => 3i16,
            },
            c.amount,
            c.notes,
            c.occurred_at,
            c.created_at,
            c.verified,
            c.verification_ref
        )
        .execute(&self.pool)
        .await
        .map_err(db_err)?;

        if let Some(bus) = &self.event_bus {
            let _ = bus
                .publish_event(VolunteerEvents::contribution_logged(
                    c.id.0,
                    c.opportunity_id.0,
                    c.contributor_id,
                ))
                .await;
        }

        Ok(())
    }

    async fn verify(
        &self,
        id: ContributionId,
        verified: bool,
        verification_ref: Option<Uuid>,
    ) -> Result<(), VolunteerRepositoryError> {
        sqlx::query!(
            r#"
            UPDATE volunteer_contributions
            SET verified = $2, verification_ref = $3
            WHERE id = $1
            "#,
            id.0,
            verified,
            verification_ref
        )
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    async fn get(
        &self,
        id: ContributionId,
    ) -> Result<Option<VolunteerContribution>, VolunteerRepositoryError> {
        let row = sqlx::query!(
            r#"
            SELECT id, opportunity_id, contributor_id, kind, amount, notes, occurred_at, created_at, verified, verification_ref
            FROM volunteer_contributions
            WHERE id = $1
            "#,
            id.0
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(row.map(|r| VolunteerContribution {
            id: ContributionId(r.id),
            opportunity_id: OpportunityId(r.opportunity_id),
            contributor_id: r.contributor_id,
            kind: match r.kind {
                0 => ContributionKind::Hours,
                1 => ContributionKind::Deliverable,
                2 => ContributionKind::Donation,
                _ => ContributionKind::Other,
            },
            amount: r.amount,
            notes: r.notes,
            occurred_at: DateTime::from(r.occurred_at),
            created_at: DateTime::from(r.created_at),
            verified: r.verified,
            verification_ref: r.verification_ref,
        }))
    }
}