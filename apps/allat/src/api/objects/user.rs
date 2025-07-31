use async_graphql::{SimpleObject, ID};
use crate::domain::auth::User;

#[derive(SimpleObject)]
pub struct UserObject {
    pub id: ID,
    pub username: String,
    pub karma: i32,
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