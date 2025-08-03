//! Social integration service for cross-app social features
//!
//! This service handles the integration of social features across CPC apps,
//! including unified feeds, cross-posting, and social event tracking.

use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::{
    post::{UnifiedPost, AppSource},
    social_event::SocialEvent,
};
use std::collections::HashMap;

/// Repository trait for unified post persistence
#[async_trait]
pub trait UnifiedPostRepository {
    /// Save a unified post
    async fn save(&self, post: &UnifiedPost) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    
    /// Find a unified post by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<UnifiedPost>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Find unified posts by author
    async fn find_by_author(&self, author_id: Uuid) -> Result<Vec<UnifiedPost>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Find unified posts from a specific source
    async fn find_by_source(&self, source: AppSource) -> Result<Vec<UnifiedPost>, Box<dyn std::error::Error + Send + Sync>>;
}

/// Service for integrating social features across apps
#[derive(Debug)]
pub struct SocialIntegrationService {
    post_repository: Box<dyn UnifiedPostRepository + Send + Sync>,
    app_clients: HashMap<AppSource, Box<dyn AppClient + Send + Sync>>,
}

impl SocialIntegrationService {
    /// Create a new social integration service
    pub fn new(
        post_repository: Box<dyn UnifiedPostRepository + Send + Sync>,
    ) -> Self {
        Self {
            post_repository,
            app_clients: HashMap::new(),
        }
    }
    
    /// Add an app client
    pub fn add_app_client(&mut self, source: AppSource, client: Box<dyn AppClient + Send + Sync>) {
        self.app_clients.insert(source, client);
    }
    
    /// Create a unified post from an app post
    pub async fn create_unified_post(
        &self,
        source: AppSource,
        original_id: Uuid,
        author_id: Uuid,
        content: String,
    ) -> Result<UnifiedPost, Box<dyn std::error::Error + Send + Sync>> {
        // Get the app client for the source
        let app_client = self.app_clients.get(&source)
            .ok_or("No client for source app")?;
        
        // Get additional metadata from the app
        let metadata = app_client.get_post_metadata(original_id).await?;
        
        // Create the unified post
        let post = UnifiedPost::new(source, original_id, author_id, content, metadata);
        
        // Save the unified post
        self.post_repository.save(&post).await?;
        
        Ok(post)
    }
    
    /// Handle a social event
    pub async fn handle_social_event(&self, event: SocialEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // In a real implementation, this would process the event for analytics and tracking purposes
                // For now, we'll just log the event
        tracing::info!("Handling social event: {:?}", event);
        Ok(())
    }
    
    /// Get a unified post by ID
    pub async fn get_unified_post(&self, id: Uuid) -> Result<Option<UnifiedPost>, Box<dyn std::error::Error + Send + Sync>> {
        self.post_repository.find_by_id(id).await
    }
    
    /// Get unified posts by author
    pub async fn get_posts_by_author(&self, author_id: Uuid) -> Result<Vec<UnifiedPost>, Box<dyn std::error::Error + Send + Sync>> {
        self.post_repository.find_by_author(author_id).await
    }
    
    /// Get unified posts from a specific source
    pub async fn get_posts_by_source(&self, source: AppSource) -> Result<Vec<UnifiedPost>, Box<dyn std::error::Error + Send + Sync>> {
        self.post_repository.find_by_source(source).await
    }
}

/// Trait for app clients that can provide post metadata
#[async_trait]
pub trait AppClient {
    /// Get metadata for a post
    async fn get_post_metadata(&self, post_id: Uuid) -> Result<crate::domain::post::PostMetadata, Box<dyn std::error::Error + Send + Sync>>;
}

/// Trait for consuming social events
#[async_trait]
pub trait SocialEventConsumer: Send + Sync {
    /// Handle a social event
    async fn handle_event(&self, event: SocialEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// Service for handling stream events
#[async_trait]
pub trait StreamEventService: Send + Sync {
    /// Handle a stream started event
    async fn handle_stream_started(&self, user_id: Uuid, stream_id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    
    /// Handle a stream ended event
    async fn handle_stream_ended(&self, user_id: Uuid, stream_id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    
    /// Handle a viewer joined event
    async fn handle_viewer_joined(&self, user_id: Uuid, broadcaster_id: Uuid, stream_id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    
    /// Handle a chat message sent event
    async fn handle_chat_message_sent(&self, user_id: Uuid, stream_id: Uuid, message_id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    
    /// Handle a subscription created event
    async fn handle_subscription_created(&self, user_id: Uuid, channel_id: Uuid, tier: crate::domain::social_event::SubscriptionTier) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}