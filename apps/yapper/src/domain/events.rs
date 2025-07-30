use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum YapperEvent {
    YapCreated {
        id: Uuid,
        user_id: Uuid,
        content: String,
    },
    YapLiked {
        yap_id: Uuid,
        user_id: Uuid,
    },
    YapShared {
        yap_id: Uuid,
        user_id: Uuid,
    },
    UserFollowed {
        follower_id: Uuid,
        following_id: Uuid,
    },
    UserRegistered {
        user_id: Uuid,
        email: String,
    },
    UserLoggedIn {
        user_id: Uuid,
        device_info: String,
    },
    PasswordResetRequested {
        user_id: Uuid,
    },
}

pub trait EventPublisher: Send + Sync {
    fn publish(&self, event: YapperEvent);
}