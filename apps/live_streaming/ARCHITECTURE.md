# Live Streaming Module Architecture

## Overview

The Live Streaming module is a Twitch-like live streaming platform built for the CPC ecosystem. It provides real-time video streaming, chat, channel management, and social features.

## Architectural Principles

This module follows several key architectural principles:

1. **Hexagonal Architecture**: Clear separation of concerns with domain logic isolated from infrastructure
2. **Screaming Architecture**: Structure clearly expresses the system's intent
3. **Vertical Slices**: Features are organized by business capability rather than technical layers
4. **Rust Syntax**: Leveraging Rust's type system and memory safety for robust implementation

## Module Structure

```
src/
├── streaming/            # WebRTC implementation
├── chat/                 # Real-time chat
├── channel/              # Channel management
├── social/               # Social features
├── media_processing/     # Video processing
└── web/                  # Web integration
```

### Streaming Module

Handles WebRTC-based live streaming functionality:

- **Broadcaster**: Manages stream creation and broadcasting
- **Viewer**: Manages stream consumption and playback

Key technologies:
- WebRTC for peer-to-peer streaming
- AV1 video codec for efficient compression
- Opus audio codec for high-quality audio

### Chat Module

Provides real-time chat functionality integrated with the shared messenger system:

- **ChatService**: Extends the shared messenger with Twitch-specific features
- Emote support with custom channel emotes
- Badge system for moderators, subscribers, etc.
- Chat commands and moderation tools

### Channel Module

Manages streaming channels and their metadata:

- **Channel**: Channel entity with settings and statistics
- **ChannelManager**: Service for channel operations and lifecycle management

### Social Module

Handles social features for the platform:

- **Follow**: Channel following system integrated with shared social features
- **Subscription**: Tiered subscription system with benefits

### Media Processing Module

Handles video transcoding and processing:

- **Transcoder**: Converts streams to WebM/AV1 format
- **Utils**: Helper functions for media processing and stream management

### Web Module

Provides web integration for HTTP APIs and GraphQL:

- **Routes**: RESTful HTTP endpoints
- **GraphQL**: GraphQL schema and resolvers
- **Module**: Integration with the CPC modular system

## Data Flow

1. **Stream Creation**: Broadcaster creates a stream with a unique stream key
2. **Streaming**: Broadcaster sends video/audio to WebRTC peers
3. **Transcoding**: Media is transcoded to WebM/AV1 for web playback
4. **Distribution**: Stream is distributed to viewers via WebRTC and HTTP adaptive streaming
5. **Chat**: Real-time chat messages are distributed via WebSocket
6. **Social**: Follows and subscriptions are managed through the social integration system

## Integration Points

### Database

- **PostgreSQL**: Primary data store via SQLx
- **Sled**: Edge intelligence and caching

### Shared Packages

- **messenger**: Core chat functionality
- **social_integration**: Follows and social features
- **notification_core**: User notifications
- **media**: Shared media processing utilities

### External Services

- **WebRTC**: Peer-to-peer streaming
- **Tauri**: Desktop application framework

## Security Considerations

- Stream keys are securely generated and validated
- Chat messages are sanitized to prevent XSS
- Access controls for channel settings and moderation
- Secure WebRTC signaling

## Performance Considerations

- Adaptive bitrate streaming for varying network conditions
- Efficient AV1 encoding for bandwidth optimization
- Caching of channel metadata and emotes
- WebSocket connections for real-time chat

## Scalability

- Horizontal scaling of stream distribution
- Load balancing of transcoding workers
- Database sharding for user data
- CDN integration for static assets

## Future Enhancements

- P2P streaming with p2panda network
- AI-powered content moderation
- Enhanced analytics and viewer insights

## Module Integration Details

### Messenger Integration Approach

The Live Streaming module integrates with the shared messenger system to provide real-time chat functionality for streams. The integration approach includes:

1. **Stream Chat Rooms**: Each stream has an associated chat room implemented as a group conversation in the messenger system
2. **Twitch-Style Messages**: Extension of the base Message model with emotes, badges, and user roles
3. **Real-time Broadcasting**: Messages are broadcast to all connected viewers through WebSocket connections
4. **Moderation Tools**: Integration with messenger's permission system for chat moderation

### Social Integration Events

The module emits several social events to the shared social integration system:

1. **StreamStarted**: When a broadcaster starts a new stream
2. **StreamEnded**: When a stream ends
3. **ViewerJoined**: When a viewer joins a stream
4. **ChatMessageSent**: When a chat message is sent
5. **SubscriptionCreated**: When a user subscribes to a channel
6. **FollowCreated**: When a user follows a channel

These events are used for:
- Unified social feeds across CPC apps
- Analytics and insights
- Notification triggering

### Notification Flows

Notifications are sent through the shared notification core system:

1. **Stream Started Notifications**: Sent to followers when a followed channel goes live
2. **Chat Mentions**: Notifications when a user is mentioned in chat
3. **Subscription Events**: Notifications for subscription milestones
4. **Moderation Actions**: Notifications for moderation actions affecting users

Notifications respect user preferences and can be delivered through multiple channels (push, email, in-app).
- Mobile app integration