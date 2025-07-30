//! WebSocket streaming for real-time visualization updates
//!
//! This module implements WebSocket connections for real-time visualization updates
//! as specified in the visualization architecture documentation.

use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message, Error as WsError},
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use url::Url;
use uuid::Uuid;

/// WebSocket connection for visualization streaming
pub struct VisualizationWebSocket {
    /// WebSocket connection
    ws_stream: Arc<Mutex<Option<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>>>,
    /// Connection URL
    url: String,
    /// Visualization ID
    visualization_id: Uuid,
}

impl VisualizationWebSocket {
    /// Create a new WebSocket connection
    pub async fn new(url: String, visualization_id: Uuid) -> Result<Self, WsError> {
        let ws_url = format!("{}/visualizations/{}/ws", url, visualization_id);
        let (ws_stream, _) = connect_async(Url::parse(&ws_url)?).await?;
        
        Ok(Self {
            ws_stream: Arc::new(Mutex::new(Some(ws_stream))),
            url,
            visualization_id,
        })
    }
    
    /// Send a message through the WebSocket
    pub async fn send_message(&mut self, message: &str) -> Result<(), WsError> {
        if let Some(ref mut ws_stream) = *self.ws_stream.lock().await {
            ws_stream.send(Message::Text(message.to_string())).await?;
            Ok(())
        } else {
            Err(WsError::AlreadyClosed)
        }
    }
    
    /// Receive a message from the WebSocket
    pub async fn receive_message(&mut self) -> Result<Option<String>, WsError> {
        if let Some(ref mut ws_stream) = *self.ws_stream.lock().await {
            if let Some(msg) = ws_stream.next().await {
                match msg {
                    Ok(Message::Text(text)) => Ok(Some(text)),
                    Ok(Message::Binary(data)) => Ok(Some(base64::encode(data))),
                    Ok(Message::Close(_)) => {
                        *self.ws_stream.lock().await = None;
                        Ok(None)
                    }
                    _ => Ok(None),
                }
            } else {
                Ok(None)
            }
        } else {
            Err(WsError::AlreadyClosed)
        }
    }
    
    /// Close the WebSocket connection
    pub async fn close(&mut self) -> Result<(), WsError> {
        if let Some(ref mut ws_stream) = *self.ws_stream.lock().await {
            ws_stream.close(None).await?;
            *self.ws_stream.lock().await = None;
        }
        Ok(())
    }
}

/// Real-time update message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMessage {
    /// Update type
    pub r#type: UpdateType,
    /// Update payload
    pub payload: serde_json::Value,
    /// Timestamp
    pub timestamp: u64,
}

/// Update type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateType {
    /// Data update
    Data,
    /// Configuration update
    Config,
    /// Error notification
    Error,
}

/// Connection upgrade request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionUpgrade {
    /// Protocol to upgrade to
    pub protocol: String,
    /// Supported versions
    pub versions: Vec<String>,
    /// Additional headers
    pub headers: std::collections::HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_websocket_struct_creation() {
        // This test would require a running WebSocket server
        // In a real scenario, we would mock the connection
        let uuid = Uuid::new_v4();
        // We can't actually connect in tests without a server
        // assert!(VisualizationWebSocket::new("http://localhost:3001".to_string(), uuid).await.is_ok());
    }
    
    #[test]
    fn test_update_message_structs() {
        let message = UpdateMessage {
            r#type: UpdateType::Data,
            payload: serde_json::json!({"test": "data"}),
            timestamp: 1234567890,
        };
        
        match message.r#type {
            UpdateType::Data => assert!(true),
            _ => panic!("Expected Data variant"),
        }
    }
}