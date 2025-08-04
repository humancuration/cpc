//! GraphQL mutations and queries for Collaborative Workspace
//!
//! Follows the style of social_interactions GraphQL implementation.
//! Provides mutations and queries for documents, project boards, files, and meetings.

use async_graphql::{Context, Object, Result, SimpleObject, ID};
use uuid::Uuid;
use shared_packages::collaborative_workspace::domain::models::{
    CollaborativeDocument, FileVersion, MeetingRoom, ProjectBoard, ProjectColumn, ProjectTask,
};
use shared_packages::collaborative_workspace::domain::service::{
    DocumentService, FileService, MeetingService, ProjectService, ServiceError,
};

/// GraphQL representation of a Collaborative Document
#[derive(SimpleObject, Clone)]
pub struct DocumentDto {
    /// Unique identifier
    pub id: ID,
    /// Title of the document
    pub title: String,
    /// Creator user ID
    pub created_by: ID,
    /// ISO timestamp created_at
    pub created_at: String,
    /// ISO timestamp updated_at
    pub updated_at: String,
}

impl From<CollaborativeDocument> for DocumentDto {
    fn from(d: CollaborativeDocument) -> Self {
        Self {
            id: d.id.to_string().into(),
            title: d.title,
            created_by: d.created_by.to_string().into(),
            created_at: d.created_at.to_rfc3339(),
            updated_at: d.updated_at.to_rfc3339(),
        }
    }
}

/// GraphQL representation of a Project Board
#[derive(SimpleObject, Clone)]
pub struct ProjectBoardDto {
    pub id: ID,
    pub title: String,
    pub owner_id: ID,
    pub created_at: String,
}

impl From<ProjectBoard> for ProjectBoardDto {
    fn from(b: ProjectBoard) -> Self {
        Self {
            id: b.id.to_string().into(),
            title: b.title,
            owner_id: b.owner_id.to_string().into(),
            created_at: b.created_at.to_rfc3339(),
        }
    }
}

/// GraphQL representation of a Project Column
#[derive(SimpleObject, Clone)]
pub struct ProjectColumnDto {
    pub id: ID,
    pub board_id: ID,
    pub title: String,
    pub position: i32,
}

impl From<ProjectColumn> for ProjectColumnDto {
    fn from(c: ProjectColumn) -> Self {
        Self {
            id: c.id.to_string().into(),
            board_id: c.board_id.to_string().into(),
            title: c.title,
            position: c.position,
        }
    }
}

/// GraphQL representation of a Project Task
#[derive(SimpleObject, Clone)]
pub struct ProjectTaskDto {
    pub id: ID,
    pub column_id: ID,
    pub title: String,
    pub description: Option<String>,
    pub position: i32,
    pub created_at: String,
    pub updated_at: Option<String>,
}

impl From<ProjectTask> for ProjectTaskDto {
    fn from(t: ProjectTask) -> Self {
        Self {
            id: t.id.to_string().into(),
            column_id: t.column_id.to_string().into(),
            title: t.title,
            description: t.description,
            position: t.position,
            created_at: t.created_at.to_rfc3339(),
            updated_at: t.updated_at.map(|dt| dt.to_rfc3339()),
        }
    }
}

/// GraphQL representation of a File Version
#[derive(SimpleObject, Clone)]
pub struct FileVersionDto {
    pub id: ID,
    pub file_id: ID,
    pub version: i32,
    pub created_by: ID,
    pub created_at: String,
}

impl From<FileVersion> for FileVersionDto {
    fn from(v: FileVersion) -> Self {
        Self {
            id: v.id.to_string().into(),
            file_id: v.file_id.to_string().into(),
            version: v.version,
            created_by: v.created_by.to_string().into(),
            created_at: v.created_at.to_rfc3339(),
        }
    }
}

/// GraphQL representation of a Meeting Room
#[derive(SimpleObject, Clone)]
pub struct MeetingRoomDto {
    pub id: ID,
    pub title: String,
    pub owner_id: ID,
    pub created_at: String,
    pub ended_at: Option<String>,
}

