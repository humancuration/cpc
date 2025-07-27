//! Custom error types for the website builder module

use thiserror::Error;
use crate::domain::value_objects::{ColorHexError, ValidUrlError};

#[derive(Debug, Error)]
pub enum WebsiteBuilderError {
    #[error("Site not found: {0}")]
    SiteNotFound(String),
    
    #[error("Template not found: {0}")]
    TemplateNotFound(String),
    
    #[error("User is not authorized to perform this action")]
    Unauthorized,
    
    #[error("Invalid site type: {0}")]
    InvalidSiteType(String),
    
    #[error("Site name is required")]
    SiteNameRequired,
    
    #[error("Invalid color format: {0}")]
    InvalidColor(#[from] ColorHexError),
    
    #[error("Invalid URL format: {0}")]
    InvalidUrl(#[from] ValidUrlError),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("P2P storage error: {0}")]
    P2PStorageError(String),
    
    #[error("Media processing error: {0}")]
    MediaProcessingError(String),
    
    #[error("A link-in-bio site already exists for this user")]
    LinkInBioSiteExists,
    
    #[error("Template is not compatible with site type")]
    TemplateIncompatible,
    
    #[error("Invalid page slug: {0}")]
    InvalidPageSlug(String),
    
    #[error("Page not found: {0}")]
    PageNotFound(String),
    
    #[error("Link not found: {0}")]
    LinkNotFound(String),
    
    #[error("Maximum number of links reached")]
    MaxLinksReached,
    
    #[error("Invalid template structure: {0}")]
    InvalidTemplateStructure(String),
}

impl From<sqlx::Error> for WebsiteBuilderError {
    fn from(error: sqlx::Error) -> Self {
        WebsiteBuilderError::DatabaseError(error.to_string())
    }
}