use crate::domain::session::Session;
use crate::domain::auth_error::SessionError;
use uuid::Uuid;

pub trait SessionManagement: Send + Sync {
    fn create_session(&self, user_id: Uuid, device_info: String) -> Result<Session, SessionError>;
    fn invalidate_session(&self, session_id: Uuid) -> Result<(), SessionError>;
    fn find_session(&self, session_id: Uuid) -> Result<Option<Session>, SessionError>;
}