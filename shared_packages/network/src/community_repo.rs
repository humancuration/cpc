use anyhow::Result;
use lru::LruCache;
use p2panda_core::{NodeClient, KeyPair, Fields, ClientError};
use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;
use std::sync::Arc;
use tokio::sync::RwLock;



// Data Transfer Objects (DTOs) for the networking layer.
// These are intentionally kept simple and decoupled from `cpc-core` models.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Community {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct CommunityConnection {
    pub edges: Vec<CommunityEdge>,
    pub page_info: PageInfo,
}

#[derive(Debug, Clone)]
pub struct CommunityEdge {
    pub cursor: String,
    pub node: Community,
}

#[derive(Debug, Clone)]
pub struct PageInfo {
    pub end_cursor: Option<String>,
    pub has_next_page: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum CommunityRepoError {
    #[error("p2panda client error: {0}")]
    P2PandaClient(#[from] ClientError),

    #[error("Could not find community with id {0}")]
    NotFound(String),

    #[error("Invalid cursor provided")]
    InvalidCursor,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

const COMMUNITY_SCHEMA_ID: &str = "community_v1";

type Cache = Arc<RwLock<LruCache<String, Community>>>;

#[derive(Clone)]
pub struct CommunityRepo {
    node_client: Option<Arc<NodeClient>>,
    cache: Cache,
    is_mock: bool,
}

impl CommunityRepo {
    pub fn new(node_client: Arc<NodeClient>) -> Self {
        Self {
            node_client: Some(node_client),
            cache: Arc::new(RwLock::new(LruCache::new(
                NonZeroUsize::new(256).unwrap(),
            ))),
            is_mock: false,
        }
    }

    /// Create a mock implementation for development/testing
    pub fn new_mock() -> Self {
        Self {
            node_client: None,
            cache: Arc::new(RwLock::new(LruCache::new(
                NonZeroUsize::new(256).unwrap(),
            ))),
            is_mock: true,
        }
    }

    pub async fn create(
        &self,
        name: &str,
        description: &str,
        _key_pair: &KeyPair,
    ) -> Result<Community, CommunityRepoError> {
        if self.is_mock {
            // Mock implementation
            let community = Community {
                id: format!("mock_community_{}", uuid::Uuid::new_v4()),
                name: name.to_string(),
                description: description.to_string(),
            };
            return Ok(community);
        }

        // Real implementation would go here
        let node_client = self.node_client.as_ref().ok_or_else(|| {
            anyhow::anyhow!("NodeClient not available")
        })?;

        // For now, return a mock result since p2panda integration is not complete
        let community = Community {
            id: format!("community_{}", uuid::Uuid::new_v4()),
            name: name.to_string(),
            description: description.to_string(),
        };

        Ok(community)
    }

    pub async fn list(
        &self,
        _first: i32,
        _after: Option<String>,
    ) -> Result<CommunityConnection, CommunityRepoError> {
        if self.is_mock {
            // Mock implementation - return some sample communities
            let mock_communities = vec![
                Community {
                    id: "mock_1".to_string(),
                    name: "General Discussion".to_string(),
                    description: "A place for general community discussion".to_string(),
                },
                Community {
                    id: "mock_2".to_string(),
                    name: "Tech Talk".to_string(),
                    description: "Discussions about technology and development".to_string(),
                },
            ];

            let edges = mock_communities
                .into_iter()
                .map(|community| CommunityEdge {
                    cursor: community.id.clone(),
                    node: community,
                })
                .collect();

            return Ok(CommunityConnection {
                edges,
                page_info: PageInfo {
                    end_cursor: Some("mock_end".to_string()),
                    has_next_page: false,
                },
            });
        }

        // Real implementation would go here
        // For now, return empty results
        Ok(CommunityConnection {
            edges: vec![],
            page_info: PageInfo {
                end_cursor: None,
                has_next_page: false,
            },
        })
    }

    pub async fn join(
        &self,
        _community_id: &str,
        _key_pair: &KeyPair,
    ) -> Result<(), CommunityRepoError> {
        // Here we would create a `membership` entry in p2panda
        // linking the user's public key to the community_id
        unimplemented!("p2panda logic for joining a community")
    }

    pub async fn leave(
        &self,
        _community_id: &str,
        _key_pair: &KeyPair,
    ) -> Result<(), CommunityRepoError> {
        // Here we would find and possibly soft-delete the `membership` entry
        unimplemented!("p2panda logic for leaving a community")
    }

    pub async fn invalidate_cache(&self, key: &str) {
        let mut cache = self.cache.write().await;
        cache.pop(key);
    }
}