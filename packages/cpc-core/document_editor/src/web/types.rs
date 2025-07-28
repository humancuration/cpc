//! GraphQL input/output types for the document editor module

use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Input types

#[derive(InputObject)]
pub struct CreateDocumentInput {
    pub title: String,
    pub content: String, // JSON string representation of document content
}

#[derive(InputObject)]
pub struct UpdateDocumentContentInput {
    pub document_id: Uuid,
    pub content: String, // JSON string representation of document content
}

#[derive(InputObject)]
pub struct UpdateDocumentTitleInput {
    pub document_id: Uuid,
    pub title: String,
}

#[derive(InputObject)]
pub struct ShareDocumentInput {
    pub document_id: Uuid,
    pub shared_with: Uuid,
    pub permission_level: String, // "view", "comment", "edit"
}

#[derive(InputObject)]
pub struct FormattingInput {
    pub document_id: Uuid,
    pub operation: String, // "bold", "italic", "underline", etc.
    pub range_start: usize,
    pub range_end: usize,
}

#[derive(InputObject)]
pub struct ImageInsertInput {
    pub document_id: Uuid,
    pub image_data: String, // Base64 encoded image data
    pub alt_text: Option<String>,
    pub position: usize,
}

#[derive(InputObject)]
pub struct ExportInput {
    pub document_id: Uuid,
    pub format: String, // "pdf", "docx"
}

// Output types

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct DocumentOutput {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub title: String,
    pub content: String, // JSON string representation of document content
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_deleted: bool,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct DocumentShareOutput {
    pub id: Uuid,
    pub document_id: Uuid,
    pub shared_with: Uuid,
    pub permission_level: String, // "view", "comment", "edit"
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct DocumentVersionOutput {
    pub id: Uuid,
    pub document_id: Uuid,
    pub version_number: i32,
    pub content: String, // JSON string representation of document content
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct ExportResult {
    pub document_id: Uuid,
    pub format: String, // "pdf", "docx"
    pub data: String, // Base64 encoded export data
    pub filename: String,
}

#[derive(SimpleObject)]
pub struct DocumentUpdateEvent {
    pub document_id: Uuid,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Uuid,
}