//! PostgreSQL storage adapter for collaborative documents

use crate::core::{DocProvider, DocumentContent, DocumentError, DocumentMetadata, DocumentPermission};
use async_trait::async_trait;
use serde_json::Value as JsonValue;
use sqlx::{PgPool, Row};
use std::str::FromStr;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// PostgreSQL storage adapter for documents
pub struct PostgresDocStore {
    pool: PgPool,
}

impl PostgresDocStore {
    /// Create a new PostgreSQL document store
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Initialize the database schema
    pub async fn init_schema(&self) -> Result<(), DocumentError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS document_metadata (
                id UUID PRIMARY KEY,
                title TEXT NOT NULL,
                owner_id UUID NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
                content_type TEXT NOT NULL,
                tags TEXT[] NOT NULL,
                version BIGINT NOT NULL,
                visibility TEXT NOT NULL DEFAULT 'private'
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS document_content (
                document_id UUID PRIMARY KEY,
                data JSONB NOT NULL,
                format TEXT NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS document_permissions (
                id UUID PRIMARY KEY,
                document_id UUID NOT NULL,
                user_id UUID NOT NULL,
                access_level TEXT NOT NULL,
                granted_at TIMESTAMP WITH TIME ZONE NOT NULL,
                granted_by UUID NOT NULL,
                UNIQUE(document_id, user_id)
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl DocProvider for PostgresDocStore {
    /// Store document metadata
    async fn store_metadata(
        &self,
        metadata: &DocumentMetadata,
    ) -> Result<(), DocumentError> {
        sqlx::query(
            r#"
            INSERT INTO document_metadata (
                id, title, owner_id, created_at, updated_at, content_type, tags, version, visibility
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                title = EXCLUDED.title,
                updated_at = EXCLUDED.updated_at,
                version = EXCLUDED.version,
                visibility = EXCLUDED.visibility
            "#,
        )
        .bind(metadata.id)
        .bind(&metadata.title)
        .bind(metadata.owner_id)
        .bind(metadata.created_at)
        .bind(metadata.updated_at)
        .bind(&metadata.content_type)
        .bind(&metadata.tags)
        .bind(metadata.version as i64)
        .bind(match &metadata.visibility {
            crate::core::Visibility::Public => "public",
            crate::core::Visibility::FriendsOnly => "friends_only",
            crate::core::Visibility::Private => "private",
        })
        .execute(&self.pool)
        .await
        .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        Ok(())
    }

    /// Retrieve document metadata
    async fn retrieve_metadata(
        &self,
        document_id: Uuid,
    ) -> Result<DocumentMetadata, DocumentError> {
        let row = sqlx::query(
            r#"
            SELECT id, title, owner_id, created_at, updated_at, content_type, tags, version, visibility
            FROM document_metadata
            WHERE id = $1
            "#,
        )
        .bind(document_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => DocumentError::DocumentNotFound(document_id),
            _ => DocumentError::StorageError(e.to_string()),
        })?;

        Ok(DocumentMetadata {
            id: row.get("id"),
            title: row.get("title"),
            owner_id: row.get("owner_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            content_type: row.get("content_type"),
            tags: row.get("tags"),
            version: row.get::<i64, _>("version") as u64,
            visibility: {
                let visibility_str: String = row.get("visibility");
                match visibility_str.as_str() {
                    "public" => crate::core::Visibility::Public,
                    "friends_only" => crate::core::Visibility::FriendsOnly,
                    "private" => crate::core::Visibility::Private,
                    _ => crate::core::Visibility::Private, // Default to private for unknown values
                }
            },
        })
    }

    /// Store document content
    async fn store_content(
        &self,
        document_id: Uuid,
        content: &DocumentContent,
    ) -> Result<(), DocumentError> {
        sqlx::query(
            r#"
            INSERT INTO document_content (document_id, data, format)
            VALUES ($1, $2, $3)
            ON CONFLICT (document_id) DO UPDATE SET
                data = EXCLUDED.data,
                format = EXCLUDED.format,
                updated_at = NOW()
            "#,
        )
        .bind(document_id)
        .bind(&content.data)
        .bind(&content.format)
        .execute(&self.pool)
        .await
        .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        Ok(())
    }

    /// Retrieve document content
    async fn retrieve_content(
        &self,
        document_id: Uuid,
    ) -> Result<DocumentContent, DocumentError> {
        let row = sqlx::query(
            r#"
            SELECT data, format
            FROM document_content
            WHERE document_id = $1
            "#,
        )
        .bind(document_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => DocumentError::DocumentNotFound(document_id),
            _ => DocumentError::StorageError(e.to_string()),
        })?;

        Ok(DocumentContent {
            data: row.get("data"),
            format: row.get("format"),
        })
    }

    /// Delete document
    async fn delete_document(
        &self,
        document_id: Uuid,
    ) -> Result<(), DocumentError> {
        // Delete content first
        sqlx::query("DELETE FROM document_content WHERE document_id = $1")
            .bind(document_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        // Delete permissions
        sqlx::query("DELETE FROM document_permissions WHERE document_id = $1")
            .bind(document_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        // Delete metadata
        sqlx::query("DELETE FROM document_metadata WHERE id = $1")
            .bind(document_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        Ok(())
    }

    /// List documents for a user
    async fn list_documents(
        &self,
        user_id: Uuid,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<DocumentMetadata>, DocumentError> {
        let rows = sqlx::query(
            r#"
            SELECT id, title, owner_id, created_at, updated_at, content_type, tags, version, visibility
            FROM document_metadata
            WHERE owner_id = $1
            ORDER BY updated_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        let mut documents = Vec::new();
        for row in rows {
            documents.push(DocumentMetadata {
                id: row.get("id"),
                title: row.get("title"),
                owner_id: row.get("owner_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                content_type: row.get("content_type"),
                tags: row.get("tags"),
                version: row.get::<i64, _>("version") as u64,
                visibility: {
                    let visibility_str: String = row.get("visibility");
                    match visibility_str.as_str() {
                        "public" => crate::core::Visibility::Public,
                        "friends_only" => crate::core::Visibility::FriendsOnly,
                        "private" => crate::core::Visibility::Private,
                        _ => crate::core::Visibility::Private, // Default to private for unknown values
                    }
                },
            });
        }

        Ok(documents)
    }

    /// Store document permission
    async fn store_permission(
        &self,
        document_id: Uuid,
        permission: &DocumentPermission,
    ) -> Result<(), DocumentError> {
        sqlx::query(
            r#"
            INSERT INTO document_permissions (
                id, document_id, user_id, access_level, granted_at, granted_by
            ) VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (document_id, user_id) DO UPDATE SET
                access_level = EXCLUDED.access_level,
                granted_at = EXCLUDED.granted_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(document_id)
        .bind(permission.user_id)
        .bind(match &permission.access_level {
            crate::core::AccessLevel::Read => "read",
            crate::core::AccessLevel::Write => "write",
            crate::core::AccessLevel::Admin => "admin",
        })
        .bind(permission.granted_at)
        .bind(permission.granted_by)
        .execute(&self.pool)
        .await
        .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        Ok(())
    }

    /// Retrieve document permissions
    async fn retrieve_permissions(
        &self,
        document_id: Uuid,
    ) -> Result<Vec<DocumentPermission>, DocumentError> {
        let rows = sqlx::query(
            r#"
            SELECT user_id, access_level, granted_at, granted_by
            FROM document_permissions
            WHERE document_id = $1
            "#,
        )
        .bind(document_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        let mut permissions = Vec::new();
        for row in rows {
            let access_level_str: String = row.get("access_level");
            let access_level = match access_level_str.as_str() {
                "read" => crate::core::AccessLevel::Read,
                "write" => crate::core::AccessLevel::Write,
                "admin" => crate::core::AccessLevel::Admin,
                _ => return Err(DocumentError::InvalidFormat(format!("Invalid access level: {}", access_level_str))),
            };

            permissions.push(DocumentPermission {
                user_id: row.get("user_id"),
                access_level,
                granted_at: row.get("granted_at"),
                granted_by: row.get("granted_by"),
            });
        }

        Ok(permissions)
    }

    /// Delete document permission
    async fn delete_permission(
        &self,
        document_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), DocumentError> {
        sqlx::query("DELETE FROM document_permissions WHERE document_id = $1 AND user_id = $2")
            .bind(document_id)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use sqlx::PgPoolOptions;

    #[ignore] // This test requires a PostgreSQL database
    #[tokio::test]
    async fn test_postgres_store() -> Result<(), Box<dyn std::error::Error>> {
        // This is a simplified test that would require a real PostgreSQL instance
        // In a real scenario, you would use a test database
        
        /*
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgresql://user:password@localhost/test_db")
            .await?;

        let store = PostgresDocStore::new(pool);
        store.init_schema().await?;

        let document_id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();

        let metadata = DocumentMetadata {
            id: document_id,
            title: "Test Document".to_string(),
            owner_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            content_type: "text/plain".to_string(),
            tags: vec!["test".to_string()],
            version: 1,
            visibility: crate::core::Visibility::Private,
        };

        // Test storing metadata
        store.store_metadata(&metadata).await?;

        // Test retrieving metadata
        let retrieved_metadata = store.retrieve_metadata(document_id).await?;
        assert_eq!(retrieved_metadata.title, "Test Document");

        let content = DocumentContent {
            data: json!({"text": "Hello, world!"}),
            format: "json".to_string(),
        };

        // Test storing content
        store.store_content(document_id, &content).await?;

        // Test retrieving content
        let retrieved_content = store.retrieve_content(document_id).await?;
        assert_eq!(retrieved_content.format, "json");

        // Test listing documents
        let documents = store.list_documents(owner_id, 10, 0).await?;
        assert!(!documents.is_empty());

        // Clean up
        store.delete_document(document_id).await?;
        */

        Ok(())
    }
}