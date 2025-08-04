use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::model::{Relationship, RelationshipType};

#[async_trait]
pub trait RelationshipRepository {
    async fn create_relationship(&self, relationship: Relationship) -> Result<Relationship, Box<dyn std::error::Error>>;
    async fn get_relationship(&self, id: Uuid) -> Result<Option<Relationship>, Box<dyn std::error::Error>>;
    async fn get_relationships_by_user(&self, user_id: Uuid) -> Result<Vec<Relationship>, Box<dyn std::error::Error>>;
    async fn get_friends(&self, user_id: Uuid) -> Result<Vec<Relationship>, Box<dyn std::error::Error>>;
    async fn get_followers(&self, user_id: Uuid) -> Result<Vec<Relationship>, Box<dyn std::error::Error>>;
    async fn get_following(&self, user_id: Uuid) -> Result<Vec<Relationship>, Box<dyn std::error::Error>>;
    async fn update_relationship(&self, relationship: Relationship) -> Result<Relationship, Box<dyn std::error::Error>>;
    async fn delete_relationship(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_mutual_relationships(&self, user_id: Uuid) -> Result<Vec<Relationship>, Box<dyn std::error::Error>>;
}