//! Meeting service
//!
//! Responsibilities:
//! - Create/end meetings
//! - Stub WebRTC offer generation
//! - Publish MeetingStarted events

use async_trait::async_trait;
use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::models::MeetingRoom;

#[derive(thiserror::Error, Debug)]
pub enum MeetingServiceError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("repository error: {0}")]
    Repository(String),
    #[error("event publish error: {0}")]
    Event(String),
}

/// Repository abstraction for meetings
#[async_trait]
pub trait MeetingRepository: Send + Sync {
    async fn create_meeting(&self, room: &MeetingRoom) -> Result<(), String>;
    async fn get_meeting(&self, id: Uuid) -> Result<MeetingRoom, String>;
    async fn end_meeting(&self, id: Uuid, ended_at: DateTime<Utc>) -> Result<MeetingRoom, String>;
}

/// Event publisher for meeting-related events
#[async_trait]
pub trait MeetingEventPublisher: Send + Sync {
    async fn publish_meeting_started(&self, event: MeetingStarted) -> Result<(), String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingStarted {
    pub meeting_id: Uuid,
    pub owner_id: Uuid,
    pub started_at: DateTime<Utc>,
}

#[async_trait]
pub trait MeetingService: Send + Sync {
    async fn create_meeting(&self, title: String, owner_id: Uuid) -> Result<MeetingRoom, MeetingServiceError>;
    async fn end_meeting(&self, meeting_id: Uuid) -> Result<MeetingRoom, MeetingServiceError>;
    async fn generate_webrtc_offer(&self, meeting_id: Uuid, user_id: Uuid) -> Result<String, MeetingServiceError>;
}

pub struct MeetingServiceImpl<R: MeetingRepository, P: MeetingEventPublisher> {
    repo: Arc<R>,
    publisher: Arc<P>,
}

impl<R: MeetingRepository, P: MeetingEventPublisher> MeetingServiceImpl<R, P> {
    pub fn new(repo: Arc<R>, publisher: Arc<P>) -> Self {
        Self { repo, publisher }
    }
}

#[async_trait]
impl<R: MeetingRepository, P: MeetingEventPublisher> MeetingService for MeetingServiceImpl<R, P> {
    async fn create_meeting(&self, title: String, owner_id: Uuid) -> Result<MeetingRoom, MeetingServiceError> {
        if title.trim().is_empty() {
            return Err(MeetingServiceError::Repository("title cannot be empty".into()));
        }
        let room = MeetingRoom {
            id: Uuid::new_v4(),
            title,
            owner_id,
            created_at: Utc::now(),
            ended_at: None,
        };
        self.repo.create_meeting(&room).await.map_err(MeetingServiceError::Repository)?;

        // publish MeetingStarted
        let evt = MeetingStarted {
            meeting_id: room.id,
            owner_id: room.owner_id,
            started_at: room.created_at,
        };
        self.publisher.publish_meeting_started(evt).await.map_err(MeetingServiceError::Event)?;

        Ok(room)
    }

    async fn end_meeting(&self, meeting_id: Uuid) -> Result<MeetingRoom, MeetingServiceError> {
        let ended_at = Utc::now();
        self.repo.end_meeting(meeting_id, ended_at).await.map_err(|e| {
            if e.to_lowercase().contains("not found") {
                MeetingServiceError::NotFound(meeting_id.to_string())
            } else {
                MeetingServiceError::Repository(e)
            }
        })
    }

    async fn generate_webrtc_offer(&self, meeting_id: Uuid, user_id: Uuid) -> Result<String, MeetingServiceError> {
        // Stub: produce a deterministic fake SDP-like string
        // Later this will call into a WebRTC component.
        let sdp = format!("v=0\no=- {} 1 IN IP4 127.0.0.1\ns=CPC-Meeting-{}\n", user_id, meeting_id);
        Ok(sdp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct InMemRepo;
    #[async_trait]
    impl MeetingRepository for InMemRepo {
        async fn create_meeting(&self, _room: &MeetingRoom) -> Result<(), String> { Ok(()) }
        async fn get_meeting(&self, _id: Uuid) -> Result<MeetingRoom, String> {
            Err("not implemented".into())
        }
        async fn end_meeting(&self, id: Uuid, ended_at: DateTime<Utc>) -> Result<MeetingRoom, String> {
            Ok(MeetingRoom {
                id,
                title: "x".into(),
                owner_id: Uuid::new_v4(),
                created_at: Utc::now(),
                ended_at: Some(ended_at),
            })
        }
    }

    struct NoopPublisher;
    #[async_trait]
    impl MeetingEventPublisher for NoopPublisher {
        async fn publish_meeting_started(&self, _event: MeetingStarted) -> Result<(), String> { Ok(()) }
    }

    #[tokio::test]
    async fn create_publishes_started() {
        let svc = MeetingServiceImpl::new(Arc::new(InMemRepo), Arc::new(NoopPublisher));
        let room = svc.create_meeting("Standup".into(), Uuid::new_v4()).await.unwrap();
        assert_eq!(room.ended_at, None);
    }

    #[tokio::test]
    async fn end_meeting_sets_ended_at() {
        let svc = MeetingServiceImpl::new(Arc::new(InMemRepo), Arc::new(NoopPublisher));
        let ended = svc.end_meeting(Uuid::new_v4()).await.unwrap();
        assert!(ended.ended_at.is_some());
    }

    #[tokio::test]
    async fn webrtc_offer_stub() {
        let svc = MeetingServiceImpl::new(Arc::new(InMemRepo), Arc::new(NoopPublisher));
        let offer = svc.generate_webrtc_offer(Uuid::new_v4(), Uuid::new_v4()).await.unwrap();
        assert!(offer.contains("v=0"));
    }
}