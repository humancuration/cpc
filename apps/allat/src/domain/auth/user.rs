use serde::{Deserialize, Serialize};
use cpc_auth::models::User as BaseUser;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(flatten)]
    pub base: BaseUser,
    pub karma: i32,
    pub consent: HashMap<String, bool>, // Consent preferences
}