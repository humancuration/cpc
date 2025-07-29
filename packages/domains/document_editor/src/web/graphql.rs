//! GraphQL implementation for the document editor module

use async_graphql::{
    Context, Object, Result, SimpleObject, Subscription,
};
use futures_util::stream::Stream;
use std::sync::Arc;
use uuid::Uuid;

use crate::application::document_service::DocumentService;
use crate::application::export_service::ExportService;
use crate::application::collaboration_service::CollaborationService;
use crate::domain::errors::DocumentError;
use crate::web::types::*;

// Conversion functions
fn convert_document_to_output(document: crate::domain::models::Document) -> DocumentOutput {
    DocumentOutput {
        id: document.id,
        owner_id: document.owner_id,
        title: document.title.as_str().to_string(),
        content: serde_json::to_string(document.content.as_json()).unwrap_or_else(|_| "{}".to_string()),
        created_at: document.created_at,
        updated_at: document.updated_at,
        is_deleted: document.is_deleted,
    }
}

// GraphQL Query Root
#[derive(Default)]
pub struct DocumentEditorQuery;

#[Object]
impl DocumentEditorQuery {
    async fn document(&self, ctx: &Context<'_>, id: Uuid) -> Result<DocumentOutput> {
        let document_service = ctx.data_unchecked::<Arc<DocumentService>>();
        let user_id = ctx.data_unchecked::<Uuid>(); // Get from auth middleware

        let document = document_service.get_document(id, *user_id).await?;
        Ok(convert_document_to_output(document))
    }

    async fn documents_by_owner(&self, ctx: &Context<'_>, owner_id: Uuid) -> Result<Vec<DocumentOutput>> {
        let document_service = ctx.data_unchecked::<Arc<DocumentService>>();
        let user_id = ctx.data_unchecked::<Uuid>(); // Get from auth middleware

        // Check if user is requesting their own documents or has admin privileges
        if *user_id != owner_id {
            return Err(async_graphql::Error::new("Unauthorized"));
        }

        let documents = document_service.get_documents_by_owner(owner_id).await?;
        
        let documents_output: Vec<DocumentOutput> = documents.into_iter().map(convert_document_to_output).collect();
        Ok(documents_output)
    }
}

// GraphQL Mutation Root
#[derive(Default)]
pub struct DocumentEditorMutation;

#[Object]
impl DocumentEditorMutation {
    async fn create_document(&self, ctx: &Context<'_>, input: CreateDocumentInput) -> Result<DocumentOutput> {
        let document_service = ctx.data_unchecked::<Arc<DocumentService>>();
        let user_id = ctx.data_unchecked::<Uuid>(); // Get from auth middleware

        // Parse content from JSON string
        let content: serde_json::Value = serde_json::from_str(&input.content)
            .map_err(|e| DocumentError::SerializationError(e))?;

        let document = document_service
            .create_document(*user_id, input.title, content)
            .await?;

        Ok(convert_document_to_output(document))
    }

    async fn update_document_content(
        &self,
        ctx: &Context<'_>,
        input: UpdateDocumentContentInput,
    ) -> Result<DocumentOutput> {
        let document_service = ctx.data_unchecked::<Arc<DocumentService>>();
        let user_id = ctx.data_unchecked::<Uuid>(); // Get from auth middleware

        // Parse content from JSON string
        let content: serde_json::Value = serde_json::from_str(&input.content)
            .map_err(|e| DocumentError::SerializationError(e))?;

        let document = document_service
            .update_document_content(input.document_id, *user_id, content)
            .await?;

        Ok(convert_document_to_output(document))
    }

    async fn update_document_title(
        &self,
        ctx: &Context<'_>,
        input: UpdateDocumentTitleInput,
    ) -> Result<DocumentOutput> {
        let document_service = ctx.data_unchecked::<Arc<DocumentService>>();
        let user_id = ctx.data_unchecked::<Uuid>(); // Get from auth middleware

        let document = document_service
            .update_document_title(input.document_id, *user_id, input.title)
            .await?;

        Ok(convert_document_to_output(document))
    }

