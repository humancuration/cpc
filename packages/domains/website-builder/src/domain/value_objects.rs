//! Value objects for the website builder module

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;
use uuid::Uuid;

/// Valid color format with validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorHex(String);

#[derive(Debug, Error)]
pub enum ColorHexError {
    #[error("Invalid color hex format: {0}")]
    InvalidFormat(String),
    #[error("Color hex must be 7 characters long (including #)")]
    InvalidLength,
}

impl ColorHex {
    pub fn new(hex: &str) -> Result<Self, ColorHexError> {
        if hex.len() != 7 {
            return Err(ColorHexError::InvalidLength);
        }
        
        if !hex.starts_with('#') {
            return Err(ColorHexError::InvalidFormat(hex.to_string()));
        }
        
        // Check if all characters after # are valid hex digits
        for c in hex[1..].chars() {
            if !c.is_ascii_hexdigit() {
                return Err(ColorHexError::InvalidFormat(hex.to_string()));
            }
        }
        
        Ok(ColorHex(hex.to_string()))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ColorHex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Valid URL with validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidUrl(String);

#[derive(Debug, Error)]
pub enum ValidUrlError {
    #[error("Invalid URL format: {0}")]
    InvalidFormat(String),
    #[error("URL is empty")]
    Empty,
    #[error("URL must start with http:// or https://")]
    MissingProtocol,
}

impl ValidUrl {
    pub fn new(url: &str) -> Result<Self, ValidUrlError> {
        if url.is_empty() {
            return Err(ValidUrlError::Empty);
        }
        
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(ValidUrlError::MissingProtocol);
        }
        
        // Basic URL validation - in a real implementation, you might want to use
        // a more comprehensive URL parsing library
        if url.len() < 10 { // Minimum length for a valid URL
            return Err(ValidUrlError::InvalidFormat(url.to_string()));
        }
        
        Ok(ValidUrl(url.to_string()))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ValidUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Template identifier with validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateId(Uuid);

impl TemplateId {
    pub fn new(uuid: Uuid) -> Self {
        TemplateId(uuid)
    }
    
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl fmt::Display for TemplateId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_hex_valid() {
        let valid_hex = "#FF0000";
        let color = ColorHex::new(valid_hex);
        assert!(color.is_ok());
        assert_eq!(color.unwrap().as_str(), valid_hex);
    }

    #[test]
    fn test_color_hex_invalid_format() {
        let invalid_hex = "FF0000";
        let color = ColorHex::new(invalid_hex);
        assert!(color.is_err());
    }

    #[test]
    fn test_color_hex_invalid_length() {
        let short_hex = "#FF00";
        let color = ColorHex::new(short_hex);
        assert!(color.is_err());

        let long_hex = "#FF000000";
        let color = ColorHex::new(long_hex);
        assert!(color.is_err());
    }

    #[test]
    fn test_color_hex_invalid_characters() {
        let invalid_chars = "#FF00GG";
        let color = ColorHex::new(invalid_chars);
        assert!(color.is_err());
    }
}
}