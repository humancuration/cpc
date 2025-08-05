use super::*;
use crate::domain::model::{ContentItem, ContentType, FeedFilter, Visibility};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::json;

#[async_trait]
impl ContentProvider for SocialPostProvider {
    fn content_type(&self) -> ContentType {
        ContentType::SocialPost
    }

    async fn get_content(
        &self,
        user_id: Uuid,
        after: Option<DateTime<Utc>>,
        limit: usize,
        filters: &[FeedFilter]
    ) -> Result<Vec<ContentItem>, ContentProviderError> {
        // Check if filters apply to this content type
        let mut applies = true;
        for filter in filters {
            if let Some(content_type) = &filter.content_type {
                if content_type != &ContentType::SocialPost {
                    applies = false;
                    break;
                }
            }
        }
        
        if !applies {
            return Ok(vec![]);
        }
        
        // In a real implementation, we would fetch social posts from a repository
        // For now, we'll create some placeholder content
        let mut items = Vec::new();
        
        // Create a few example social posts
        for i in 0..limit.min(3) {
            let timestamp = after.unwrap_or_else(Utc::now) - chrono::Duration::minutes(i as i64 * 10);
            
            let item = ContentItem {
                id: Uuid::new_v4(),
                owner_id: user_id,
                content_type: ContentType::SocialPost,
                source_package: "social_graph".to_string(),
                metadata: json!({
                    "title": format!("Social Post {}", i + 1),
                    "content": format!("This is the content of social post number {}", i + 1),
                    "author_id": user_id.to_string()
                }),
                timestamp,
                visibility: Visibility::Public,
                relevance_score: 0.8 - (i as f32 * 0.1),
            };
            
            items.push(item);
        }
        
        Ok(items)
    }
}