# Chat System

This document describes the real-time chat system implemented in the Live Streaming module.

## Overview

The chat system provides real-time communication between streamers and viewers. It builds upon the CPC messenger system with Twitch-like features such as emotes, badges, and chat commands.

## Architecture

The chat system consists of:

1. **Message Storage**: Persistent storage of chat messages
2. **Real-time Delivery**: Instant message broadcasting
3. **Message Processing**: Emote parsing, moderation, etc.
4. **Integration**: Connection with CPC's shared messenger package

## Implementation

### Chat Service

Location: `src/chat/chat_service.rs`

The chat service extends the shared messenger system with live streaming features:

- Emote support
- Badge system
- Chat commands
- Moderation tools

Key components:
- `ChatService`: Main chat service implementation
- `TwitchChatMessage`: Extended message format with emotes/badges
- `Emote`: Custom emote representation
- `Badge`: User badge representation

Key methods:
- `create_stream_chat()`: Create chat room for a stream
- `send_chat_message()`: Send message with emotes/badges
- `get_recent_messages()`: Retrieve recent chat history
- `add_emote()`: Add custom emote to system
- `get_emote_by_name()`: Find emote by name

## Integration with CPC Messenger

The chat system builds upon `cpc-messenger`:

- Reuses core message and conversation models
- Extends with live streaming-specific features
- Maintains compatibility with other CPC messaging

### Conversation Model

- Each stream has a dedicated group conversation
- Participants are stream viewers and the broadcaster
- Settings control chat permissions (followers-only, subscribers-only, etc.)

### Message Model

- Base message from CPC messenger
- Extended with emotes, badges, and moderation info
- Support for system messages (user joined, etc.)

## Features

### Emotes

Custom emotes enhance chat expression:

- Channel-specific emotes
- Subscriber-only emotes
- Global emotes
- Animated emotes (GIF/APNG)

Emote system:
- Emote parsing in messages
- Emote storage and retrieval
- Emote permissions based on subscription tier

### Badges

Badges identify user status and roles:

- Broadcaster badge
- Moderator badges
- Subscriber badges (tier-based)
- VIP badges
- Custom achievement badges

Badge system:
- Badge assignment based on user status
- Visual display in chat UI
- Custom badge creation for channels

### Chat Commands

Special commands provide interactive features:

- `/me` - Action messages
- `/ban` - Moderator ban command
- `/timeout` - Moderator timeout command
- `/subscribers` - Enable subscribers-only chat
- `/followers` - Enable followers-only chat
- `/slow` - Enable slow mode
- `/clear` - Clear chat history

Command system:
- Command parsing and validation
- Permission checking
- Integration with moderation tools

### Moderation

Tools to maintain chat quality:

- Automated moderation (spam, profanity filters)
- Manual moderation (ban, timeout, delete)
- Moderator hierarchy (broadcaster, mods, VIPs)
- Moderation logging

Moderation features:
- Real-time message filtering
- User action history
- Appeal process for penalties

## Real-time Communication

### WebSocket Integration

- WebSocket connections for instant message delivery
- Connection management and reconnection
- Message broadcasting to all connected clients

### Message Broadcasting

- Efficient distribution to all viewers
- Scalability for large audiences
- Message ordering and consistency

## API

### Sending Messages

```rust
// Send chat message with emotes and badges
let message = chat_service.send_chat_message(
    conversation_id,
    sender_id,
    content,
    emotes,
    badges,
    is_moderator,
    is_subscriber
).await?;
```

### Managing Emotes

```rust
// Add custom emote
chat_service.add_emote(emote).await?;

// Get emote by name
let emote = chat_service.get_emote_by_name("Kappa").await?;
```

### Chat Room Management

```rust
// Create stream chat
let conversation = chat_service.create_stream_chat(stream_id, channel_owner_id).await?;

// Get recent messages
let messages = chat_service.get_recent_messages(conversation_id, 100).await?;
```

## Data Models

### TwitchChatMessage

```rust
pub struct TwitchChatMessage {
    pub base_message: Message,
    pub emotes: Vec<Emote>,
    pub badges: Vec<Badge>,
    pub is_moderator: bool,
    pub is_subscriber: bool,
}
```

### Emote

```rust
pub struct Emote {
    pub id: Uuid,
    pub name: String,
    pub positions: (usize, usize),
}
```

### Badge

```rust
pub struct Badge {
    pub id: Uuid,
    pub name: String,
    pub version: Option<String>,
}
```

## Performance Considerations

### Message Throughput

- Efficient message processing pipeline
- Memory optimization for high-volume chats
- Rate limiting to prevent abuse

### Scalability

- Horizontal scaling for large streams
- Message batching for efficiency
- Load balancing across servers

### Storage

- Efficient database queries for message history
- Caching of recent messages
- Archival of older messages

## Security

### Message Filtering

- Automated spam detection
- Profanity filtering
- Link moderation
- Unicode character filtering

### User Authentication

- Integration with CPC authentication system
- Permission validation for commands
- Role-based access control

### Privacy

- Private message support
- User blocking capabilities
- Data retention policies

## Testing

Chat system can be tested using:

- Unit tests for message processing
- Integration tests with database
- Load testing for high-volume scenarios
- Security testing for moderation features

## UI Integration

### Message Display

- Real-time message rendering
- Emote rendering in messages
- Badge display next to usernames
- Message styling based on user roles

### User Interaction

- Message input with emote suggestions
- Command auto-completion
- Moderation tools for authorized users
- Chat settings (font size, timestamps, etc.)

## Future Enhancements

Planned improvements:

- Rich media messages (images, videos)
- Chat bots with custom commands
- Advanced moderation AI
- Multi-language support
- Chat replay for VODs
- Integration with CPC's feedback analysis system
- Enhanced accessibility features