impl From<MeetingRoom> for MeetingRoomDto {
    fn from(m: MeetingRoom) -> Self {
        Self {
            id: m.id.to_string().into(),
            title: m.title,
            owner_id: m.owner_id.to_string().into(),
            created_at: m.created_at.to_rfc3339(),
            ended_at: m.ended_at.map(|dt| dt.to_rfc3339()),
        }
    }
}

/// Helper: get current authenticated user id from context
fn current_user_id(ctx: &Context<'_>) -> Result<Uuid> {
    // Mirrors social_interactions: ctx.data::<Uuid>()
    let user_id = ctx
        .data::<Uuid>()
        .map_err(|_| "Unauthorized: missing user id in context")?;
    Ok(*user_id)
}

/// Helper: map ServiceError to GraphQL Result error
fn map_service_err(e: ServiceError) -> async_graphql::Error {
    async_graphql::Error::new(e.to_string())
}

/// Collaborative Workspace Mutations
#[derive(Default)]
pub struct CollaborativeWorkspaceMutations;

#[Object]
impl CollaborativeWorkspaceMutations {
    /// Create a new collaborative document
    async fn create_document(
        &self,
        ctx: &Context<'_>,
        title: String,
    ) -> Result<DocumentDto> {
        let user_id = current_user_id(ctx)?;
        let svc = ctx
            .data::<Box<dyn DocumentService>>()
            .or_else(|_| ctx.data::<std::sync::Arc<dyn DocumentService>>().map(|a| a as _))
            .map_err(|_| "DocumentService not in context")?;

        let doc = svc
            .create_document(title, user_id)
            .await
            .map_err(map_service_err)?;
        Ok(doc.into())
    }

    /// Apply an operation to a collaborative document (CRDT / OT)
    async fn apply_document_operation(
        &self,
        ctx: &Context<'_>,
        document_id: ID,
        operation: String,
    ) -> Result<DocumentDto> {
        let user_id = current_user_id(ctx)?;
        let doc_id = Uuid::parse_str(document_id.as_str()).map_err(|_| "Invalid document ID")?;
        let svc = ctx
            .data::<Box<dyn DocumentService>>()
            .or_else(|_| ctx.data::<std::sync::Arc<dyn DocumentService>>().map(|a| a as _))
            .map_err(|_| "DocumentService not in context")?;

        let doc = svc
            .apply_operation(doc_id, operation, user_id)
            .await
            .map_err(map_service_err)?;
        Ok(doc.into())
    }

    /// Create a project board
    async fn create_project_board(
        &self,
        ctx: &Context<'_>,
        title: String,
    ) -> Result<ProjectBoardDto> {
        let user_id = current_user_id(ctx)?;
        let svc = ctx
            .data::<Box<dyn ProjectService>>()
            .or_else(|_| ctx.data::<std::sync::Arc<dyn ProjectService>>().map(|a| a as _))
            .map_err(|_| "ProjectService not in context")?;

        let board = svc
            .create_board(title, user_id)
            .await
            .map_err(map_service_err)?;
        Ok(board.into())
    }

    /// Add a column to a project board
    async fn add_project_column(
        &self,
        ctx: &Context<'_>,
        board_id: ID,
        title: String,
        position: i32,
    ) -> Result<ProjectColumnDto> {
        let _user_id = current_user_id(ctx)?; // authorization can be enforced in service
        let board_uuid = Uuid::parse_str(board_id.as_str()).map_err(|_| "Invalid board ID")?;
        let svc = ctx
            .data::<Box<dyn ProjectService>>()
            .or_else(|_| ctx.data::<std::sync::Arc<dyn ProjectService>>().map(|a| a as _))
            .map_err(|_| "ProjectService not in context")?;

        let col = svc
            .add_column(board_uuid, title, position)
            .await
            .map_err(map_service_err)?;
        Ok(col.into())
    }

