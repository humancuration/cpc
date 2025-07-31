//! PostgreSQL repository for unified posts

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::post::{UnifiedPost, AppSource, PostMetadata};
use crate::application::social_integration_service::UnifiedPostRepository;
use std::collections::HashMap;
use serde_json::Value;
use chrono::{DateTime, Utc};

/// PostgreSQL repository for unified posts
#[derive(Debug)]
pub struct PostgresUnifiedPostRepository {
    pool: PgPool,
}

impl PostgresUnifiedPostRepository {
    /// Create a new PostgreSQL unified post repository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UnifiedPostRepository for PostgresUnifiedPostRepository {
    async fn save(&self, post: &UnifiedPost) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Convert properties HashMap to JSON
        let properties_json = serde_json::to_value(&post.properties)?;
        let full_metadata_json = serde_json::to_value(&post.metadata)?;
        
        // Convert AppSource to string
        let source_str = match post.source {
            AppSource::Allat => "allat",
            AppSource::Yapper => "yapper",
        };
        
        sqlx::query!(
            r#"
            INSERT INTO unified_posts (
                id, author_id, source, original_id, content, properties, created_at, updated_at, upvotes, comments, shares
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
            )
            ON CONFLICT (id) DO UPDATE SET
                author_id = $2,
                source = $3,
                original_id = $4,
                content = $5,
                properties = $6,
                updated_at = $8,
                upvotes = $9,
                comments = $10,
                shares = $11
            "#,
            post.id,
            post.author_id,
            source_str,
            post.original_id,
            post.content,
            full_metadata_json,
            post.metadata.created_at,
            post.metadata.updated_at,
            post.metadata.engagement.upvotes as i32,
            post.metadata.engagement.comments as i32,
            post.metadata.engagement.shares as i32
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn find_by_id(&self, id: Uuid) -> Result<Option<UnifiedPost>, Box<dyn std::error::Error + Send + Sync>> {
        let row = sqlx::query!(
            r#"
            SELECT id, author_id, source, original_id, content, properties, created_at, updated_at, upvotes, comments, shares
            FROM unified_posts
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                // Convert source string back to AppSource
                let source = match row.source.as_str() {
                    "allat" => AppSource::Allat,
                    "yapper" => AppSource::Yapper,
                    _ => return Err("Invalid source".into()),
                };
                
                // Convert properties JSON back to HashMap
                let properties: HashMap<String, String> = match &row.properties {
                    Some(Value::Object(map)) => {
                        map.iter()
                            .filter_map(|(k, v)| {
                                if let Value::String(s) = v {
                                    Some((k.clone(), s.clone()))
                                } else {
                                    None
                                }
                            })
                            .collect()
                    }
                    _ => HashMap::new(),
                };
                
                // Convert full metadata JSON back to PostMetadata
                let metadata: PostMetadata = match &row.properties {
                    Some(value) => serde_json::from_value(value.clone())?,
                    None => PostMetadata {
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                        engagement: crate::domain::post::EngagementMetrics {
                            upvotes: row.upvotes as u64,
                            comments: row.comments as u64,
                            shares: row.shares as u64,
                            views: 0,
                        },
                        media_attachments: Vec::new(),
                        hashtags: Vec::new(),
                        privacy: crate::domain::post::PrivacySettings {
                            is_public: true,
                            allowed_viewers: Vec::new(),
                            shareable: true,
                        },
                    },
                };
                
                let post = UnifiedPost {
                    id: row.id,
                    source,
                    original_id: row.original_id.unwrap_or(Uuid::nil()),
                    author_id: row.author_id,
                    content: row.content.clone().unwrap_or_default(),
                    metadata,
                    properties,
                };
                
                Ok(Some(post))
            }
            None => Ok(None),
        }
    }
    
    async fn find_by_author(&self, author_id: Uuid) -> Result<Vec<UnifiedPost>, Box<dyn std::error::Error + Send + Sync>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, author_id, source, original_id, content, properties, created_at, updated_at, upvotes, comments, shares
            FROM unified_posts
            WHERE author_id = $1
            ORDER BY created_at DESC
            "#,
            author_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut posts = Vec::new();
        
