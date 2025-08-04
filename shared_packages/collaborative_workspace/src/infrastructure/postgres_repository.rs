#![allow(clippy::unused_async)]
//! PostgreSQL repository implementations for collaborative workspace
//!
//! Implements DocumentRepository, ProjectRepository, FileRepository, MeetingRepository
//! using SQLx against the schema defined in ADR-0008.

use crate::domain::models::{
    CollaborativeDocument, FileVersion, MeetingRoom, ProjectBoard, ProjectColumn, ProjectTask,
};
use crate::domain::repository::{
    DocumentRepository, FileRepository, MeetingRepository, ProjectRepository, RepositoryError,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::infrastructure::event_bus::{CollaborationEvents, SocialEventBus};

fn db_err(e: impl ToString) -> RepositoryError {
    RepositoryError::DatabaseError(e.to_string())
}

/// PostgreSQL implementation of DocumentRepository
pub struct PostgresDocumentRepository {
    pool: PgPool,
    event_bus: Option<SocialEventBus>,
}

impl PostgresDocumentRepository {
    /// Create a new PostgresDocumentRepository
    pub fn new(pool: PgPool, event_bus: Option<SocialEventBus>) -> Self {
        Self { pool, event_bus }
    }
}

#[async_trait]
impl DocumentRepository for PostgresDocumentRepository {
    /// Fetch document metadata by ID (document contents/CRDT state are stored in DB but not exposed at domain level)
    async fn get_by_id(&self, document_id: Uuid) -> Result<Option<CollaborativeDocument>, RepositoryError> {
        let row = sqlx::query!(
            r#"
            SELECT id, title, created_by, created_at, updated_at
            FROM collaborative_documents
            WHERE id = $1
            "#,
            document_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(row.map(|r| CollaborativeDocument {
            id: r.id,
            title: r.title,
            created_by: r.created_by,
            created_at: DateTime::<Utc>::from(r.created_at),
            updated_at: DateTime::<Utc>::from(r.updated_at),
        }))
    }

    /// Upsert document row. For now stores an empty CRDT state to satisfy NOT NULL.
    async fn save(&self, doc: &CollaborativeDocument) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO collaborative_documents (id, title, created_by, created_at, updated_at, current_state)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE
            SET title = EXCLUDED.title,
                updated_at = EXCLUDED.updated_at
            "#,
            doc.id,
            doc.title,
            doc.created_by,
            doc.created_at,
            doc.updated_at,
            // Placeholder CRDT state storage; services should handle real state serialization.
            &[] as &[u8]
        )
        .execute(&self.pool)
        .await
        .map_err(db_err)?;

        if let Some(bus) = &self.event_bus {
            let _ = bus
                .publish_event(CollaborationEvents::document_updated(doc.id, doc.created_by))
                .await;
        }

        Ok(())
    }

    /// Delete document by ID
    async fn delete(&self, document_id: Uuid) -> Result<(), RepositoryError> {
        let result = sqlx::query!(
            r#"DELETE FROM collaborative_documents WHERE id = $1"#,
            document_id
        )
        .execute(&self.pool)
        .await
        .map_err(db_err)?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }
}

/// PostgreSQL implementation of ProjectRepository
pub struct PostgresProjectRepository {
    pool: PgPool,
    event_bus: Option<SocialEventBus>,
}

impl PostgresProjectRepository {
    /// Create a new PostgresProjectRepository
    pub fn new(pool: PgPool, event_bus: Option<SocialEventBus>) -> Self {
        Self { pool, event_bus }
    }
}

