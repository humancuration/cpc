use super::*;
use crate::domain::model::{ContentItem, ContentType, FeedFilter, Visibility};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::json;

pub struct VideoProvider;

#[async_trait]
impl ContentProvider for VideoProvider {
    fn content_type(&self) -> ContentType {
        ContentType::Video
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
                if content_type != &ContentType::Video {
                    applies = false;
                    break;
                }
            }
        }
        
        if !applies {
            return Ok(vec![]);
        }
        
        // In a real implementation, we would fetch videos from a repository
        // For now, we'll create some placeholder content
        let mut items = Vec::new();
        
        // Create a few example videos
        for i in 0..limit.min(2) {
            let timestamp = after.unwrap_or_else(Utc::now) - chrono::Duration::minutes(i as i64 * 15);
            
            let item = ContentItem {
                id: Uuid::new_v4(),
                owner_id: user_id,
                content_type: ContentType::Video,
                source_package: "video_package".to_string(),
                metadata: json!({
                    "title": format!("Video Content {}", i + 1),
                    "description": format!("This is a video about topic number {}", i + 1),
                    "duration": "00:05:30",
                    "author_id": user_id.to_string()
                }),
                timestamp,
                visibility: Visibility::Public,
                relevance_score: 0.9 - (i as f32 * 0.1),
            };
            
            items.push(item);
        }
        
        Ok(items)
    }
}