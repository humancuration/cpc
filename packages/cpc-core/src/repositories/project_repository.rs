//! Manages projects in the database.

use crate::business::project::{Project, ProjectStatus, UpdateProject};
use sqlx::PgPool;
use uuid::Uuid;

/// A repository for managing projects in the database.
#[derive(Debug, Clone)]
pub struct ProjectRepository {
    db_pool: PgPool,
}

impl ProjectRepository {
    /// Creates a new `ProjectRepository`.
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    /// Creates a new project in the database.
    pub async fn create(
        &self,
        name: &str,
        description: Option<&str>,
        cooperative_id: Uuid,
    ) -> Result<Project, sqlx::Error> {
        let project = sqlx::query_as!(
            Project,
            r#"
            INSERT INTO projects (name, description, cooperative_id, id, status)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, description, cooperative_id, status, start_date, end_date, created_at, updated_at
            "#,
            name,
            description,
            cooperative_id,
            Uuid::new_v4(), // Generate new UUID for the project
            ProjectStatus::NotStarted as ProjectStatus
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(project)
    }

    /// Retrieves a project by its ID.
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Project>, sqlx::Error> {
        let project = sqlx::query_as!(
            Project,
            r#"
            SELECT id, name, description, cooperative_id, status, start_date, end_date, created_at, updated_at 
            FROM projects WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(project)
    }

    /// Lists all projects for a given cooperative.
    pub async fn list_by_cooperative_id(
        &self,
        cooperative_id: Uuid,
    ) -> Result<Vec<Project>, sqlx::Error> {
        let projects = sqlx::query_as!(
            Project,
            r#"
            SELECT id, name, description, cooperative_id, status, start_date, end_date, created_at, updated_at 
            FROM projects 
            WHERE cooperative_id = $1
            ORDER BY created_at DESC
            "#,
            cooperative_id
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(projects)
    }

    /// Lists all projects.
    pub async fn list_all(&self) -> Result<Vec<Project>, sqlx::Error> {
        let projects = sqlx::query_as!(
            Project,
            r#"
            SELECT id, name, description, cooperative_id, status, start_date, end_date, created_at, updated_at
            FROM projects
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(projects)
    }

    /// Updates the status of a project.
    pub async fn update_status(
        &self,
        id: Uuid,
        status: ProjectStatus,
    ) -> Result<Project, sqlx::Error> {
        let project = sqlx::query_as!(
            Project,
            r#"
            UPDATE projects
            SET status = $1, updated_at = now()
            WHERE id = $2
            RETURNING id, name, description, cooperative_id, status, start_date, end_date, created_at, updated_at
            "#,
            status as ProjectStatus,
            id
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(project)
    }

    /// Updates a project's details.
    pub async fn update(&self, id: Uuid, payload: UpdateProject) -> Result<Project, sqlx::Error> {
        let project = sqlx::query_as!(
            Project,
            r#"
            UPDATE projects
            SET
                name = COALESCE($1, name),
                description = COALESCE($2, description),
                updated_at = now()
            WHERE id = $3
            RETURNING id, name, description, cooperative_id, status, start_date, end_date, created_at, updated_at
            "#,
            payload.name,
            payload.description,
            id
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(project)
    }
}