    async fn delete_document(&self, ctx: &Context<'_>, document_id: Uuid) -> Result<bool> {
        let document_service = ctx.data_unchecked::<Arc<DocumentService>>();
        let user_id = ctx.data_unchecked::<Uuid>(); // Get from auth middleware

        document_service.delete_document(document_id, *user_id).await?;
        Ok(true)
    }

    async fn share_document(&self, ctx: &Context<'_>, input: ShareDocumentInput) -> Result<DocumentShareOutput> {
        let document_service = ctx.data_unchecked::<Arc<DocumentService>>();
        let user_id = ctx.data_unchecked::<Uuid>(); // Get from auth middleware

        let share = document_service
            .share_document(input.document_id, *user_id, input.shared_with, input.permission_level)
            .await?;

        Ok(DocumentShareOutput {
            id: share.id,
            document_id: share.document_id,
            shared_with: share.shared_with,
            permission_level: match share.permission_level {
                crate::domain::models::PermissionLevel::View => "view".to_string(),
                crate::domain::models::PermissionLevel::Comment => "comment".to_string(),
                crate::domain::models::PermissionLevel::Edit => "edit".to_string(),
            },
            created_at: share.created_at,
            expires_at: share.expires_at,
        })
    }

    async fn apply_formatting(&self, ctx: &Context<'_>, input: FormattingInput) -> Result<DocumentOutput> {
        // TODO: Implement formatting operations
        Err(async_graphql::Error::new("Not implemented"))
    }

    async fn insert_image(&self, ctx: &Context<'_>, input: ImageInsertInput) -> Result<DocumentOutput> {
        // TODO: Implement image insertion
        Err(async_graphql::Error::new("Not implemented"))
    }

    async fn export_document(&self, ctx: &Context<'_>, input: ExportInput) -> Result<ExportResult> {
        let document_service = ctx.data_unchecked::<Arc<DocumentService>>();
        let export_service = ctx.data_unchecked::<Arc<ExportService>>();
        let user_id = ctx.data_unchecked::<Uuid>(); // Get from auth middleware

        let document = document_service.get_document(input.document_id, *user_id).await?;
        let format = crate::application::export_service::ExportFormat::from_str(&input.format)?;
        
        let export_data = export_service.export_document(&document, format).await?;

        // Convert to base64 for transport
        let base64_data = base64::encode(export_data);
        
        let filename = format!("{}.{}", document.title.as_str(), input.format);

        Ok(ExportResult {
            document_id: document.id,
            format: input.format,
            data: base64_data,
            filename,
        })
    }
}

// GraphQL Subscription Root
#[derive(Default)]
pub struct DocumentEditorSubscription;

#[Subscription]
impl DocumentEditorSubscription {
    async fn document_updated(
        &self,
        ctx: &Context<'_>,
        document_id: Uuid,
    ) -> impl Stream<Item = DocumentUpdateEvent> {
        // Get the collaboration service
        let collaboration_service = ctx.data_unchecked::<Arc<crate::collaboration::service::RealtimeCollaborationService>>();
        
        // Subscribe to document operations
        let receiver = match collaboration_service.subscribe_to_operations(document_id) {
            Ok(receiver) => receiver,
            Err(_) => return futures_util::stream::empty(),
        };
        
        // Convert the receiver to a stream
        tokio_stream::wrappers::BroadcastStream::new(receiver)
            .filter_map(|result| async move {
                match result {
                    Ok(operation) => {
                        // Get the updated content
                        let content = collaboration_service.get_document_content(document_id).ok();
                        let content_str = content.map(|c| serde_json::to_string(c.as_json()).unwrap_or_default());
                        
                        Some(DocumentUpdateEvent {
                            document_id,
                            updated_at: chrono::Utc::now(),
                            updated_by: Uuid::nil(), // In a real implementation, this would be the user who made the operation
                            operation: Some(serde_json::to_string(&operation).unwrap_or_default()),
                            content: content_str,
                        })
                    }
                    Err(_) => None,
                }
            })
    }
}