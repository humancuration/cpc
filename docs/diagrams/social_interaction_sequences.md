# Social Interaction Sequence Diagrams

This document contains sequence diagrams for the social interaction flows implemented in the CPC platform.

## 1. Adding a Reaction Flow

```mermaid
sequenceDiagram
    participant User
    participant GraphQL_API
    participant ReactionService
    participant ReactionRepository
    participant EventBus
    participant NotificationService

    User->>GraphQL_API: addReaction(targetId, targetType, reactionType)
    GraphQL_API->>ReactionService: addReaction(userId, targetId, targetType, reactionType)
    
    ReactionService->>ReactionRepository: user_has_reacted(userId, targetId, targetType)
    ReactionRepository-->>ReactionService: false
    
    ReactionService->>ReactionRepository: add_reaction(reaction)
    ReactionRepository-->>ReactionService: success
    
    ReactionService->>EventBus: publish_event(ReactionAdded)
    EventBus-->>ReactionService: success
    
    EventBus->>NotificationService: send_reaction_notification(reaction, targetOwnerId)
    NotificationService-->>EventBus: success
    
    ReactionService-->>GraphQL_API: reaction
    GraphQL_API-->>User: ReactionDto
```

### Flow Description:
1. User sends a GraphQL mutation to add a reaction to content
2. GraphQL API validates the input and calls the ReactionService
3. ReactionService checks if the user has already reacted to this target
4. If not, it creates a new reaction and saves it via the repository
5. The service publishes a ReactionAdded event to the EventBus
6. EventBus triggers notification sending for the target owner
7. The reaction is returned to the user

## 2. Posting a Comment Flow

```mermaid
sequenceDiagram
    participant User
    participant GraphQL_API
    participant CommentService
    participant CommentRepository
    participant EventBus
    participant NotificationService

    User->>GraphQL_API: addComment(targetId, targetType, content, parentId)
    GraphQL_API->>CommentService: add_comment(userId, targetId, targetType, content, parentId)
    
    Note over CommentService: Validate content and parent comment
    
    CommentService->>CommentRepository: add_comment(comment)
    CommentRepository-->>CommentService: success
    
    CommentService->>EventBus: publish_event(CommentAdded)
    EventBus-->>CommentService: success
    
    EventBus->>NotificationService: send_comment_notification(comment, targetOwnerId, parentOwnerId)
    NotificationService-->>EventBus: success
    
    CommentService-->>GraphQL_API: comment
    GraphQL_API-->>User: CommentDto
```

### Flow Description:
1. User sends a GraphQL mutation to add a comment to content
2. GraphQL API validates the input and calls the CommentService
3. CommentService validates the content and checks parent comment (if any)
4. It creates a new comment and saves it via the repository
5. The service publishes a CommentAdded event to the EventBus
6. EventBus triggers notification sending for the target owner and parent comment owner (if applicable)
7. The comment is returned to the user

## 3. Sharing Content Flow

```mermaid
sequenceDiagram
    participant User
    participant GraphQL_API
    participant ShareService
    participant ShareRepository
    participant EventBus
    participant NotificationService

    User->>GraphQL_API: shareContent(contentId, contentType, sharedWith)
    GraphQL_API->>ShareService: share_content(userId, contentId, contentType, sharedWith)
    
    ShareService->>ShareRepository: add_share(share)
    ShareRepository-->>ShareService: success
    
    ShareService->>EventBus: publish_event(ContentShared)
    EventBus-->>ShareService: success
    
    EventBus->>NotificationService: send_share_notification(share, contentOwnerId)
    NotificationService-->>EventBus: success
    
    ShareService-->>GraphQL_API: share
    GraphQL_API-->>User: ShareDto
```

### Flow Description:
1. User sends a GraphQL mutation to share content
2. GraphQL API validates the input and calls the ShareService
3. ShareService creates a new share and saves it via the repository
4. The service publishes a ContentShared event to the EventBus
5. EventBus triggers notification sending for the content owner
6. The share is returned to the user

