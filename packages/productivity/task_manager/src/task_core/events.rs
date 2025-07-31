use uuid::Uuid;
use chrono::{DateTime, Utc};

pub enum DomainEvent {
    TaskCreated {
        task_id: Uuid,
        title: String,
        assignee: Option<Uuid>,
        due_date: Option<DateTime<Utc>>,
    },
    TaskAssigned {
        task_id: Uuid,
        assignee: Uuid,
    },
    TaskProgressUpdated {
        task_id: Uuid,
        progress: u8,
    },
    // Add other domain events
}