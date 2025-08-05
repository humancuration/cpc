//! Application service for social graph operations

use crate::{
    domain::{
        model::{User, Relationship, RelationshipType, Activity, ActivityType, ContentItem, FeedFilter, ContentProvider, Visibility, ContentProviderError},
        repository::RelationshipRepository,
        service::consent_service::ConsentService,
    },
    error::SocialGraphError,
    infrastructure::content_providers::ContentProviderRegistry,
};
use uuid::Uuid;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use tokio::sync::RwLock;
use std::sync::Arc;
use crate::infrastructure::content_providers::registry::ProviderChangeListener;
use uuid::Uuid;

/// Listener for content provider changes in the registry
struct SocialServiceListener;

impl SocialServiceListener {
    fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl ProviderChangeListener for SocialServiceListener {
    async fn on_provider_added(&self, provider_id: Uuid) {
        // In a real implementation, we might update caches or other state
        println!("Provider added: {}", provider_id);
    }
    
    async fn on_provider_removed(&self, provider_id: Uuid) {
        // In a real implementation, we might update caches or other state
        println!("Provider removed: {}", provider_id);
    }
}

pub struct SocialService<R: RelationshipRepository> {
    repository: Arc<R>,
    consent_service: Arc<dyn ConsentService>,
    content_provider_registry: Arc<ContentProviderRegistry>,
}

/// Wrapper for content items in the feed heap
struct FeedItem {
    item: ContentItem,
    stream_index: usize,
}

impl Ord for FeedItem {
    fn cmp(&self, other: &Self) -> Ordering {
        // Sort by relevance_score (desc) then timestamp (desc)
        other.item.relevance_score
            .partial_cmp(&self.item.relevance_score)
            .unwrap_or(Ordering::Equal)
            .then_with(|| other.item.timestamp.cmp(&self.item.timestamp))
    }
}

impl PartialOrd for FeedItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for FeedItem {
    fn eq(&self, other: &Self) -> bool {
        self.item.id == other.item.id
    }
}

impl Eq for FeedItem {}

impl<R: RelationshipRepository> SocialService<R> {
    pub fn new(
        repository: Arc<R>,
        consent_service: Arc<dyn ConsentService>,
        content_provider_registry: Arc<ContentProviderRegistry>
    ) -> Self {
        // Register as listener to registry
        content_provider_registry.add_change_listener(
            Arc::new(SocialServiceListener::new())
        );
        
        Self {
            repository,
            consent_service,
            content_provider_registry,
        }
    }
    
    /// Create a new SocialService with a content provider registry
    #[deprecated(note = "Use new() instead")]
    pub fn with_registry(
        repository: Arc<R>,
        consent_service: Arc<dyn ConsentService>,
        registry: Arc<ContentProviderRegistry>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self::new(repository, consent_service, registry))
    }
    
    pub async fn create_friendship(&self, user_id: Uuid, friend_id: Uuid) -> Result<Relationship, Box<dyn std::error::Error>> {
        // Check consent before creating relationship
        // For now, we'll skip consent check as the new ConsentService doesn't have this method
        // In a real implementation, we would integrate with the consent system properly
        
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
    ) -> Result<Vec<ContentItem>, ContentProviderError> {
        let filters = filters.unwrap_or_default();

        // Collect content from all providers using streaming aggregation
        let mut heap = BinaryHeap::new();
        let mut streams = Vec::new();

        // Initialize streams from all providers
        // Get current providers from registry
        let content_providers = self.content_provider_registry.get_all_providers()
            .map_err(|e| ContentProviderError::FetchFailed(e.to_string()))?;
            
        for provider in &content_providers {
            let items = provider.get_content(user_id, after, limit, &filters).await
                .map_err(|e| {
                    // Log the error and return it
                    eprintln!("Content provider error: {:?}", e);
                    e
                })?;
            if !items.is_empty() {
                streams.push(items.into_iter());
            }
        }

        // Fill initial heap with first item from each stream
        for (stream_index, stream) in streams.iter_mut().enumerate() {
            if let Some(item) = stream.next() {
                heap.push(FeedItem {
                    item,
                    stream_index,
                });
            }
        }

        // Merge streams using heap-based aggregation
        let mut result = Vec::with_capacity(limit);
        while let Some(feed_item) = heap.pop() {
            // Apply consent checks
            if self.consent_service
                .can_view_content(user_id, feed_item.item.owner_id, feed_item.item.visibility)
                .await
                .map_err(|e| SocialGraphError::Consent(e))?
            {
                result.push(feed_item.item);
                
                if result.len() >= limit {
                    break;
                }
            }
            
            // Add next item from the same stream to heap
            if let Some(next_item) = streams[feed_item.stream_index].next() {
                heap.push(FeedItem {
                    item: next_item,
                    stream_index: feed_item.stream_index,
                });
            }
        }
        
        Ok(result)
    }
}