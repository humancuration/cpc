# CPC Messenger Application

## Overview

The CPC Messenger application provides real-time communication capabilities with Discord/Slack-like features. This implementation follows hexagonal architecture principles with a strict separation between domain logic, application use cases, and infrastructure concerns.

## New Features Implemented

### 1. Media Sharing Pipeline

The media sharing pipeline enables users to share various types of media with end-to-end encryption:

- **AV1/Opus transcoding** via ffmpeg.wasm for royalty-free codecs
- **Thumbnail generation** for images and videos
- **E2EE using AES-GCM** with per-media keys
- Integration with the shared `media` package for processing

### 2. Message Reactions System

Users can react to messages with emojis, similar to Discord/Slack:

- Add/remove reactions to messages
- View all reactions on a message
- Integration with the shared `social_interactions` package

### 3. Threaded Conversations

Messages can be organized into threads for better conversation organization:

- Create threads from any message
- View messages within a thread
- Nested conversation structure

### 4. Advanced Group Management

Enhanced group management features for better community moderation:

- Enhanced permissions model with granular controls
- Admin transfer protocol for smooth leadership transitions
- Participant banning and timeout features
- Moderation tools for content management

## Architecture

The application follows a hexagonal architecture with the following layers:

```
┌─────────────────┐
│   Infrastructure│ ◄── External interfaces (GraphQL, WebSocket, etc.)
├─────────────────┤
│    Application  │ ◄── Use case implementations
├─────────────────┤
│     Domain      │ ◄── Core business logic and entities
└─────────────────┘
```

### Domain Layer

Contains core business entities and logic:
- Conversation, Message, Participant models
- Reaction, Thread, and enhanced Permission models
- Service interfaces for all features

### Application Layer

Implements use cases and orchestrates domain services:
- ReactionServiceImpl, ThreadServiceImpl
- GroupServiceImpl, ModerationServiceImpl
- MediaServiceImpl with integration to shared media processing

### Infrastructure Layer

Technical implementations:
- GraphQL endpoints for all new features
- PostgreSQL repositories for data persistence
- WebSocket server for real-time communication

## Database Schema

New tables for enhanced features:

### reactions
```sql
CREATE TABLE reactions (
    id UUID PRIMARY KEY,
    message_id UUID REFERENCES messages(id),
    user_id UUID REFERENCES users(id),
    reaction_type VARCHAR(20) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### message_threads
```sql
CREATE TABLE message_threads (
    id UUID PRIMARY KEY,
    parent_message_id UUID REFERENCES messages(id) NOT NULL,
    root_message_id UUID REFERENCES messages(id),
    conversation_id UUID REFERENCES conversations(id) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### media
```sql
CREATE TABLE media (
    id UUID PRIMARY KEY,
    owner_id UUID REFERENCES users(id) NOT NULL,
    media_type VARCHAR(20) NOT NULL,
    storage_path TEXT NOT NULL,
    encryption_key BYTEA NOT NULL,
    iv BYTEA NOT NULL,
    thumbnail_id UUID REFERENCES media(id),
    original_filename VARCHAR(255),
    size_bytes BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

Enhanced permissions in the `participants` table:
```sql
ALTER TABLE participants ADD COLUMN is_admin BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE participants ADD COLUMN can_manage_participants BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE participants ADD COLUMN can_change_settings BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE participants ADD COLUMN can_delete_messages BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE participants ADD COLUMN can_moderate_content BOOLEAN NOT NULL DEFAULT FALSE;
```

## GraphQL API

New endpoints for enhanced features:

### Mutations

```graphql
# Group Management
updateGroupSettings(conversationId: ID!, settings: GroupSettingsInput!): Boolean!
transferAdmin(conversationId: ID!, currentAdminId: ID!, newAdminId: ID!): Boolean!
banParticipant(conversationId: ID!, adminId: ID!, userId: ID!): Boolean!

# Message Interactions
addReaction(messageId: ID!, reactionType: String!): Reaction!
removeReaction(messageId: ID!, reactionType: String!): Boolean!

# Media Processing
generateMediaThumbnail(mediaId: ID!, size: String!): String!
```

### Queries

```graphql
messageReactions(messageId: ID!): [Reaction!]!
thread(threadId: ID!): Thread!
```

## Security

Security is implemented at multiple layers:

1. **Transport Security**: TLS 1.3 for all external communications
2. **Authentication**: OAuth 2.0 with JWT tokens
3. **Authorization**: Role-based access control through participant permissions
4. **Data Protection**: Server-side encryption for stored messages and media
5. **Privacy**: Consent management integrated with the core consent framework

## Integration Points

### Social Interactions
Leverages the shared `social_interactions` package for reaction functionality.

### Media Processing
Uses the shared `media` package for media processing and transcoding.

### Consent Management
Integrates with the core consent framework for privacy controls.

## Testing

The implementation includes:

1. **Unit Tests**: For domain logic
2. **Integration Tests**: For service implementations
3. **Repository Tests**: For database implementations
4. **API Tests**: For GraphQL endpoints

## Deployment

The application can be built and run with:

### Desktop Application

```bash
cargo build --release
./target/release/messenger
```

### Web Application

The web frontend is located in the `apps/messenger_web` directory. To run it:

```bash
cd ../messenger_web
trunk serve
```

This will start a development server at http://localhost:3001

Environment variables:
- `DATABASE_URL`: PostgreSQL connection string
- `SLED_PATH`: Path to Sled database for presence caching