#[async_trait]
impl ProjectRepository for PostgresProjectRepository {
    /// Create a project board
    async fn create_board(&self, board: &ProjectBoard) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO project_boards (id, title, owner_id, created_at)
            VALUES ($1, $2, $3, $4)
            "#,
            board.id,
            board.title,
            board.owner_id,
            board.created_at
        )
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    /// Fetch board by ID
    async fn get_board_by_id(&self, board_id: Uuid) -> Result<Option<ProjectBoard>, RepositoryError> {
        let row = sqlx::query!(
            r#"
            SELECT id, title, owner_id, created_at
            FROM project_boards
            WHERE id = $1
            "#,
            board_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(db_err)?;

        Ok(row.map(|r| ProjectBoard {
            id: r.id,
            title: r.title,
            owner_id: r.owner_id,
            created_at: DateTime::<Utc>::from(r.created_at),
        }))
    }

    /// Add a column to a board
    async fn add_column(&self, column: &ProjectColumn) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO project_columns (id, board_id, title, position)
            VALUES ($1, $2, $3, $4)
            "#,
            column.id,
            column.board_id,
            column.title,
            column.position
        )
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    /// Add a task to a column
    async fn add_task(&self, task: &ProjectTask) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO project_tasks (id, column_id, title, description, position, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            task.id,
            task.column_id,
            task.title,
            task.description,
            task.position,
            task.created_at,
            task.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    /// Move a task to a new column/position and publish TaskMoved event
    async fn move_task(&self, task_id: Uuid, new_column_id: Uuid, position: i32) -> Result<(), RepositoryError> {
        let result = sqlx::query!(
            r#"
            UPDATE project_tasks
            SET column_id = $1, position = $2, updated_at = NOW()
            WHERE id = $3
            "#,
            new_column_id,
            position,
            task_id
        )
        .execute(&self.pool)
        .await
        .map_err(db_err)?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound);
        }

        if let Some(bus) = &self.event_bus {
            let _ = bus
                .publish_event(CollaborationEvents::task_moved(task_id, new_column_id))
                .await;
        }

        Ok(())
    }
}

/// PostgreSQL implementation of FileRepository
pub struct PostgresFileRepository {
    pool: PgPool,
}

impl PostgresFileRepository {
    /// Create a new PostgresFileRepository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FileRepository for PostgresFileRepository {
    /// Insert a new file version (content bytes are stored in DB; domain exposes metadata)
    async fn create_version(&self, version: &FileVersion) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO file_versions (id, file_id, version, content, created_by, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            version.id,
            version.file_id,
            version.version,
            // Placeholder: content should be provided by the service. Using empty bytes to satisfy NOT NULL.
            &[] as &[u8],
            version.created_by,
            version.created_at
        )
        .execute(&self.pool)
        .await
        .map_err(db_err)?;
        Ok(())
    }

    /// Get all versions for a file ordered by version
    async fn get_versions(&self, file_id: Uuid) -> Result<Vec<FileVersion>, RepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, file_id, version, created_by, created_at
            FROM file_versions
            WHERE file_id = $1
            ORDER BY version ASC
            "#,
            file_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        let versions = rows
            .into_iter()
            .map(|r| FileVersion {
                id: r.id,
                file_id: r.file_id,
                version: r.version,
                created_by: r.created_by,
                created_at: DateTime::<Utc>::from(r.created_at),
            })
            .collect();

        Ok(versions)
    }
}

/// PostgreSQL implementation of MeetingRepository
pub struct PostgresMeetingRepository {
    pool: PgPool,
    event_bus: Option<SocialEventBus>,
}

impl PostgresMeetingRepository {
    /// Create a new PostgresMeetingRepository
    pub fn new(pool: PgPool, event_bus: Option<SocialEventBus>) -> Self {
        Self { pool, event_bus }
    }
}

#[async_trait]
impl MeetingRepository for PostgresMeetingRepository {
    /// Insert a new meeting room and publish MeetingStarted event
    async fn create_room(&self, room: &MeetingRoom) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO meeting_rooms (id, title, owner_id, created_at, ended_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            room.id,
            room.title,
            room.owner_id,
            room.created_at,
            room.ended_at
        )
        .execute(&self.pool)
        .await
        .map_err(db_err)?;

        if let Some(bus) = &self.event_bus {
            let _ = bus
                .publish_event(CollaborationEvents::meeting_started(room.id))
                .await;
        }

        Ok(())
    }

    /// Mark meeting ended by setting ended_at
    async fn end_meeting(&self, room_id: Uuid) -> Result<(), RepositoryError> {
        let result = sqlx::query!(
            r#"
            UPDATE meeting_rooms
            SET ended_at = NOW()
            WHERE id = $1 AND ended_at IS NULL
            "#,
            room_id
        )
        .execute(&self.pool)
        .await
        .map_err(db_err)?;

        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }
}