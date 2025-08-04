//! In-memory implementation of the RelationshipRepository trait for testing

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::domain::{
    model::{Relationship, RelationshipType},
    repository::RelationshipRepository,
};

pub struct InMemoryRelationshipRepository {
    relationships: Arc<RwLock<HashMap<Uuid, Relationship>>>,
}

impl InMemoryRelationshipRepository {
    pub fn new() -> Self {
        Self {
            relationships: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl RelationshipRepository for InMemoryRelationshipRepository {
    async fn create_relationship(&self, relationship: Relationship) -> Result<Relationship, Box<dyn std::error::Error>> {
        let mut relationships = self.relationships.write().await;
        relationships.insert(relationship.id, relationship.clone());
        Ok(relationship)
    }
    
    async fn get_relationship(&self, id: Uuid) -> Result<Option<Relationship>, Box<dyn std::error::Error>> {
        let relationships = self.relationships.read().await;
        Ok(relationships.get(&id).cloned())
    }
    
    async fn get_relationships_by_user(&self, user_id: Uuid) -> Result<Vec<Relationship>, Box<dyn std::error::Error>> {
        let relationships = self.relationships.read().await;
        Ok(relationships
            .values()
            .filter(|r| r.source_user_id == user_id || r.target_user_id == user_id)
            .cloned()
            .collect())
    }
    
    async fn get_friends(&self, user_id: Uuid) -> Result<Vec<Relationship>, Box<dyn std::error::Error>> {
        let relationships = self.relationships.read().await;
        Ok(relationships
            .values()
            .filter(|r| {
                r.source_user_id == user_id && r.relationship_type == RelationshipType::Friend
            })
            .cloned()
            .collect())
    }
    
    async fn get_followers(&self, user_id: Uuid) -> Result<Vec<Relationship>, Box<dyn std::error::Error>> {
        let relationships = self.relationships.read().await;
        Ok(relationships
            .values()
            .filter(|r| {
                r.target_user_id == user_id && r.relationship_type == RelationshipType::Follower
            })
            .cloned()
            .collect())
    }
    
    async fn get_following(&self, user_id: Uuid) -> Result<Vec<Relationship>, Box<dyn std::error::Error>> {
        let relationships = self.relationships.read().await;
        Ok(relationships
            .values()
            .filter(|r| {
                r.source_user_id == user_id && r.relationship_type == RelationshipType::Follower
            })
            .cloned()
            .collect())
    }
    
    async fn update_relationship(&self, relationship: Relationship) -> Result<Relationship, Box<dyn std::error::Error>> {
        let mut relationships = self.relationships.write().await;
        relationships.insert(relationship.id, relationship.clone());
        Ok(relationship)
    }
    
    async fn delete_relationship(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        let mut relationships = self.relationships.write().await;
        relationships.remove(&id);
        Ok(())
    }
    
    async fn get_mutual_relationships(&self, user_id: Uuid) -> Result<Vec<Relationship>, Box<dyn std::error::Error>> {
        let relationships = self.relationships.read().await;
        let user_friends: Vec<Uuid> = relationships
            .values()
            .filter(|r| {
                r.source_user_id == user_id && r.relationship_type == RelationshipType::Friend
            })
            .map(|r| r.target_user_id)
            .collect();
            
        Ok(relationships
            .values()
            .filter(|r| {
                r.source_user_id == user_id 
                && r.relationship_type == RelationshipType::Friend
                && user_friends.contains(&r.target_user_id)
            })
            .cloned()
            .collect())
    }
}