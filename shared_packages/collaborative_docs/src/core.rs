//! Core traits and services for collaborative documents

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Error types for document operations
#[derive(Error, Debug)]
pub enum DocumentError {
    #[error("Document not found: {0}")]
    DocumentNotFound(Uuid),
    #[error("Access denied for document: {0}")]
    AccessDenied(Uuid),
    #[error("Invalid document format: {0}")]
    InvalidFormat(String),
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Conflict resolution error: {0}")]
    ConflictError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Conflict detected: {0}")]
    ConflictDetected(String),
}

/// Document metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub id: Uuid,
    pub title: String,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub content_type: String,
    pub tags: Vec<String>,
    pub version: u64,
    pub visibility: Visibility,
}

/// Document content representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentContent {
    pub data: serde_json::Value,
    pub format: String, // e.g., "json", "markdown", "html"
}

/// Document preview for feed integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentPreview {
    pub id: Uuid,
    pub title: String,
    pub content_type: String,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub excerpt: String, // Short preview of content
    pub word_count: usize,
}

/// Document access level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccessLevel {
    Read,
    Write,
    Admin,
}
/// Document visibility settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Visibility {
    Public,
    FriendsOnly,
    Private,
}
}

/// Document permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentPermission {
    pub user_id: Uuid,
    pub access_level: AccessLevel,
    pub granted_at: DateTime<Utc>,
    pub granted_by: Uuid,
}

/// Core service for document operations
#[async_trait]
pub trait DocumentService: Send + Sync {
    /// Create a new document
    async fn create_document(
        &self,
        owner_id: Uuid,
        title: String,
        content: DocumentContent,
        tags: Vec<String>,
    ) -> Result<DocumentMetadata, DocumentError>;

    /// Get document metadata
    async fn get_document_metadata(
        &self,
        document_id: Uuid,
        user_id: Uuid,
    ) -> Result<DocumentMetadata, DocumentError>;

    /// Get document content
    async fn get_document_content(
        &self,
        document_id: Uuid,
        user_id: Uuid,
    ) -> Result<DocumentContent, DocumentError>;

    /// Update document content
    async fn update_document(
        &self,
        document_id: Uuid,
        user_id: Uuid,
        content: DocumentContent,
    ) -> Result<DocumentMetadata, DocumentError>;

    /// Delete a document
    async fn delete_document(
        &self,
        document_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), DocumentError>;

    /// List documents for a user
    async fn list_documents(
        &self,
        user_id: Uuid,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<DocumentMetadata>, DocumentError>;

    /// Grant access to a document
    async fn grant_access(
        &self,
        document_id: Uuid,
        owner_id: Uuid,
        user_id: Uuid,
        access_level: AccessLevel,
    ) -> Result<(), DocumentError>;

    /// Revoke access to a document
    async fn revoke_access(
        &self,
        document_id: Uuid,
        owner_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), DocumentError>;

    /// Get document permissions
    async fn get_permissions(
        &self,
        document_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<DocumentPermission>, DocumentError>;
    
    /// Get document preview for feed integration
    async fn get_document_preview(
        &self,
        document_id: Uuid,
        user_id: Uuid,
    ) -> Result<DocumentPreview, DocumentError>;
}

/// Provider trait for document storage backends
#[async_trait]
pub trait DocProvider: Send + Sync {
    /// Store document metadata
    async fn store_metadata(
        &self,
        metadata: &DocumentMetadata,
    ) -> Result<(), DocumentError>;

    /// Retrieve document metadata
    async fn retrieve_metadata(
        &self,
        document_id: Uuid,
    ) -> Result<DocumentMetadata, DocumentError>;

    /// Store document content
    async fn store_content(
        &self,
        document_id: Uuid,
        content: &DocumentContent,
    ) -> Result<(), DocumentError>;

    /// Retrieve document content
    async fn retrieve_content(
        &self,
        document_id: Uuid,
    ) -> Result<DocumentContent, DocumentError>;

    /// Delete document
    async fn delete_document(
        &self,
        document_id: Uuid,
    ) -> Result<(), DocumentError>;

    /// List documents for a user
    async fn list_documents(
        &self,
        user_id: Uuid,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<DocumentMetadata>, DocumentError>;

    /// Store document permission
    async fn store_permission(
        &self,
        document_id: Uuid,
        permission: &DocumentPermission,
    ) -> Result<(), DocumentError>;

    /// Retrieve document permissions
    async fn retrieve_permissions(
        &self,
        document_id: Uuid,
    ) -> Result<Vec<DocumentPermission>, DocumentError>;

    /// Delete document permission
    async fn delete_permission(
        &self,
        document_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), DocumentError>;
}