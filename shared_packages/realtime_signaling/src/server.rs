//! WebSocket server for real-time signaling

use crate::message::SignalingMessage;
use crate::signaling::SignalingService;
use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_tungstenite::tungstenite::protocol::Message;
use tracing::{info, error, warn};
use warp::Filter;

/// WebSocket-based signaling server
pub struct SignalingServer {
    /// Core signaling service
    signaling_service: Arc<SignalingService>,
    
    /// Server address
    address: SocketAddr,
}

impl SignalingServer {
    /// Create a new signaling server
    pub fn new(signaling_service: Arc<SignalingService>, address: SocketAddr) -> Self {
        Self {
            signaling_service,
            address,
        }
    }
    
    /// Start the WebSocket server
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Starting signaling server on {}", self.address);
        
        // Create a WebSocket route
        let signaling_service = self.signaling_service.clone();
        let ws_route = warp::path("ws")
            .and(warp::ws())
            .map(move |ws: warp::ws::Ws| {
                let signaling_service = signaling_service.clone();
                ws.on_upgrade(move |websocket| handle_connection(websocket, signaling_service))
            });
        
        // Start the server
        warp::serve(ws_route)
            .run(self.address)
            .await;
        
        Ok(())
    }
}

/// Handle WebSocket connection
async fn handle_connection(
    ws: warp::ws::WebSocket,
    signaling_service: Arc<SignalingService>,
) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    
    // Create a broadcast receiver for this connection
    // In a real implementation, we would associate this with a specific document/user
    // For now, we'll just handle messages as they come in
    
    while let Some(result) = ws_rx.next().await {
        match result {
            Ok(msg) => {
                if let Message::Text(text) = msg {
                    match serde_json::from_str::<SignalingMessage>(&text) {
                        Ok(message) => {
                            // Handle the message
                            // In a real implementation, we would route this to the appropriate document
                            // For now, we'll just echo it back
                            match serde_json::to_string(&message) {
                                Ok(response) => {
                                    if let Err(e) = ws_tx.send(Message::Text(response)).await {
                                        error!("Failed to send response: {}", e);
                                        break;
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to serialize response: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to deserialize message: {}", e);
                            let error_msg = SignalingMessage::Error {
                                code: "PARSE_ERROR".to_string(),
                                message: format!("Failed to parse message: {}", e),
                            };
                            if let Ok(error_text) = serde_json::to_string(&error_msg) {
                                if let Err(e) = ws_tx.send(Message::Text(error_text)).await {
                                    error!("Failed to send error response: {}", e);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
        }
    }
    
    info!("WebSocket connection closed");
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    
    #[test]
    fn test_server_creation() {
        let signaling_service = Arc::new(SignalingService::new());
        let address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let server = SignalingServer::new(signaling_service, address);
        assert_eq!(server.address, address);
    }
}