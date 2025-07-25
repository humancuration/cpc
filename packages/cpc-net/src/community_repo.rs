use anyhow::Result;
use lru::LruCache;
use p2panda::prelude::*;
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
    P2PandaClient(#[from] p2panda::ClientError),

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
    node_client: Arc<NodeClient>,
    cache: Cache,
}

impl CommunityRepo {
    pub fn new(node_client: Arc<NodeClient>) -> Self {
        Self {
            node_client,
            cache: Arc::new(RwLock::new(LruCache::new(
                NonZeroUsize::new(256).unwrap(),
            ))),
        }
    }

    pub async fn create(
        &self,
        name: &str,
        description: &str,
        key_pair: &KeyPair,
    ) -> Result<Community, CommunityRepoError> {
        let mut fields = Fields::new();
        fields.insert("name".to_string(), (name, None).into());
        fields.insert("description".to_string(), (description, None).into());

        let (doc_id, _op_id) = self
            .node_client
            .create_document(key_pair, COMMUNITY_SCHEMA_ID, fields)
            .await?;

        let community = Community {
            id: doc_id,
            name: name.to_string(),
            description: description.to_string(),
        };

        Ok(community)
    }

    pub async fn list(
        &self,
        first: i32,
        after: Option<String>,
    ) -> Result<CommunityConnection, CommunityRepoError> {
        let query = self
            .node_client
            .query_documents(COMMUNITY_SCHEMA_ID)
            .limit(first as usize);

        let query = if let Some(cursor) = after {
            query.start_cursor(&cursor)
        } else {
            query
        };

        let results = query.all().await?;
        let edges = results
            .documents
            .into_iter()
            .map(|doc| {
                let community: Community =
                    serde_json::from_value(doc.fields.into()).unwrap_or_else(|_| Community {
                        id: doc.id.clone(),
                        name: "Invalid Community Data".into(),
                        description: "".into(),
                    });
                CommunityEdge {
                    cursor: doc.id,
                    node: community,
                }
            })
            .collect();

        Ok(CommunityConnection {
            edges,
            page_info: PageInfo {
                end_cursor: results.end_cursor,
                has_next_page: results.has_next_page,
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