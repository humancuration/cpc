use async_trait::async_trait;
use uuid::Uuid;
use crate::models::social::post::{MediaItem, MediaType, ProcessingStatus};

#[async_trait]
pub trait MediaRepository: Send + Sync {
    async fn create_media_item(&self, post_id: Uuid, url: String, media_type: MediaType) -> Result<MediaItem, sqlx::Error>;
    async fn find_media_by_post_id(&self, post_id: Uuid) -> Result<Vec<MediaItem>, sqlx::Error>;
    async fn update_processing_status(&self, media_id: Uuid, status: ProcessingStatus) -> Result<(), sqlx::Error>;
    async fn find_pending_media(&self) -> Result<Vec<MediaItem>, sqlx::Error>;
}

pub struct SqliteMediaRepository {
    pool: sqlx::SqlitePool,
}

impl SqliteMediaRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MediaRepository for SqliteMediaRepository {
    async fn create_media_item(&self, post_id: Uuid, url: String, media_type: MediaType) -> Result<MediaItem, sqlx::Error> {
        let media_item = sqlx::query_as!(
            MediaItem,
            r#"
            INSERT INTO media_items (id, post_id, url, media_type, processing_status)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, post_id, url, media_type as "media_type: _", processing_status as "processing_status: _", created_at
            "#,
            Uuid::new_v4(),
            post_id,
            url,
            media_type,
            ProcessingStatus::Pending
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(media_item)
    }

    async fn find_media_by_post_id(&self, post_id: Uuid) -> Result<Vec<MediaItem>, sqlx::Error> {
        let media_items = sqlx::query_as!(
            MediaItem,
            r#"
            SELECT id, post_id, url, media_type as "media_type: _", processing_status as "processing_status: _", created_at
            FROM media_items
            WHERE post_id = $1
            ORDER BY created_at ASC
            "#,
            post_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(media_items)
    }

    async fn update_processing_status(&self, media_id: Uuid, status: ProcessingStatus) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE media_items
            SET processing_status = $1
            WHERE id = $2
            "#,
            status,
            media_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_pending_media(&self) -> Result<Vec<MediaItem>, sqlx::Error> {
        let media_items = sqlx::query_as!(
            MediaItem,
            r#"
            SELECT id, post_id, url, media_type as "media_type: _", processing_status as "processing_status: _", created_at
            FROM media_items
            WHERE processing_status = 'PENDING'
            ORDER BY created_at ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(media_items)
    }
}