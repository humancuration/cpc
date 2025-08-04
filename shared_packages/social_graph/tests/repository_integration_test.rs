//! Integration tests for the social_graph repository

use social_graph::{
    User, Relationship, RelationshipType,
    InMemoryRelationshipRepository, RelationshipRepository
};
use uuid::Uuid;

#[tokio::test]
async fn test_relationship_repository() -> Result<(), Box<dyn std::error::Error>> {
    let repo = InMemoryRelationshipRepository::new();
    
    // Create users
    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();
    
    // Create a relationship
    let relationship = Relationship::new(
        user1_id,
        user2_id,
        RelationshipType::Friend,
    );
    
    // Save the relationship
    let saved_relationship = repo.create_relationship(relationship.clone()).await?;
    assert_eq!(saved_relationship.id, relationship.id);
    
    // Retrieve the relationship
    let retrieved = repo.get_relationship(relationship.id).await?;
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, relationship.id);
    
    // Get relationships by user
    let user_relationships = repo.get_relationships_by_user(user1_id).await?;
    assert_eq!(user_relationships.len(), 1);
    assert_eq!(user_relationships[0].id, relationship.id);
    
    // Get friends
    let friends = repo.get_friends(user1_id).await?;
    assert_eq!(friends.len(), 1);
    assert_eq!(friends[0].id, relationship.id);
    
    // Update relationship
    let mut updated_relationship = relationship.clone();
    updated_relationship.relationship_type = RelationshipType::Blocked;
    let updated = repo.update_relationship(updated_relationship).await?;
    assert_eq!(updated.relationship_type, RelationshipType::Blocked);
    
    // Delete relationship
    repo.delete_relationship(relationship.id).await?;
    let deleted = repo.get_relationship(relationship.id).await?;
    assert!(deleted.is_none());
    
    Ok(())
}

#[tokio::test]
async fn test_mutual_relationships() -> Result<(), Box<dyn std::error::Error>> {
    let repo = InMemoryRelationshipRepository::new();
    
    // Create users
    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();
    
    // Create mutual friendship (user1 -> user2 and user2 -> user1)
    let relationship1 = Relationship::new(
        user1_id,
        user2_id,
        RelationshipType::Friend,
    );
    
    let relationship2 = Relationship::new(
        user2_id,
        user1_id,
        RelationshipType::Friend,
    );
    
    // Save both relationships
    repo.create_relationship(relationship1).await?;
    repo.create_relationship(relationship2).await?;
    
    // Check mutual relationships
    let mutual = repo.get_mutual_relationships(user1_id).await?;
    assert_eq!(mutual.len(), 1);
    
    Ok(())
}