use crate::domain::post::Yap;
use uuid::Uuid;
use sled::Db;
use serde_json;

pub struct FeedRepository {
    db: Db,
}

impl FeedRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub fn save_feed(&self, user_id: Uuid, feed: Vec<Yap>) -> Result<(), String> {
        let key = format!("feed_{}", user_id);
        let value = serde_json::to_vec(&feed).map_err(|e| e.to_string())?;
        self.db.insert(key.as_bytes(), value).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_user_feed(&self, user_id: Uuid) -> Result<Vec<Yap>, String> {
        let key = format!("feed_{}", user_id);
        match self.db.get(key.as_bytes()).map_err(|e| e.to_string())? {
            Some(value) => {
                let feed = serde_json::from_slice(&value).map_err(|e| e.to_string())?;
                Ok(feed)
            }
            None => Ok(Vec::new()),
        }
    }

    pub fn get_home_feed(&self, user_id: Uuid) -> Result<Vec<Yap>, String> {
        // For now, we'll just return the user feed
        // In a real implementation, this would be a more complex algorithm
        self.get_user_feed(user_id)
    }
}