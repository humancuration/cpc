//! PostgreSQL repository implementations for social interactions
//!
//! This module provides concrete implementations of the repository traits using PostgreSQL.

use crate::domain::models::{Reaction, Comment, Share, TargetType, ContentType, ReactionType};
use crate::domain::repository::{ReactionRepository, CommentRepository, ShareRepository, RepositoryError};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use std::collections::HashMap;
use serde_json::Value;

/// PostgreSQL implementation of ReactionRepository
pub struct PostgresReactionRepository {
    pool: PgPool,
}

impl PostgresReactionRepository {
    /// Create a new PostgresReactionRepository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReactionRepository for PostgresReactionRepository {
    async fn add_reaction(&self, reaction: &Reaction) -> Result<(), RepositoryError> {
        let target_type_str = reaction.target_type.to_string();
        let reaction_type_str = reaction.reaction_type.to_string();
        
        sqlx::query!(
            r#"
            INSERT INTO reactions (id, user_id, target_id, target_type, reaction_type, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            reaction.id,
            reaction.user_id,
            reaction.target_id,
            target_type_str,
            reaction_type_str,
            reaction.created_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn remove_reaction(&self, reaction_id: Uuid) -> Result<(), RepositoryError> {
        sqlx::query!(
            "DELETE FROM reactions WHERE id = $1",
            reaction_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn get_reactions_for_target(
        &self,
        target_id: Uuid,
        target_type: TargetType,
    ) -> Result<Vec<Reaction>, RepositoryError> {
        let target_type_str = target_type.to_string();
        
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, target_id, target_type::text, reaction_type::text, created_at
            FROM reactions
            WHERE target_id = $1 AND target_type = $2
            ORDER BY created_at DESC
            "#,
            target_id,
            target_type_str
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        let mut reactions = Vec::new();
        for row in rows {
            let target_type = match row.target_type.as_str() {
                "post" => TargetType::Post,
                "comment" => TargetType::Comment,
                "achievement" => TargetType::Achievement,
                "volunteer_activity" => TargetType::VolunteerActivity,
                "skill_exchange" => TargetType::SkillExchange,
                _ => return Err(RepositoryError::ValidationError("Invalid target type".to_string())),
            };
            
            let reaction_type = match row.reaction_type.as_str() {
                "like" => ReactionType::Like,
                "heart" => ReactionType::Heart,
                "celebrate" => ReactionType::Celebrate,
                "insightful" => ReactionType::Insightful,
                "funny" => ReactionType::Funny,
                "sad" => ReactionType::Sad,
                "angry" => ReactionType::Angry,
                _ => return Err(RepositoryError::ValidationError("Invalid reaction type".to_string())),
            };
            
            reactions.push(Reaction {
                id: row.id,
                user_id: row.user_id,
                target_id: row.target_id,
                target_type,
                reaction_type,
                created_at: row.created_at,
            });
        }
        
        Ok(reactions)
    }
    
    async fn get_reaction_summary(
        &self,
        target_id: Uuid,
        target_type: TargetType,
    ) -> Result<HashMap<String, usize>, RepositoryError> {
        let target_type_str = target_type.to_string();
        
        let rows = sqlx::query!(
            r#"
            SELECT reaction_type::text, COUNT(*) as count
            FROM reactions
            WHERE target_id = $1 AND target_type = $2
            GROUP BY reaction_type
            "#,
            target_id,
            target_type_str
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        let mut summary = HashMap::new();
        for row in rows {
            summary.insert(row.reaction_type, row.count.unwrap_or(0) as usize);
        }
        
        Ok(summary)
    }
    
    async fn user_has_reacted(
        &self,
        user_id: Uuid,
        target_id: Uuid,
        target_type: TargetType,
    ) -> Result<bool, RepositoryError> {
        let target_type_str = target_type.to_string();
        
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as count
            FROM reactions
            WHERE user_id = $1 AND target_id = $2 AND target_type = $3
            "#,
            user_id,
            target_id,
            target_type_str
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        Ok(count.count.unwrap_or(0) > 0)
    }
}

/// PostgreSQL implementation of CommentRepository
pub struct PostgresCommentRepository {
    pool: PgPool,
}

impl PostgresCommentRepository {
    /// Create a new PostgresCommentRepository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CommentRepository for PostgresCommentRepository {
    async fn add_comment(&self, comment: &Comment) -> Result<(), RepositoryError> {
        let target_type_str = comment.target_type.to_string();
        
        sqlx::query!(
            r#"
            INSERT INTO comments (id, user_id, parent_id, target_id, target_type, content, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            comment.id,
            comment.user_id,
            comment.parent_id,
            comment.target_id,
            target_type_str,
            comment.content,
            comment.created_at,
            comment.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn update_comment(&self, comment: &Comment) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            UPDATE comments
            SET content = $1, updated_at = $2
            WHERE id = $3
            "#,
            comment.content,
            comment.updated_at,
            comment.id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn delete_comment(&self, comment_id: Uuid) -> Result<(), RepositoryError> {
        sqlx::query!(
            "DELETE FROM comments WHERE id = $1",
            comment_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn get_comment(&self, comment_id: Uuid) -> Result<Option<Comment>, RepositoryError> {
        let row = sqlx::query!(
            r#"
            SELECT id, user_id, parent_id, target_id, target_type::text, content, created_at, updated_at
            FROM comments
            WHERE id = $1
            "#,
            comment_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        match row {
            Some(row) => {
                let target_type = match row.target_type.as_str() {
                    "post" => TargetType::Post,
                    "comment" => TargetType::Comment,
                    "achievement" => TargetType::Achievement,
                    "volunteer_activity" => TargetType::VolunteerActivity,
                    "skill_exchange" => TargetType::SkillExchange,
                    _ => return Err(RepositoryError::ValidationError("Invalid target type".to_string())),
                };
                
                Ok(Some(Comment {
                    id: row.id,
                    user_id: row.user_id,
                    parent_id: row.parent_id,
                    target_id: row.target_id,
                    target_type,
                    content: row.content,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                }))
            }
            None => Ok(None),
        }
    }
    
    async fn get_comments_for_target(
        &self,
        target_id: Uuid,
        target_type: TargetType,
        max_depth: Option<usize>,
    ) -> Result<Vec<Comment>, RepositoryError> {
        let target_type_str = target_type.to_string();
        
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, parent_id, target_id, target_type::text, content, created_at, updated_at
            FROM comments
            WHERE target_id = $1 AND target_type = $2
            ORDER BY created_at ASC
            "#,
            target_id,
            target_type_str
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        let mut comments = Vec::new();
        for row in rows {
            let target_type = match row.target_type.as_str() {
                "post" => TargetType::Post,
                "comment" => TargetType::Comment,
                "achievement" => TargetType::Achievement,
                "volunteer_activity" => TargetType::VolunteerActivity,
                "skill_exchange" => TargetType::SkillExchange,
                _ => return Err(RepositoryError::ValidationError("Invalid target type".to_string())),
            };
            
            comments.push(Comment {
                id: row.id,
                user_id: row.user_id,
                parent_id: row.parent_id,
                target_id: row.target_id,
                target_type,
                content: row.content,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }
        
        // TODO: Implement depth filtering for nested comments
        // For now, we return all comments
        Ok(comments)
    }
    
    async fn get_replies(&self, comment_id: Uuid) -> Result<Vec<Comment>, RepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, parent_id, target_id, target_type::text, content, created_at, updated_at
            FROM comments
            WHERE parent_id = $1
            ORDER BY created_at ASC
            "#,
            comment_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        let mut replies = Vec::new();
        for row in rows {
            let target_type = match row.target_type.as_str() {
                "post" => TargetType::Post,
                "comment" => TargetType::Comment,
                "achievement" => TargetType::Achievement,
                "volunteer_activity" => TargetType::VolunteerActivity,
                "skill_exchange" => TargetType::SkillExchange,
                _ => return Err(RepositoryError::ValidationError("Invalid target type".to_string())),
            };
            
            replies.push(Comment {
                id: row.id,
                user_id: row.user_id,
                parent_id: row.parent_id,
                target_id: row.target_id,
                target_type,
                content: row.content,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }
        
        Ok(replies)
    }
}

/// PostgreSQL implementation of ShareRepository
pub struct PostgresShareRepository {
    pool: PgPool,
}

impl PostgresShareRepository {
    /// Create a new PostgresShareRepository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ShareRepository for PostgresShareRepository {
    async fn add_share(&self, share: &Share) -> Result<(), RepositoryError> {
        let content_type_str = share.content_type.to_string();
        
        sqlx::query!(
            r#"
            INSERT INTO shares (id, user_id, content_id, content_type, shared_with, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            share.id,
            share.user_id,
            share.content_id,
            content_type_str,
            share.shared_with,
            share.created_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn get_shares_by_user(&self, user_id: Uuid) -> Result<Vec<Share>, RepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, content_id, content_type::text, shared_with, created_at
            FROM shares
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        let mut shares = Vec::new();
        for row in rows {
            let content_type = match row.content_type.as_str() {
                "post" => ContentType::Post,
                "achievement" => ContentType::Achievement,
                "volunteer_activity" => ContentType::VolunteerActivity,
                "skill_exchange" => ContentType::SkillExchange,
                "comment" => ContentType::Comment,
                _ => return Err(RepositoryError::ValidationError("Invalid content type".to_string())),
            };
            
            shares.push(Share {
                id: row.id,
                user_id: row.user_id,
                content_id: row.content_id,
                content_type,
                shared_with: row.shared_with,
                created_at: row.created_at,
            });
        }
        
        Ok(shares)
    }
    
    async fn get_shares_of_content(
        &self,
        content_id: Uuid,
        content_type: ContentType,
    ) -> Result<Vec<Share>, RepositoryError> {
        let content_type_str = content_type.to_string();
        
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, content_id, content_type::text, shared_with, created_at
            FROM shares
            WHERE content_id = $1 AND content_type = $2
            ORDER BY created_at DESC
            "#,
            content_id,
            content_type_str
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        let mut shares = Vec::new();
        for row in rows {
            let content_type = match row.content_type.as_str() {
                "post" => ContentType::Post,
                "achievement" => ContentType::Achievement,
                "volunteer_activity" => ContentType::VolunteerActivity,
                "skill_exchange" => ContentType::SkillExchange,
                "comment" => ContentType::Comment,
                _ => return Err(RepositoryError::ValidationError("Invalid content type".to_string())),
            };
            
            shares.push(Share {
                id: row.id,
                user_id: row.user_id,
                content_id: row.content_id,
                content_type,
                shared_with: row.shared_with,
                created_at: row.created_at,
            });
        }
        
        Ok(shares)
    }
    
    async fn delete_share(&self, share_id: Uuid) -> Result<(), RepositoryError> {
        sqlx::query!(
            "DELETE FROM shares WHERE id = $1",
            share_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
}