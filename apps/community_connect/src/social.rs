// Social integration functionality
use uuid::Uuid;

pub struct SocialConnection {
    pub id: Uuid,
    pub user_id: Uuid,
    pub platform: String,
    pub connection_id: String,
}

impl SocialConnection {
    pub fn new(user_id: Uuid, platform: String, connection_id: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            platform,
            connection_id,
        }
    }
}
