# Live Streaming Module

A Twitch-like live streaming platform built for the CPC ecosystem.

## Features

- **Live Video Streaming**: WebRTC-based streaming with AV1/Opus codecs
- **Real-time Chat**: Integrated chat system with emotes and badges
- **Channel Management**: Create and customize streaming channels
- **Social Features**: Follow channels and subscribe to content creators
- **Media Processing**: Transcoding to WebM/AV1 format with adaptive bitrate streaming

## Architecture

The live streaming module follows a hexagonal architecture pattern with clear separation of concerns:

```
src/
├── streaming/            # WebRTC implementation
│   ├── broadcaster.rs    # Stream broadcaster
│   └── viewer.rs         # Stream viewer
├── chat/                 # Real-time chat
│   └── chat_service.rs   # Integrates with shared messenger
├── channel/              # Channel management
│   ├── channel.rs        # Channel model
│   └── manager.rs        # Channel operations
├── social/               # Social features
│   ├── follow.rs         # Follow system
│   └── subscription.rs   # Channel subscriptions
├── media_processing/     # Video processing
│   ├── transcoder.rs     # AV1 transcoding
│   └── utils.rs          # Media helpers
└── web/                  # Web integration
    ├── routes.rs         # HTTP routes
    ├── module.rs         # Module initialization
    ├── graphql.rs        # GraphQL schema
    └── modular_module.rs # Modular system integration
```

## Integration Points

- **Authentication**: Uses CPC's auth system
- **Notifications**: Integrates with `shared_packages/notification_core`
- **Data Storage**: PostgreSQL via SQLx
- **Messaging**: Reuses components from `shared_packages/messenger`
- **Social Features**: Integrates with `shared_packages/social_integration`

## Development

### Prerequisites

