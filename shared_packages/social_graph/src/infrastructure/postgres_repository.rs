//! PostgreSQL implementation of the RelationshipRepository trait

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::{
    model::{Relationship, RelationshipType, User, Activity, ActivityType},
    repository::RelationshipRepository,
};

pub struct PostgresRelationshipRepository {
    pool: PgPool,
}

impl PostgresRelationshipRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RelationshipRepository for PostgresRelationshipRepository {
    async fn create_relationship(&self, relationship: Relationship) -> Result<Relationship, Box<dyn std::error::Error>> {
        let rec = sqlx::query!(
            r#"
            INSERT INTO social_relationships (id, source_user_id, target_user_id, relationship_type, created_at, updated_at, is_active)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, source_user_id, target_user_id, relationship_type as "relationship_type: String", created_at, updated_at, is_active
            "#,
            relationship.id,
            relationship.source_user_id,
            relationship.target_user_id,
            match relationship.relationship_type {
                RelationshipType::Friend => "friend",
                RelationshipType::Follower => "follower",
                RelationshipType::Blocked => "blocked",
                RelationshipType::Pending => "pending",
            },
            relationship.created_at,
            relationship.updated_at,
            relationship.is_active
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Relationship {
            id: rec.id,
            source_user_id: rec.source_user_id,
            target_user_id: rec.target_user_id,
            relationship_type: match rec.relationship_type.as_str() {
                "friend" => RelationshipType::Friend,
                "follower" => RelationshipType::Follower,
                "blocked" => RelationshipType::Blocked,
                "pending" => RelationshipType::Pending,
                _ => RelationshipType::Pending, // Default fallback
            },
            created_at: rec.created_at,
            updated_at: rec.updated_at,
            is_active: rec.is_active,
        })
    }
    
