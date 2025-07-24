use async_trait::async_trait;
use uuid::Uuid;
use std::sync::Arc;
use chrono::{DateTime, Utc};

use crate::models::media::{MediaItem, CreateMediaItem, UpdateMediaItem, MediaType, ProcessingStatus};

#[async_trait]
pub trait MediaRepository: Send + Sync {
    async fn create_media_item(&self, item: CreateMediaItem) -> Result<MediaItem, sqlx::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<MediaItem>, sqlx::Error>;
    async fn find_by_post_id(&self, post_id: Uuid) -> Result<Vec<MediaItem>, sqlx::Error>;
    async fn find_pending_media(&self) -> Result<Vec<MediaItem>, sqlx::Error>;
    async fn update_processing_status(&self, id: Uuid, status: ProcessingStatus) -> Result<(), sqlx::Error>;
    async fn update_media_item(&self, id: Uuid, update: UpdateMediaItem) -> Result<MediaItem, sqlx::Error>;
    async fn delete_media_item(&self, id: Uuid) -> Result<(), sqlx::Error>;
    async fn associate_media_to_post(&self, media_ids: Vec<Uuid>, post_id: Uuid) -> Result<(), sqlx::Error>;
}

#[derive(Clone)]
pub struct MediaRepositoryImpl {
    pool: Arc<sqlx::SqlitePool>,
}

impl MediaRepositoryImpl {
    pub fn new(pool: Arc<sqlx::SqlitePool>) -> Self {
        Self { pool }
    }

    async fn init_db(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS media_items (
                id TEXT PRIMARY KEY,
                post_id TEXT,
                url TEXT NOT NULL,
                media_type TEXT NOT NULL,
                processing_status TEXT NOT NULL DEFAULT 'pending',
                file_size INTEGER,
                mime_type TEXT,
                original_filename TEXT,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                processed_at DATETIME,
                metadata TEXT,
                FOREIGN KEY (post_id) REFERENCES posts(id)
            )
            "#,
        )
        .execute(&*self.pool)
        .await?;

        // Create indexes
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_media_post_id ON media_items(post_id)",
        )
        .execute(&*self.pool)
        .await?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_media_processing_status ON media_items(processing_status)",
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }
}

#[async_trait]
impl MediaRepository for MediaRepositoryImpl {
    async fn create_media_item(&self, item: CreateMediaItem) -> Result<MediaItem, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        sqlx::query_as::<_, MediaItem>(
            r#"
            INSERT INTO media_items (id, post_id, url, media_type, file_size, mime_type, original_filename, metadata, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(id.to_string())
        .bind(item.post_id.map(|id| id.to_string()))
        .bind(item.url)
        .bind(item.media_type.to_string())
        .bind(item.file_size)
        .bind(item.mime_type)
        .bind(item.original_filename)
        .bind(item.metadata.map(|m| serde_json::to_string(&m).unwrap_or_default()))
        .bind(now)
        .bind(now)
        .fetch_one(&*self.pool)
        .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<MediaItem>, sqlx::Error> {
        sqlx::query_as::<_, MediaItem>(
            "SELECT * FROM media_items WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(&*self.pool)
        .await
    }

    async fn find_by_post_id(&self, post_id: Uuid) -> Result<Vec<MediaItem>, sqlx::Error> {
        sqlx::query_as::<_, MediaItem>(
            "SELECT * FROM media_items WHERE post_id = ? ORDER BY created_at ASC",
        )
        .bind(post_id.to_string())
        .fetch_all(&*self.pool)
        .await
    }

    async fn find_pending_media(&self) -> Result<Vec<MediaItem>, sqlx::Error> {
        sqlx::query_as::<_, MediaItem>(
            "SELECT * FROM media_items WHERE processing_status IN ('pending', 'processing') ORDER BY created_at ASC",
        )
        .fetch_all(&*self.pool)
        .await
    }

    async fn update_processing_status(&self, id: Uuid, status: ProcessingStatus) -> Result<(), sqlx::Error> {
        let now = Utc::now();
        
        sqlx::query(
            "UPDATE media_items SET processing_status = ?, updated_at = ? WHERE id = ?",
        )
        .bind(status.to_string())
        .bind(now)
        .bind(id.to_string())
        .execute(&*self.pool)
        .await?;
        
        Ok(())
    }

    async fn update_media_item(&self, id: Uuid, update: UpdateMediaItem) -> Result<MediaItem, sqlx::Error> {
        let now = Utc::now();
        
        let mut query = String::from("UPDATE media_items SET updated_at = ?");
        let mut params = vec![now.to_string(), id.to_string()];
        
        if update.processing_status.is_some() {
            query.push_str(", processing_status = ?");
            params.insert(1, update.processing_status.unwrap().to_string());
        }
        if update.url.is_some() {
            query.push_str(", url = ?");
            params.insert(1, update.url.unwrap());
        }
        if update.file_size.is_some() {
            query.push_str(", file_size = ?");
            params.insert(1, update.file_size.unwrap().to_string());
        }
        if update.metadata.is_some() {
            query.push_str(", metadata = ?");
            params.insert(1, update.metadata.map(|m| serde_json::to_string(&m).unwrap_or_default()).unwrap_or_default());
        }
        if update.processed_at.is_some() {
            query.push_str(", processed_at = ?");
            params.insert(1, update.processed_at.unwrap().to_string());
        }
        
        query.push_str(" WHERE id = ? RETURNING *");
        
        sqlx::query_as::<_, MediaItem>(&query)
            .bind(&params[0])
            .bind(&params[1])
            .bind(&params[2])
            .fetch_one(&*self.pool)
            .await
    }

    async fn delete_media_item(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM media_items WHERE id = ?",
        )
        .bind(id.to_string())
        .execute(&*self.pool)
        .await?;
        
        Ok(())
    }

    async fn associate_media_to_post(&self, media_ids: Vec<Uuid>, post_id: Uuid) -> Result<(), sqlx::Error> {
        let now = Utc::now();
        
        let placeholders: Vec<String> = media_ids.iter()
            .map(|_| "?")
            .collect();
        let placeholders_str = placeholders.join(",");
        
        let mut query = format!(
            "UPDATE media_items SET post_id = ?, updated_at = ? WHERE id IN ({})",
            placeholders_str
        );
        
        let mut params = vec![post_id.to_string(), now.to_string()];
        for media_id in &media_ids {
            params.push(media_id.to_string());
        }
        
        sqlx::query(&query)
            .bind(&params[0])
            .bind(&params[1])
            .bind(&params[2..])
            .execute(&*self.pool)
            .await?;
            
        Ok(())
    }
}