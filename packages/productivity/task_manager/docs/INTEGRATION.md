# Task Manager Integration Guide

## Calendar Integration
Tasks with due dates automatically create calendar events. The Calendar app subscribes to task updates via GraphQL subscriptions.

### Integration Points
1. **TaskCreated Event**
   - Payload: Task ID, title, due_date, assignee
   - Calendar Action: Creates new event

2. **TaskUpdated Event**
   - Payload: Updated fields (especially due_date changes)
   - Calendar Action: Updates existing event

3. **TaskCompleted Event**
   - Payload: Task ID
   - Calendar Action: Marks event as completed

### Data Flow
```
Task Manager → GraphQL Subscription → Calendar App
```

## Social Integration
Leverages the `social_integration` crate for team collaboration features.

### Unified Social Features
1. **Task Comments**
   - Implemented as SocialEvent::Comment
   - Attached to task entities

2. **Assignment Notifications**
   - SocialEvent::Notification sent on task assignment
   - Includes task title and due date

3. **Public Task Boards**
   - SocialEvent::SharedContent for public projects
   - Privacy controls from `consent_manager`

### Integration Components
```rust
// Example: Creating a task comment
use social_integration::domain::social_event::{SocialEvent, Comment};
use uuid::Uuid;

let comment = Comment {
    content: "Need clarification on requirements".to_string(),
    parent_id: Some(task_id), // Reference to task
};

let event = SocialEvent::Comment(comment);
social_service.publish_event(user_id, event).await?;
```

## Dependency Management
Tasks can define dependencies that create relationships between:
- Tasks within same project
- Tasks across different projects
- Calendar events (future implementation)

### Cross-App Dependencies
```graphql
# GraphQL mutation for dependency
mutation {
  createTaskDependency(
    taskId: "c0d3c0d3-c0d3-c0d3-c0d3-c0d3c0d3c0d3"
    dependsOn: ["event:fc3fc3fc-3fc3-3fc3-3fc3-fc3fc3fc3fc3"]
  ) {
    success
  }
}
```

## API Contracts
### GraphQL Endpoints
- `createTask`
- `updateTask`
- `createComment`
- `assignTask`
- `createDependency`

### gRPC Services (Internal)
- `TaskService`
- `CalendarIntegrationService`
- `SocialEventService`