    async fn get_relationship(&self, id: Uuid) -> Result<Option<Relationship>, Box<dyn std::error::Error>> {
        let rec = sqlx::query!(
            r#"
            SELECT id, source_user_id, target_user_id, relationship_type as "relationship_type: String", created_at, updated_at, is_active
            FROM social_relationships
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match rec {
            Some(rec) => Ok(Some(Relationship {
                id: rec.id,
                source_user_id: rec.source_user_id,
                target_user_id: rec.target_user_id,
                relationship_type: match rec.relationship_type.as_str() {
                    "friend" => RelationshipType::Friend,
                    "follower" => RelationshipType::Follower,
                    "blocked" => RelationshipType::Blocked,
                    "pending" => RelationshipType::Pending,
                    _ => RelationshipType::Pending, // Default fallback
                },
                created_at: rec.created_at,
                updated_at: rec.updated_at,
                is_active: rec.is_active,
            })),
            None => Ok(None),
        }
    }
    
    async fn get_relationships_by_user(&self, user_id: Uuid) -> Result<Vec<Relationship>, Box<dyn std::error::Error>> {
        let recs = sqlx::query!(
            r#"
            SELECT id, source_user_id, target_user_id, relationship_type as "relationship_type: String", created_at, updated_at, is_active
            FROM social_relationships
            WHERE (source_user_id = $1 OR target_user_id = $1) AND is_active = true
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let relationships = recs
            .into_iter()
            .map(|rec| Relationship {
                id: rec.id,
                source_user_id: rec.source_user_id,
                target_user_id: rec.target_user_id,
                relationship_type: match rec.relationship_type.as_str() {
                    "friend" => RelationshipType::Friend,
                    "follower" => RelationshipType::Follower,
                    "blocked" => RelationshipType::Blocked,
                    "pending" => RelationshipType::Pending,
                    _ => RelationshipType::Pending, // Default fallback
                },
                created_at: rec.created_at,
                updated_at: rec.updated_at,
                is_active: rec.is_active,
            })
            .collect();

        Ok(relationships)
    }
    
    async fn get_friends(&self, user_id: Uuid) -> Result<Vec<Relationship>, Box<dyn std::error::Error>> {
        let recs = sqlx::query!(
            r#"
            SELECT id, source_user_id, target_user_id, relationship_type as "relationship_type: String", created_at, updated_at, is_active
            FROM social_relationships
            WHERE source_user_id = $1 AND relationship_type = 'friend' AND is_active = true
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let relationships = recs
            .into_iter()
            .map(|rec| Relationship {
                id: rec.id,
                source_user_id: rec.source_user_id,
                target_user_id: rec.target_user_id,
                relationship_type: RelationshipType::Friend,
                created_at: rec.created_at,
                updated_at: rec.updated_at,
                is_active: rec.is_active,
            })
            .collect();

        Ok(relationships)
    }
    
    async fn get_followers(&self, user_id: Uuid) -> Result<Vec<Relationship>, Box<dyn std::error::Error>> {
        let recs = sqlx::query!(
            r#"
            SELECT id, source_user_id, target_user_id, relationship_type as "relationship_type: String", created_at, updated_at, is_active
            FROM social_relationships
            WHERE target_user_id = $1 AND relationship_type = 'follower' AND is_active = true
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let relationships = recs
            .into_iter()
            .map(|rec| Relationship {
                id: rec.id,
                source_user_id: rec.source_user_id,
                target_user_id: rec.target_user_id,
                relationship_type: RelationshipType::Follower,
                created_at: rec.created_at,
                updated_at: rec.updated_at,
                is_active: rec.is_active,
            })
            .collect();

        Ok(relationships)
    }
    
    async fn get_following(&self, user_id: Uuid) -> Result<Vec<Relationship>, Box<dyn std::error::Error>> {
        let recs = sqlx::query!(
            r#"
            SELECT id, source_user_id, target_user_id, relationship_type as "relationship_type: String", created_at, updated_at, is_active
            FROM social_relationships
            WHERE source_user_id = $1 AND relationship_type = 'follower' AND is_active = true
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let relationships = recs
            .into_iter()
            .map(|rec| Relationship {
                id: rec.id,
                source_user_id: rec.source_user_id,
                target_user_id: rec.target_user_id,
                relationship_type: RelationshipType::Follower,
                created_at: rec.created_at,
                updated_at: rec.updated_at,
                is_active: rec.is_active,
            })
            .collect();

        Ok(relationships)
    }
    
    async fn update_relationship(&self, relationship: Relationship) -> Result<Relationship, Box<dyn std::error::Error>> {
        let rec = sqlx::query!(
            r#"
            UPDATE social_relationships
            SET source_user_id = $2, target_user_id = $3, relationship_type = $4, updated_at = $5, is_active = $6
            WHERE id = $1
            RETURNING id, source_user_id, target_user_id, relationship_type as "relationship_type: String", created_at, updated_at, is_active
            "#,
            relationship.id,
            relationship.source_user_id,
            relationship.target_user_id,
            match relationship.relationship_type {
                RelationshipType::Friend => "friend",
                RelationshipType::Follower => "follower",
                RelationshipType::Blocked => "blocked",
                RelationshipType::Pending => "pending",
            },
            relationship.updated_at,
            relationship.is_active
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Relationship {
            id: rec.id,
            source_user_id: rec.source_user_id,
            target_user_id: rec.target_user_id,
            relationship_type: match rec.relationship_type.as_str() {
                "friend" => RelationshipType::Friend,
                "follower" => RelationshipType::Follower,
                "blocked" => RelationshipType::Blocked,
                "pending" => RelationshipType::Pending,
                _ => RelationshipType::Pending, // Default fallback
            },
            created_at: rec.created_at,
            updated_at: rec.updated_at,
            is_active: rec.is_active,
        })
    }
    
    async fn delete_relationship(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            "DELETE FROM social_relationships WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
    
    async fn get_mutual_relationships(&self, user_id: Uuid) -> Result<Vec<Relationship>, Box<dyn std::error::Error>> {
        let recs = sqlx::query!(
            r#"
            SELECT r1.id, r1.source_user_id, r1.target_user_id, r1.relationship_type as "relationship_type: String", r1.created_at, r1.updated_at, r1.is_active
            FROM social_relationships r1
            JOIN social_relationships r2 ON r1.target_user_id = r2.source_user_id AND r1.source_user_id = r2.target_user_id
            WHERE r1.source_user_id = $1 
            AND r1.relationship_type = 'friend' 
            AND r2.relationship_type = 'friend'
            AND r1.is_active = true 
            AND r2.is_active = true
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let relationships = recs
            .into_iter()
            .map(|rec| Relationship {
                id: rec.id,
                source_user_id: rec.source_user_id,
                target_user_id: rec.target_user_id,
                relationship_type: RelationshipType::Friend,
                created_at: rec.created_at,
                updated_at: rec.updated_at,
                is_active: rec.is_active,
            })
            .collect();

        Ok(relationships)
    }
}