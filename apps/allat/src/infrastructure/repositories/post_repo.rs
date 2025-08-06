use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::post::Post;
use crate::domain::vote::Vote;
use crate::domain::vote::VoteType;
use uuid::Uuid;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PostRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Post not found")]
    NotFound,
}

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn create(&self, post: &Post) -> Result<(), PostRepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, PostRepositoryError>;
    async fn find_by_community(&self, community_id: Uuid) -> Result<Vec<Post>, PostRepositoryError>;
    async fn find_comments_by_post(&self, post_id: Uuid) -> Result<Vec<Post>, PostRepositoryError>;
    async fn update(&self, post: &Post) -> Result<(), PostRepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<(), PostRepositoryError>;
    async fn get_vote_count(&self, post_id: Uuid) -> Result<i32, PostRepositoryError>;
    async fn search(&self, criteria: crate::application::search_service::SearchCriteria) -> Result<Vec<Post>, PostRepositoryError>;
}

pub struct PgPostRepository {
    pool: PgPool,
}

impl PgPostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn load_votes_for_post(&self, post_id: Uuid) -> Result<Vec<Vote>, PostRepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, post_id, vote_type as "vote_type: String", created_at
            FROM votes
            WHERE post_id = $1
            "#,
            post_id
        )
        .fetch_all(&self.pool)
        .await?;

        let votes = rows
            .into_iter()
            .map(|row| {
                let vote_type = match row.vote_type.as_str() {
                    "Upvote" => VoteType::Upvote,
                    "Downvote" => VoteType::Downvote,
                    _ => panic!("Invalid vote type"),
                };
                Vote {
                    id: row.id,
                    user_id: row.user_id,
                    post_id: row.post_id,
                    vote_type,
                    created_at: row.created_at,
                }
            })
            .collect();

        Ok(votes)
    }
}

#[async_trait]
impl PostRepository for PgPostRepository {
    async fn create(&self, post: &Post) -> Result<(), PostRepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO posts (id, community_id, user_id, title, content, created_at, updated_at, parent_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            post.id,
            post.community_id,
            post.user_id,
            post.title,
            post.content,
            post.created_at,
            post.updated_at,
            post.parent_id
        )
        .execute(&self.pool)
        .await?;

        // Insert media assets
        for media_asset in &post.media_assets {
            sqlx::query!(
                r#"
                INSERT INTO media_assets (id, post_id, url, thumbnail_url, media_type, alt_text, created_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
                media_asset.id,
                post.id,
                media_asset.url,
                media_asset.thumbnail_url,
                match media_asset.media_type {
                    crate::domain::media_asset::MediaType::Image => "Image",
                    crate::domain::media_asset::MediaType::Video => "Video",
                },
                media_asset.alt_text,
                media_asset.created_at
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, PostRepositoryError> {
        let row = sqlx::query!(
            r#"
            SELECT id, community_id, user_id, title, content, created_at, updated_at, parent_id
            FROM posts
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                // Load media assets
                let media_rows = sqlx::query!(
                    r#"
                    SELECT id, post_id, url, thumbnail_url, media_type as "media_type: String", alt_text, created_at
                    FROM media_assets
                    WHERE post_id = $1
                    "#,
                    id
                )
                .fetch_all(&self.pool)
                .await?;

                let media_assets = media_rows
                    .into_iter()
                    .map(|row| {
                        let media_type = match row.media_type.as_str() {
                            "Image" => crate::domain::media_asset::MediaType::Image,
                            "Video" => crate::domain::media_asset::MediaType::Video,
                            _ => panic!("Invalid media type"),
                        };
                        crate::domain::media_asset::MediaAsset {
                            id: row.id,
                            url: row.url,
                            thumbnail_url: row.thumbnail_url,
                            media_type,
                            alt_text: row.alt_text,
                            created_at: row.created_at,
                        }
                    })
                    .collect();

                // Load votes
                let votes = self.load_votes_for_post(id).await?;

                let post = Post {
                    id: row.id,
                    community_id: row.community_id,
                    user_id: row.user_id,
                    title: row.title,
                    content: row.content,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    parent_id: row.parent_id,
                    media_assets,
                    votes,
                };
                Ok(Some(post))
            }
            None => Ok(None),
        }
    }

    async fn find_by_community(&self, community_id: Uuid) -> Result<Vec<Post>, PostRepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, community_id, user_id, title, content, created_at, updated_at, parent_id
            FROM posts
            WHERE community_id = $1 AND parent_id IS NULL
            ORDER BY created_at DESC
            "#,
            community_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut posts = Vec::new();
        for row in rows {
            // Load media assets
            let media_rows = sqlx::query!(
                r#"
                SELECT id, post_id, url, thumbnail_url, media_type as "media_type: String", alt_text, created_at
                FROM media_assets
                WHERE post_id = $1
                "#,
                row.id
            )
            .fetch_all(&self.pool)
            .await?;

            let media_assets = media_rows
                .into_iter()
                .map(|row| {
                    let media_type = match row.media_type.as_str() {
                        "Image" => crate::domain::media_asset::MediaType::Image,
                        "Video" => crate::domain::media_asset::MediaType::Video,
                        _ => panic!("Invalid media type"),
                    };
                    crate::domain::media_asset::MediaAsset {
                        id: row.id,
                        url: row.url,
                        thumbnail_url: row.thumbnail_url,
                        media_type,
                        alt_text: row.alt_text,
                        created_at: row.created_at,
                    }
                })
                .collect();

            // Load votes
            let votes = self.load_votes_for_post(row.id).await?;

            let post = Post {
                id: row.id,
                community_id: row.community_id,
                user_id: row.user_id,
                title: row.title,
                content: row.content,
                created_at: row.created_at,
                updated_at: row.updated_at,
                parent_id: row.parent_id,
                media_assets,
                votes,
            };
            posts.push(post);
        }

        Ok(posts)
    }

    async fn find_comments_by_post(&self, post_id: Uuid) -> Result<Vec<Post>, PostRepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, community_id, user_id, title, content, created_at, updated_at, parent_id
            FROM posts
            WHERE parent_id = $1
            ORDER BY created_at ASC
            "#,
            post_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut comments = Vec::new();
        for row in rows {
            // Load media assets (comments typically don't have media, but we'll load them anyway)
            let media_rows = sqlx::query!(
                r#"
                SELECT id, post_id, url, thumbnail_url, media_type as "media_type: String", alt_text, created_at
                FROM media_assets
                WHERE post_id = $1
                "#,
                row.id
            )
            .fetch_all(&self.pool)
            .await?;

            let media_assets = media_rows
                .into_iter()
                .map(|row| {
                    let media_type = match row.media_type.as_str() {
                        "Image" => crate::domain::media_asset::MediaType::Image,
                        "Video" => crate::domain::media_asset::MediaType::Video,
                        _ => panic!("Invalid media type"),
                    };
                    crate::domain::media_asset::MediaAsset {
                        id: row.id,
                        url: row.url,
                        thumbnail_url: row.thumbnail_url,
                        media_type,
                        alt_text: row.alt_text,
                        created_at: row.created_at,
                    }
                })
                .collect();

            // Load votes
            let votes = self.load_votes_for_post(row.id).await?;

            let comment = Post {
                id: row.id,
                community_id: row.community_id,
                user_id: row.user_id,
                title: row.title,
                content: row.content,
                created_at: row.created_at,
                updated_at: row.updated_at,
                parent_id: row.parent_id,
                media_assets,
                votes,
            };
            comments.push(comment);
        }

        Ok(comments)
    }

