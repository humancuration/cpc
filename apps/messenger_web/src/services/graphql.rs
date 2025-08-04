//! GraphQL service for the Messenger web application

use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use std::future::Future;
use std::pin::Pin;

/// Service for handling GraphQL API calls
pub struct GraphQLService {
    base_url: String,
}

impl GraphQLService {
    /// Create a new GraphQL service
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
    
    /// Execute a GraphQL query
    pub async fn query<T: for<'de> Deserialize<'de>>(
        &self, 
        query: &str
    ) -> Result<T, String> {
        let request_body = serde_json::json!({
            "query": query
        });
        
        let response = Request::post(&format!("{}/graphql", self.base_url))
            .header("Content-Type", "application/json")
            .body(request_body.to_string())
            .send()
            .await
            .map_err(|e| format!("Network error: {:?}", e))?;
            
        let response_text = response.text().await
            .map_err(|e| format!("Failed to read response: {:?}", e))?;
            
        // In a real implementation, we would parse the GraphQL response properly
        // For now, we'll just return an error
        Err(format!("Not implemented: {}", response_text))
    }
    
    /// Execute a GraphQL mutation
    pub async fn mutate<T: for<'de> Deserialize<'de>>(
        &self, 
        mutation: &str
    ) -> Result<T, String> {
        self.query(mutation).await
    }
}

impl Default for GraphQLService {
    fn default() -> Self {
        Self::new("http://localhost:3000".to_string())
    }
}