use async_trait::async_trait;
use sqlx::{PgPool, Error};
use uuid::Uuid;
use cpc_core::models::social::post::{Post, Visibility};
use chrono::Utc;

pub struct PostRepositoryImpl {
    pool: PgPool,
}

impl PostRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
pub trait PostRepository {
    async fn create_post(&self, post: NewPost) -> Result<Post, Error>;
    async fn get_posts_by_user(&self, user_id: Uuid) -> Result<Vec<Post>, Error>;
}

pub struct NewPost {
    pub author_id: Uuid,
    pub content: String,
    pub visibility: Visibility,
    pub cooperative_id: Option<Uuid>,
}

#[async_trait]
impl PostRepository for PostRepositoryImpl {
    async fn create_post(&self, post: NewPost) -> Result<Post, Error> {
        let new_post = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (id, author_id, content, visibility, cooperative_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
            Uuid::new_v4(),
            post.author_id,
            post.content,
            post.visibility as _,
            post.cooperative_id,
            Utc::now(),
            Utc::now()
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(new_post)
    }

    async fn get_posts_by_user(&self, user_id: Uuid) -> Result<Vec<Post>, Error> {
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT * FROM posts
            WHERE author_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(posts)
    }
}