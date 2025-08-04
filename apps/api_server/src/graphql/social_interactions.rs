//! GraphQL mutations and queries for social interactions

use async_graphql::{Context, Object, InputObject, Result, SimpleObject, ID};
use uuid::Uuid;
use shared_packages::social_interactions::{
    domain::models::{Reaction, Comment, Share, ReactionType, TargetType, ContentType},
    application::{ReactionService, CommentService, ShareService},
};

/// Input for adding a reaction
#[derive(InputObject)]
struct AddReactionInput {
    /// The ID of the target content
    target_id: ID,
    /// The type of target content
    target_type: String,
    /// The type of reaction
    reaction_type: String,
}

/// Input for removing a reaction
#[derive(InputObject)]
struct RemoveReactionInput {
    /// The ID of the reaction to remove
    reaction_id: ID,
}

/// Input for adding a comment
#[derive(InputObject)]
struct AddCommentInput {
    /// The ID of the target content
    target_id: ID,
    /// The type of target content
    target_type: String,
    /// The content of the comment
    content: String,
    /// The ID of the parent comment (for replies)
    parent_id: Option<ID>,
}

/// Input for editing a comment
#[derive(InputObject)]
struct EditCommentInput {
    /// The ID of the comment to edit
    comment_id: ID,
    /// The new content of the comment
    content: String,
}

/// Input for deleting a comment
#[derive(InputObject)]
struct DeleteCommentInput {
    /// The ID of the comment to delete
    comment_id: ID,
}

/// Input for sharing content
#[derive(InputObject)]
struct ShareContentInput {
    /// The ID of the content to share
    content_id: ID,
    /// The type of content to share
    content_type: String,
    /// The ID of the user to share with (None for public)
    shared_with: Option<ID>,
}

/// Input for unsharing content
#[derive(InputObject)]
struct UnshareContentInput {
    /// The ID of the share to remove
    share_id: ID,
}

/// GraphQL representation of a reaction
#[derive(SimpleObject)]
struct ReactionDto {
    id: ID,
    user_id: ID,
    target_id: ID,
    target_type: String,
    reaction_type: String,
    created_at: String,
}

/// GraphQL representation of a comment
#[derive(SimpleObject)]
struct CommentDto {
    id: ID,
    user_id: ID,
    parent_id: Option<ID>,
    target_id: ID,
    target_type: String,
    content: String,
    created_at: String,
    updated_at: Option<String>,
}

/// GraphQL representation of a share
#[derive(SimpleObject)]
struct ShareDto {
    id: ID,
    user_id: ID,
    content_id: ID,
    content_type: String,
    shared_with: Option<ID>,
    created_at: String,
    is_public: bool,
}

/// Reaction summary for a target
#[derive(SimpleObject)]
struct ReactionSummary {
    target_id: ID,
    target_type: String,
    reactions: Vec<ReactionCount>,
}

/// Count of reactions by type
#[derive(SimpleObject)]
struct ReactionCount {
    reaction_type: String,
    count: usize,
}

/// Social interaction mutations
#[derive(Default)]
pub struct SocialInteractionMutations;

#[Object]
impl SocialInteractionMutations {
    /// Add a reaction to content
    async fn add_reaction(
        &self,
        ctx: &Context<'_>,
        input: AddReactionInput,
    ) -> Result<ReactionDto> {
        // Get the current user ID from context
        let user_id = ctx.data::<Uuid>().unwrap().clone();
        
        // Parse the target ID
        let target_id = Uuid::parse_str(&input.target_id).map_err(|_| "Invalid target ID")?;
        
        // Parse the target type
        let target_type = match input.target_type.as_str() {
            "post" => TargetType::Post,
            "comment" => TargetType::Comment,
            "achievement" => TargetType::Achievement,
            "volunteer_activity" => TargetType::VolunteerActivity,
            "skill_exchange" => TargetType::SkillExchange,
            _ => return Err("Invalid target type".into()),
        };
        
        // Parse the reaction type
        let reaction_type = match input.reaction_type.as_str() {
            "like" => ReactionType::Like,
            "heart" => ReactionType::Heart,
            "celebrate" => ReactionType::Celebrate,
            "insightful" => ReactionType::Insightful,
            "funny" => ReactionType::Funny,
            "sad" => ReactionType::Sad,
            "angry" => ReactionType::Angry,
            _ => return Err("Invalid reaction type".into()),
        };
        
        // Get the reaction service from context
        let reaction_service = ctx.data::<ReactionService>().unwrap();
        
        // Add the reaction
        let reaction = reaction_service
            .add_reaction(user_id, target_id, target_type, reaction_type)
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(ReactionDto {
            id: reaction.id.to_string(),
            user_id: reaction.user_id.to_string(),
            target_id: reaction.target_id.to_string(),
            target_type: reaction.target_type.to_string(),
            reaction_type: reaction.reaction_type.to_string(),
            created_at: reaction.created_at.to_rfc3339(),
        })
    }
    
