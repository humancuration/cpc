//! WebSocket server for real-time messaging in the Messenger application

use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use tokio::sync::{mpsc, RwLock};
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{trace, debug, error};

use messenger_domain::{
    models::{Message as DomainMessage, MessageStatusUpdate, DeliveryStatus},
    errors::MessengerError,
};
use messenger_app::services::MessageService;

/// WebSocket server for real-time messaging
pub struct WebSocketServer {
    /// Message service for handling message operations
    message_service: Arc<dyn MessageService>,
    
    /// Active connections
    connections: Arc<RwLock<HashMap<Uuid, mpsc::UnboundedSender<Message>>>>,
    
    /// Channel for internal message distribution
    message_tx: mpsc::UnboundedSender<InternalMessage>,
}

/// Internal message types for the WebSocket server
#[derive(Debug, Clone)]
enum InternalMessage {
    /// A new message was sent
    NewMessage(DomainMessage),
    
    /// A message status was updated
    StatusUpdate(MessageStatusUpdate),
    
    /// A user connected
    UserConnected(Uuid),
    
    /// A user disconnected
    UserDisconnected(Uuid),
}

impl WebSocketServer {
    /// Create a new WebSocket server
    pub fn new(message_service: Arc<dyn MessageService>) -> Self {
        let (message_tx, message_rx) = mpsc::unbounded_channel();
        let connections = Arc::new(RwLock::new(HashMap::new()));
        
        let server = Self {
            message_service,
            connections: connections.clone(),
            message_tx: message_tx.clone(),
        };
        
        // Start the message distribution task
        tokio::spawn(Self::message_distribution_task(
            message_rx,
            connections,
        ));
        
        server
    }
    
