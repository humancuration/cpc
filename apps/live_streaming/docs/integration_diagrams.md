# Live Streaming Integration Diagrams

## Stream Start/Stop Flow

```mermaid
sequenceDiagram
    participant User
    participant LiveStreamingModule
    participant SocialIntegration
    participant NotificationCore
    participant Messenger

    User->>LiveStreamingModule: Start Stream
    LiveStreamingModule->>SocialIntegration: Create StreamStarted Event
    SocialIntegration->>NotificationCore: Notify Followers
    NotificationCore->>User: Send Stream Started Notification
    LiveStreamingModule->>Messenger: Create Stream Chat
    Messenger-->>LiveStreamingModule: Return Chat Conversation
```

## Chat Message Propagation

```mermaid
sequenceDiagram
    participant Viewer
    participant LiveStreamingModule
    participant Messenger
    participant SocialIntegration

    Viewer->>LiveStreamingModule: Send Chat Message
    LiveStreamingModule->>Messenger: Process Message
    Messenger->>LiveStreamingModule: Broadcast Message
    LiveStreamingModule->>SocialIntegration: Record Chat Event
    SocialIntegration-->>LiveStreamingModule: Event Recorded
```

## Subscription Notification Flow

```mermaid
sequenceDiagram
    participant Broadcaster
    participant SocialIntegration
    participant NotificationCore
    participant Follower

    Broadcaster->>SocialIntegration: Stream Started
    SocialIntegration->>NotificationCore: Create Notification
    NotificationCore->>Follower: Send Stream Notification
    Follower->>LiveStreamingModule: Join Stream