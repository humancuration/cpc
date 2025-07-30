use sled::Db;
use crate::domain::user_profile::UserProfile;
use uuid::Uuid;
use serde_json;

pub struct UserProfileRepository {
    db: Db,
}

impl UserProfileRepository {
    pub fn new(db: Db) -> Self {
        // Create user_id index tree
        db.open_tree("user_profile_user_ids").expect("Failed to create user_profile_user_ids tree");
        Self { db }
    }

    pub fn save(&self, profile: &UserProfile) -> Result<(), String> {
        let key = profile.id.to_string();
        let value = serde_json::to_vec(profile).map_err(|e| e.to_string())?;
        self.db.insert(key.as_bytes(), value).map_err(|e| e.to_string())?;
        
        // Create user_id index
        let user_id_tree = self.db.open_tree("user_profile_user_ids").map_err(|e| e.to_string())?;
        user_id_tree.insert(profile.user_id.to_string().as_bytes(), profile.id.to_string().as_bytes()).map_err(|e| e.to_string())?;
        
        Ok(())
    }

    pub fn find_by_id(&self, id: Uuid) -> Result<Option<UserProfile>, String> {
        let key = id.to_string();
        match self.db.get(key.as_bytes()).map_err(|e| e.to_string())? {
            Some(value) => {
                let profile = serde_json::from_slice(&value).map_err(|e| e.to_string())?;
                Ok(Some(profile))
            }
            None => Ok(None),
        }
    }

    pub fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserProfile>, String> {
        let user_id_tree = self.db.open_tree("user_profile_user_ids").map_err(|e| e.to_string())?;
        match user_id_tree.get(user_id.to_string().as_bytes()).map_err(|e| e.to_string())? {
            Some(profile_id_bytes) => {
                let profile_id_str = String::from_utf8(profile_id_bytes.to_vec()).map_err(|e| e.to_string())?;
                let profile_id = Uuid::parse_str(&profile_id_str).map_err(|e| e.to_string())?;
                self.find_by_id(profile_id)
            }
            None => Ok(None),
        }
    }
}