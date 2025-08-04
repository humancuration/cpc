//! Full example showing how to use all components of the social_graph package

use social_graph::{
    User, Relationship, RelationshipType, Activity, ActivityType,
    ContentType, Visibility, ContentItem, FeedFilter,
    ConsentAdapter, InMemoryRelationshipRepository, RelationshipRepository,
    SocialService, create_schema, SocialGraphSchema
};
use std::sync::Arc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Social Graph Full Example ===");
    
    // 1. Create users
    let user1 = User::new(
        "alice".to_string(),
        "Alice Smith".to_string(),
        "alice@example.com".to_string()
    );
    
    let user2 = User::new(
        "bob".to_string(),
        "Bob Johnson".to_string(),
        "bob@example.com".to_string()
    );
    
    println!("Created users: {} and {}", user1.username, user2.username);
    
    // 2. Create repository
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    
    // 3. Create consent adapter (in a real app, you would inject the actual ConsentService)
    // For this example, we'll create a mock consent adapter
    let consent_adapter = Arc::new(create_mock_consent_adapter());
    
    // 4. Create social service
    let social_service = SocialService::new(repository.clone(), consent_adapter.clone());
    
    // 5. Create friendship
    match social_service.create_friendship(user1.id, user2.id).await {
        Ok(relationship) => {
            println!("Created friendship between {} and {}", user1.username, user2.username);
            println!("Relationship ID: {}", relationship.id);
        },
        Err(e) => {
            println!("Failed to create friendship: {}", e);
        }
    }
    
    // 6. Get friends
    match social_service.get_friends(user1.id).await {
        Ok(friends) => {
            println!("{} has {} friends:", user1.username, friends.len());
            for friend in friends {
                println!("  - {}", friend.display_name);
            }
        },
        Err(e) => {
            println!("Failed to get friends: {}", e);
        }
    }
    
    // 7. Create an activity
    let activity = Activity::new(
        user1.id,
        ActivityType::PostCreated,
        Some(Uuid::new_v4()),
        Some("post".to_string()),
        Some(serde_json::json!({"title": "Hello World"})),
        );
        
        println!("Created activity for user {}: {:?}", user1.username, activity.activity_type);
        
        // 8. Create a content item for the universal feed
        let content_item = ContentItem {
            id: Uuid::new_v4(),
            content_type: ContentType::SocialPost,
            source_package: "social_graph".to_string(),
            metadata: serde_json::json!({
                "title": "Hello World",
                "content": "This is my first post in the universal feed!"
            }),
            timestamp: chrono::Utc::now(),
            visibility: Visibility::Public,
            relevance_score: 0.9,
        };
        
        println!("Created content item for universal feed: {:?}", content_item.content_type);
        
        // 9. Create GraphQL schema
        let schema = create_schema();
        println!("Created GraphQL schema: {:?}", schema);
        
        println!("\n=== Example completed successfully ===");
        
        Ok(())
    }
}

// Mock consent adapter for demonstration purposes
fn create_mock_consent_adapter() -> ConsentAdapter {
    // In a real implementation, this would use the actual consent_manager crate
    // For this example, we'll create a simple mock
    
    // Since we can't easily create a mock ConsentService without the storage,
    // we'll create a simplified version that always returns true for consent
    
    // This is a placeholder implementation - in a real app you would inject
    // the actual ConsentService from the consent_manager crate
    ConsentAdapter::new(/* actual ConsentService would go here */)
}

// Note: The above function won't compile because we need to provide a real ConsentService.
// In a real application, you would do something like this:
//
// let consent_storage = /* your storage implementation */;
// let consent_service = ConsentService::new(consent_storage);
// let consent_adapter = ConsentAdapter::new(consent_service);