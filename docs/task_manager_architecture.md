# Task Manager Module Architecture

## Crate Structure
```
apps/task_manager/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── domain/
    │   ├── models.rs
    │   └── value_objects.rs
    ├── application/
    │   ├── service.rs
    │   └── ports.rs
    ├── infrastructure/
    │   ├── repository.rs
    │   ├── p2p_sync.rs
    │   └── notification_adapter.rs
    └── web/
        ├── routes.rs
        ├── graphql.rs
        ├── dto.rs
        └── module.rs
```

## Domain Models (domain/models.rs)
```rust
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub project_id: Option<Uuid>,
    pub due_date: Option<DateTime<Utc>>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub color: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Reminder {
    pub id: Uuid,
    pub task_id: Uuid,
    pub remind_at: DateTime<Utc>,
    pub notified: bool,
}
```

## GraphQL Schema (web/graphql.rs)
```rust
use async_graphql::*;

#[derive(InputObject)]
pub struct CreateTaskInput {
    pub title: String,
    pub description: Option<String>,
    pub project_id: Option<Uuid>,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(SimpleObject)]
pub struct TaskPayload {
    pub task: Task,
}

#[derive(Default)]
pub struct TaskMutation;

#[Object]
impl TaskMutation {
    async fn create_task(&self, ctx: &Context<'_>, input: CreateTaskInput) -> Result<TaskPayload> {
        let service = ctx.data::<Arc<dyn TaskService>>()?;
        let task = service.create_task(input).await?;
        Ok(TaskPayload { task })
    }
    
    // Additional mutations: update_task, delete_task, complete_task
}

#[derive(Default)]
pub struct TaskSubscription;

#[Subscription]
impl TaskSubscription {
    async fn reminder_alert(
        &self,
        ctx: &Context<'_>,
    ) -> impl Stream<Item = Result<Task>> {
        let service = ctx.data::<Arc<dyn TaskService>>()?;
        service.reminder_stream().await
    }
}
```

## Service Layer Architecture (application/service.rs)
```rust
use crate::domain::{Task, Project, Reminder};
use crate::application::ports::{TaskRepository, NotificationService, P2pSyncService};

#[async_trait]
pub trait TaskService: Send + Sync {
    async fn create_task(&self, input: CreateTaskInput) -> Result<Task>;
    async fn get_task(&self, id: Uuid) -> Result<Task>;
    async fn update_task(&self, id: Uuid, updates: TaskUpdate) -> Result<Task>;
    async fn list_tasks(&self, project_id: Option<Uuid>) -> Result<Vec<Task>>;
    async fn reminder_stream(&self) -> impl Stream<Item = Result<Task>>;
}

pub struct TaskServiceImpl {
    repo: Arc<dyn TaskRepository>,
    notifier: Arc<dyn NotificationService>,
    p2p_sync: Arc<dyn P2pSyncService>,
}

#[async_trait]
impl TaskService for TaskServiceImpl {
    async fn create_task(&self, input: CreateTaskInput) -> Result<Task> {
        let task = self.repo.create_task(input).await?;
        self.p2p_sync.broadcast_task_update(&task).await;
        Ok(task)
    }
    
    // Other implementations...
}
```

## P2P Synchronization Strategy (infrastructure/p2p_sync.rs)
1. **Data Model**: 
   - Tasks stored as CRDTs using p2panda's schema system
   - Each task/project is a separate document
   - Operations: CREATE, UPDATE, DELETE

2. **Sync Flow**:
   - On create/update: Broadcast document to p2p network
   - On receive: Merge conflicts using last-write-wins with vector clocks
   - Subscribe to task/project document types for real-time updates

3. **Conflict Resolution**:
   - Use metadata fields: `updated_at` and `version` 
   - Implement custom merge logic for critical fields

4. **Device Sync**:
   - Use p2panda's replication protocol
   - Encrypt sensitive data with user's keypair
   - Support offline-first operations with local cache

## Integration Points
1. **Notification System**:
   - Implement ReminderScheduler that checks due tasks
   - Integrate with platform notification APIs (desktop/mobile)
   - Use async channels for reminder events

2. **UI Component Library** (cpc-platform):
   - TaskList component: Displays tasks with filtering
   - TaskForm component: Create/edit tasks
   - ProjectPicker component: Select project for task
   - ReminderBadge: Shows upcoming reminders

3. **cpc-net Integration**:
   - Register task/project document schemas
   - Implement P2pSyncService trait using p2panda client
   - Handle incoming sync events from network