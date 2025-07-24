//! Permission model implementation
//!
//! Provides access control for social features based on user relationships
//! and content visibility settings.

use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use cpc_core::models::social::post::Visibility;

#[derive(Debug, Clone)]
pub struct PermissionContext {
    pub user_id: Uuid,
    pub target_user_id: Option<Uuid>,
    pub target_experience_id: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Permission {
    ViewExperience,
    CommentOnExperience,
    ShareExperience,
    DeleteExperience,
    DeleteComment,
    InviteFriend,
    AcceptFriendRequest,
    ViewPrivateContent,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PermissionResult {
    Granted,
    Denied(String),
}

/// Permission manager for social features
pub struct PermissionManager {
    /// User relationships (friend lists)
    user_relationships: Arc<RwLock<HashMap<Uuid, HashSet<Uuid>>>>,
    /// Experience ownership and visibility
    experience_permissions: Arc<RwLock<HashMap<Uuid, ExperiencePermission>>>,
}

#[derive(Debug, Clone)]
pub struct ExperiencePermission {
    pub owner_id: Uuid,
    pub visibility: Visibility,
    pub collaborators: HashSet<Uuid>,
}

impl PermissionManager {
    pub fn new() -> Self {
        Self {
            user_relationships: Arc::new(RwLock::new(HashMap::new())),
            experience_permissions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Check if user has permission
    pub async fn check_permission(
        &self,
        permission: Permission,
        context: &PermissionContext,
    ) -> PermissionResult {
        match permission {
            Permission::ViewExperience => self.check_view_experience(context).await,
            Permission::CommentOnExperience => self.check_comment_permission(context).await,
            Permission::ShareExperience => self.check_share_permission(context).await,
            Permission::DeleteExperience => self.check_delete_experience_permission(context).await,
            Permission::DeleteComment => self.check_delete_comment_permission(context).await,
            Permission::InviteFriend => self.check_invite_permission(context).await,
            Permission::AcceptFriendRequest => self.check_accept_friend_permission(context).await,
            Permission::ViewPrivateContent => self.check_private_content_permission(context).await,
        }
    }

    /// Add a friend relationship
    pub async fn add_friend(&self, user_id: Uuid, friend_id: Uuid) {
        let mut relationships = self.user_relationships.write().await;
        
        relationships
            .entry(user_id)
            .or_insert_with(HashSet::new)
            .insert(friend_id);
            
        relationships
            .entry(friend_id)
            .or_insert_with(HashSet::new)
            .insert(user_id);
    }

    /// Remove a friend relationship
    pub async fn remove_friend(&self, user_id: Uuid, friend_id: Uuid) {
        let mut relationships = self.user_relationships.write().await;
        
        if let Some(friends) = relationships.get_mut(&user_id) {
            friends.remove(&friend_id);
        }
        
        if let Some(friends) = relationships.get_mut(&friend_id) {
            friends.remove(&user_id);
        }
    }

    /// Set experience permissions
    pub async fn set_experience_permission(
        &self,
        experience_id: Uuid,
        owner_id: Uuid,
        visibility: Visibility,
        collaborators: Vec<Uuid>,
    ) {
        let mut permissions = self.experience_permissions.write().await;
        
        permissions.insert(
            experience_id,
            ExperiencePermission {
                owner_id,
                visibility,
                collaborators: collaborators.into_iter().collect(),
            },
        );
    }

    // Private permission check methods
    async fn check_view_experience(&self, context: &PermissionContext) -> PermissionResult {
        let Some(experience_id) = context.target_experience_id else {
            return PermissionResult::Denied("Experience ID required".to_string());
        };

        let permissions = self.experience_permissions.read().await;
        
        let Some(permission) = permissions.get(&experience_id) else {
            return PermissionResult::Denied("Experience not found".to_string());
        };

        match permission.visibility {
            Visibility::Public => PermissionResult::Granted,
            Visibility::Friends => {
                if permission.owner_id == context.user_id {
                    return PermissionResult::Granted;
                }
                
                let relationships = self.user_relationships.read().await;
                if let Some(friends) = relationships.get(&permission.owner_id) {
                    if friends.contains(&context.user_id) {
                        PermissionResult::Granted
                    } else {
                        PermissionResult::Denied("Only friends can view this experience".to_string())
                    }
                } else {
                    PermissionResult::Denied("No friend relationship found".to_string())
                }
            }
            Visibility::Private => {
                if permission.owner_id == context.user_id {
                    PermissionResult::Granted
                } else {
                    PermissionResult::Denied("Private experience".to_string())
                }
            }
        }
    }

    async fn check_comment_permission(&self, context: &PermissionContext) -> PermissionResult {
        // Same as view permission for now
        self.check_view_experience(context).await
    }

    async fn check_share_permission(&self, context: &PermissionContext) -> PermissionResult {
        let Some(experience_id) = context.target_experience_id else {
            return PermissionResult::Denied("Experience ID required".to_string());
        };

        let permissions = self.experience_permissions.read().await;
        
        let Some(permission) = permissions.get(&experience_id) else {
            return PermissionResult::Denied("Experience not found".to_string());
        };

        if permission.owner_id == context.user_id {
            PermissionResult::Granted
        } else {
            PermissionResult::Denied("Only owner can share experience".to_string())
        }
    }

    async fn check_delete_experience_permission(&self, context: &PermissionContext) -> PermissionResult {
        let Some(experience_id) = context.target_experience_id else {
            return PermissionResult::Denied("Experience ID required".to_string());
        };

        let permissions = self.experience_permissions.read().await;
        
        let Some(permission) = permissions.get(&experience_id) else {
            return PermissionResult::Denied("Experience not found".to_string());
        };

        if permission.owner_id == context.user_id {
            PermissionResult::Granted
        } else {
            PermissionResult::Denied("Only owner can delete experience".to_string())
        }
    }

    async fn check_delete_comment_permission(&self, context: &PermissionContext) -> PermissionResult {
        // For now, only experience owners can delete comments
        self.check_delete_experience_permission(context).await
    }

    async fn check_invite_permission(&self, context: &PermissionContext) -> PermissionResult {
        let Some(target_user_id) = context.target_user_id else {
            return PermissionResult::Denied("Target user ID required".to_string());
        };

        if context.user_id == target_user_id {
            return PermissionResult::Denied("Cannot invite yourself".to_string());
        }

        PermissionResult::Granted
    }

    async fn check_accept_friend_permission(&self, context: &PermissionContext) -> PermissionResult {
        let Some(target_user_id) = context.target_user_id else {
            return PermissionResult::Denied("Target user ID required".to_string());
        };

        if context.user_id == target_user_id {
            return PermissionResult::Denied("Cannot accept your own invitation".to_string());
        }

        PermissionResult::Granted
    }

    async fn check_private_content_permission(&self, context: &PermissionContext) -> PermissionResult {
        let Some(target_user_id) = context.target_user_id else {
            return PermissionResult::Denied("Target user ID required".to_string());
        };

        if context.user_id == target_user_id {
            PermissionResult::Granted
        } else {
            let relationships = self.user_relationships.read().await;
            if let Some(friends) = relationships.get(&target_user_id) {
                if friends.contains(&context.user_id) {
                    PermissionResult::Granted
                } else {
                    PermissionResult::Denied("Only friends can view private content".to_string())
                }
            } else {
                PermissionResult::Denied("No friend relationship found".to_string())
            }
        }
    }

    /// Check if users are friends
    pub async fn are_friends(&self, user1: Uuid, user2: Uuid) -> bool {
        let relationships = self.user_relationships.read().await;
        
        if let Some(friends) = relationships.get(&user1) {
            friends.contains(&user2)
        } else {
            false
        }
    }

    /// Get user's friends
    pub async fn get_friends(&self, user_id: Uuid) -> Vec<Uuid> {
        let relationships = self.user_relationships.read().await;
        
        relationships
            .get(&user_id)
            .map(|friends| friends.iter().cloned().collect())
            .unwrap_or_default()
    }
}