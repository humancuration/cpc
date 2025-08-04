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
    ) -> Result<Vec<ContentItem>, Box<dyn std::error::Error>>;
}

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