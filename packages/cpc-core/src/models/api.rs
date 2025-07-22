use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIResponse<T> {
    pub success: bool,
    pub error: Option<String>,
    pub data: Option<T>,
}