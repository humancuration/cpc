//! Repository implementation for group management

use shared_packages::messenger::models::{Conversation, ParticipantPermissions};
use shared_packages::messenger::errors::MessengerError;
use sqlx::PgPool;
use uuid::Uuid;
use std::sync::Arc;

/// Repository for group management operations
pub struct GroupRepository {
    db_pool: Arc<PgPool>,
}

impl GroupRepository {
    /// Create a new GroupRepository
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
    
    /// Update group settings
    pub async fn update_group_settings(&self, conversation_id: Uuid, settings: crate::models::ConversationSettings) -> Result<(), MessengerError> {
        let mut conn = self.db_pool.acquire().await
            .map_err(|e| MessengerError::StorageError { message: e.to_string() })?;
            
        sqlx::query!(
            r#"
            UPDATE conversations
            SET group_name = $1
            WHERE id = $2 AND is_group = true
            "#,
            settings.name,
            conversation_id
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| MessengerError::StorageError { message: e.to_string() })?;
        
        Ok(())
    }
    
    /// Transfer admin rights to another user
    pub async fn transfer_admin(&self, conversation_id: Uuid, current_admin_id: Uuid, new_admin_id: Uuid) -> Result<(), MessengerError> {
        let mut conn = self.db_pool.acquire().await
            .map_err(|e| MessengerError::StorageError { message: e.to_string() })?;
            
        // Remove admin rights from current admin
        sqlx::query!(
            r#"
            UPDATE participants
            SET is_admin = false,
                can_manage_participants = false,
                can_change_settings = false,
                can_delete_messages = false,
                can_moderate_content = false
            WHERE conversation_id = $1 AND user_id = $2
            "#,
            conversation_id,
            current_admin_id
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| MessengerError::StorageError { message: e.to_string() })?;
        
        // Grant admin rights to new admin
        sqlx::query!(
            r#"
            UPDATE participants
            SET is_admin = true,
                can_manage_participants = true,
                can_change_settings = true,
                can_delete_messages = true,
                can_moderate_content = true
            WHERE conversation_id = $1 AND user_id = $2
            "#,
            conversation_id,
            new_admin_id
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| MessengerError::StorageError { message: e.to_string() })?;
        
        Ok(())
    }
    
    /// Ban a participant from a group
    pub async fn ban_participant(&self, conversation_id: Uuid, user_id: Uuid) -> Result<(), MessengerError> {
        let mut conn = self.db_pool.acquire().await
            .map_err(|e| MessengerError::StorageError { message: e.to_string() })?;
            
        // Remove participant from conversation
        sqlx::query!(
            r#"
            DELETE FROM participants
            WHERE conversation_id = $1 AND user_id = $2
            "#,
            conversation_id,
            user_id
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| MessengerError::StorageError { message: e.to_string() })?;
        
        Ok(())
    }
    
    /// Update participant permissions
    pub async fn update_participant_permissions(&self, conversation_id: Uuid, user_id: Uuid, permissions: ParticipantPermissions) -> Result<(), MessengerError> {
        let mut conn = self.db_pool.acquire().await
            .map_err(|e| MessengerError::StorageError { message: e.to_string() })?;
            
        sqlx::query!(
            r#"
            UPDATE participants
            SET can_send_messages = $1,
                can_manage_participants = $2,
                can_change_settings = $3,
                can_delete_messages = $4,
                can_moderate_content = $5,
                is_admin = $6
            WHERE conversation_id = $7 AND user_id = $8
            "#,
            permissions.can_send_messages,
            permissions.can_manage_participants,
            permissions.can_change_settings,
            permissions.can_delete_messages,
            permissions.can_moderate_content,
            permissions.is_admin,
            conversation_id,
            user_id
        )
        .execute(&mut *conn)
        .await
        .map_err(|e| MessengerError::StorageError { message: e.to_string() })?;
        
        Ok(())
    }
    
    /// Check if user is an admin
    pub async fn is_admin(&self, conversation_id: Uuid, user_id: Uuid) -> Result<bool, MessengerError> {
        let mut conn = self.db_pool.acquire().await
            .map_err(|e| MessengerError::StorageError { message: e.to_string() })?;
            
        let row = sqlx::query!(
            r#"
            SELECT is_admin
            FROM participants
            WHERE conversation_id = $1 AND user_id = $2
            "#,
            conversation_id,
            user_id
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| MessengerError::StorageError { message: e.to_string() })?;
        
        Ok(row.map(|r| r.is_admin).unwrap_or(false))
    }
}