    /// Remove a reaction
    async fn remove_reaction(
        &self,
        ctx: &Context<'_>,
        input: RemoveReactionInput,
    ) -> Result<bool> {
        // Get the current user ID from context
        let user_id = ctx.data::<Uuid>().unwrap().clone();
        
        // Parse the reaction ID
        let reaction_id = Uuid::parse_str(&input.reaction_id).map_err(|_| "Invalid reaction ID")?;
        
        // Get the reaction service from context
        let reaction_service = ctx.data::<ReactionService>().unwrap();
        
        // Remove the reaction
        reaction_service
            .remove_reaction(user_id, reaction_id)
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(true)
    }
    
    /// Add a comment to content
    async fn add_comment(
        &self,
        ctx: &Context<'_>,
        input: AddCommentInput,
    ) -> Result<CommentDto> {
        // Get the current user ID from context
        let user_id = ctx.data::<Uuid>().unwrap().clone();
        
        // Parse the target ID
        let target_id = Uuid::parse_str(&input.target_id).map_err(|_| "Invalid target ID")?;
        
        // Parse the target type
        let target_type = match input.target_type.as_str() {
            "post" => TargetType::Post,
            "comment" => TargetType::Comment,
            "achievement" => TargetType::Achievement,
            "volunteer_activity" => TargetType::VolunteerActivity,
            "skill_exchange" => TargetType::SkillExchange,
            _ => return Err("Invalid target type".into()),
        };
        
        // Parse the parent ID if provided
        let parent_id = input.parent_id.map(|id| Uuid::parse_str(&id).unwrap());
        
        // Get the comment service from context
        let comment_service = ctx.data::<CommentService>().unwrap();
        
        // Add the comment
        let comment = comment_service
            .add_comment(user_id, target_id, target_type, input.content, parent_id)
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(CommentDto {
            id: comment.id.to_string(),
            user_id: comment.user_id.to_string(),
            parent_id: comment.parent_id.map(|id| id.to_string()),
            target_id: comment.target_id.to_string(),
            target_type: comment.target_type.to_string(),
            content: comment.content,
            created_at: comment.created_at.to_rfc3339(),
            updated_at: comment.updated_at.map(|dt| dt.to_rfc3339()),
        })
    }
    
    /// Edit a comment
    async fn edit_comment(
        &self,
        ctx: &Context<'_>,
        input: EditCommentInput,
    ) -> Result<CommentDto> {
        // Get the current user ID from context
        let user_id = ctx.data::<Uuid>().unwrap().clone();
        
        // Parse the comment ID
        let comment_id = Uuid::parse_str(&input.comment_id).map_err(|_| "Invalid comment ID")?;
        
        // Get the comment service from context
        let comment_service = ctx.data::<CommentService>().unwrap();
        
        // Edit the comment
        let comment = comment_service
            .edit_comment(user_id, comment_id, input.content)
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(CommentDto {
            id: comment.id.to_string(),
            user_id: comment.user_id.to_string(),
            parent_id: comment.parent_id.map(|id| id.to_string()),
            target_id: comment.target_id.to_string(),
            target_type: comment.target_type.to_string(),
            content: comment.content,
            created_at: comment.created_at.to_rfc3339(),
            updated_at: comment.updated_at.map(|dt| dt.to_rfc3339()),
        })
    }
    
    /// Delete a comment
    async fn delete_comment(
        &self,
        ctx: &Context<'_>,
        input: DeleteCommentInput,
    ) -> Result<bool> {
        // Get the current user ID from context
        let user_id = ctx.data::<Uuid>().unwrap().clone();
        
        // Parse the comment ID
        let comment_id = Uuid::parse_str(&input.comment_id).map_err(|_| "Invalid comment ID")?;
        
        // Get the comment service from context
        let comment_service = ctx.data::<CommentService>().unwrap();
        
        // Delete the comment
        comment_service
            .delete_comment(user_id, comment_id)
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(true)
    }
    