    /// Move a task to another column and/or position
    async fn move_task(
        &self,
        ctx: &Context<'_>,
        task_id: ID,
        new_column_id: ID,
        position: i32,
    ) -> Result<ProjectTaskDto> {
        let _user_id = current_user_id(ctx)?;
        let task_uuid = Uuid::parse_str(task_id.as_str()).map_err(|_| "Invalid task ID")?;
        let col_uuid = Uuid::parse_str(new_column_id.as_str()).map_err(|_| "Invalid column ID")?;
        let svc = ctx
            .data::<Box<dyn ProjectService>>()
            .or_else(|_| ctx.data::<std::sync::Arc<dyn ProjectService>>().map(|a| a as _))
            .map_err(|_| "ProjectService not in context")?;

        let task = svc
            .move_task(task_uuid, col_uuid, position)
            .await
            .map_err(map_service_err)?;
        Ok(task.into())
    }

    /// Create a new file version entry for a file
    async fn create_file_version(
        &self,
        ctx: &Context<'_>,
        file_id: ID,
        _content: String,
    ) -> Result<FileVersionDto> {
        // ADR notes content may be stored externally; FileService trait currently does not take content.
        let user_id = current_user_id(ctx)?;
        let file_uuid = Uuid::parse_str(file_id.as_str()).map_err(|_| "Invalid file ID")?;
        let svc = ctx
            .data::<Box<dyn FileService>>()
            .or_else(|_| ctx.data::<std::sync::Arc<dyn FileService>>().map(|a| a as _))
            .map_err(|_| "FileService not in context")?;

        let version = svc
            .create_version(file_uuid, user_id)
            .await
            .map_err(map_service_err)?;
        Ok(version.into())
    }

    /// Create a meeting room
    async fn create_meeting(
        &self,
        ctx: &Context<'_>,
        title: String,
    ) -> Result<MeetingRoomDto> {
        let user_id = current_user_id(ctx)?;
        let svc = ctx
            .data::<Box<dyn MeetingService>>()
            .or_else(|_| ctx.data::<std::sync::Arc<dyn MeetingService>>().map(|a| a as _))
            .map_err(|_| "MeetingService not in context")?;

        let room = svc
            .create_meeting(title, user_id)
            .await
            .map_err(map_service_err)?;
        Ok(room.into())
    }
}

/// Collaborative Workspace Queries
#[derive(Default)]
pub struct CollaborativeWorkspaceQueries;

#[Object]
impl CollaborativeWorkspaceQueries {
    /// Get a collaborative document by ID
    async fn get_document(&self, ctx: &Context<'_>, id: ID) -> Result<DocumentDto> {
        let _user_id = current_user_id(ctx)?;
        let doc_id = Uuid::parse_str(id.as_str()).map_err(|_| "Invalid document ID")?;
        let svc = ctx
            .data::<Box<dyn DocumentService>>()
            .or_else(|_| ctx.data::<std::sync::Arc<dyn DocumentService>>().map(|a| a as _))
            .map_err(|_| "DocumentService not in context")?;

        let doc = svc
            .get_document(doc_id)
            .await
            .map_err(map_service_err)?
            .ok_or_else(|| async_graphql::Error::new("Document not found"))?;
        Ok(doc.into())
    }

    /// Get a project board by ID
    async fn get_project_board(&self, ctx: &Context<'_>, id: ID) -> Result<ProjectBoardDto> {
        let _user_id = current_user_id(ctx)?;
        let board_id = Uuid::parse_str(id.as_str()).map_err(|_| "Invalid board ID")?;
        let svc = ctx
            .data::<Box<dyn ProjectService>>()
            .or_else(|_| ctx.data::<std::sync::Arc<dyn ProjectService>>().map(|a| a as _))
            .map_err(|_| "ProjectService not in context")?;

        let board = svc
            .get_board(board_id)
            .await
            .map_err(map_service_err)?
            .ok_or_else(|| async_graphql::Error::new("Project board not found"))?;
        Ok(board.into())
    }
}

// Notes for integration:
// - Add .data(...) for each of the services into the async-graphql Schema builder.
// - Ensure a Uuid user id is inserted into Context (auth middleware), similar to social_interactions.