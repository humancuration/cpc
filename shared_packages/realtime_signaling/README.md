# Real-time Signaling Service

A WebSocket-based real-time signaling service for collaborative applications.

## Overview

This crate provides real-time communication capabilities for collaborative applications, including:

- WebSocket-based messaging
- Presence tracking
- Cursor positioning
- Text selection sharing
- Typing indicators

## Features

- **Real-time Messaging**: Low-latency WebSocket communication
- **Presence Management**: Track user presence in collaborative sessions
- **Cursor Sharing**: Share cursor positions between collaborators
- **Selection Sharing**: Share text selection ranges
- **Typing Indicators**: Show when users are typing
- **Broadcast System**: Efficient message broadcasting to all participants

## Usage

### Server Setup

```rust
use realtime_signaling::{SignalingServer, SignalingService};
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let signaling_service = Arc::new(SignalingService::new());
    let address: SocketAddr = "127.0.0.1:8080".parse()?;
    let server = SignalingServer::new(signaling_service, address);
    
    server.start().await
}
```

### Client Connection

```rust
use realtime_signaling::{SignalingClient, SignalingMessage};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = SignalingClient::new();
    
    // Connect to the server
    client.connect("ws://127.0.0.1:8080/ws").await?;
    
    // Set up message handler
    client.set_message_handler(|message| {
        println!("Received message: {:?}", message);
    }).await;
    
    // Start listening for messages
    client.start_listening().await?;
    
    // Send a message
    let message = SignalingMessage::JoinDocument {
        document_id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
    };
    client.send_message(&message).await?;
    
    Ok(())
}
```

## Message Types

The service supports several message types for different collaborative features:

- `JoinDocument` - User joins a document session
- `LeaveDocument` - User leaves a document session
- `PresenceUpdate` - User presence information update
- `CursorUpdate` - Cursor position update
- `SelectionUpdate` - Text selection update
- `TypingIndicator` - Typing status indicator
- `Error` - Error messages

## Integration

This service is designed to work with the CPC collaboration engine and can be integrated with document editors, chat applications, and other collaborative tools.