# CPC Messenger Application

The Messenger application is a real-time communication platform that enables secure 1:1 and group conversations. It follows hexagonal architecture principles with a strict separation between domain logic, application use cases, and infrastructure concerns.

## Features

- Real-time messaging with delivery status tracking (Sent → Delivered → Read)
- Support for both 1:1 and group conversations
- Media sharing with server-side encryption
- Standard security implementation (TLS, server-side encryption)
- OAuth 2.0 integration for social platform authentication
- Privacy and consent framework integration
- GraphQL API for web clients
- WebSocket support for real-time communication

## Architecture

The application is structured following the hexagonal architecture pattern:

```
packages/
├── domains/messenger/          # Domain layer (core business logic)
├── apps/messenger/             # Application layer (use cases)
└── infrastructure/messenger/   # Infrastructure layer (database, APIs, etc.)
```

### Domain Layer

Contains the core business entities and logic:
- `Conversation` - Represents a conversation between users
- `Message` - Represents a message in a conversation
- `Participant` - Represents a user in a conversation
- Domain services interfaces

### Application Layer

Implements the use cases and orchestrates the business logic:
- `ConversationService` - Manages conversations
- `MessageService` - Handles message operations
- `MediaService` - Manages media uploads and storage
- `PresenceService` - Tracks user presence
- Integration with the consent management system

### Infrastructure Layer

Provides concrete implementations of the application interfaces:
- PostgreSQL database repositories
- GraphQL API endpoints
- WebSocket server for real-time communication
- OAuth2 identity provider integration
- Media storage with server-side encryption

## API

The application exposes two main APIs:

### GraphQL API

Available at `/graphql`, provides a comprehensive API for:
- Creating and managing conversations
- Sending and receiving messages
- Managing user presence
- Media operations

### WebSocket API

Available at `/ws`, provides real-time communication for:
- Instant message delivery
- Presence updates
- Delivery status notifications

## Security

The Messenger application implements standard security practices:

1. **Transport Security**: TLS 1.3 for all external communications
2. **Authentication**: OAuth 2.0 with JWT tokens (24-hour expiration)
3. **Data Protection**: Server-side encryption at rest for message content
4. **Privacy**: Integration with the CPC consent management framework

Note: This implementation uses standard server-side encryption rather than end-to-end encryption to facilitate social features and regulatory compliance.

## Installation

To run the Messenger application:

1. Set up the required environment variables:
   ```bash
   DATABASE_URL=postgresql://localhost/messenger
   SLED_PATH=./data/sled
   TIKTOK_CLIENT_ID=your_tiktok_client_id
   TIKTOK_CLIENT_SECRET=your_tiktok_client_secret
   FACEBOOK_CLIENT_ID=your_facebook_client_id
   FACEBOOK_CLIENT_SECRET=your_facebook_client_secret
   ```

2. Run the database migrations:
   ```bash
   sqlx migrate run
   ```

3. Start the application:
   ```bash
   cargo run -p cpc-messenger
   ```

## Testing

The application includes unit tests for domain logic and integration tests for service implementations:

```bash
cargo test -p cpc-messenger
```

## Future Enhancements

Planned features for future development:
- Group conversation management
- Message reactions and threading
- Cross-instance federation support
- Mobile-specific optimizations
- Performance enhancements for high-concurrency scenarios