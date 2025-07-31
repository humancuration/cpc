use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::vote::{Vote, VoteType};
use uuid::Uuid;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VoteRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Vote not found")]
    NotFound,
    #[error("User has already voted on this post")]
    AlreadyVoted,
}

#[async_trait]
pub trait VoteRepository: Send + Sync {
    async fn create(&self, vote: &Vote) -> Result<(), VoteRepositoryError>;
    async fn find_by_user_and_post(&self, user_id: Uuid, post_id: Uuid) -> Result<Option<Vote>, VoteRepositoryError>;
    async fn update(&self, vote: &Vote) -> Result<(), VoteRepositoryError>;
    async fn delete(&self, user_id: Uuid, post_id: Uuid) -> Result<(), VoteRepositoryError>;
    async fn get_vote_count(&self, post_id: Uuid) -> Result<i32, VoteRepositoryError>;
}

pub struct PgVoteRepository {
    pool: PgPool,
}

impl PgVoteRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl VoteRepository for PgVoteRepository {
    async fn create(&self, vote: &Vote) -> Result<(), VoteRepositoryError> {
        // Check if user has already voted on this post
        if self.find_by_user_and_post(vote.user_id, vote.post_id).await?.is_some() {
            return Err(VoteRepositoryError::AlreadyVoted);
        }
        
        sqlx::query!(
            r#"
            INSERT INTO votes (id, user_id, post_id, vote_type, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            vote.id,
            vote.user_id,
            vote.post_id,
            match vote.vote_type {
                VoteType::Upvote => "Upvote",
                VoteType::Downvote => "Downvote",
            },
            vote.created_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn find_by_user_and_post(&self, user_id: Uuid, post_id: Uuid) -> Result<Option<Vote>, VoteRepositoryError> {
        let row = sqlx::query!(
            r#"
            SELECT id, user_id, post_id, vote_type as "vote_type: String", created_at
            FROM votes
            WHERE user_id = $1 AND post_id = $2
            "#,
            user_id,
            post_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                let vote_type = match row.vote_type.as_str() {
                    "Upvote" => VoteType::Upvote,
                    "Downvote" => VoteType::Downvote,
                    _ => panic!("Invalid vote type"),
                };
                let vote = Vote {
                    id: row.id,
                    user_id: row.user_id,
                    post_id: row.post_id,
                    vote_type,
                    created_at: row.created_at,
                };
                Ok(Some(vote))
            }
            None => Ok(None),
        }
    }
    
    async fn update(&self, vote: &Vote) -> Result<(), VoteRepositoryError> {
        sqlx::query!(
            r#"
            UPDATE votes
            SET vote_type = $1
            WHERE user_id = $2 AND post_id = $3
            "#,
            match vote.vote_type {
                VoteType::Upvote => "Upvote",
                VoteType::Downvote => "Downvote",
            },
            vote.user_id,
            vote.post_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn delete(&self, user_id: Uuid, post_id: Uuid) -> Result<(), VoteRepositoryError> {
        sqlx::query!(
            r#"
            DELETE FROM votes
            WHERE user_id = $1 AND post_id = $2
            "#,
            user_id,
            post_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_vote_count(&self, post_id: Uuid) -> Result<i32, VoteRepositoryError> {
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
}