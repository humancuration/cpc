use sled::Db;
use crate::domain::user::User;
use crate::domain::user_repository::UserRepository;
use crate::domain::auth_error::RepositoryError;
use uuid::Uuid;
use serde_json;

pub struct SledUserRepository {
    db: Db,
}

impl SledUserRepository {
    pub fn new(db: Db) -> Self {
        // Create email index tree
        db.open_tree("user_emails").expect("Failed to create user_emails tree");
        Self { db }
    }
}

#[async_trait::async_trait]
impl UserRepository for SledUserRepository {
    async fn create(&self, user: &User) -> Result<(), RepositoryError> {
        // Save user
        let key = user.id.to_string();
        let value = serde_json::to_vec(user).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        self.db.insert(key.as_bytes(), value).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        // Create email index
        let email_tree = self.db.open_tree("user_emails").map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        email_tree.insert(user.email.as_bytes(), user.id.to_string().as_bytes()).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, RepositoryError> {
        let key = id.to_string();
        match self.db.get(key.as_bytes()).map_err(|e| RepositoryError::DatabaseError(e.to_string()))? {
            Some(value) => {
                let user = serde_json::from_slice(&value).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError> {
        let email_tree = self.db.open_tree("user_emails").map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        match email_tree.get(email.as_bytes()).map_err(|e| RepositoryError::DatabaseError(e.to_string()))? {
            Some(user_id_bytes) => {
                let user_id_str = String::from_utf8(user_id_bytes.to_vec()).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
                let user_id = Uuid::parse_str(&user_id_str).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
                self.find_by_id(user_id).await
            }
            None => Ok(None),
        }
    }

    async fn update(&self, user: &User) -> Result<(), RepositoryError> {
        let key = user.id.to_string();
        let value = serde_json::to_vec(user).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        self.db.insert(key.as_bytes(), value).map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}