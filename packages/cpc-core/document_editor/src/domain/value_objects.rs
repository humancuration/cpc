use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentTitle(String);

impl DocumentTitle {
    pub fn new(title: String) -> Result<Self, String> {
        if title.is_empty() {
            return Err("Document title cannot be empty".to_string());
        }
        
        if title.len() > 255 {
            return Err("Document title cannot exceed 255 characters".to_string());
        }
        
        Ok(DocumentTitle(title))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DocumentTitle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentContent {
    pub content: serde_json::Value,
}

impl DocumentContent {
    pub fn new(content: serde_json::Value) -> Self {
        DocumentContent { content }
    }
    
    pub fn as_json(&self) -> &serde_json::Value {
        &self.content
    }
}