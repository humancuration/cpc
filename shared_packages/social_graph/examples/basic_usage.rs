//! Basic usage example for the social_graph package

use social_graph::{
    User, Relationship, RelationshipType, Activity, ActivityType,
    create_schema
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create users
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
    
    // Create a relationship
    let relationship = Relationship::new(
        user1.id,
        user2.id,
        RelationshipType::Friend
    );
    
    println!("Created relationship between {} and {}", user1.username, user2.username);
    
    // Create an activity
    let activity = Activity::new(
        user1.id,
        ActivityType::PostCreated,
        Some(Uuid::new_v4()),
        Some("post".to_string()),
        Some(serde_json::json!({"title": "Hello World"})),
        true
    );
    
    
    println!("Created activity for user {}", user1.username);
    
    // In a real app, you would create a ConsentService implementation
    // let consent_service = ConsentService::new(/* storage implementation */);
    // Create GraphQL schema
    let schema = create_schema();
    println!("Created GraphQL schema: {:?}", schema);
    
    Ok(())
}