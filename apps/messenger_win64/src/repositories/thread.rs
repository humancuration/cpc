//! Repository implementation for threads

use shared_packages::messenger::models::{MessageThread, ThreadId};
use shared_packages::messenger::errors::MessengerError;
use sqlx::PgPool;
use uuid::Uuid;
use std::sync::Arc;

/// Repository for thread operations
pub struct ThreadRepository {
    db_pool: Arc<PgPool>,
}

impl ThreadRepository {
    /// Create a new ThreadRepository
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
    
    /// Create a new thread
    pub async fn create_thread(&self, parent_message_id: Uuid, conversation_id: Uuid) -> Result<MessageThread, MessengerError> {
        let mut conn = self.db_pool.acquire().await
            .map_err(|e| MessengerError::StorageError { message: e.to_string() })?;
            
        let row = sqlx::query!(
            r#"
            INSERT INTO message_threads (parent_message_id, root_message_id, conversation_id)
            VALUES ($1, $1, $2)
            RETURNING id, parent_message_id, root_message_id, conversation_id, created_at
            "#,
            parent_message_id,
            conversation_id
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| MessengerError::StorageError { message: e.to_string() })?;
        
        Ok(MessageThread {
            id: ThreadId(row.id),
            parent_message_id: row.parent_message_id,
            root_message_id: row.root_message_id,
            conversation_id: row.conversation_id,
            created_at: row.created_at,
        })
    }
    
    /// Get a thread by ID
    pub async fn get_thread(&self, thread_id: ThreadId) -> Result<MessageThread, MessengerError> {
        let mut conn = self.db_pool.acquire().await
            .map_err(|e| MessengerError::StorageError { message: e.to_string() })?;
            
        let row = sqlx::query!(
            r#"
            SELECT id, parent_message_id, root_message_id, conversation_id, created_at
            FROM message_threads
            WHERE id = $1
            "#,
            thread_id.0
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| MessengerError::StorageError { message: e.to_string() })?
        .ok_or(MessengerError::ThreadNotFound { id: thread_id.0 })?;
        
        Ok(MessageThread {
            id: ThreadId(row.id),
            parent_message_id: row.parent_message_id,
            root_message_id: row.root_message_id,
            conversation_id: row.conversation_id,
            created_at: row.created_at,
        })
    }
    
    /// Get messages in a thread
    pub async fn get_thread_messages(&self, thread_id: ThreadId, limit: usize, before_message_id: Option<Uuid>) -> Result<Vec<Uuid>, MessengerError> {
        let mut conn = self.db_pool.acquire().await
            .map_err(|e| MessengerError::StorageError { message: e.to_string() })?;
            
        let rows = if let Some(before_id) = before_message_id {
            sqlx::query!(
                r#"
                SELECT id
                FROM messages
                WHERE thread_id = $1 AND id < $2
                ORDER BY sent_at DESC
                LIMIT $3
                "#,
                thread_id.0,
                before_id,
                limit as i64
            )
            .fetch_all(&mut *conn)
            .await
        } else {
            sqlx::query!(
                r#"
                SELECT id
                FROM messages
                WHERE thread_id = $1
                ORDER BY sent_at DESC
                LIMIT $2
                "#,
                thread_id.0,
                limit as i64
            )
            .fetch_all(&mut *conn)
            .await
        }
        .map_err(|e| MessengerError::StorageError { message: e.to_string() })?;
        
        Ok(rows.into_iter().map(|row| row.id).collect())
    }
}