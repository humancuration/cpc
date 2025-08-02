//! PostgreSQL implementation of the `OpportunityRepository`.

use async_trait::async_trait;
use sqlx::PgPool;
use chrono::Utc;
use sqlx::QueryBuilder;
use uuid::Uuid;

use crate::opportunity_management::{
    models::{OpportunityApplication, VolunteerOpportunity},
    repository::{ListOpportunitiesFilters, OpportunityRepository, OpportunityRepositoryError},
};

/// A PostgreSQL-backed implementation of the `OpportunityRepository`.
#[derive(Clone)]
pub struct PostgresOpportunityRepository {
    pool: PgPool,
}

impl PostgresOpportunityRepository {
    /// Creates a new `PostgresOpportunityRepository`.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OpportunityRepository for PostgresOpportunityRepository {
    async fn create_opportunity(
        &self,
        opportunity: &VolunteerOpportunity,
    ) -> Result<(), OpportunityRepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO volunteer_opportunities (id, cause_id, title, description, required_skills, estimated_hours, created_at, deadline, created_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            opportunity.id,
            opportunity.cause_id,
            opportunity.title,
            opportunity.description,
            &opportunity.required_skills,
            opportunity.estimated_hours,
            opportunity.created_at,
            opportunity.deadline,
            opportunity.created_by
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_opportunity_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<VolunteerOpportunity>, OpportunityRepositoryError> {
        let opportunity = sqlx::query_as!(
            VolunteerOpportunity,
            r#"
            SELECT id, cause_id, title, description, required_skills, estimated_hours, created_at, deadline, created_by
            FROM volunteer_opportunities
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(opportunity)
    }

    async fn list_opportunities(
        &self,
        filters: &ListOpportunitiesFilters,
    ) -> Result<(Vec<VolunteerOpportunity>, i64), OpportunityRepositoryError> {
        let mut query_builder: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
            "SELECT id, cause_id, title, description, required_skills, estimated_hours, created_at, deadline, created_by
             FROM volunteer_opportunities WHERE 1=1 ",
        );

        let mut count_query_builder: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
            "SELECT COUNT(*) FROM volunteer_opportunities WHERE 1=1 ",
        );

        if let Some(cause_id) = filters.cause_id {
            query_builder.push(" AND cause_id = ").push_bind(cause_id);
            count_query_builder.push(" AND cause_id = ").push_bind(cause_id);
        }

        if let Some(skill_id) = filters.skill_id {
            query_builder.push(" AND ").push_bind(skill_id).push(" = ANY(required_skills)");
            count_query_builder.push(" AND ").push_bind(skill_id).push(" = ANY(required_skills)");
        }

        if filters.only_open {
            query_builder.push(" AND deadline > ").push_bind(Utc::now());
            count_query_builder.push(" AND deadline > ").push_bind(Utc::now());
        }

        // Count total records
        let total_count: (i64,) = count_query_builder
            .build_tuple()
            .fetch_one(&self.pool)
            .await?;

        // Add pagination
        query_builder
            .push(" ORDER BY created_at DESC ")
            .push(" LIMIT ")
            .push_bind(filters.limit)
            .push(" OFFSET ")
            .push_bind(filters.offset);

        let opportunities: Vec<VolunteerOpportunity> = query_builder
            .build_query_as()
            .fetch_all(&self.pool)
            .await?;

        Ok((opportunities, total_count.0))
    }

    async fn update_opportunity(
        &self,
        opportunity: &VolunteerOpportunity,
    ) -> Result<VolunteerOpportunity, OpportunityRepositoryError> {
        let updated_opportunity = sqlx::query_as!(
            VolunteerOpportunity,
            r#"
            UPDATE volunteer_opportunities
            SET
                title = $2,
                description = $3,
                required_skills = $4,
                estimated_hours = $5,
                deadline = $6
            WHERE id = $1
            RETURNING id, cause_id, title, description, required_skills, estimated_hours, created_at, deadline, created_by
            "#,
            opportunity.id,
            opportunity.title,
            opportunity.description,
            &opportunity.required_skills,
            opportunity.estimated_hours,
            opportunity.deadline,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated_opportunity)
    }

    async fn update_application(
        &self,
        application: &OpportunityApplication,
    ) -> Result<OpportunityApplication, OpportunityRepositoryError> {
        let updated_application = sqlx::query_as!(
            OpportunityApplication,
            r#"
            UPDATE opportunity_applications
            SET
                status = $2,
                volunteer_hours = $3
            WHERE id = $1
            RETURNING id, opportunity_id, user_id, applied_at, status AS "status: _", volunteer_hours
            "#,
            application.id,
            application.status as _,
            application.volunteer_hours
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated_application)
    }

    async fn create_application(
        &self,
        user_id: Uuid,
        opportunity_id: Uuid,
    ) -> Result<OpportunityApplication, OpportunityRepositoryError> {
        let application = OpportunityApplication {
            id: Uuid::new_v4(),
            opportunity_id,
            user_id,
            applied_at: Utc::now(),
            status: crate::opportunity_management::models::ApplicationStatus::Pending,
            volunteer_hours: None,
        };

        sqlx::query!(
            r#"
            INSERT INTO opportunity_applications (id, opportunity_id, user_id, applied_at, status, volunteer_hours)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            application.id,
            application.opportunity_id,
            application.user_id,
            application.applied_at,
            application.status as _,
            application.volunteer_hours
        )
        .execute(&self.pool)
        .await?;

        Ok(application)
    }

    async fn find_application_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<OpportunityApplication>, OpportunityRepositoryError> {
        let application = sqlx::query_as!(
            OpportunityApplication,
            r#"
            SELECT id, opportunity_id, user_id, applied_at, status AS "status: _", volunteer_hours
            FROM opportunity_applications
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(application)
    }

    async fn delete_opportunity(&self, id: Uuid) -> Result<(), OpportunityRepositoryError> {
        let result = sqlx::query!(
            "DELETE FROM volunteer_opportunities WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(OpportunityRepositoryError::NotFound);
        }

        Ok(())
    }

    async fn list_user_applications(
        &self,
        filters: &ListUserApplicationsFilters,
    ) -> Result<(Vec<OpportunityApplication>, i64), OpportunityRepositoryError> {
        let mut query_builder: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
            r#"SELECT id, opportunity_id, user_id, applied_at, status AS "status: _", volunteer_hours
               FROM opportunity_applications WHERE user_id = "#,
        );
        query_builder.push_bind(filters.user_id);

        let mut count_query_builder: QueryBuilder<sqlx::Postgres> =
            QueryBuilder::new("SELECT COUNT(*) FROM opportunity_applications WHERE user_id = ");
        count_query_builder.push_bind(filters.user_id);

        if let Some(status) = &filters.status {
            query_builder.push(" AND status = ").push_bind(status);
            count_query_builder.push(" AND status = ").push_bind(status);
        }

        // Count total records
        let total_count: (i64,) = count_query_builder
            .build_tuple()
            .fetch_one(&self.pool)
            .await?;

        // Add pagination
        query_builder
            .push(" ORDER BY applied_at DESC ")
            .push(" LIMIT ")
            .push_bind(filters.limit)
            .push(" OFFSET ")
            .push_bind(filters.offset);

        let applications: Vec<OpportunityApplication> = query_builder
            .build_query_as()
            .fetch_all(&self.pool)
            .await?;

        Ok((applications, total_count.0))
    }
}