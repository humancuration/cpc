# Live Streaming Module

A Twitch-like live streaming platform built for the CPC ecosystem.

## Features

- **Live Video Streaming**: WebRTC-based streaming with AV1/Opus codecs
- **Real-time Chat**: Integrated chat system with emotes and badges
- **Channel Management**: Create and customize streaming channels
- **Social Features**: Follow channels and subscribe to content creators
- **Media Processing**: Transcoding to WebM/AV1 format with adaptive bitrate streaming
- **Cross-platform UI**: Web and desktop applications using Yew and Tauri
- **Modular Architecture**: Pluggable design for easy extension

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
├── ui/                   # User interface components
│   ├── app.rs            # Main application component
│   ├── stream_player.rs  # Stream player component
│   ├── chat.rs           # Chat component
│   ├── channel_list.rs   # Channel list component
│   └── navigation.rs     # Navigation component
└── web/                  # Web integration
    ├── routes.rs         # HTTP routes
    ├── module.rs         # Module initialization
    ├── graphql.rs        # GraphQL schema
    └── modular_module.rs # Modular system integration
```

## Key Components

### Streaming
- WebRTC-based peer-to-peer streaming
- AV1 video codec for efficient compression
- Opus audio codec for high-quality audio
- Adaptive bitrate streaming support

### Chat
- Real-time messaging with WebSocket
- Emote and badge system
- Chat commands and moderation tools
- Integration with CPC messenger

### Channel Management
- Channel creation and customization
- Branding with profile/banner images
- Analytics and statistics
- Custom emote management

### Social Features
- Follow/unfollow channels
- Tiered subscription system
- Notification integration
- Community building tools

### Media Processing
- Real-time transcoding to WebM/AV1
- Adaptive bitrate ladder generation
- Segment-based streaming
- Hardware acceleration support

### User Interface
- Responsive web interface with Yew
- Native desktop application with Tauri
- CSS-in-Rust styling with Stylist
- Consistent experience across platforms

## Integration Points

- **Authentication**: Uses CPC's auth system
- **Notifications**: Integrates with `shared_packages/notification_core`
- **Data Storage**: PostgreSQL via SQLx
- **Messaging**: Reuses components from `shared_packages/messenger`
- **Social Features**: Integrates with `shared_packages/social_integration`
- **Media Processing**: Uses `shared_packages/media`

## Development

### Prerequisites

- Rust 2021 edition
- PostgreSQL 17.5
- Node.js (for frontend development)
- wasm-pack (for WebAssembly compilation)

### Building

```bash
# Build the application
cargo build

# Run the application
cargo run

# Run the Tauri desktop application
cargo run --bin live-streaming-tauri

# Build for web
cd web
npm install
npm run build
```

### Testing

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --features integration

# Run UI tests
cargo test --features ui
```

### Examples

```bash
# Run basic usage example
cargo run --example basic_usage

# Run UI demo
cargo run --example ui_demo
```

## Documentation

Detailed documentation is available in the `docs/` directory:

- [Architecture](docs/architecture.md) - Detailed architectural overview
- [Domain Models](docs/domain.md) - Core domain entities and relationships
- [Usage Guide](docs/usage.md) - How to use the module
- [UI Components](docs/ui.md) - User interface implementation
- [Tauri Integration](docs/tauri.md) - Desktop application integration
- [WebRTC Integration](docs/webrtc.md) - Streaming implementation details
- [Media Processing](docs/media_processing.md) - Transcoding pipeline
- [Social Features](docs/social.md) - Following and subscription systems
- [Chat System](docs/chat.md) - Real-time messaging implementation
- [Channel Management](docs/channel.md) - Channel creation and customization

## License

This module is licensed under the CPC license, promoting sharing within the federation.