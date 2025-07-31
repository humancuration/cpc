use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::comment::Comment;
use crate::domain::vote::Vote;
use crate::domain::vote::VoteType;
use uuid::Uuid;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommentRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Comment not found")]
    NotFound,
}

#[async_trait]
pub trait CommentRepository: Send + Sync {
    async fn create(&self, comment: &Comment) -> Result<(), CommentRepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Comment>, CommentRepositoryError>;
    async fn find_by_post(&self, post_id: Uuid) -> Result<Vec<Comment>, CommentRepositoryError>;
    async fn find_replies(&self, comment_id: Uuid) -> Result<Vec<Comment>, CommentRepositoryError>;
    async fn update(&self, comment: &Comment) -> Result<(), CommentRepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<(), CommentRepositoryError>;
    async fn get_vote_count(&self, comment_id: Uuid) -> Result<i32, CommentRepositoryError>;
}

pub struct PgCommentRepository {
    pool: PgPool,
}

impl PgCommentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn load_votes_for_comment(&self, comment_id: Uuid) -> Result<Vec<Vote>, CommentRepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, post_id, vote_type as "vote_type: String", created_at
            FROM votes
            WHERE post_id = $1
            "#,
            comment_id
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
impl CommentRepository for PgCommentRepository {
    async fn create(&self, comment: &Comment) -> Result<(), CommentRepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO posts (id, community_id, user_id, title, content, created_at, updated_at, parent_id)
            VALUES ($1, (SELECT community_id FROM posts WHERE id = $2), $3, '', $4, $5, $6, $7)
            "#,
            comment.id,
            comment.post_id,
            comment.user_id,
            comment.content,
            comment.created_at,
            comment.updated_at,
            comment.parent_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Comment>, CommentRepositoryError> {
        let row = sqlx::query!(
            r#"
            SELECT id, community_id, user_id, content, created_at, updated_at, parent_id
            FROM posts
            WHERE id = $1 AND parent_id IS NOT NULL
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                // Load votes
                let votes = self.load_votes_for_comment(id).await?;

                let comment = Comment {
                    id: row.id,
                    post_id: row.community_id, // This is actually the parent post ID for comments
                    user_id: row.user_id,
                    content: row.content,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    parent_id: row.parent_id,
                    votes,
                };
                Ok(Some(comment))
            }
            None => Ok(None),
        }
    }

    async fn find_by_post(&self, post_id: Uuid) -> Result<Vec<Comment>, CommentRepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, community_id, user_id, content, created_at, updated_at, parent_id
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
            // Load votes
            let votes = self.load_votes_for_comment(row.id).await?;

            let comment = Comment {
                id: row.id,
                post_id: row.community_id, // This is actually the parent post ID for comments
                user_id: row.user_id,
                content: row.content,
                created_at: row.created_at,
                updated_at: row.updated_at,
                parent_id: row.parent_id,
                votes,
            };
            comments.push(comment);
        }

        Ok(comments)
    }

    async fn find_replies(&self, comment_id: Uuid) -> Result<Vec<Comment>, CommentRepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, community_id, user_id, content, created_at, updated_at, parent_id
            FROM posts
            WHERE parent_id = $1
            ORDER BY created_at ASC
            "#,
            comment_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut replies = Vec::new();
        for row in rows {
            // Load votes
            let votes = self.load_votes_for_comment(row.id).await?;

            let reply = Comment {
                id: row.id,
                post_id: row.community_id, // This is actually the parent post ID for comments
                user_id: row.user_id,
                content: row.content,
                created_at: row.created_at,
                updated_at: row.updated_at,
                parent_id: row.parent_id,
                votes,
            };
            replies.push(reply);
        }

        Ok(replies)
    }

    async fn update(&self, comment: &Comment) -> Result<(), CommentRepositoryError> {
        sqlx::query!(
            r#"
            UPDATE posts
            SET content = $1, updated_at = $2
            WHERE id = $3
            "#,
            comment.content,
            comment.updated_at,
            comment.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), CommentRepositoryError> {
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

    async fn get_vote_count(&self, comment_id: Uuid) -> Result<i32, CommentRepositoryError> {
        let row = sqlx::query!(
            r#"
            SELECT 
                COALESCE(SUM(CASE WHEN vote_type = 'Upvote' THEN 1 ELSE 0 END), 0) as upvotes,
                COALESCE(SUM(CASE WHEN vote_type = 'Downvote' THEN 1 ELSE 0 END), 0) as downvotes
            FROM votes
            WHERE post_id = $1
            "#,
            comment_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok((row.upvotes - row.downvotes) as i32)
    }
}