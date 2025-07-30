use sled::Db;
use crate::domain::password_reset_repository::PasswordResetRepository;
use crate::domain::auth_error::RepositoryError;
use uuid::Uuid;
use async_trait::async_trait;

pub struct SledPasswordResetRepository {
    db: Db,
}

impl SledPasswordResetRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl PasswordResetRepository for SledPasswordResetRepository {
    async fn create_token(&self, token: String, user_id: Uuid) -> Result<(), RepositoryError> {
        let key = token;
        let value = user_id.to_string();
        self.db.insert(key.as_bytes(), value.as_bytes()).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(())
    }
    
    async fn find_user_id_by_token(&self, token: &str) -> Result<Option<Uuid>, RepositoryError> {
        match self.db.get(token.as_bytes()).map_err(|e| RepositoryError::DatabaseError(e.to_string()))? {
            Some(value) => {
                let user_id_str = String::from_utf8(value.to_vec()).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
                let user_id = Uuid::parse_str(&user_id_str).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
                Ok(Some(user_id))
            }
            None => Ok(None),
        }
    }
    
    async fn delete_token(&self, token: &str) -> Result<(), RepositoryError> {
        self.db.remove(token.as_bytes()).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}