//! WebSocket server implementation for real-time messaging

use async_tungstenite::tokio::accept_async;
use async_tungstenite::tungstenite::protocol::Message as WsMessage;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tracing::{error, info};
use uuid::Uuid;

/// WebSocket server for handling real-time messaging
pub struct WebSocketServer {
    /// Broadcast channel for sending messages to all connected clients
    event_sender: broadcast::Sender<WebSocketEvent>,
    /// Active connections
    connections: Arc<RwLock<HashMap<Uuid, broadcast::Sender<WebSocketEvent>>>>,
}

/// Events that can be sent over WebSocket
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketEvent {
    /// A reaction was added to a message
    ReactionAdded {
        message_id: Uuid,
        reaction: crate::models::Reaction,
    },
    /// A reaction was removed from a message
    ReactionRemoved {
        message_id: Uuid,
        reaction_id: Uuid,
    },
    /// A message was updated
    MessageUpdated {
        message: crate::models::Message,
    },
    /// A message was deleted
    MessageDeleted {
        message_id: Uuid,
    },
    /// Connection acknowledged
    Connected {
        user_id: Uuid,
    },
}

impl WebSocketServer {
    /// Create a new WebSocket server
    pub fn new() -> Self {
        let (event_sender, _) = broadcast::channel(100);
        let connections = Arc::new(RwLock::new(HashMap::new()));
        
        Self {
            event_sender,
            connections,
        }
    }
    
    /// Handle a new WebSocket connection
    pub async fn handle_connection(
        &self,
        user_id: Uuid,
        socket: async_tungstenite::tokio::WebSocketStream<async_tungstenite::tokio::TokioAdapter<tokio::net::TcpStream>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("New WebSocket connection for user: {}", user_id);
        
        let (mut ws_sender, mut ws_receiver) = socket.split();
        
        // Create a subscription for this client
        let mut event_receiver = self.event_sender.subscribe();
        let (client_sender, mut client_receiver) = broadcast::channel::<WebSocketEvent>(100);
        
        // Store the connection
        {
            let mut connections = self.connections.write().await;
            connections.insert(user_id, client_sender.clone());
        }
        
        // Send connection acknowledgment
        let ack_event = WebSocketEvent::Connected { user_id };
        if let Ok(json) = serde_json::to_string(&ack_event) {
            if ws_sender.send(WsMessage::Text(json)).await.is_err() {
                error!("Failed to send connection acknowledgment to user: {}", user_id);
            }
        }
        
        // Spawn task to handle incoming messages
        let connections_clone = self.connections.clone();
        let event_sender_clone = self.event_sender.clone();
        let user_id_clone = user_id;
        
        tokio::spawn(async move {
            while let Some(msg) = ws_receiver.next().await {
                match msg {
                    Ok(WsMessage::Text(text)) => {
                        // Handle incoming text messages
                        if let Ok(event) = serde_json::from_str::<WebSocketEvent>(&text) {
                            // Process the event
                            match event {
                                WebSocketEvent::ReactionAdded { message_id, reaction } => {
                                    // Broadcast to all clients
                                    let _ = event_sender_clone.send(WebSocketEvent::ReactionAdded {
                                        message_id,
                                        reaction,
                                    });
                                }
                                WebSocketEvent::ReactionRemoved { message_id, reaction_id } => {
                                    // Broadcast to all clients
                                    let _ = event_sender_clone.send(WebSocketEvent::ReactionRemoved {
                                        message_id,
                                        reaction_id,
                                    });
                                }
                                WebSocketEvent::MessageUpdated { message } => {
                                    // Broadcast to all clients
                                    let _ = event_sender_clone.send(WebSocketEvent::MessageUpdated {
                                        message,
                                    });
                                }
                                WebSocketEvent::MessageDeleted { message_id } => {
                                    // Broadcast to all clients
                                    let _ = event_sender_clone.send(WebSocketEvent::MessageDeleted {
                                        message_id,
                                    });
                                }
                                WebSocketEvent::Connected { .. } => {
                                    // Ignore, this is sent by server
                                }
                            }
                        }
                    }
                    Ok(WsMessage::Close(_)) => {
                        info!("WebSocket connection closed for user: {}", user_id_clone);
                        break;
                    }
                    Ok(_) => {
                        // Ignore other message types
                    }
                    Err(e) => {
                        error!("WebSocket error for user {}: {}", user_id_clone, e);
                        break;
                    }
                }
            }
            
            // Remove connection when done
            let mut connections = connections_clone.write().await;
            connections.remove(&user_id_clone);
        });
        
        // Spawn task to handle outgoing messages
        tokio::spawn(async move {
            while let Ok(event) = event_receiver.recv().await {
                // Send event to client
                if let Ok(json) = serde_json::to_string(&event) {
                    if ws_sender.send(WsMessage::Text(json)).await.is_err() {
                        error!("Failed to send message to user: {}", user_id);
                        break;
                    }
                }
            }
        });
        
        Ok(())
    }
    
    /// Broadcast a reaction event to all connected clients
    pub async fn broadcast_reaction_event(&self, message_id: Uuid, reaction: crate::models::Reaction) {
        let event = WebSocketEvent::ReactionAdded {
            message_id,
            reaction,
        };
        
        let _ = self.event_sender.send(event);
    }
    
    /// Broadcast a message update event to all connected clients
    pub async fn broadcast_message_update(&self, message: crate::models::Message) {
        let event = WebSocketEvent::MessageUpdated {
            message,
        };
        
        let _ = self.event_sender.send(event);
    }
    
    /// Broadcast a message deletion event to all connected clients
    pub async fn broadcast_message_deletion(&self, message_id: Uuid) {
        let event = WebSocketEvent::MessageDeleted {
            message_id,
        };
        
        let _ = self.event_sender.send(event);
    }
}

impl Default for WebSocketServer {
    fn default() -> Self {
        Self::new()
    }
}