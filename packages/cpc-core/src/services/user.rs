use crate::{
    models::user::User,
    repositories::user_repository::UserRepository,
    utils::datetime::now_utc,
};
use anyhow::{anyhow, Result};
use uuid::Uuid;

/// Service for user profile management
pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    /// Creates a new UserService instance
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    /// Retrieves a user by ID
    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<User> {
        self.user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| anyhow!("User not found"))
    }

    /// Updates a user's profile information
    pub async fn update_profile(
        &self,
        user_id: Uuid,
        display_name: Option<String>,
        bio: Option<String>,
        avatar_url: Option<String>,
    ) -> Result<User> {
        let mut user = self.get_user_by_id(user_id).await?;

        // Update fields if provided
        if let Some(name) = display_name {
            // Validate display name length
            if name.len() > 50 {
                return Err(anyhow!("Display name must be 50 characters or less"));
            }
            user.display_name = Some(name);
        }

        if let Some(bio_text) = bio {
            user.bio = Some(bio_text);
        }

        if let Some(avatar) = avatar_url {
            // Basic URL validation
            if !avatar.starts_with("http://") && !avatar.starts_with("https://") {
                return Err(anyhow!("Invalid avatar URL format"));
            }
            user.avatar_url = Some(avatar);
        }

        user.updated_at = now_utc();

        self.user_repo.update(&user).await?;

        Ok(user)
    }

    /// Deletes a user account
    pub async fn delete_user(&self, user_id: Uuid) -> Result<()> {
        self.user_repo.delete(user_id).await
    }
}