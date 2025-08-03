# Live Streaming Module Documentation

This directory contains comprehensive documentation for the Live Streaming module.

## Table of Contents

1. [Architecture](architecture.md) - High-level architectural overview
2. [Domain Models](domain.md) - Core domain entities and relationships
3. [Usage Guide](usage.md) - How to use the module
4. [UI Components](ui.md) - User interface implementation
5. [Tauri Integration](tauri.md) - Desktop application integration
6. [WebRTC Integration](webrtc.md) - Streaming implementation details
7. [Media Processing](media_processing.md) - Transcoding pipeline
8. [Social Features](social.md) - Following and subscription systems
9. [Chat System](chat.md) - Real-time messaging implementation
10. [Channel Management](channel.md) - Channel creation and customization

## Overview

The Live Streaming module is a comprehensive platform for live video streaming with social features. It provides:

- Real-time video streaming with WebRTC
- Interactive chat with emotes and badges
- Channel management and customization
- Social features including following and subscriptions
- Media processing and transcoding
- Cross-platform user interface (web and desktop)

## Getting Started

For development setup and basic usage, see the main [README.md](../README.md) file.

## Architecture

The module follows a hexagonal architecture with clear separation of concerns. Key components include:

- **Streaming Layer**: WebRTC implementation for real-time video
- **Domain Layer**: Core business logic and entities
- **Application Layer**: Service orchestration and use cases
- **Infrastructure Layer**: Database, messaging, and external integrations
- **Presentation Layer**: Web and desktop user interfaces

## Integration Points

The module integrates with several CPC shared packages:

- `cpc-messenger` for chat functionality
- `cpc-social-integration` for following and social features
- `cpc-notification-core` for user notifications
- `cpc-media` for shared media processing utilities

## Development

Each documentation file focuses on a specific aspect of the system:

- **Domain models** explain the core entities and their relationships
- **Architecture** provides a high-level view of the system structure
- **Usage** guides show how to implement features
- **Component-specific** docs dive deep into individual systems

## Contributing

To contribute to the documentation:

1. Follow the existing structure and formatting
2. Update the table of contents when adding new files
3. Cross-reference related documents where appropriate
4. Use clear, concise language with technical accuracy