# UI Components

This document describes the UI components for the Live Streaming platform.

## Overview

The UI is built using Yew, a modern Rust framework for creating multi-threaded frontends for the web. Stylist is used for styling components with CSS-in-Rust.

## Components

### App Component

The main application component that ties everything together.

Location: `src/ui/app.rs`

Features:
- Navigation bar
- Stream player
- Chat interface
- Channel list
- Responsive design

### Stream Player Component

Displays the live video stream with stream information.

Location: `src/ui/stream_player.rs`

Props:
- `title`: Stream title
- `streamer_name`: Name of the streamer
- `viewer_count`: Current number of viewers
- `stream_url`: URL to the video stream

### Chat Component

Real-time chat interface for viewers to communicate.

Location: `src/ui/chat.rs`

Props:
- `messages`: List of chat messages
- `on_send`: Callback for sending new messages

### Channel List Component

Displays a list of live channels for browsing.

Location: `src/ui/channel_list.rs`

Props:
- `channels`: List of channels
- `on_select`: Callback for when a channel is selected

### Navigation Component

Navigation bar with links to different sections of the application.

Location: `src/ui/navigation.rs`

Props:
- `active_route`: Currently active route
- `on_navigate`: Callback for navigation

## Styling

All components use Stylist for CSS-in-Rust styling. Styles are defined within each component using the `style!` macro.

## Responsive Design

The UI is designed to be responsive and works on both desktop and mobile devices. Media queries are used to adjust the layout for smaller screens.

## Integration with Backend

The UI components communicate with the backend through:
- GraphQL queries and mutations
- RESTful HTTP endpoints
- WebSocket connections for real-time features

## Development

To run the web frontend in development mode:

```bash
cd apps/live_streaming/web
npm run dev
```

To build the web frontend for production:

```bash
cd apps/live_streaming/web
npm run build
```

## Tauri Integration

The same UI components are used in the Tauri desktop application, providing a consistent experience across platforms.