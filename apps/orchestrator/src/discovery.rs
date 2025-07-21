use tonic::{Request, Response, Status};
use crate::cpc_orchestrator::*;
use sqlx::PgPool;
use std::sync::Arc;
use opensearch::{OpenSearch, IndexParts};
use opensearch::http::request::JsonBody;
use serde_json::{json, Value};
use tracing::info;

pub struct DiscoveryService {
    db_pool: Arc<PgPool>,
    search_client: Arc<OpenSearch>,
}

impl DiscoveryService {
    pub fn new(db_pool: Arc<PgPool>, search_client: Arc<OpenSearch>) -> Self {
        Self { db_pool, search_client }
    }
}

#[tonic::async_trait]
impl DiscoveryService for DiscoveryService {
    async fn publish_content(
        &self,
        request: Request<ContentMetadata>,
    ) -> Result<Response<PublishResponse>, Status> {
        let metadata = request.into_inner();
        info!("Publishing content: {}", metadata.id);
        
        // TODO: Implement content publishing
        // 1. Insert into database
        // 2. Index in OpenSearch
        // 3. Track content availability
        
        // Example OpenSearch indexing
        let document: JsonBody<_> = json!({
            "title": metadata.title,
            "description": metadata.description,
            "tags": metadata.tags,
            "owner_id": metadata.owner_id
        }).into();
        
        match self.search_client
            .index(IndexParts::IndexId("content", &metadata.id))
            .body(document)
            .send()
            .await
        {
            Ok(_) => info!("Content indexed successfully"),
            Err(e) => tracing::error!("Failed to index content: {:?}", e),
        }
        
        Ok(Response::new(PublishResponse {
            success: true,
            content_id: metadata.id,
        }))
    }

    async fn search_content(
        &self,
        request: Request<SearchQuery>,
    ) -> Result<Response<SearchResults>, Status> {
        let query = request.into_inner();
        info!("Searching content: {}", query.query);
        
        // TODO: Implement search
        // 1. Query OpenSearch
        // 2. Return results
        
        // Example search query
        let search_query = json!({
            "query": {
                "multi_match": {
                    "query": query.query,
                    "fields": ["title", "description", "tags"]
                }
            },
            "size": query.limit
        });
        
        let mut results = Vec::new();
        
        match self.search_client
            .search()
            .index(&["content"])
            .body(search_query)
            .send()
            .await
        {
            Ok(response) => {
                let response_body: Value = response.json().await.map_err(|e| {
                    Status::internal(format!("Failed to parse search response: {:?}", e))
                })?;
                
                if let Some(hits) = response_body["hits"]["hits"].as_array() {
                    for hit in hits {
                        if let Some(source) = hit.get("_source") {
                            // TODO: Map to ContentMetadata
                            results.push(ContentMetadata {
                                id: hit["_id"].as_str().unwrap_or_default().to_string(),
                                owner_id: source["owner_id"].as_str().unwrap_or_default().to_string(),
                                title: source["title"].as_str().unwrap_or_default().to_string(),
                                description: source["description"].as_str().unwrap_or_default().to_string(),
                                tags: source["tags"]
                                    .as_array()
                                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                                    .unwrap_or_default(),
                            });
                        }
                    }
                }
            }
            Err(e) => tracing::error!("Search failed: {:?}", e),
        }
        
        Ok(Response::new(SearchResults { results }))
    }
}