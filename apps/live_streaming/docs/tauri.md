# Tauri Integration

This document describes how the Live Streaming module integrates with Tauri for desktop application deployment.

## Overview

Tauri is used to create a desktop application for the Live Streaming platform. The same UI components used in the web version are reused in the desktop application, providing a consistent experience across platforms.

## Architecture

The Tauri application consists of:

1. **Rust Backend**: Handles business logic, database operations, and system integration
2. **Web Frontend**: UI components built with Yew and Stylist
3. **Tauri Core**: Bridges the Rust backend with the web frontend

## Entry Points

### Main Binary

Location: `src/main_tauri.rs`

This is the main entry point for the Tauri desktop application. It:

- Initializes the database connection
- Sets up the live streaming module
- Configures Tauri application settings
- Handles application lifecycle events

### Web Frontend

Location: `src/web/main.rs`

This is the entry point for the web version of the application. It:

- Initializes the Yew application
- Renders the main App component

## Configuration

### Tauri Configuration

Location: `tauri.conf.json`

Key configuration options:

- Window dimensions and properties
- Application identifier and version
- Bundle settings for different platforms
- Security settings (CSP)

### Build Configuration

Location: `build.rs`

This build script uses `tauri_build` to compile the application.

## Features

### Desktop-Specific Features

- Native window management
- System tray integration
- File system access
- Native notifications

### Shared Features

- Live streaming
- Real-time chat
- Channel management
- Social features

## Development

To run the Tauri application in development mode:

```bash
cd apps/live_streaming
cargo run --bin live-streaming-tauri
```

To build the Tauri application for production:

```bash
cd apps/live_streaming
cargo build --bin live-streaming-tauri --release
```

## Bundling

Tauri can bundle the application for different platforms:

- Windows: `.exe`, `.msi`
- macOS: `.app`, `.dmg`
- Linux: `.deb`, `.appimage`

To create bundles:

```bash
cd apps/live_streaming
cargo tauri build
```

## Communication Between Frontend and Backend

### Commands

Rust functions can be exposed to the frontend using the `#[tauri::command]` attribute.

Example:
```rust
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

### Events

The backend can emit events that the frontend can listen to using the Tauri event system.

Example:
```rust
// Backend
app.emit("stream-started", payload)?;

// Frontend
import { listen } from '@tauri-apps/api/event';

const unlisten = await listen('stream-started', (event) => {
    console.log('Stream started:', event.payload);
});
```

## Security

Tauri provides several security features:

- Content Security Policy (CSP)
- IPC (Inter-Process Communication) security
- Asset isolation
- Permission system

## Testing

Tauri applications can be tested using:

- Unit tests for Rust code
- Integration tests for the Tauri API
- End-to-end tests for the UI

## Deployment

The Tauri application can be deployed as:

- Standalone desktop application
- Part of a larger CPC application suite
- Distributed through app stores or direct download