use async_trait::async_trait;
use crate::domain::auth::{User, community_role::CommunityRole};
use sled::IVec;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct StoredUser {
    pub base_user: cpc_auth::models::User,
    pub karma: i32,
    pub community_roles: Vec<CommunityRole>,
}

impl From<User> for StoredUser {
    fn from(user: User) -> Self {
        // Note: In a full implementation, we would need to store community roles in the User struct
        // For now, we're initializing with an empty vector
        // In a real implementation, the User struct would contain community roles
        Self {
            base_user: user.base,
            karma: user.karma,
            community_roles: vec![],
        }
    }
}

impl From<StoredUser> for User {
    fn from(stored: StoredUser) -> Self {
        Self {
            base: stored.base_user,
            karma: stored.karma,
        }
    }
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), String>;
    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, String>;
    async fn update_karma(&self, user_id: Uuid, delta: i32) -> Result<(), String>;
    async fn assign_community_role(&self, user_id: Uuid, role: CommunityRole) -> Result<(), String>;
    async fn get_community_roles(&self, user_id: Uuid) -> Result<Vec<CommunityRole>, String>;
}

pub struct SledUserRepository {
    db: sled::Db,
}

impl SledUserRepository {
    pub fn new(db: sled::Db) -> Self {
        Self { db }
    }
    
    fn user_key(&self, user_id: Uuid) -> String {
        format!("user:{}", user_id)
    }
}

#[async_trait]
impl UserRepository for SledUserRepository {
    async fn save(&self, user: &User) -> Result<(), String> {
        let key = self.user_key(user.base.id);
        let stored_user: StoredUser = user.clone().into();
        let serialized = bincode::serialize(&stored_user).map_err(|e| e.to_string())?;
        self.db.insert(key.as_bytes(), serialized).map_err(|e| e.to_string())?;
        Ok(())
    }
    
    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, String> {
        let key = self.user_key(user_id);
        match self.db.get(key.as_bytes()).map_err(|e| e.to_string())? {
            Some(iv) => {
                let stored_user: StoredUser = bincode::deserialize(&iv).map_err(|e| e.to_string())?;
                Ok(Some(stored_user.into()))
            }
            None => Ok(None),
        }
    }
    async fn update_karma(&self, user_id: Uuid, delta: i32) -> Result<(), String> {
        let key = self.user_key(user_id);
        let existing = self.db.get(key.as_bytes()).map_err(|e| e.to_string())?;
        
        if let Some(iv) = existing {
            let mut stored_user: StoredUser = bincode::deserialize(&iv).map_err(|e| e.to_string())?;
            
            // Check for karma overflow
            if stored_user.karma + delta > 10000 {
                return Err("Karma limit exceeded: max 10000".to_string());
            }
            
            stored_user.karma += delta;
            
            let serialized = bincode::serialize(&stored_user).map_err(|e| e.to_string())?;
            self.db.insert(key.as_bytes(), serialized).map_err(|e| e.to_string())?;
        }
        
        Ok(())
    }
    }
    
    async fn assign_community_role(&self, user_id: Uuid, role: CommunityRole) -> Result<(), String> {
        let key = self.user_key(user_id);
        let existing = self.db.get(key.as_bytes()).map_err(|e| e.to_string())?;
        
        if let Some(iv) = existing {
            let mut stored_user: StoredUser = bincode::deserialize(&iv).map_err(|e| e.to_string())?;
            
            // Prevent role escalation by checking if the user already has a higher role
            // Role hierarchy: Contributor < Moderator < Admin
            let has_higher_role = stored_user.community_roles.iter().any(|r| {
                match r {
                    CommunityRole::Admin => true, // Admin is the highest role
                    CommunityRole::Moderator => matches!(role, CommunityRole::Contributor), // Moderator > Contributor
                    CommunityRole::Contributor => false, // Contributor is the lowest role
                }
            });
            
            // Prevent duplicate roles
            if !stored_user.community_roles.contains(&role) && !has_higher_role {
                stored_user.community_roles.push(role);
            } else if has_higher_role {
                return Err("Role assignment conflict: User already has a higher role".to_string());
            }
            
            let serialized = bincode::serialize(&stored_user).map_err(|e| e.to_string())?;
            self.db.insert(key.as_bytes(), serialized).map_err(|e| e.to_string())?;
        }
        
        Ok(())
    }
    
    async fn get_community_roles(&self, user_id: Uuid) -> Result<Vec<CommunityRole>, String> {
        let key = self.user_key(user_id);
        let existing = self.db.get(key.as_bytes()).map_err(|e| e.to_string())?;
        
        if let Some(iv) = existing {
            let stored_user: StoredUser = bincode::deserialize(&iv).map_err(|e| e.to_string())?;
            Ok(stored_user.community_roles)
        } else {
            Ok(vec![])
        }
    }
}