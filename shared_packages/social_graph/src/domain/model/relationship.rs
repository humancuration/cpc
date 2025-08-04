use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    Friend,
    Follower,
    Blocked,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub id: Uuid,
    pub source_user_id: Uuid,
    pub target_user_id: Uuid,
    pub relationship_type: RelationshipType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

impl Relationship {
    pub fn new(source_user_id: Uuid, target_user_id: Uuid, relationship_type: RelationshipType) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            source_user_id,
            target_user_id,
            relationship_type,
            created_at: now,
            updated_at: now,
            is_active: true,
        }
    }
    
    pub fn is_mutual(&self, other: &Relationship) -> bool {
        self.source_user_id == other.target_user_id 
        && self.target_user_id == other.source_user_id
        && self.relationship_type == RelationshipType::Friend
        && other.relationship_type == RelationshipType::Friend
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_relationship_creation() {
        let source_user_id = Uuid::new_v4();
        let target_user_id = Uuid::new_v4();
        
        let relationship = Relationship::new(
            source_user_id,
            target_user_id,
            RelationshipType::Friend,
        );
        
        assert_eq!(relationship.source_user_id, source_user_id);
        assert_eq!(relationship.target_user_id, target_user_id);
        assert_eq!(relationship.relationship_type, RelationshipType::Friend);
        assert!(relationship.is_active);
    }
    
    #[test]
    fn test_mutual_relationship() {
        let user1_id = Uuid::new_v4();
        let user2_id = Uuid::new_v4();
        
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
        
        assert!(relationship1.is_mutual(&relationship2));
    }
}