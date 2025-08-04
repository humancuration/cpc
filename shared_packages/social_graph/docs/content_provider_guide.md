# ContentProvider System Guide

## Overview

The ContentProvider system is a flexible, extensible architecture for aggregating content from multiple sources into a unified feed. It allows different packages in the CPC ecosystem to contribute content types while maintaining a consistent interface.

## Core Components

### ContentProvider Trait

The `ContentProvider` trait is the foundation of the system:

```rust
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
```

### ContentType Enum

The system supports several built-in content types:

```rust
pub enum ContentType {
    SocialPost,
    Video,
    JobPosting,
    CourseSnippet,
    BusinessPlan,
    CommunityEvent,
    Custom(String),
}
```

### ContentItem Structure

All content is normalized into the `ContentItem` structure:

```rust
pub struct ContentItem {
    pub id: Uuid,
    pub content_type: ContentType,
    pub source_package: String,
    pub metadata: JsonValue,
    pub timestamp: DateTime<Utc>,
    pub visibility: Visibility,
    pub relevance_score: f32,
}
```

## Implementing a ContentProvider

To implement a new content provider, you need to:

1. Create a struct that implements the `ContentProvider` trait
2. Register it with the `SocialService`

### Example Implementation

```rust
use social_graph::domain::model::{ContentItem, ContentType, FeedFilter, Visibility};
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::json;

pub struct MyCustomProvider;

#[async_trait]
impl ContentProvider for MyCustomProvider {
    fn content_type(&self) -> ContentType {
        ContentType::Custom("MyCustomType".to_string())
    }

    async fn get_content(
        &self,
        user_id: Uuid,
        after: Option<DateTime<Utc>>,
        limit: usize,
        filters: &[FeedFilter]
    ) -> Result<Vec<ContentItem>, Box<dyn std::error::Error>> {
        // Apply filters
        let mut applies = true;
        for filter in filters {
            if let Some(content_type) = &filter.content_type {
                if content_type != &self.content_type() {
                    applies = false;
                    break;
                }
            }
        }
        
        if !applies {
            return Ok(vec![]);
        }
        
        // Fetch content from your source
        let mut items = Vec::new();
        
        // Example: Create some placeholder content
        for i in 0..limit.min(5) {
            let timestamp = after.unwrap_or_else(Utc::now) - chrono::Duration::minutes(i as i64 * 5);
            
            let item = ContentItem {
                id: Uuid::new_v4(),
                content_type: self.content_type(),
                source_package: "my_custom_package".to_string(),
                metadata: json!({
                    "title": format!("Custom Content {}", i + 1),
                    "description": format!("This is custom content number {}", i + 1),
                }),
                timestamp,
                visibility: Visibility::Public,
                relevance_score: 0.7 - (i as f32 * 0.1),
            };
            
            items.push(item);
        }
        
        Ok(items)
    }
}
```

## Registering ContentProviders

Content providers are registered with the `SocialService`:

```rust
use social_graph::{
    application::SocialService,
    infrastructure::{
        content_providers::{register_providers},
        in_memory_repository::InMemoryRelationshipRepository,
        consent_adapter::ConsentAdapter,
    },
};
use std::sync::Arc;

// Create social service
let repository = Arc::new(InMemoryRelationshipRepository::new());
let consent_service = consent_manager::ConsentService::new();
let consent_adapter = Arc::new(ConsentAdapter::new(consent_service));
let mut social_service = SocialService::new(repository, consent_adapter);

// Register all built-in providers
register_providers(&mut social_service);

// Or register individual providers
social_service.register_content_provider(Arc::new(MyCustomProvider));
```

## Using the Universal Feed

Once providers are registered, you can fetch the universal feed:

```rust
use social_graph::domain::model::{ContentType, FeedFilter};
use uuid::Uuid;

let user_id = Uuid::new_v4();

// Get all content
let all_content = social_service.get_universal_feed(
    user_id,
    None,  // after timestamp
    20,    // limit
    None   // filters
).await?;

// Get only social posts
let social_post_filter = vec![FeedFilter {
    content_type: Some(ContentType::SocialPost),
    package: None,
    visibility: None,
};

let social_posts = social_service.get_universal_feed(
    user_id,
    None,
    20,
    Some(social_post_filter)
).await?;
```

## Consent and Privacy

The system integrates with the consent management system to ensure content is only shown to users who have appropriate consent. The `apply_consent` method in `SocialService` handles this filtering.

## Performance Considerations

- Content providers should implement efficient pagination using the `after` parameter
- Results should be limited to the requested `limit` parameter
- Providers should apply filters early to avoid fetching unnecessary data
- Consider implementing caching for frequently accessed content

## Extending the System

To add new content types:

1. Add the type to the `ContentType` enum if it's not already there
2. Implement a new `ContentProvider` for your content source
3. Register the provider with the `SocialService`
4. Update the consent system if needed for new content types