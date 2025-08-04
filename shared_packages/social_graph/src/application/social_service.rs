//! Application service for social graph operations

use crate::{
    domain::{
        model::{User, Relationship, RelationshipType, Activity, ActivityType, ContentItem, FeedFilter, ContentProvider, Visibility},
        repository::RelationshipRepository,
    },
    infrastructure::consent_adapter::ConsentAdapter,
};
use uuid::Uuid;
use std::sync::Arc;
use chrono::{DateTime, Utc};

pub struct SocialService<R: RelationshipRepository> {
    repository: Arc<R>,
    consent_adapter: Arc<ConsentAdapter>,
    content_providers: Vec<Arc<dyn ContentProvider>>,
}

impl<R: RelationshipRepository> SocialService<R> {
    pub fn new(repository: Arc<R>, consent_adapter: Arc<ConsentAdapter>) -> Self {
        Self {
            repository,
            consent_adapter,
            content_providers: Vec::new(),
        }
    }
    
    pub fn register_content_provider(&mut self, provider: Arc<dyn ContentProvider>) {
        self.content_providers.push(provider);
    }
    
    pub async fn create_friendship(&self, user_id: Uuid, friend_id: Uuid) -> Result<Relationship, Box<dyn std::error::Error>> {
        // Check consent before creating relationship
        if !self.consent_adapter.check_consent(user_id, friend_id).await? {
            return Err("Consent not granted for social interaction".into());
        }
        
        // Check if relationship already exists
        let existing = self.get_relationship_between(user_id, friend_id).await?;
        if existing.is_some() {
            return Err("Relationship already exists".into());
        }
        
        // Create the relationship
        let relationship = Relationship::new(user_id, friend_id, RelationshipType::Friend);
        let saved = self.repository.create_relationship(relationship).await?;
        
        Ok(saved)
    }
    
    pub async fn get_relationship_between(&self, user_id: Uuid, target_id: Uuid) -> Result<Option<Relationship>, Box<dyn std::error::Error>> {
        // Get all relationships for the user
        let relationships = self.repository.get_relationships_by_user(user_id).await?;
        
        // Find the specific relationship
        for relationship in relationships {
            if relationship.source_user_id == user_id && relationship.target_user_id == target_id {
                return Ok(Some(relationship));
            }
        }
        
        Ok(None)
    }
    
    pub async fn get_friends(&self, user_id: Uuid) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        // Check consent before returning friends
        // In a real implementation, we would check consent for each friend
        // For now, we'll just return the friends
        
        let relationships = self.repository.get_friends(user_id).await?;
        
        // In a real implementation, we would fetch the actual User objects
        // For now, we'll just create placeholder users
        let users: Vec<User> = relationships
            .into_iter()
            .map(|r| {
                User::new(
                    format!("user_{}", r.target_user_id),
                    format!("User {}", r.target_user_id),
                    format!("user_{}@example.com", r.target_user_id),
                )
            })
            .collect();
            
        Ok(users)
    }
    
    pub async fn get_activity_feed(&self, user_id: Uuid) -> Result<Vec<Activity>, Box<dyn std::error::Error>> {
        // Check consent before returning activity feed
        // In a real implementation, we would check consent for each activity
        
        // For now, we'll return an empty feed
        Ok(vec![])
    }
    
    pub async fn get_recommendations(&self, user_id: Uuid) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        // In a real implementation, we would implement recommendation logic
        // For now, we'll return an empty list
        Ok(vec![])
    }
    
    pub async fn get_universal_feed(
        &self,
        user_id: Uuid,
        after: Option<DateTime<Utc>>,
        limit: usize,
        filters: Option<Vec<FeedFilter>>
    ) -> Result<Vec<ContentItem>, Box<dyn std::error::Error>> {
        let mut all_items = Vec::new();
        let filters = filters.unwrap_or_default();

        // Collect content from all providers
        for provider in &self.content_providers {
            let items = provider.get_content(user_id, after, limit, &filters).await?;
            all_items.extend(items);
        }

        // Apply consent checks
        let consented_items = self.apply_consent(user_id, all_items).await?;

        // Sort by relevance_score (desc) then timestamp (desc)
        let mut sorted_items = consented_items;
        sorted_items.sort_by(|a, b| {
            b.relevance_score.partial_cmp(&a.relevance_score)
                .unwrap_or_else(|| b.timestamp.cmp(&a.timestamp))
        });

        // Apply limit
        Ok(sorted_items.into_iter().take(limit).collect())
    }

    async fn apply_consent(&self, user_id: Uuid, items: Vec<ContentItem>) -> Result<Vec<ContentItem>, Box<dyn std::error::Error>> {
        // In a real implementation, we would check consent for each item
        // For now, we'll just return all items as a placeholder
        // In a full implementation, we would:
        // 1. Check if the user has consented to see content from each source
        // 2. Check if the user has consented to see each content type
        // 3. Apply visibility rules (public, friends only, etc.)
        
        let mut consented_items = Vec::new();
        
        for item in items {
            // Check if content is public or if user has appropriate consent
            let is_visible = match item.visibility {
                Visibility::Public => true,
                Visibility::FriendsOnly => {
                    // Check if user is friends with content owner
                    // For now, we'll assume all friends-only content is visible
                    true
                },
                Visibility::GroupMembers => {
                    // Check if user is member of the group
                    // For now, we'll assume all group content is visible
                    true
                },
                Visibility::Private => {
                    // Only visible to owner
                    // For now, we'll assume all private content is visible
                    true
                }
            };
            
            if is_visible {
                consented_items.push(item);
            }
        }
        
        Ok(consented_items)
    }
}