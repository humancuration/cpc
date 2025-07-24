//! Social features service implementation
//!
//! Provides core social functionality including experience sharing,
//! friend invitations, comments, and real-time updates.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use cpc_core::models::social::post::Visibility;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum SocialError {
    #[error("Experience not found")]
    ExperienceNotFound,
    #[error("User not found")]
    UserNotFound,
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Invalid visibility setting")]
    InvalidVisibility,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Storage error: {0}")]
    StorageError(String),
}

pub type Result<T> = std::result::Result<T, SocialError>;

/// Experience metadata structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub visibility: Visibility,
    pub content_hash: String,
    pub file_size: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Comment structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub experience_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Friend invitation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendInvitation {
    pub id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub status: InvitationStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Rejected,
}

/// Input for sharing an experience
#[derive(Debug, Clone, Deserialize)]
pub struct ShareExperienceInput {
    pub experience_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub visibility: Visibility,
    pub content_hash: String,
    pub file_size: u64,
}

/// Input for posting a comment
#[derive(Debug, Clone, Deserialize)]
pub struct PostCommentInput {
    pub experience_id: Uuid,
    pub content: String,
}

/// Social events for real-time updates
#[derive(Debug, Clone, Serialize)]
pub enum SocialEvent {
    NewComment {
        experience_id: Uuid,
        comment: Comment,
    },
    FriendRequest {
        from_user_id: Uuid,
        to_user_id: Uuid,
        status: InvitationStatus,
    },
    ExperienceShared {
        experience_id: Uuid,
        owner_id: Uuid,
        visibility: Visibility,
    },
}

/// Main social features service
pub struct SocialFeaturesService {
    /// Experience storage
    experiences: Arc<RwLock<HashMap<Uuid, Experience>>>,
    /// Comments storage
    comments: Arc<RwLock<HashMap<Uuid, Vec<Comment>>>>,
    /// Friend invitations
    invitations: Arc<RwLock<HashMap<Uuid, FriendInvitation>>>,
    /// User relationships (friend lists)
    user_relationships: Arc<RwLock<HashMap<Uuid, Vec<Uuid>>>>,
    /// Event broadcaster for real-time updates
    event_tx: broadcast::Sender<SocialEvent>,
    /// File hosting service reference
    file_hosting: Arc<dyn crate::services::file_hosting::FileHostingService>,
}

impl SocialFeaturesService {
    pub fn new(file_hosting: Arc<dyn crate::services::file_hosting::FileHostingService>) -> Self {
        let (event_tx, _) = broadcast::channel(100);
        
        Self {
            experiences: Arc::new(RwLock::new(HashMap::new())),
            comments: Arc::new(RwLock::new(HashMap::new())),
            invitations: Arc::new(RwLock::new(HashMap::new())),
            user_relationships: Arc::new(RwLock::new(HashMap::new())),
            event_tx,
            file_hosting,
        }
    }

