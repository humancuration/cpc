//! Integration tests for the social_graph package

use social_graph::{
    User, Relationship, RelationshipType, Activity, ActivityType,
    ContentType, Visibility, ContentItem, FeedFilter,
    ConsentAdapter
};
use uuid::Uuid;

#[test]
fn test_user_creation() {
    let user = User::new(
        "testuser".to_string(),
        "Test User".to_string(),
        "test@example.com".to_string(),
    );
    
    assert_eq!(user.username, "testuser");
    assert_eq!(user.display_name, "Test User");
    assert_eq!(user.email, "test@example.com");
    assert!(user.is_active);
}

#[test]
fn test_relationship_creation() {
    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();
    
    let relationship = Relationship::new(
        user1_id,
        user2_id,
        RelationshipType::Friend,
    );
    
    assert_eq!(relationship.source_user_id, user1_id);
    assert_eq!(relationship.target_user_id, user2_id);
    assert_eq!(relationship.relationship_type, RelationshipType::Friend);
    assert!(relationship.is_active);
}

#[test]
fn test_activity_creation() {
    let user_id = Uuid::new_v4();
    let target_id = Some(Uuid::new_v4());
    let metadata = Some(serde_json::json!({"message": "test"}));
    
    let activity = Activity::new(
        user_id,
        ActivityType::PostCreated,
        target_id,
        Some("post".to_string()),
        metadata.clone(),
        true,
    );
    
    assert_eq!(activity.user_id, user_id);
    assert_eq!(activity.activity_type, ActivityType::PostCreated);
    assert_eq!(activity.target_id, target_id);
    assert_eq!(activity.target_type, Some("post".to_string()));
    assert_eq!(activity.metadata, metadata);
    assert!(activity.is_public);
}

#[test]
fn test_content_item_creation() {
    let id = Uuid::new_v4();
    let timestamp = chrono::Utc::now();
    let metadata = serde_json::json!({
        "title": "Test Post",
        "content": "Hello World"
    });
    
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

// Note: Integration tests with the consent_manager would require a running service
// or a mock implementation, which is beyond the scope of this basic test.