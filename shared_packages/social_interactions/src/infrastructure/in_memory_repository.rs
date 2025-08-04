//! In-memory repository implementations for social interactions
//!
//! This module provides in-memory implementations of the repository traits for testing purposes.

use crate::domain::models::{Reaction, Comment, Share, TargetType, ContentType, ReactionType};
use crate::domain::repository::{ReactionRepository, CommentRepository, ShareRepository, RepositoryError};
use async_trait::async_trait;
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use chrono::{DateTime, Utc};

/// In-memory implementation of ReactionRepository
pub struct InMemoryReactionRepository {
    reactions: Arc<Mutex<HashMap<Uuid, Reaction>>>,
    target_reactions: Arc<Mutex<HashMap<(Uuid, TargetType), Vec<Uuid>>>>,
}

impl InMemoryReactionRepository {
    /// Create a new InMemoryReactionRepository
    pub fn new() -> Self {
        Self {
            reactions: Arc::new(Mutex::new(HashMap::new())),
            target_reactions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Clear all reactions (for testing)
    pub fn clear(&self) {
        let mut reactions = self.reactions.lock().unwrap();
        let mut target_reactions = self.target_reactions.lock().unwrap();
        reactions.clear();
        target_reactions.clear();
    }
}

#[async_trait]
impl ReactionRepository for InMemoryReactionRepository {
    async fn add_reaction(&self, reaction: &Reaction) -> Result<(), RepositoryError> {
        let mut reactions = self.reactions.lock().unwrap();
        let mut target_reactions = self.target_reactions.lock().unwrap();
        
        reactions.insert(reaction.id, reaction.clone());
        target_reactions
            .entry((reaction.target_id, reaction.target_type.clone()))
            .or_insert_with(Vec::new)
            .push(reaction.id);
        
        Ok(())
    }
    
    async fn remove_reaction(&self, reaction_id: Uuid) -> Result<(), RepositoryError> {
        let mut reactions = self.reactions.lock().unwrap();
        let mut target_reactions = self.target_reactions.lock().unwrap();
        
        if let Some(reaction) = reactions.remove(&reaction_id) {
            if let Some(reaction_list) = target_reactions.get_mut(&(reaction.target_id, reaction.target_type)) {
                reaction_list.retain(|id| *id != reaction_id);
            }
        } else {
            return Err(RepositoryError::NotFound);
        }
        
        Ok(())
    }
    
    async fn get_reactions_for_target(
        &self,
        target_id: Uuid,
        target_type: TargetType,
    ) -> Result<Vec<Reaction>, RepositoryError> {
        let reactions = self.reactions.lock().unwrap();
        let target_reactions = self.target_reactions.lock().unwrap();
        
        if let Some(reaction_ids) = target_reactions.get(&(target_id, target_type)) {
            let mut result = Vec::new();
            for id in reaction_ids {
                if let Some(reaction) = reactions.get(id) {
                    result.push(reaction.clone());
                }
            }
            Ok(result)
        } else {
            Ok(Vec::new())
        }
    }
    
    async fn get_reaction_summary(
        &self,
        target_id: Uuid,
        target_type: TargetType,
    ) -> Result<HashMap<String, usize>, RepositoryError> {
        let reactions = self.reactions.lock().unwrap();
        let target_reactions = self.target_reactions.lock().unwrap();
        
        let mut summary = HashMap::new();
        
        if let Some(reaction_ids) = target_reactions.get(&(target_id, target_type)) {
            for reaction_id in reaction_ids {
                if let Some(reaction) = reactions.get(reaction_id) {
                    let count = summary.entry(reaction.reaction_type.to_string()).or_insert(0);
                    *count += 1;
                }
            }
        }
        
        Ok(summary)
    }
    
    async fn user_has_reacted(
        &self,
        user_id: Uuid,
        target_id: Uuid,
        target_type: TargetType,
    ) -> Result<bool, RepositoryError> {
        let reactions = self.reactions.lock().unwrap();
        let target_reactions = self.target_reactions.lock().unwrap();
        
        if let Some(reaction_ids) = target_reactions.get(&(target_id, target_type)) {
            for reaction_id in reaction_ids {
                if let Some(reaction) = reactions.get(reaction_id) {
                    if reaction.user_id == user_id {
                        return Ok(true);
                    }
                }
            }
        }
        
        Ok(false)
    }
}

/// In-memory implementation of CommentRepository
pub struct InMemoryCommentRepository {
    comments: Arc<Mutex<HashMap<Uuid, Comment>>>,
    target_comments: Arc<Mutex<HashMap<(Uuid, TargetType), Vec<Uuid>>>>,
    comment_replies: Arc<Mutex<HashMap<Uuid, Vec<Uuid>>>>,
}

impl InMemoryCommentRepository {
    /// Create a new InMemoryCommentRepository
    pub fn new() -> Self {
        Self {
            comments: Arc::new(Mutex::new(HashMap::new())),
            target_comments: Arc::new(Mutex::new(HashMap::new())),
            comment_replies: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Clear all comments (for testing)
    pub fn clear(&self) {
        let mut comments = self.comments.lock().unwrap();
        let mut target_comments = self.target_comments.lock().unwrap();
        let mut comment_replies = self.comment_replies.lock().unwrap();
        comments.clear();
        target_comments.clear();
        comment_replies.clear();
    }
}

#[async_trait]
impl CommentRepository for InMemoryCommentRepository {
    async fn add_comment(&self, comment: &Comment) -> Result<(), RepositoryError> {
        let mut comments = self.comments.lock().unwrap();
        let mut target_comments = self.target_comments.lock().unwrap();
        let mut comment_replies = self.comment_replies.lock().unwrap();
        
        comments.insert(comment.id, comment.clone());
        target_comments
            .entry((comment.target_id, comment.target_type.clone()))
            .or_insert_with(Vec::new)
            .push(comment.id);
        
        if let Some(parent_id) = comment.parent_id {
            comment_replies
                .entry(parent_id)
                .or_insert_with(Vec::new)
                .push(comment.id);
        }
        
        Ok(())
    }
    
    async fn update_comment(&self, comment: &Comment) -> Result<(), RepositoryError> {
        let mut comments = self.comments.lock().unwrap();
        
        if comments.contains_key(&comment.id) {
            comments.insert(comment.id, comment.clone());
            Ok(())
        } else {
            Err(RepositoryError::NotFound)
        }
    }
    
    async fn delete_comment(&self, comment_id: Uuid) -> Result<(), RepositoryError> {
        let mut comments = self.comments.lock().unwrap();
        let mut target_comments = self.target_comments.lock().unwrap();
        let mut comment_replies = self.comment_replies.lock().unwrap();
        
        if let Some(comment) = comments.remove(&comment_id) {
            // Remove from target comments
            if let Some(comment_list) = target_comments.get_mut(&(comment.target_id, comment.target_type)) {
                comment_list.retain(|id| *id != comment_id);
            }
            
            // Remove from parent's replies
            if let Some(parent_id) = comment.parent_id {
                if let Some(reply_list) = comment_replies.get_mut(&parent_id) {
                    reply_list.retain(|id| *id != comment_id);
                }
            }
            
            // Remove this comment's replies
            if let Some(reply_list) = comment_replies.remove(&comment_id) {
                for reply_id in reply_list {
                    comments.remove(&reply_id);
                }
            }
            
            Ok(())
        } else {
            Err(RepositoryError::NotFound)
        }
    }
    
    async fn get_comment(&self, comment_id: Uuid) -> Result<Option<Comment>, RepositoryError> {
        let comments = self.comments.lock().unwrap();
        Ok(comments.get(&comment_id).cloned())
    }
    
    async fn get_comments_for_target(
        &self,
        target_id: Uuid,
        target_type: TargetType,
        _max_depth: Option<usize>,
    ) -> Result<Vec<Comment>, RepositoryError> {
        let comments = self.comments.lock().unwrap();
        let target_comments = self.target_comments.lock().unwrap();
        
        if let Some(comment_ids) = target_comments.get(&(target_id, target_type)) {
            let mut result = Vec::new();
            for id in comment_ids {
                if let Some(comment) = comments.get(id) {
                    result.push(comment.clone());
                }
            }
            Ok(result)
        } else {
            Ok(Vec::new())
        }
    }
    
    async fn get_replies(&self, comment_id: Uuid) -> Result<Vec<Comment>, RepositoryError> {
        let comments = self.comments.lock().unwrap();
        let comment_replies = self.comment_replies.lock().unwrap();
        
        if let Some(reply_ids) = comment_replies.get(&comment_id) {
            let mut result = Vec::new();
            for id in reply_ids {
                if let Some(comment) = comments.get(id) {
                    result.push(comment.clone());
                }
            }
            Ok(result)
        } else {
            Ok(Vec::new())
        }
    }
}

/// In-memory implementation of ShareRepository
pub struct InMemoryShareRepository {
    shares: Arc<Mutex<HashMap<Uuid, Share>>>,
    user_shares: Arc<Mutex<HashMap<Uuid, Vec<Uuid>>>>,
    content_shares: Arc<Mutex<HashMap<(Uuid, ContentType), Vec<Uuid>>>>,
}

impl InMemoryShareRepository {
    /// Create a new InMemoryShareRepository
    pub fn new() -> Self {
        Self {
            shares: Arc::new(Mutex::new(HashMap::new())),
            user_shares: Arc::new(Mutex::new(HashMap::new())),
            content_shares: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Clear all shares (for testing)
    pub fn clear(&self) {
        let mut shares = self.shares.lock().unwrap();
        let mut user_shares = self.user_shares.lock().unwrap();
        let mut content_shares = self.content_shares.lock().unwrap();
        shares.clear();
        user_shares.clear();
        content_shares.clear();
    }
}

#[async_trait]
impl ShareRepository for InMemoryShareRepository {
    async fn add_share(&self, share: &Share) -> Result<(), RepositoryError> {
        let mut shares = self.shares.lock().unwrap();
        let mut user_shares = self.user_shares.lock().unwrap();
        let mut content_shares = self.content_shares.lock().unwrap();
        
        shares.insert(share.id, share.clone());
        user_shares
            .entry(share.user_id)
            .or_insert_with(Vec::new)
            .push(share.id);
        content_shares
            .entry((share.content_id, share.content_type.clone()))
            .or_insert_with(Vec::new)
            .push(share.id);
        
        Ok(())
    }
    
    async fn get_shares_by_user(&self, user_id: Uuid) -> Result<Vec<Share>, RepositoryError> {
        let shares = self.shares.lock().unwrap();
        let user_shares = self.user_shares.lock().unwrap();
        
        if let Some(share_ids) = user_shares.get(&user_id) {
            let mut result = Vec::new();
            for id in share_ids {
                if let Some(share) = shares.get(id) {
                    result.push(share.clone());
                }
            }
            Ok(result)
        } else {
            Ok(Vec::new())
        }
    }
    
    async fn get_shares_of_content(
        &self,
        content_id: Uuid,
        content_type: ContentType,
    ) -> Result<Vec<Share>, RepositoryError> {
        let shares = self.shares.lock().unwrap();
        let content_shares = self.content_shares.lock().unwrap();
        
        if let Some(share_ids) = content_shares.get(&(content_id, content_type)) {
            let mut result = Vec::new();
            for id in share_ids {
                if let Some(share) = shares.get(id) {
                    result.push(share.clone());
                }
            }
            Ok(result)
        } else {
            Ok(Vec::new())
        }
    }
    
    async fn delete_share(&self, share_id: Uuid) -> Result<(), RepositoryError> {
        let mut shares = self.shares.lock().unwrap();
        let mut user_shares = self.user_shares.lock().unwrap();
        let mut content_shares = self.content_shares.lock().unwrap();
        
        if let Some(share) = shares.remove(&share_id) {
            // Remove from user shares
            if let Some(share_list) = user_shares.get_mut(&share.user_id) {
                share_list.retain(|id| *id != share_id);
            }
            
            // Remove from content shares
            if let Some(share_list) = content_shares.get_mut(&(share.content_id, share.content_type)) {
                share_list.retain(|id| *id != share_id);
            }
            
            Ok(())
        } else {
            Err(RepositoryError::NotFound)
        }
    }
}