        for row in rows {
            // Convert source string back to AppSource
            let source = match row.source.as_str() {
                "allat" => AppSource::Allat,
                "yapper" => AppSource::Yapper,
                _ => return Err("Invalid source".into()),
            };
            
            // Convert properties JSON back to HashMap
            let properties: HashMap<String, String> = match &row.properties {
                Some(Value::Object(map)) => {
                    map.iter()
                        .filter_map(|(k, v)| {
                            if let Value::String(s) = v {
                                Some((k.clone(), s.clone()))
                            } else {
                                None
                            }
                        })
                        .collect()
                }
                _ => HashMap::new(),
            };
            
            // Convert full metadata JSON back to PostMetadata
            let metadata: PostMetadata = match &row.properties {
                Some(value) => serde_json::from_value(value.clone())?,
                None => PostMetadata {
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    engagement: crate::domain::post::EngagementMetrics {
                        upvotes: row.upvotes as u64,
                        comments: row.comments as u64,
                        shares: row.shares as u64,
                        views: 0,
                    },
                    media_attachments: Vec::new(),
                    hashtags: Vec::new(),
                    privacy: crate::domain::post::PrivacySettings {
                        is_public: true,
                        allowed_viewers: Vec::new(),
                        shareable: true,
                    },
                },
            };
            
            let post = UnifiedPost {
                id: row.id,
                source,
                original_id: row.original_id.unwrap_or(Uuid::nil()),
                author_id: row.author_id,
                content: row.content.clone().unwrap_or_default(),
                metadata,
                properties,
            };
            
            posts.push(post);
        }
        
        Ok(posts)
    }
    
    async fn find_by_source(&self, source: AppSource) -> Result<Vec<UnifiedPost>, Box<dyn std::error::Error + Send + Sync>> {
        // Convert AppSource to string
        let source_str = match source {
            AppSource::Allat => "allat",
            AppSource::Yapper => "yapper",
        };
        
        let rows = sqlx::query!(
            r#"
            SELECT id, author_id, source, original_id, content, properties, created_at, updated_at, upvotes, comments, shares
            FROM unified_posts
            WHERE source = $1
            ORDER BY created_at DESC
            "#,
            source_str
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut posts = Vec::new();
        
        for row in rows {
            // Convert source string back to AppSource
            let source = match row.source.as_str() {
                "allat" => AppSource::Allat,
                "yapper" => AppSource::Yapper,
                _ => return Err("Invalid source".into()),
            };
            
            // Convert properties JSON back to HashMap
            let properties: HashMap<String, String> = match &row.properties {
                Some(Value::Object(map)) => {
                    map.iter()
                        .filter_map(|(k, v)| {
                            if let Value::String(s) = v {
                                Some((k.clone(), s.clone()))
                            } else {
                                None
                            }
                        })
                        .collect()
                }
                _ => HashMap::new(),
            };
            
            // Convert full metadata JSON back to PostMetadata
            let metadata: PostMetadata = match &row.properties {
                Some(value) => serde_json::from_value(value.clone())?,
                None => PostMetadata {
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    engagement: crate::domain::post::EngagementMetrics {
                        upvotes: row.upvotes as u64,
                        comments: row.comments as u64,
                        shares: row.shares as u64,
                        views: 0,
                    },
                    media_attachments: Vec::new(),
                    hashtags: Vec::new(),
                    privacy: crate::domain::post::PrivacySettings {
                        is_public: true,
                        allowed_viewers: Vec::new(),
                        shareable: true,
                    },
                },
            };
            
            let post = UnifiedPost {
                id: row.id,
                source,
                original_id: row.original_id.unwrap_or(Uuid::nil()),
                author_id: row.author_id,
                content: row.content.clone().unwrap_or_default(),
                metadata,
                properties,
            };
            
            posts.push(post);
        }
        
        Ok(posts)
    }
}