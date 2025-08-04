use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Re-export the shared SocialEvent and SocialEventBus for collaborative workspace.
///
/// We extend SocialEvent in the social_interactions crate with collaboration-specific
/// variants (DocumentUpdated, TaskMoved, WhiteboardModified, MeetingStarted), so this
/// module simply provides convenience helpers and re-exports for consumers.
pub use social_interactions::infrastructure::event_bus::{SocialEvent, SocialEventBus};

/// Convenience helpers for publishing collaboration events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationEvents;

impl CollaborationEvents {
    /// Build a SocialEvent for a document update.
    pub fn document_updated(document_id: Uuid, user_id: Uuid) -> SocialEvent {
        SocialEvent::DocumentUpdated { document_id, user_id }
    }

    /// Build a SocialEvent for a task move.
    pub fn task_moved(task_id: Uuid, new_column_id: Uuid) -> SocialEvent {
        SocialEvent::TaskMoved { task_id, new_column_id }
    }

    /// Build a SocialEvent for a whiteboard modification.
    pub fn whiteboard_modified(whiteboard_id: Uuid) -> SocialEvent {
        SocialEvent::WhiteboardModified { whiteboard_id }
    }

    /// Build a SocialEvent for a meeting start.
    pub fn meeting_started(meeting_id: Uuid) -> SocialEvent {
        SocialEvent::MeetingStarted { meeting_id }
    }
}