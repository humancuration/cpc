//! Web routes for the BI & Analytics module

use axum::{
    routing::{get, post},
    Router,
    extract::{State, WebSocketUpgrade, Query},
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use axum::extract::ws::{WebSocket, Message};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use crate::presentation::web::graphql::BiAnalyticsGraphQLSchema;
use tracing::{info, error, warn};
/// Module-specific state
#[derive(Clone)]
pub struct BiAnalyticsState {
    pub graphql_schema: BiAnalyticsGraphQLSchema,
    pub subscription_manager: Arc<SubscriptionManager>,
}
    pub subscription_manager: Arc<SubscriptionManager>,
}

/// Subscription manager for WebSocket connections
#[derive(Default)]
pub struct SubscriptionManager {
    subscriptions: RwLock<HashMap<Uuid, Vec<mpsc::UnboundedSender<WebSocketMessage>>>>,
}

/// WebSocket message types
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketMessage {
    Connected { report_id: Uuid, message: String },
    Update { report_id: Uuid, delta: serde_json::Value, accessibility_announcement: String },
    Error { report_id: Uuid, message: String },
    Ping,
    Pong,
}

impl SubscriptionManager {
    /// Add a new subscription
    pub async fn subscribe(&self, report_id: Uuid, sender: mpsc::UnboundedSender<WebSocketMessage>) {
        let mut subs = self.subscriptions.write().await;
        subs.entry(report_id).or_insert_with(Vec::new).push(sender);
    }

    /// Remove a subscription
    pub async fn unsubscribe(&self, report_id: Uuid, sender: &mpsc::UnboundedSender<WebSocketMessage>) {
        let mut subs = self.subscriptions.write().await;
        if let Some(subscribers) = subs.get_mut(&report_id) {
            subscribers.retain(|s| !std::ptr::eq(s, sender));
            if subscribers.is_empty() {
                subs.remove(&report_id);
            }
        }
    }

    /// Broadcast an update to all subscribers of a report
    pub async fn broadcast(&self, report_id: Uuid, message: WebSocketMessage) {
        let subs = self.subscriptions.read().await;
        if let Some(subscribers) = subs.get(&report_id) {
            for sender in subscribers {
                if let Err(e) = sender.send(message.clone()) {
                    error!("Failed to send message to subscriber: {}", e);
                }
            }
        }
    }
}

/// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    info!("Health check requested");
    (StatusCode::OK, "BI Analytics service is healthy")
}

/// GraphQL endpoint
pub async fn graphql_handler(
    State(state): State<BiAnalyticsState>,
    req: async_graphql_axum::GraphQLRequest,
) -> impl IntoResponse {
    info!("GraphQL request received");
    let response = state.graphql_schema.execute(req.into_inner()).await;
    let json = serde_json::to_string(&response).unwrap();
    axum::response::Html(json)
}

/// Query parameters for WebSocket connection
#[derive(Deserialize)]
pub struct VisualizationQuery {
    report_id: Uuid,
    token: String,
}

/// WebSocket handler for visualization updates
pub async fn ws_visualization_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<VisualizationQuery>,
    State(state): State<BiAnalyticsState>,
) -> impl IntoResponse {
    info!("WebSocket connection requested for report: {}", query.report_id);
    
    // In a real implementation, we would validate the token here
    // For now, we'll just accept the connection
    
    ws.on_upgrade(move |socket| handle_ws_connection(socket, query.report_id, state))
}

/// Handle WebSocket connection
async fn handle_ws_connection(
    mut socket: WebSocket,
    report_id: Uuid,
    state: BiAnalyticsState,
) {
    info!("WebSocket connected for report: {}", report_id);
    
    // Create channel for sending messages to this client
    let (tx, mut rx) = mpsc::unbounded_channel();
    
    // Subscribe to updates for this report
    state.subscription_manager.subscribe(report_id, tx.clone()).await;
    
    // Send initial connection message
    let welcome_msg = WebSocketMessage::Connected {
        report_id,
        message: "Connected to visualization stream".to_string(),
    };
    
    if let Err(e) = socket.send(Message::Text(serde_json::to_string(&welcome_msg).unwrap())).await {
        error!("Failed to send welcome message: {}", e);
        return;
    }
    
    // Spawn a task to handle incoming messages from the subscription manager
    let mut socket_sender = socket.clone();
    let subscription_manager = state.subscription_manager.clone();
    let report_id_clone = report_id;
    
    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if let Err(e) = socket_sender.send(Message::Text(serde_json::to_string(&message).unwrap())).await {
                error!("Failed to send message to client: {}", e);
                break;
            }
        }
        
        // Clean up subscription when the task ends
        subscription_manager.unsubscribe(report_id_clone, &tx).await;
    });
    
    // Handle incoming messages from the client
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                info!("Received message from client: {}", text);
                
                // Handle client messages (e.g., requesting data refresh)
                match serde_json::from_str::<WebSocketMessage>(&text) {
                    Ok(WebSocketMessage::Ping) => {
                        let pong = WebSocketMessage::Pong;
                        if let Err(e) = tx.send(pong) {
                            error!("Failed to send pong: {}", e);
                            break;
                        }
                    }
                    Ok(_) => {
                        // Handle other message types
                    }
                    Err(e) => {
                        error!("Failed to parse client message: {}", e);
                        let error_msg = WebSocketMessage::Error {
                            report_id,
                            message: format!("Invalid message format: {}", e),
                        };
                        if let Err(e) = tx.send(error_msg) {
                            error!("Failed to send error message: {}", e);
                            break;
                        }
                    }
                }
            }
            Message::Binary(_) => {
                warn!("Binary messages not supported");
            }
            Message::Ping(_) => {
                // Respond to ping
                if let Err(e) = socket.send(Message::Pong(vec![])).await {
                    error!("Failed to send pong: {}", e);
                    break;
                }
            }
            Message::Pong(_) => {
                // Pong received, connection is alive
            }
            Message::Close(_) => {
                info!("WebSocket closed by client");
                break;
            }
        }
    }
    
    // Clean up subscription
    state.subscription_manager.unsubscribe(report_id, &tx).await;
    info!("WebSocket disconnected for report: {}", report_id);
}

/// Create the BI Analytics web router
pub fn create_router(state: BiAnalyticsState) -> Router {
    info!("Creating BI Analytics web router");
    
    Router::new()
        .route("/health", get(health_check))
        .route("/graphql", post(graphql_handler))
        .route("/ws/visualization", get(ws_visualization_handler))
        .with_state(state)
}

/// Response structure for API endpoints
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

/// Error response structure
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
}

/// Dataset response structure
#[derive(Serialize)]
pub struct DatasetResponse {
    pub id: Uuid,
    pub name: String,
    pub source: String,
    pub description: Option<String>,
}

/// Report response structure
#[derive(Serialize)]
pub struct ReportResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

/// Dashboard response structure
#[derive(Serialize)]
pub struct DashboardResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, Method},
    };
    use tower::ServiceExt; // for `call`
    
    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }
    
    // Note: Testing the GraphQL endpoint would require a more complex setup
    // including creating mock services and a full GraphQL schema
}