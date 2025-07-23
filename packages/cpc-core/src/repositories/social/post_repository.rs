use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::models::social::post::{Post, Visibility};

// Using a struct for creation data promotes clarity and type safety
pub struct CreatePostData {
    pub author_id: Uuid,
    pub content: String,
    pub visibility: Visibility,
    pub cooperative_id: Option<Uuid>,
    // In the future, this could include media item data
}

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn create_post(&self, data: CreatePostData) -> Result<Post, sqlx::Error>;
    async fn find_post_by_id(&self, id: Uuid) -> Result<Option<Post>, sqlx::Error>;
}

pub struct SqlitePostRepository {
    pool: SqlitePool,
}

impl SqlitePostRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostRepository for SqlitePostRepository {
    async fn create_post(&self, data: CreatePostData) -> Result<Post, sqlx::Error> {
        // The 'RETURNING *' clause is specific to PostgreSQL and SQLite,
        // making it easy to get the created record back.
        let post = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (id, author_id, content, visibility, cooperative_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, author_id, content, visibility as "visibility: _", cooperative_id, created_at, updated_at
            "#,
            Uuid::new_v4(),
            data.author_id,
            data.content,
            data.visibility,
            data.cooperative_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(post)
    }

    async fn find_post_by_id(&self, id: Uuid) -> Result<Option<Post>, sqlx::Error> {
        let post = sqlx::query_as!(
            Post,
            r#"
            SELECT id, author_id, content, visibility as "visibility: _", cooperative_id, created_at, updated_at
            FROM posts
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(post)
    }
}