## 4. Real-time Updates Flow

```mermaid
sequenceDiagram
    participant UserA
    participant GraphQL_API
    participant ReactionService
    participant EventBus
    participant UserB
    participant WebSocket

    UserA->>GraphQL_API: addReaction(targetId, targetType, reactionType)
    GraphQL_API->>ReactionService: add_reaction(userId, targetId, targetType, reactionType)
    ReactionService->>EventBus: publish_event(ReactionAdded)
    EventBus-->>ReactionService: success
    
    Note over EventBus: Event is broadcast to all subscribers
    
    EventBus->>WebSocket: broadcast_update(reactionAdded)
    WebSocket->>UserB: real_time_update(reactionAdded)
    
    UserB->>WebSocket: acknowledge_update()
```

### Flow Description:
1. UserA adds a reaction to content
2. The reaction service processes the request and publishes an event
3. EventBus broadcasts the event to all subscribed WebSocket clients
4. UserB receives a real-time update about the new reaction
5. UserB acknowledges the update

## 5. Nested Comments Flow

```mermaid
sequenceDiagram
    participant User
    participant GraphQL_API
    participant CommentService
    participant CommentRepository
    participant EventBus
    participant NotificationService

    User->>GraphQL_API: addComment(targetId, "post", content, parentId)
    GraphQL_API->>CommentService: add_comment(userId, targetId, "post", content, parentId)
    
    CommentService->>CommentRepository: get_comment(parentId)
    CommentRepository-->>CommentService: parent_comment
    
    CommentService->>CommentRepository: add_comment(comment)
    CommentRepository-->>CommentService: success
    
    CommentService->>EventBus: publish_event(CommentAdded)
    EventBus-->>CommentService: success
    
    EventBus->>NotificationService: send_comment_notification(comment, targetOwnerId, parentOwnerId)
    NotificationService-->>EventBus: success
    
    CommentService-->>GraphQL_API: comment
    GraphQL_API-->>User: CommentDto
```

### Flow Description:
1. User submits a reply to an existing comment
2. CommentService validates that the parent comment exists
3. It creates a nested comment and saves it via the repository
4. The service publishes a CommentAdded event to the EventBus
5. EventBus triggers notifications for both the target owner and parent comment owner
6. The nested comment is returned to the user

## Error Handling Flow

```mermaid
sequenceDiagram
    participant User
    participant GraphQL_API
    participant ReactionService
    participant ReactionRepository

    User->>GraphQL_API: addReaction(targetId, targetType, reactionType)
    GraphQL_API->>ReactionService: add_reaction(userId, targetId, targetType, reactionType)
    
    ReactionService->>ReactionRepository: user_has_reacted(userId, targetId, targetType)
    ReactionRepository-->>ReactionService: true
    
    ReactionService-->>GraphQL_API: ValidationError("User has already reacted")
    GraphQL_API-->>User: Error("User has already reacted to this target")
```

### Flow Description:
1. User attempts to add a reaction to content they've already reacted to
2. ReactionService checks for existing reactions and finds one
3. The service returns a validation error
4. The error is propagated back to the user via GraphQL

## Performance Considerations

- **Database Optimization**: All repository operations use indexed queries for efficient lookups
- **Event Processing**: EventBus uses weak references to prevent memory leaks
- **Notification Batching**: Multiple notifications for the same user are batched when possible
- **Real-time Updates**: WebSocket connections are efficiently managed for real-time updates
- **Caching**: Reaction summaries are cached to reduce database load

## Security Considerations

- **Authorization**: All operations verify user ownership before modifications
- **Input Validation**: All user inputs are validated before processing
- **Rate Limiting**: API endpoints implement rate limiting to prevent abuse
- **Content Sanitization**: Comment content is sanitized to prevent XSS attacks
- **Privacy Settings**: Share visibility is enforced at the repository level