    /// Share an experience with specified visibility
    pub async fn share_experience(
        &self,
        user_id: Uuid,
        input: ShareExperienceInput,
    ) -> Result<Experience> {
        // Validate user has permission to share this experience
        self.validate_experience_owner(user_id, input.experience_id).await?;
        
        let experience = Experience {
            id: input.experience_id,
            owner_id: user_id,
            title: input.title,
            description: input.description,
            visibility: input.visibility,
            content_hash: input.content_hash,
            file_size: input.file_size,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Store experience metadata
        {
            let mut experiences = self.experiences.write().await;
            experiences.insert(experience.id, experience.clone());
        }

        // Broadcast sharing event
        let _ = self.event_tx.send(SocialEvent::ExperienceShared {
            experience_id: experience.id,
            owner_id: user_id,
            visibility: experience.visibility.clone(),
        });

        Ok(experience)
    }

    /// Add a comment to an experience
    pub async fn add_comment(
        &self,
        user_id: Uuid,
        input: PostCommentInput,
    ) -> Result<Comment> {
        // Check if experience exists and user has permission to comment
        let experience = self.get_experience(input.experience_id).await?;
        self.check_comment_permission(user_id, &experience).await?;

        let comment = Comment {
            id: Uuid::new_v4(),
            experience_id: input.experience_id,
            author_id: user_id,
            content: input.content,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Store comment
        {
            let mut comments = self.comments.write().await;
            comments
                .entry(input.experience_id)
                .or_insert_with(Vec::new)
                .push(comment.clone());
        }

        // Broadcast new comment event
        let _ = self.event_tx.send(SocialEvent::NewComment {
            experience_id: input.experience_id,
            comment: comment.clone(),
        });

        Ok(comment)
    }

    /// Invite a friend to view experiences
    pub async fn invite_friend(
        &self,
        from_user_id: Uuid,
        to_user_id: Uuid,
    ) -> Result<FriendInvitation> {
        // Check if invitation already exists
        let existing_invitation = {
            let invitations = self.invitations.read().await;
            invitations.values()
                .find(|inv| inv.from_user_id == from_user_id && inv.to_user_id == to_user_id)
                .cloned()
        };

        if let Some(invitation) = existing_invitation {
            return Ok(invitation);
        }

        let invitation = FriendInvitation {
            id: Uuid::new_v4(),
            from_user_id,
            to_user_id,
            status: InvitationStatus::Pending,
            created_at: Utc::now(),
        };

        // Store invitation
        {
            let mut invitations = self.invitations.write().await;
            invitations.insert(invitation.id, invitation.clone());
        }

        // Broadcast friend request event
        let _ = self.event_tx.send(SocialEvent::FriendRequest {
            from_user_id,
            to_user_id,
            status: InvitationStatus::Pending,
        });

        Ok(invitation)
    }

    /// Accept a friend invitation
    pub async fn accept_friend_invitation(
        &self,
        user_id: Uuid,
        invitation_id: Uuid,
    ) -> Result<()> {
        let mut invitation = {
            let invitations = self.invitations.read().await;
            invitations.get(&invitation_id)
                .ok_or(SocialError::UserNotFound)?
                .clone()
        };

        if invitation.to_user_id != user_id {
            return Err(SocialError::PermissionDenied);
        }

        invitation.status = InvitationStatus::Accepted;

        // Update invitation
        {
            let mut invitations = self.invitations.write().await;
            invitations.insert(invitation_id, invitation.clone());
        }

        // Add to friend relationships
        {
            let mut relationships = self.user_relationships.write().await;
            relationships
                .entry(invitation.from_user_id)
                .or_insert_with(Vec::new)
                .push(user_id);
            relationships
                .entry(user_id)
                .or_insert_with(Vec::new)
                .push(invitation.from_user_id);
        }

        // Broadcast acceptance event
        let _ = self.event_tx.send(SocialEvent::FriendRequest {
            from_user_id: invitation.from_user_id,
            to_user_id: user_id,
            status: InvitationStatus::Accepted,
        });

        Ok(())
    }

    /// Get experiences visible to a user
    pub async fn get_visible_experiences(
        &self,
        user_id: Uuid,
        viewer_id: Option<Uuid>,
    ) -> Result<Vec<Experience>> {
        let experiences = self.experiences.read().await;
        let relationships = self.user_relationships.read().await;
        
        let visible_experiences: Vec<Experience> = experiences
            .values()
            .filter(|exp| {
                match exp.visibility {
                    Visibility::Public => true,
                    Visibility::Friends => {
                        viewer_id.map_or(false, |viewer| {
                            relationships.get(&exp.owner_id)
                                .map_or(false, |friends| friends.contains(&viewer))
                        })
                    }
                    Visibility::Private => exp.owner_id == user_id,
                }
            })
            .cloned()
            .collect();

        Ok(visible_experiences)
    }

    /// Get comments for an experience
    pub async fn get_comments(
        &self,
        experience_id: Uuid,
        user_id: Option<Uuid>,
    ) -> Result<Vec<Comment>> {
        let experience = self.get_experience(experience_id).await?;
        
        // Check if user has permission to view comments
        if let Some(viewer_id) = user_id {
            self.check_view_permission(viewer_id, &experience).await?;
        }

        let comments = self.comments.read().await;
        Ok(comments.get(&experience_id).cloned().unwrap_or_default())
    }

    /// Subscribe to social events
    pub fn subscribe_events(&self) -> broadcast::Receiver<SocialEvent> {
        self.event_tx.subscribe()
    }

    // Helper methods
    async fn get_experience(&self, experience_id: Uuid) -> Result<Experience> {
        let experiences = self.experiences.read().await;
        experiences
            .get(&experience_id)
            .cloned()
            .ok_or(SocialError::ExperienceNotFound)
    }

    async fn validate_experience_owner(&self, user_id: Uuid, experience_id: Uuid) -> Result<()> {
        let experience = self.get_experience(experience_id).await?;
        if experience.owner_id != user_id {
            return Err(SocialError::PermissionDenied);
        }
        Ok(())
    }

    async fn check_comment_permission(&self, user_id: Uuid, experience: &Experience) -> Result<()> {
        match experience.visibility {
            Visibility::Public => Ok(()),
            Visibility::Friends => {
                let relationships = self.user_relationships.read().await;
                let is_friend = relationships
                    .get(&experience.owner_id)
                    .map_or(false, |friends| friends.contains(&user_id));
                
                if is_friend || experience.owner_id == user_id {
                    Ok(())
                } else {
                    Err(SocialError::PermissionDenied)
                }
            }
            Visibility::Private => {
                if experience.owner_id == user_id {
                    Ok(())
                } else {
                    Err(SocialError::PermissionDenied)
                }
            }
        }
    }

    async fn check_view_permission(&self, user_id: Uuid, experience: &Experience) -> Result<()> {
        self.check_comment_permission(user_id, experience).await
    }
}