    /// Handle a new WebSocket connection
    pub async fn handle_connection(
        &self,
        user_id: Uuid,
        ws_stream: tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    ) -> Result<(), MessengerError> {
        trace!("Handling new WebSocket connection for user {}", user_id);
        
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        
        // Create a channel for sending messages to this connection
        let (tx, mut rx) = mpsc::unbounded_channel();
        
        // Add the connection to our connections map
        {
            let mut connections = self.connections.write().await;
            connections.insert(user_id, tx);
        }
        
        // Notify that a user connected
        if let Err(e) = self.message_tx.send(InternalMessage::UserConnected(user_id)) {
            error!("Failed to send UserConnected message: {}", e);
        }
        
        // Task to send messages to the client
        let connections = self.connections.clone();
        let send_task = tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Err(e) = ws_sender.send(message).await {
                    error!("Failed to send message to user {}: {}", user_id, e);
                    break;
                }
            }
        });
        
        // Task to receive messages from the client
        let message_tx = self.message_tx.clone();
        let message_service = self.message_service.clone();
        let recv_task = tokio::spawn(async move {
            while let Some(result) = ws_receiver.next().await {
                match result {
                    Ok(message) => {
                        if let Err(e) = Self::handle_client_message(
                            &message,
                            user_id,
                            &message_service,
                            &message_tx,
                        ).await {
                            error!("Failed to handle client message from user {}: {}", user_id, e);
                        }
                    }
                    Err(e) => {
                        error!("WebSocket error for user {}: {}", user_id, e);
                        break;
                    }
                }
            }
        });
        
        // Wait for either task to complete
        tokio::select! {
            _ = send_task => {
                debug!("Send task completed for user {}", user_id);
            }
            _ = recv_task => {
                debug!("Receive task completed for user {}", user_id);
            }
        }
        
        // Remove the connection from our connections map
        {
            let mut connections = self.connections.write().await;
            connections.remove(&user_id);
        }
        
        // Notify that a user disconnected
        if let Err(e) = self.message_tx.send(InternalMessage::UserDisconnected(user_id)) {
            error!("Failed to send UserDisconnected message: {}", e);
        }
        
        Ok(())
    }
    
    /// Handle a message from a client
    async fn handle_client_message(
        message: &Message,
        user_id: Uuid,
        message_service: &Arc<dyn MessageService>,
        message_tx: &mpsc::UnboundedSender<InternalMessage>,
    ) -> Result<(), MessengerError> {
        match message {
            Message::Text(text) => {
                // Parse the message as JSON
                let client_message: ClientMessage = serde_json::from_str(text)
                    .map_err(|e| MessengerError::InvalidInput {
                        message: format!("Failed to parse client message: {}", e),
                    })?;
                
                match client_message.message_type.as_str() {
                    "send_message" => {
                        // Handle sending a new message
                        if let Some(payload) = &client_message.payload {
                            let send_payload: SendMessagePayload = serde_json::from_value(payload.clone())
                                .map_err(|e| MessengerError::InvalidInput {
                                    message: format!("Failed to parse send message payload: {}", e),
                                })?;
                            
                            let content = messenger_domain::models::MessageContent::Text(send_payload.content);
                            let domain_message = message_service
                                .send_message(send_payload.conversation_id, user_id, content)
                                .await?;
                            
                            // Notify about the new message
                            if let Err(e) = message_tx.send(InternalMessage::NewMessage(domain_message)) {
                                error!("Failed to send NewMessage notification: {}", e);
                            }
                        }
                    }
                    "mark_read" => {
                        // Handle marking messages as read
                        if let Some(payload) = &client_message.payload {
                            let mark_read_payload: MarkReadPayload = serde_json::from_value(payload.clone())
                                .map_err(|e| MessengerError::InvalidInput {
                                    message: format!("Failed to parse mark read payload: {}", e),
                                })?;
                            
                            let count = message_service
                                .mark_messages_read(
                                    mark_read_payload.conversation_id,
                                    user_id,
                                    mark_read_payload.up_to_message_id,
                                )
                                .await?;
                            
                            debug!("Marked {} messages as read for user {} in conversation {}", 
                                   count, user_id, mark_read_payload.conversation_id);
                        }
                    }
                    _ => {
                        return Err(MessengerError::InvalidInput {
                            message: format!("Unknown message type: {}", client_message.message_type),
                        });
                    }
                }
            }
            Message::Close(_) => {
                // Client closed the connection
                debug!("Client closed connection for user {}", user_id);
            }
            _ => {
                // Ignore other message types
                debug!("Ignoring non-text message from user {}", user_id);
            }
        }
        
        Ok(())
    }
    
    /// Task to distribute messages to connected clients
    async fn message_distribution_task(
        mut message_rx: mpsc::UnboundedReceiver<InternalMessage>,
        connections: Arc<RwLock<HashMap<Uuid, mpsc::UnboundedSender<Message>>>>,
    ) {
        while let Some(internal_message) = message_rx.recv().await {
            match internal_message {
                InternalMessage::NewMessage(domain_message) => {
                    // Send the message to all participants in the conversation
                    // In a real implementation, we would look up the conversation and get all participants
                    // For now, we'll just send it to the sender as an example
                    let sender_id = domain_message.sender_id;
                    
                    if let Some(sender_tx) = {
                        let connections = connections.read().await;
                        connections.get(&sender_id).cloned()
                    } {
                        let message_json = serde_json::to_string(&domain_message)
                            .unwrap_or_else(|_| "{}".to_string());
                        
                        if let Err(e) = sender_tx.send(Message::Text(message_json)) {
                            error!("Failed to send message to user {}: {}", sender_id, e);
                        }
                    }
                }
                InternalMessage::StatusUpdate(status_update) => {
                    // Handle status updates
                    // In a real implementation, we would send this to relevant users
                    debug!("Status update for message {}: {:?}", status_update.message_id, status_update.new_status);
                }
                InternalMessage::UserConnected(user_id) => {
                    debug!("User connected: {}", user_id);
                }
                InternalMessage::UserDisconnected(user_id) => {
                    debug!("User disconnected: {}", user_id);
                }
            }
        }
    }
}

/// Message sent by clients
#[derive(serde::Deserialize)]
struct ClientMessage {
    #[serde(rename = "type")]
    message_type: String,
    payload: Option<serde_json::Value>,
}

/// Payload for sending a message
#[derive(serde::Deserialize)]
struct SendMessagePayload {
    conversation_id: Uuid,
    content: String,
}

/// Payload for marking messages as read
#[derive(serde::Deserialize)]
struct MarkReadPayload {
    conversation_id: Uuid,
    up_to_message_id: Uuid,
}