    /// Share content
    async fn share_content(
        &self,
        ctx: &Context<'_>,
        input: ShareContentInput,
    ) -> Result<ShareDto> {
        // Get the current user ID from context
        let user_id = ctx.data::<Uuid>().unwrap().clone();
        
        // Parse the content ID
        let content_id = Uuid::parse_str(&input.content_id).map_err(|_| "Invalid content ID")?;
        
        // Parse the content type
        let content_type = match input.content_type.as_str() {
            "post" => ContentType::Post,
            "achievement" => ContentType::Achievement,
            "volunteer_activity" => ContentType::VolunteerActivity,
            "skill_exchange" => ContentType::SkillExchange,
            "comment" => ContentType::Comment,
            _ => return Err("Invalid content type".into()),
        };
        
        // Parse the shared_with ID if provided
        let shared_with = input.shared_with.map(|id| Uuid::parse_str(&id).unwrap());
        
        // Get the share service from context
        let share_service = ctx.data::<ShareService>().unwrap();
        
        // Share the content
        let share = share_service
            .share_content(user_id, content_id, content_type, shared_with)
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(ShareDto {
            id: share.id.to_string(),
            user_id: share.user_id.to_string(),
            content_id: share.content_id.to_string(),
            content_type: share.content_type.to_string(),
            shared_with: share.shared_with.map(|id| id.to_string()),
            created_at: share.created_at.to_rfc3339(),
            is_public: share.is_public(),
        })
    }
    
    /// Unshare content
    async fn unshare_content(
        &self,
        ctx: &Context<'_>,
        input: UnshareContentInput,
    ) -> Result<bool> {
        // Get the current user ID from context
        let user_id = ctx.data::<Uuid>().unwrap().clone();
        
        // Parse the share ID
        let share_id = Uuid::parse_str(&input.share_id).map_err(|_| "Invalid share ID")?;
        
        // Get the share service from context
        let share_service = ctx.data::<ShareService>().unwrap();
        
        // Unshare the content
        share_service
            .unshare_content(user_id, share_id)
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(true)
    }
}

/// Social interaction queries
#[derive(Default)]
pub struct SocialInteractionQueries;

#[Object]
impl SocialInteractionQueries {
    /// Get reactions for a target
    async fn reactions_for_target(
        &self,
        ctx: &Context<'_>,
        target_id: ID,
        target_type: String,
    ) -> Result<Vec<ReactionDto>> {
        // Parse the target ID
        let target_id = Uuid::parse_str(&target_id).map_err(|_| "Invalid target ID")?;
        
        // Parse the target type
        let target_type = match target_type.as_str() {
            "post" => TargetType::Post,
            "comment" => TargetType::Comment,
            "achievement" => TargetType::Achievement,
            "volunteer_activity" => TargetType::VolunteerActivity,
            "skill_exchange" => TargetType::SkillExchange,
            _ => return Err("Invalid target type".into()),
        };
        
        // Get the reaction service from context
        let reaction_service = ctx.data::<ReactionService>().unwrap();
        
        // Get the reactions
        let reactions = reaction_service
            .get_reactions_for_target(target_id, target_type)
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(reactions
            .into_iter()
            .map(|reaction| ReactionDto {
                id: reaction.id.to_string(),
                user_id: reaction.user_id.to_string(),
                target_id: reaction.target_id.to_string(),
                target_type: reaction.target_type.to_string(),
                reaction_type: reaction.reaction_type.to_string(),
                created_at: reaction.created_at.to_rfc3339(),
            })
            .collect())
    }
    
    /// Get reaction summary for a target
    async fn reaction_summary(
        &self,
        ctx: &Context<'_>,
        target_id: ID,
        target_type: String,
    ) -> Result<ReactionSummary> {
        // Parse the target ID
        let target_id = Uuid::parse_str(&target_id).map_err(|_| "Invalid target ID")?;
        
        // Parse the target type
        let target_type = match target_type.as_str() {
            "post" => TargetType::Post,
            "comment" => TargetType::Comment,
            "achievement" => TargetType::Achievement,
            "volunteer_activity" => TargetType::VolunteerActivity,
            "skill_exchange" => TargetType::SkillExchange,
            _ => return Err("Invalid target type".into()),
        };
        
        // Get the reaction service from context
        let reaction_service = ctx.data::<ReactionService>().unwrap();
        
        // Get the reaction summary
        let summary = reaction_service
            .get_reaction_summary(target_id, target_type)
            .await
            .map_err(|e| e.to_string())?;
        
        let reactions = summary
            .into_iter()
            .map(|(reaction_type, count)| ReactionCount {
                reaction_type,
                count,
            })
            .collect();
        
        Ok(ReactionSummary {
            target_id: target_id.to_string(),
            target_type: target_type.to_string(),
            reactions,
        })
    }
    
