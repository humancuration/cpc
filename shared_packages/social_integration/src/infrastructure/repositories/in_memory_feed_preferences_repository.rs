//! In-memory implementation of feed preferences repository

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::domain::feed_preferences::{FeedPreferences, FeedPreferencesRepository};

/// In-memory repository for feed preferences
#[derive(Debug, Clone)]
pub struct InMemoryFeedPreferencesRepository {
    preferences: Arc<RwLock<HashMap<Uuid, FeedPreferences>>>,
}

impl InMemoryFeedPreferencesRepository {
    pub fn new() -> Self {
        Self {
            preferences: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl FeedPreferencesRepository for InMemoryFeedPreferencesRepository {
    async fn get_preferences(
        &self, 
        user_id: Uuid
    ) -> Result<FeedPreferences, Box<dyn std::error::Error + Send + Sync>> {
        let preferences = self.preferences.read().await;
        Ok(preferences.get(&user_id).cloned().unwrap_or_default())
    }
    
    async fn save_preferences(
        &self, 
        user_id: Uuid,
        preferences: FeedPreferences
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut prefs = self.preferences.write().await;
        prefs.insert(user_id, preferences);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::feed_preferences::FeedAlgorithmType;

    #[tokio::test]
    async fn test_get_preferences_default() {
        let repo = InMemoryFeedPreferencesRepository::new();
        let user_id = Uuid::new_v4();
        
        let preferences = repo.get_preferences(user_id).await.unwrap();
        assert_eq!(preferences.algorithm, FeedAlgorithmType::Chronological);
        assert_eq!(preferences.max_items, 100);
    }
    
    #[tokio::test]
    async fn test_save_and_get_preferences() {
        let repo = InMemoryFeedPreferencesRepository::new();
        let user_id = Uuid::new_v4();
        
        let preferences = FeedPreferences {
            algorithm: FeedAlgorithmType::Engagement,
            max_items: 50,
            include_media: false,
            include_external: false,
        };
        
        repo.save_preferences(user_id, preferences.clone()).await.unwrap();
        
        let retrieved = repo.get_preferences(user_id).await.unwrap();
        assert_eq!(retrieved.algorithm, preferences.algorithm);
        assert_eq!(retrieved.max_items, preferences.max_items);
        assert_eq!(retrieved.include_media, preferences.include_media);
        assert_eq!(retrieved.include_external, preferences.include_external);
    }
}