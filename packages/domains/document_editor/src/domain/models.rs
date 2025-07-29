use crate::domain::value_objects::{DocumentTitle, DocumentContent};
use crate::domain::errors::DocumentError;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub title: DocumentTitle,
    pub content: DocumentContent,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_deleted: bool,
}

impl Document {
    pub fn new(
        owner_id: Uuid,
        title: DocumentTitle,
        content: DocumentContent,
    ) -> Self {
        let now = Utc::now();
        Document {
            id: Uuid::new_v4(),
            owner_id,
            title,
            content,
            created_at: now,
            updated_at: now,
            is_deleted: false,
        }
    }
    
    pub fn update_content(&mut self, content: DocumentContent) -> Result<(), DocumentError> {
        self.content = content;
        self.updated_at = Utc::now();
        Ok(())
    }
    
    pub fn update_title(&mut self, title: DocumentTitle) -> Result<(), DocumentError> {
        self.title = title;
        self.updated_at = Utc::now();
        Ok(())
    }
    
    pub fn delete(&mut self) {
        self.is_deleted = true;
        self.updated_at = Utc::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentShare {
    pub id: Uuid,
    pub document_id: Uuid,
    pub shared_with: Uuid,
    pub permission_level: PermissionLevel,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PermissionLevel {
    #[serde(rename = "view")]
    View,
    #[serde(rename = "comment")]
    Comment,
    #[serde(rename = "edit")]
    Edit,
}

impl PermissionLevel {
    pub fn from_str(s: &str) -> Result<Self, DocumentError> {
        match s {
            "view" => Ok(PermissionLevel::View),
            "comment" => Ok(PermissionLevel::Comment),
            "edit" => Ok(PermissionLevel::Edit),
            _ => Err(DocumentError::InvalidPermission(s.to_string())),
        }
    }
    
    pub fn can_edit(&self) -> bool {
        matches!(self, PermissionLevel::Edit)
    }
    
    pub fn can_comment(&self) -> bool {
        matches!(self, PermissionLevel::Edit | PermissionLevel::Comment)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentVersion {
    pub id: Uuid,
    pub document_id: Uuid,
    pub version_number: i32,
    pub content: DocumentContent,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
}