//! WebSocket client for real-time signaling

use crate::message::SignalingMessage;
use crate::signaling::SignalingError;
use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;
use uuid::Uuid;
use tracing::{info, error, warn};

/// Callback for handling incoming messages
pub type MessageHandler = Box<dyn Fn(SignalingMessage) + Send + Sync>;

/// WebSocket-based signaling client
pub struct SignalingClient {
    /// WebSocket connection
    ws_stream: Arc<Mutex<Option<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>>>,
    
    /// Message handler callback
    message_handler: Arc<RwLock<Option<MessageHandler>>>,
    
    /// Connection state
    connected: Arc<RwLock<bool>>,
}

impl SignalingClient {
    /// Create a new signaling client
    pub fn new() -> Self {
        Self {
            ws_stream: Arc::new(Mutex::new(None)),
            message_handler: Arc::new(RwLock::new(None)),
            connected: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Connect to the signaling server
    pub async fn connect(&self, server_url: &str) -> Result<(), SignalingError> {
        let url = Url::parse(server_url)
            .map_err(|e| SignalingError::InvalidMessage(format!("Invalid URL: {}", e)))?;
        
        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| SignalingError::BroadcastError(format!("Connection failed: {}", e)))?;
        
        *self.ws_stream.lock().await = Some(ws_stream);
        *self.connected.write().await = true;
        
        info!("Connected to signaling server at {}", server_url);
        Ok(())
    }
    
    /// Disconnect from the signaling server
    pub async fn disconnect(&self) -> Result<(), SignalingError> {
        let mut ws_stream = self.ws_stream.lock().await;
        if let Some(stream) = ws_stream.as_mut() {
            stream.close(None)
                .await
                .map_err(|e| SignalingError::BroadcastError(format!("Disconnect failed: {}", e)))?;
        }
        *ws_stream = None;
        *self.connected.write().await = false;
        
        info!("Disconnected from signaling server");
        Ok(())
    }
    
    /// Check if the client is connected
    pub async fn is_connected(&self) -> bool {
        *self.connected.read().await
    }
    
    /// Set the message handler callback
    pub async fn set_message_handler<F>(&self, handler: F) 
    where 
        F: Fn(SignalingMessage) + Send + Sync + 'static,
    {
        *self.message_handler.write().await = Some(Box::new(handler));
    }
    
    /// Send a message to the server
    pub async fn send_message(&self, message: &SignalingMessage) -> Result<(), SignalingError> {
        if !self.is_connected().await {
            return Err(SignalingError::BroadcastError("Not connected to server".to_string()));
        }
        
        let serialized = serde_json::to_string(message)
            .map_err(|e| SignalingError::SerializationError(e.to_string()))?;
        
        let mut ws_stream = self.ws_stream.lock().await;
        if let Some(stream) = ws_stream.as_mut() {
            stream.send(Message::Text(serialized))
                .await
                .map_err(|e| SignalingError::BroadcastError(format!("Send failed: {}", e)))
        } else {
            Err(SignalingError::BroadcastError("No active connection".to_string()))
        }
    }
    
    /// Start listening for messages from the server
    pub async fn start_listening(&self) -> Result<(), SignalingError> {
        let ws_stream = {
            let guard = self.ws_stream.lock().await;
            guard.as_ref().cloned()
        };
        
        if let Some(mut stream) = ws_stream {
            let message_handler = self.message_handler.clone();
            
            tokio::spawn(async move {
                while let Some(msg) = stream.next().await {
                    match msg {
                        Ok(Message::Text(text)) => {
                            match serde_json::from_str::<SignalingMessage>(&text) {
                                Ok(message) => {
                                    let handler = message_handler.read().await;
                                    if let Some(handler) = handler.as_ref() {
                                        handler(message);
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to deserialize message: {}", e);
                                }
                            }
                        }
                        Ok(Message::Close(_)) => {
                            info!("Connection closed by server");
                            break;
                        }
                        Ok(_) => {
                            // Ignore other message types
                        }
                        Err(e) => {
                            error!("WebSocket error: {}", e);
                            break;
                        }
                    }
                }
            });
            
            Ok(())
        } else {
            Err(SignalingError::BroadcastError("No active connection".to_string()))
        }
    }
}

impl Default for SignalingClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    
    #[tokio::test]
    async fn test_client_creation() {
        let client = SignalingClient::new();
        assert!(!client.is_connected().await);
    }
}