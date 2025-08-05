# Signaling Protocol Documentation

## Overview

The CPC signaling protocol enables real-time collaboration features for applications. It uses WebSocket connections for bidirectional communication between clients and signaling servers, with optional Redis pub/sub for horizontal scaling.

## Message Types

### Core Messages

#### JoinDocument
Sent when a user joins a document.
```json
{
  "JoinDocument": {
    "document_id": "uuid",
    "user_id": "uuid"
  }
}
```

#### LeaveDocument
Sent when a user leaves a document.
```json
{
  "LeaveDocument": {
    "document_id": "uuid",
    "user_id": "uuid"
  }
}
```

#### PresenceUpdate
Sent to update user presence information.
```json
{
  "PresenceUpdate": {
    "document_id": "uuid",
    "user_id": "uuid",
    "cursor": {
      "line": 0,
      "column": 0
    },
    "selection": {
      "start": {
        "line": 0,
        "column": 0
      },
      "end": {
        "line": 0,
        "column": 5
      }
    },
    "is_typing": true,
    "timestamp": "2023-01-01T00:00:00Z"
  }
}
```

#### CursorUpdate
Sent to update cursor position.
```json
{
  "CursorUpdate": {
    "document_id": "uuid",
    "user_id": "uuid",
    "position": {
      "line": 0,
      "column": 0
    },
    "timestamp": "2023-01-01T00:00:00Z"
  }
}
```

#### SelectionUpdate
Sent to update text selection.
```json
{
  "SelectionUpdate": {
    "document_id": "uuid",
    "user_id": "uuid",
    "selection": {
      "start": {
        "line": 0,
        "column": 0
      },
      "end": {
        "line": 0,
        "column": 5
      }
    },
    "timestamp": "2023-01-01T00:00:00Z"
  }
}
```

#### TypingIndicator
Sent to indicate typing activity.
```json
{
  "TypingIndicator": {
    "document_id": "uuid",
    "user_id": "uuid",
    "is_typing": true,
    "timestamp": "2023-01-01T00:00:00Z"
  }
}
```

#### Error
Sent to report errors.
```json
{
  "Error": {
    "code": "string",
    "message": "string"
  }
}
```

### Enhanced Messages

#### Annotation
Sent to add an annotation to a document.
```json
{
  "Annotation": {
    "document_id": "uuid",
    "user_id": "uuid",
    "position": {
      "line": 0,
      "column": 0
    },
    "content": "string",
    "timestamp": "2023-01-01T00:00:00Z"
  }
}
```

#### Comment
Sent to add a comment to a document.
```json
{
  "Comment": {
    "document_id": "uuid",
    "user_id": "uuid",
    "position": {
      "line": 0,
      "column": 0
    },
    "content": "string",
    "timestamp": "2023-01-01T00:00:00Z"
  }
}
```

#### PresenceStatus
Sent to update user presence status.
```json
{
  "PresenceStatus": {
    "document_id": "uuid",
    "user_id": "uuid",
    "status": "string", // e.g., "online", "away", "busy"
    "timestamp": "2023-01-01T00:00:00Z"
  }
}
```

## Connection Flow

1. Client connects to WebSocket server
2. Client sends `JoinDocument` message
3. Server responds with current presence information
4. Client and server exchange real-time updates
5. Client sends `LeaveDocument` when disconnecting

## Scaling with Redis

For horizontal scaling, the signaling service can use Redis pub/sub:

1. Each server instance subscribes to document-specific channels
2. Messages are published to Redis channels
3. All server instances receive and broadcast messages to their local clients
4. This enables multiple server instances to coordinate

## Error Handling

Common error codes:
- `PARSE_ERROR`: Failed to parse message
- `CONNECTION_ERROR`: Connection issue
- `AUTH_ERROR`: Authentication failed
- `DOCUMENT_ERROR`: Document-related error