    /// Get comments for a target
    async fn comments_for_target(
        &self,
        ctx: &Context<'_>,
        target_id: ID,
        target_type: String,
        max_depth: Option<i32>,
    ) -> Result<Vec<CommentDto>> {
        // Parse the target ID
        let target_id = Uuid::parse_str(&target_id).map_err(|_| "Invalid target ID")?;
        
        // Parse the target type
        let target_type = match target_type.as_str() {
            "post" => TargetType::Post,
            "comment" => TargetType::Comment,
            "achievement" => TargetType::Achievement,
            "volunteer_activity" => TargetType::VolunteerActivity,
            "skill_exchange" => TargetType::SkillExchange,
            _ => return Err("Invalid target type".into()),
        };
        
        // Convert max_depth to usize
        let max_depth = max_depth.map(|d| d as usize);
        
        // Get the comment service from context
        let comment_service = ctx.data::<CommentService>().unwrap();
        
        // Get the comments
        let comments = comment_service
            .get_comments_for_target(target_id, target_type, max_depth)
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(comments
            .into_iter()
            .map(|comment| CommentDto {
                id: comment.id.to_string(),
                user_id: comment.user_id.to_string(),
                parent_id: comment.parent_id.map(|id| id.to_string()),
                target_id: comment.target_id.to_string(),
                target_type: comment.target_type.to_string(),
                content: comment.content,
                created_at: comment.created_at.to_rfc3339(),
                updated_at: comment.updated_at.map(|dt| dt.to_rfc3339()),
            })
            .collect())
    }
    
    /// Get shares by user
    async fn shares_by_user(
        &self,
        ctx: &Context<'_>,
        user_id: ID,
    ) -> Result<Vec<ShareDto>> {
        // Parse the user ID
        let user_id = Uuid::parse_str(&user_id).map_err(|_| "Invalid user ID")?;
        
        // Get the share service from context
        let share_service = ctx.data::<ShareService>().unwrap();
        
        // Get the shares
        let shares = share_service
            .get_shares_by_user(user_id)
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(shares
            .into_iter()
            .map(|share| ShareDto {
                id: share.id.to_string(),
                user_id: share.user_id.to_string(),
                content_id: share.content_id.to_string(),
                content_type: share.content_type.to_string(),
                shared_with: share.shared_with.map(|id| id.to_string()),
                created_at: share.created_at.to_rfc3339(),
                is_public: share.is_public(),
            })
            .collect())
    }
    
    /// Get shares of content
    async fn shares_of_content(
        &self,
        ctx: &Context<'_>,
        content_id: ID,
        content_type: String,
    ) -> Result<Vec<ShareDto>> {
        // Parse the content ID
        let content_id = Uuid::parse_str(&content_id).map_err(|_| "Invalid content ID")?;
        
        // Parse the content type
        let content_type = match content_type.as_str() {
            "post" => ContentType::Post,
            "achievement" => ContentType::Achievement,
            "volunteer_activity" => ContentType::VolunteerActivity,
            "skill_exchange" => ContentType::SkillExchange,
            "comment" => ContentType::Comment,
            _ => return Err("Invalid content type".into()),
        };
        
        // Get the share service from context
        let share_service = ctx.data::<ShareService>().unwrap();
        
        // Get the shares
        let shares = share_service
            .get_shares_of_content(content_id, content_type)
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(shares
            .into_iter()
            .map(|share| ShareDto {
                id: share.id.to_string(),
                user_id: share.user_id.to_string(),
                content_id: share.content_id.to_string(),
                content_type: share.content_type.to_string(),
                shared_with: share.shared_with.map(|id| id.to_string()),
                created_at: share.created_at.to_rfc3339(),
                is_public: share.is_public(),
            })
            .collect())
    }
}