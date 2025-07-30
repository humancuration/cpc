use crate::models::Session;
use crate::error::SessionError;
use uuid::Uuid;

pub trait SessionService: Send + Sync {
    fn create_session(&self, user_id: Uuid, device_info: String) -> Result<Session, SessionError>;
    fn invalidate_session(&self, session_id: Uuid) -> Result<(), SessionError>;
    fn find_session(&self, session_id: Uuid) -> Result<Option<Session>, SessionError>;
}

pub struct SessionServiceImpl;

impl SessionServiceImpl {
    pub fn new() -> Self {
        Self
    }
}