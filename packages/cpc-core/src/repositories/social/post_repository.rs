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
    pub media_ids: Vec<Uuid>,
}

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn create_post(&self, data: CreatePostData) -> Result<Post, sqlx::Error>;
    async fn find_post_by_id(&self, id: Uuid) -> Result<Option<Post>, sqlx::Error>;
    async fn get_feed_posts(&self, user_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Post>, sqlx::Error>;
    async fn get_user_posts(&self, user_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Post>, sqlx::Error>;
    async fn get_cooperative_posts(&self, cooperative_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Post>, sqlx::Error>;
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
            SELECT id, author_id, content, visibility as "visibility: _", cooperative_id, feed_position, created_at, updated_at
            FROM posts
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(post)
    }

    async fn get_feed_posts(&self, user_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Post>, sqlx::Error> {
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT id, author_id, content, visibility as "visibility: _", cooperative_id, feed_position, created_at, updated_at
            FROM posts
            WHERE visibility = 'PUBLIC'
               OR (visibility = 'COOPERATIVE' AND cooperative_id IN (
                   SELECT cooperative_id FROM cooperative_members WHERE user_id = $1
               ))
               OR author_id = $1
            ORDER BY feed_position ASC, created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            user_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(posts)
    }

    async fn get_user_posts(&self, user_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Post>, sqlx::Error> {
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT id, author_id, content, visibility as "visibility: _", cooperative_id, feed_position, created_at, updated_at
            FROM posts
            WHERE author_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            user_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(posts)
    }

    async fn get_cooperative_posts(&self, cooperative_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Post>, sqlx::Error> {
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT id, author_id, content, visibility as "visibility: _", cooperative_id, feed_position, created_at, updated_at
            FROM posts
            WHERE cooperative_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            cooperative_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(posts)
    }
}