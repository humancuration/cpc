//! Consent middleware for filtering content based on user consent preferences

use crate::domain::model::{ContentItem, ContentType, FeedFilter, ContentProvider, ContentProviderError};
use crate::domain::service::consent_service::{ConsentService, ConsentError};
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;

/// Middleware that wraps a content provider and applies consent filtering
pub struct ConsentMiddleware {
    /// The wrapped content provider
    provider: Arc<dyn ContentProvider>,
    
    /// The consent service for checking permissions
    consent_service: Arc<dyn ConsentService>,
}

impl ConsentMiddleware {
    /// Create a new consent middleware
    pub fn new(
        provider: Arc<dyn ContentProvider>,
        consent_service: Arc<dyn ConsentService>,
    ) -> Self {
        Self {
            provider,
            consent_service,
        }
    }
    
    /// Create a new consent middleware with state migrator capabilities
    pub fn with_state_migrator(
        provider: Arc<dyn ContentProvider>,
        consent_service: Arc<dyn ConsentService>,
    ) -> Self {
        Self::new(provider, consent_service)
    }
}

#[async_trait]
impl ContentProvider for ConsentMiddleware {
    fn content_type(&self) -> ContentType {
        self.provider.content_type()
    }

    async fn get_content(
        &self,
        user_id: Uuid,
        after: Option<DateTime<Utc>>,
        limit: usize,
        filters: &[FeedFilter]
    ) -> Result<Vec<ContentItem>, ContentProviderError> {
        // Get content from the wrapped provider
        let content = self.provider.get_content(user_id, after, limit, filters).await?;
        
        // Filter content based on consent
        let mut filtered_content = Vec::new();
        
        for item in content {
            // Check if the user can view this content based on visibility settings
            match self.consent_service
                .can_view_content(user_id, item.owner_id, item.visibility)
                .await
            {
                Ok(true) => {
                    // User can view the content, add it to the filtered list
                    filtered_content.push(item);
                }
                Ok(false) => {
                    // User cannot view the content, skip it
                    continue;
                }
                Err(ConsentError::FriendshipCheckFailed) => {
                    // Log the error but don't fail the entire operation
                    eprintln!("Friendship check failed for user {} and owner {}", user_id, item.owner_id);
                    continue;
                }
                Err(ConsentError::GroupCheckFailed) => {
                    // Log the error but don't fail the entire operation
                    eprintln!("Group membership check failed for user {} and owner {}", user_id, item.owner_id);
                    continue;
                }
                Err(ConsentError::Other(msg)) => {
                    // Log the error but don't fail the entire operation
                    eprintln!("Consent check failed: {}", msg);
                    continue;
                }
            }
        }
        
        Ok(filtered_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::{Visibility, ContentType};
    use crate::infrastructure::content_providers::SocialPostProvider;
    use crate::infrastructure::consent_service_impl::ConsentServiceImpl;
    use crate::infrastructure::in_memory_repository::InMemoryRelationshipRepository;
    use std::sync::Arc;
    use uuid::Uuid;

    // Mock consent service for testing
    struct MockConsentService {
        can_view: bool,
    }

    #[async_trait]
    impl ConsentService for MockConsentService {
        async fn can_view_content(
            &self,
            _viewer_id: Uuid,
            _content_owner_id: Uuid,
            _visibility: Visibility,
        ) -> Result<bool, ConsentError> {
            Ok(self.can_view)
        }
    }

    #[tokio::test]
    async fn test_consent_middleware_allows_content() {
        let provider = Arc::new(SocialPostProvider);
        let consent_service = Arc::new(MockConsentService { can_view: true });
        let middleware = ConsentMiddleware::new(provider, consent_service);
        
        let user_id = Uuid::new_v4();
        let content = middleware.get_content(
            user_id,
            None,
            5,
            &[]
        ).await.unwrap();
        
        // Should return some content since consent is granted
        assert!(!content.is_empty());
    }
    
    #[tokio::test]
    async fn test_consent_middleware_blocks_content() {
        let provider = Arc::new(SocialPostProvider);
        let consent_service = Arc::new(MockConsentService { can_view: false });
        let middleware = ConsentMiddleware::new(provider, consent_service);
        
        let user_id = Uuid::new_v4();
        let content = middleware.get_content(
            user_id,
            None,
            5,
            &[]
        ).await.unwrap();
        
        // Should return no content since consent is denied
        assert!(content.is_empty());
    }
}