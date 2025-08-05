//! Full example showing how to use all components of the social_graph package

use social_graph::{
    User, Relationship, RelationshipType, Activity, ActivityType,
    ContentType, Visibility, ContentItem, FeedFilter,
    InMemoryRelationshipRepository, RelationshipRepository,
    SocialService, create_schema, SocialGraphSchema, ConsentServiceImpl
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
    
    // 3. Create consent service
    let consent_service = Arc::new(ConsentServiceImpl::new(repository.clone()));
    
    // 4. Create social service
    let social_service = SocialService::new(repository.clone(), consent_service);
    
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
        Some(serde_json::json!({"title": "Hello World"})),
        );
        
    println!("Created activity for user {}: {:?}", user1.username, activity.activity_type);
    
    Ok(())
}