    async fn update(&self, post: &Post) -> Result<(), PostRepositoryError> {
        sqlx::query!(
            r#"
            UPDATE posts
            SET title = $1, content = $2, updated_at = $3
            WHERE id = $4
            "#,
            post.title,
            post.content,
            post.updated_at,
            post.id
        )
        .execute(&self.pool)
        .await?;

        // Delete existing media assets and insert new ones
        sqlx::query!(
            r#"
            DELETE FROM media_assets
            WHERE post_id = $1
            "#,
            post.id
        )
        .execute(&self.pool)
        .await?;

        for media_asset in &post.media_assets {
            sqlx::query!(
                r#"
                INSERT INTO media_assets (id, post_id, url, thumbnail_url, media_type, alt_text, created_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
                media_asset.id,
                post.id,
                media_asset.url,
                media_asset.thumbnail_url,
                match media_asset.media_type {
                    crate::domain::media_asset::MediaType::Image => "Image",
                    crate::domain::media_asset::MediaType::Video => "Video",
                },
                media_asset.alt_text,
                media_asset.created_at
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), PostRepositoryError> {
        sqlx::query!(
            r#"
            DELETE FROM posts
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_vote_count(&self, post_id: Uuid) -> Result<i32, PostRepositoryError> {
        let row = sqlx::query!(
            r#"
            SELECT 
                COALESCE(SUM(CASE WHEN vote_type = 'Upvote' THEN 1 ELSE 0 END), 0) as upvotes,
                COALESCE(SUM(CASE WHEN vote_type = 'Downvote' THEN 1 ELSE 0 END), 0) as downvotes
            FROM votes
            WHERE post_id = $1
            "#,
            post_id
        )
        .fetch_one(&self.pool)
        .await?;
Ok((row.upvotes - row.downvotes) as i32)
}

async fn search(&self, criteria: crate::application::search_service::SearchCriteria) -> Result<Vec<Post>, PostRepositoryError> {
// Base query with full-text search
let mut query = "SELECT id, community_id, user_id, title, content, created_at, updated_at, parent_id
                FROM posts
                WHERE search_vector @@ websearch_to_tsquery('english', $1)".to_string();

let mut params: Vec<Box<dyn postgres_types::ToSql + Sync>> = vec![
    Box::new(criteria.query)
];
let mut param_index = 2;

// Add community filter if provided
if let Some(community_id) = criteria.community_id {
    query.push_str(&format!(" AND community_id = ${}", param_index));
    params.push(Box::new(community_id));
    param_index += 1;
}

// Add author filter if provided
if let Some(author_id) = criteria.author_id {
    query.push_str(&format!(" AND user_id = ${}", param_index));
    params.push(Box::new(author_id));
    param_index += 1;
}

// Add date range filters if provided
if let Some(date_from) = criteria.date_from {
    query.push_str(&format!(" AND created_at >= ${}", param_index));
    params.push(Box::new(date_from));
    param_index += 1;
}

if let Some(date_to) = criteria.date_to {
    query.push_str(&format!(" AND created_at <= ${}", param_index));
    params.push(Box::new(date_to));
    param_index += 1;
}

// Add ordering by relevance and date
query.push_str(" ORDER BY ts_rank(search_vector, websearch_to_tsquery('english', $1)) DESC, created_at DESC");

// Add limit and offset if provided
if let Some(limit) = criteria.limit {
    query.push_str(&format!(" LIMIT ${}", param_index));
    params.push(Box::new(limit as i64));
    param_index += 1;
}

if let Some(offset) = criteria.offset {
    query.push_str(&format!(" OFFSET ${}", param_index));
    params.push(Box::new(offset as i64));
}

// Execute query
// Note: This is a simplified approach. In practice, we'd need to use a more complex
// parameter binding mechanism or a query builder.
todo!("Implement parameter binding for dynamic query")
}
}
}