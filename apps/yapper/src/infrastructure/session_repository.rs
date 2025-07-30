use sled::Db;
use crate::domain::session::Session;
use crate::domain::session_repository::SessionRepository;
use crate::domain::auth_error::RepositoryError;
use uuid::Uuid;
use serde_json;

pub struct SledSessionRepository {
    db: Db,
}

impl SledSessionRepository {
    pub fn new(db: Db) -> Self {
        // Create user sessions index tree
        db.open_tree("user_sessions").expect("Failed to create user_sessions tree");
        Self { db }
    }
}

#[async_trait::async_trait]
impl SessionRepository for SledSessionRepository {
    async fn create(&self, session: &Session) -> Result<(), RepositoryError> {
        // Save session
        let key = session.id.to_string();
        let value = serde_json::to_vec(session).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        self.db.insert(key.as_bytes(), value).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        // Update user sessions index
        let user_sessions_tree = self.db.open_tree("user_sessions").map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        let user_id_str = session.user_id.to_string();
        
        // Get existing sessions for this user
        let existing_sessions_bytes = user_sessions_tree.get(user_id_str.as_bytes()).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        let mut session_ids: Vec<String> = match existing_sessions_bytes {
            Some(bytes) => serde_json::from_slice(&bytes).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?,
            None => Vec::new(),
        };
        
        // Add new session
        session_ids.push(session.id.to_string());
        
        // Save updated list
        let session_ids_bytes = serde_json::to_vec(&session_ids).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        user_sessions_tree.insert(user_id_str.as_bytes(), session_ids_bytes).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Session>, RepositoryError> {
        let key = id.to_string();
        match self.db.get(key.as_bytes()).map_err(|e| RepositoryError::DatabaseError(e.to_string()))? {
            Some(value) => {
                let session = serde_json::from_slice(&value).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
                Ok(Some(session))
            }
            None => Ok(None),
        }
    }

    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let key = id.to_string();
        self.db.remove(key.as_bytes()).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<Session>, RepositoryError> {
        let user_sessions_tree = self.db.open_tree("user_sessions").map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        let user_id_str = user_id.to_string();
        
        let existing_sessions_bytes = user_sessions_tree.get(user_id_str.as_bytes()).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        let session_ids: Vec<String> = match existing_sessions_bytes {
            Some(bytes) => serde_json::from_slice(&bytes).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?,
            None => Vec::new(),
        };
        
        let mut sessions = Vec::new();
        for session_id_str in session_ids {
            if let Ok(session_id) = Uuid::parse_str(&session_id_str) {
                if let Ok(Some(session)) = self.find_by_id(session_id).await {
                    sessions.push(session);
                }
            }
        }
        
        Ok(sessions)
    }
}