//! Integration tests for the PgDocumentRepository
//!
//! These tests require a PostgreSQL database to run.
//! Set the TEST_DATABASE_URL environment variable to point to a test database.

use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::infrastructure::repository::{DocumentRepository, PgDocumentRepository};
use crate::domain::models::{Document, DocumentShare, DocumentVersion, PermissionLevel};
use crate::domain::value_objects::{DocumentTitle, DocumentContent};

/// Setup a test database connection
async fn setup_test_db() -> PgPool {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost/cpc_test".to_string());
    
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");
    
    // Create tables for testing
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS documents (
            id UUID PRIMARY KEY,
            owner_id UUID NOT NULL,
            title VARCHAR(255) NOT NULL DEFAULT 'Untitled Document',
            content JSONB NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            is_deleted BOOLEAN NOT NULL DEFAULT false
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create documents table");
    
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS document_shares (
            id UUID PRIMARY KEY,
            document_id UUID NOT NULL REFERENCES documents(id),
            shared_with UUID NOT NULL,
            permission_level VARCHAR(20) NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            expires_at TIMESTAMPTZ,
            UNIQUE(document_id, shared_with)
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create document_shares table");
    
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS document_versions (
            id UUID PRIMARY KEY,
            document_id UUID NOT NULL REFERENCES documents(id),
            version_number INTEGER NOT NULL,
            content JSONB NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            created_by UUID NOT NULL,
            UNIQUE(document_id, version_number)
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create document_versions table");
    
    // Create indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_documents_owner_id ON documents(owner_id)")
        .execute(&pool)
        .await
        .expect("Failed to create index on documents.owner_id");
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_document_shares_document_id ON document_shares(document_id)")
        .execute(&pool)
        .await
        .expect("Failed to create index on document_shares.document_id");
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_document_versions_document_id ON document_versions(document_id)")
        .execute(&pool)
        .await
        .expect("Failed to create index on document_versions.document_id");
    
    pool
}

/// Cleanup test data
async fn cleanup_test_data(pool: &PgPool) {
    sqlx::query("DELETE FROM document_versions")
        .execute(pool)
        .await
        .expect("Failed to cleanup document_versions");
        
    sqlx::query("DELETE FROM document_shares")
        .execute(pool)
        .await
        .expect("Failed to cleanup document_shares");
        
    sqlx::query("DELETE FROM documents")
        .execute(pool)
        .await
        .expect("Failed to cleanup documents");
}

#[tokio::test]
async fn test_create_and_get_document() {
    let pool = setup_test_db().await;
    let repository = PgDocumentRepository::new(pool.clone());
    
    // Create a test document
    let owner_id = Uuid::new_v4();
    let title = DocumentTitle::new("Test Document".to_string()).unwrap();
    let content = DocumentContent::new(serde_json::json!({"text": "Hello, world!"}));
    
    let document = Document {
        id: Uuid::new_v4(),
        owner_id,
        title: title.clone(),
        content: content.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        is_deleted: false,
    };
    
    // Test create
    let result = repository.create_document(&document).await;
    assert!(result.is_ok());
    
    // Test get
    let retrieved_document = repository.get_document(document.id).await.unwrap();
    assert_eq!(retrieved_document.id, document.id);
    assert_eq!(retrieved_document.owner_id, document.owner_id);
    assert_eq!(retrieved_document.title.as_str(), document.title.as_str());
    assert_eq!(retrieved_document.content.as_json(), document.content.as_json());
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_update_document() {
    let pool = setup_test_db().await;
    let repository = PgDocumentRepository::new(pool.clone());
    
    // Create a test document
    let owner_id = Uuid::new_v4();
    let title = DocumentTitle::new("Test Document".to_string()).unwrap();
    let content = DocumentContent::new(serde_json::json!({"text": "Hello, world!"}));
    
    let mut document = Document {
        id: Uuid::new_v4(),
        owner_id,
        title: title.clone(),
        content: content.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        is_deleted: false,
    };
    
    repository.create_document(&document).await.unwrap();
    
    // Update the document
    let updated_content = DocumentContent::new(serde_json::json!({"text": "Updated content"}));
    document.content = updated_content.clone();
    document.updated_at = Utc::now();
    
    let result = repository.update_document(&document).await;
    assert!(result.is_ok());
    
    // Verify update
    let retrieved_document = repository.get_document(document.id).await.unwrap();
    assert_eq!(retrieved_document.content.as_json(), updated_content.as_json());
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_delete_document() {
    let pool = setup_test_db().await;
    let repository = PgDocumentRepository::new(pool.clone());
    
    // Create a test document
    let owner_id = Uuid::new_v4();
    let title = DocumentTitle::new("Test Document".to_string()).unwrap();
    let content = DocumentContent::new(serde_json::json!({"text": "Hello, world!"}));
    
    let document = Document {
        id: Uuid::new_v4(),
        owner_id,
        title: title.clone(),
        content: content.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        is_deleted: false,
    };
    
    repository.create_document(&document).await.unwrap();
    
    // Verify document exists
    let retrieved_document = repository.get_document(document.id).await;
    assert!(retrieved_document.is_ok());
    
    // Delete document
    let result = repository.delete_document(document.id).await;
    assert!(result.is_ok());
    
    // Verify document is marked as deleted
    let retrieved_document = repository.get_document(document.id).await;
    assert!(retrieved_document.is_err());
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_get_documents_by_owner() {
    let pool = setup_test_db().await;
    let repository = PgDocumentRepository::new(pool.clone());
    
    let owner_id = Uuid::new_v4();
    
    // Create test documents
    let title1 = DocumentTitle::new("Test Document 1".to_string()).unwrap();
    let content1 = DocumentContent::new(serde_json::json!({"text": "Content 1"}));
    
    let document1 = Document {
        id: Uuid::new_v4(),
        owner_id,
        title: title1.clone(),
        content: content1.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        is_deleted: false,
    };
    
    let title2 = DocumentTitle::new("Test Document 2".to_string()).unwrap();
    let content2 = DocumentContent::new(serde_json::json!({"text": "Content 2"}));
    
    let document2 = Document {
        id: Uuid::new_v4(),
        owner_id,
        title: title2.clone(),
        content: content2.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        is_deleted: false,
    };
    
    // Create document for another owner
    let other_owner_id = Uuid::new_v4();
    let title3 = DocumentTitle::new("Other Owner Document".to_string()).unwrap();
    let content3 = DocumentContent::new(serde_json::json!({"text": "Content 3"}));
    
    let document3 = Document {
        id: Uuid::new_v4(),
        owner_id: other_owner_id,
        title: title3.clone(),
        content: content3.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        is_deleted: false,
    };
    
    repository.create_document(&document1).await.unwrap();
    repository.create_document(&document2).await.unwrap();
    repository.create_document(&document3).await.unwrap();
    
    // Test get documents by owner
    let documents = repository.get_documents_by_owner(owner_id).await.unwrap();
    assert_eq!(documents.len(), 2);
    
    let document_ids: Vec<Uuid> = documents.iter().map(|d| d.id).collect();
    assert!(document_ids.contains(&document1.id));
    assert!(document_ids.contains(&document2.id));
    assert!(!document_ids.contains(&document3.id));
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_create_and_get_document_share() {
    let pool = setup_test_db().await;
    let repository = PgDocumentRepository::new(pool.clone());
    
    // Create a test document
    let owner_id = Uuid::new_v4();
    let title = DocumentTitle::new("Test Document".to_string()).unwrap();
    let content = DocumentContent::new(serde_json::json!({"text": "Hello, world!"}));
    
    let document = Document {
        id: Uuid::new_v4(),
        owner_id,
        title: title.clone(),
        content: content.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        is_deleted: false,
    };
    
    repository.create_document(&document).await.unwrap();
    
    // Create a document share
    let shared_with = Uuid::new_v4();
    let share = DocumentShare {
        id: Uuid::new_v4(),
        document_id: document.id,
        shared_with,
        permission_level: PermissionLevel::Edit,
        created_at: Utc::now(),
        expires_at: None,
    };
    
    // Test create share
    let result = repository.create_document_share(&share).await;
    assert!(result.is_ok());
    
    // Test get share
    let retrieved_share = repository.get_document_share(document.id, shared_with).await.unwrap();
    assert_eq!(retrieved_share.id, share.id);
    assert_eq!(retrieved_share.document_id, share.document_id);
    assert_eq!(retrieved_share.shared_with, share.shared_with);
    assert_eq!(retrieved_share.permission_level, PermissionLevel::Edit);
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_create_and_get_document_version() {
    let pool = setup_test_db().await;
    let repository = PgDocumentRepository::new(pool.clone());
    
    // Create a test document
    let owner_id = Uuid::new_v4();
    let title = DocumentTitle::new("Test Document".to_string()).unwrap();
    let content = DocumentContent::new(serde_json::json!({"text": "Hello, world!"}));
    
    let document = Document {
        id: Uuid::new_v4(),
        owner_id,
        title: title.clone(),
        content: content.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        is_deleted: false,
    };
    
    repository.create_document(&document).await.unwrap();
    
    // Create a document version
    let version_content = DocumentContent::new(serde_json::json!({"text": "Version 1 content"}));
    let version = DocumentVersion {
        id: Uuid::new_v4(),
        document_id: document.id,
        version_number: 1,
        content: version_content.clone(),
        created_at: Utc::now(),
        created_by: owner_id,
    };
    
    // Test create version
    let result = repository.create_document_version(&version).await;
    assert!(result.is_ok());
    
    // Test get versions
    let versions = repository.get_document_versions(document.id).await.unwrap();
    assert_eq!(versions.len(), 1);
    assert_eq!(versions[0].id, version.id);
    assert_eq!(versions[0].document_id, version.document_id);
    assert_eq!(versions[0].version_number, version.version_number);
    assert_eq!(versions[0].content.as_json(), version_content.as_json());
    
    // Test get latest version number
    let latest_version = repository.get_latest_version_number(document.id).await.unwrap();
    assert_eq!(latest_version, 1);
    
    cleanup_test_data(&pool).await;
}