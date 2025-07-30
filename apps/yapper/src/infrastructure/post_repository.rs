use crate::domain::post::Yap;
use uuid::Uuid;
use sled::Db;
use serde_json;

pub struct PostRepository {
    db: Db,
}

impl PostRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub fn save(&self, yap: &Yap) -> Result<(), String> {
        let key = yap.id.to_string();
        let value = serde_json::to_vec(yap).map_err(|e| e.to_string())?;
        self.db.insert(key.as_bytes(), value).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn find_by_id(&self, id: Uuid) -> Result<Option<Yap>, String> {
        let key = id.to_string();
        match self.db.get(key.as_bytes()).map_err(|e| e.to_string())? {
            Some(value) => {
                let yap = serde_json::from_slice(&value).map_err(|e| e.to_string())?;
                Ok(Some(yap))
            }
            None => Ok(None),
        }
    }

    pub fn find_by_user(&self, user_id: Uuid) -> Result<Vec<Yap>, String> {
        // In a real implementation, we would have an index on user_id
        // For now, we'll iterate through all posts (inefficient but works for demo)
        let mut yaps = Vec::new();
        for result in self.db.iter() {
            let (_, value) = result.map_err(|e| e.to_string())?;
            let yap: Yap = serde_json::from_slice(&value).map_err(|e| e.to_string())?;
            if yap.user_id == user_id {
                yaps.push(yap);
            }
        }
        Ok(yaps)
    }
}