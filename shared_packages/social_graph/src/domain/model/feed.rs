use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;
use async_trait::async_trait;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentType {
    SocialPost,
    Video,
    JobPosting,
    CourseSnippet,
    BusinessPlan,
    CommunityEvent,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Visibility {
    Public,
    FriendsOnly,
    GroupMembers,
    Private,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItem {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub content_type: ContentType,
    pub source_package: String,
    pub metadata: JsonValue,
    pub timestamp: DateTime<Utc>,
    pub visibility: Visibility,
    pub relevance_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedFilter {
    pub content_type: Option<ContentType>,
    pub package: Option<String>,
    pub visibility: Option<Visibility>,
}

#[async_trait]
pub trait ContentProvider: Send + Sync {
    fn content_type(&self) -> ContentType;
    async fn get_content(
        &self,
        user_id: Uuid,
        after: Option<DateTime<Utc>>,
        limit: usize,
        filters: &[FeedFilter]
    ) -> Result<Vec<ContentItem>, ContentProviderError>;
    
    /// Serialize the provider's state for migration purposes
    fn serialize_state(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Default implementation returns empty state
        Ok(Vec::new())
    }
    
    /// Deserialize state into the provider
    fn deserialize_state(&self, _data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // Default implementation does nothing
        Ok(())
    }
}

/// Errors that can occur during content provider operations
#[derive(Debug)]
pub enum ContentProviderError {
    /// Failed to fetch content from the provider
    FetchFailed(String),
    
    /// Consent check failed for a content item
    ConsentCheckFailed(Uuid),
    
    /// Invalid parameters provided to the content provider
    InvalidParameters,
    
    /// A required dependency is unavailable
    DependencyUnavailable(String),
    
    /// Failed to serialize provider state during migration
    StateSerializationError,
    
    /// Failed to deserialize provider state during migration
    StateDeserializationError,
}

impl std::fmt::Display for ContentProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentProviderError::FetchFailed(msg) => write!(f, "Fetch failed: {}", msg),
            ContentProviderError::ConsentCheckFailed(id) => write!(f, "Consent check failed for user: {}", id),
            ContentProviderError::InvalidParameters => write!(f, "Invalid parameters provided"),
            ContentProviderError::DependencyUnavailable(dep) => write!(f, "Dependency unavailable: {}", dep),
            ContentProviderError::StateSerializationError => write!(f, "Failed to serialize provider state"),
            ContentProviderError::StateDeserializationError => write!(f, "Failed to deserialize provider state"),
        }
    }
}

impl std::error::Error for ContentProviderError {}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use serde_json::json;

    #[test]
    fn test_content_type_enum() {
        let content_type = ContentType::SocialPost;
        assert_eq!(content_type, ContentType::SocialPost);
        
        let custom_type = ContentType::Custom("CustomType".to_string());
        match custom_type {
            ContentType::Custom(name) => assert_eq!(name, "CustomType"),
            _ => panic!("Expected Custom variant"),
        }
    }

    #[test]
    fn test_visibility_enum() {
        let visibility = Visibility::Public;
        assert_eq!(visibility, Visibility::Public);
    }

    #[test]
    fn test_content_item_creation() {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();
        let metadata = json!({"title": "Test Post", "content": "Hello World"});
        let content_item = ContentItem {
            id,
            owner_id: Uuid::new_v4(),
            content_type: ContentType::SocialPost,
            source_package: "social_graph".to_string(),
            metadata: metadata.clone(),
            timestamp,
            visibility: Visibility::Public,
            relevance_score: 0.8,
        };
        
        assert_eq!(content_item.id, id);
        assert_eq!(content_item.content_type, ContentType::SocialPost);
        assert_eq!(content_item.source_package, "social_graph");
        assert_eq!(content_item.metadata, metadata);
        assert_eq!(content_item.visibility, Visibility::Public);
        assert_eq!(content_item.relevance_score, 0.8);
    }

    #[test]
    fn test_feed_filter_creation() {
        let filter = FeedFilter {
            content_type: Some(ContentType::Video),
            package: Some("video_package".to_string()),
            visibility: Some(Visibility::FriendsOnly),
        };
        
        assert_eq!(filter.content_type, Some(ContentType::Video));
        assert_eq!(filter.package, Some("video_package".to_string()));
        assert_eq!(filter.visibility, Some(Visibility::FriendsOnly));
    }
}