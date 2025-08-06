use async_graphql::{SimpleObject, ID, Context};
use crate::domain::auth::User;
use crate::application::error::ApplicationError;
#[derive(SimpleObject)]
pub struct UserObject {
    pub id: ID,
    pub username: String,
    pub karma: i32,
}

#[Object]
impl UserObject {
    async fn notification_preferences(&self, ctx: &Context<'_>) -> Result<NotificationPreferencesObject, ApplicationError> {
        // In a real implementation, we would fetch the user's notification preferences
        // from the notification core service
        Ok(NotificationPreferencesObject {
            email_notifications: true,
            push_notifications: true,
            social_notifications: true,
        })
    }
}
}

#[derive(SimpleObject)]
pub struct NotificationPreferencesObject {
    email_notifications: bool,
    push_notifications: bool,
    social_notifications: bool,
}

impl From<User> for UserObject {
    fn from(user: User) -> Self {
        Self {
            id: ID::from(user.base.id.to_string()),
            username: user.base.username,
            karma: user.karma,
        }
    }
}