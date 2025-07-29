//! Domain-specific value objects with validation

use serde::{Deserialize, Serialize};
use std::fmt;

/// A validated hex color value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorHex(String);

impl ColorHex {
    /// Create a new ColorHex, validating the format
    pub fn new(hex: String) -> Result<Self, ValueError> {
        // Check if it's a valid hex color format
        if hex.len() != 7 || !hex.starts_with('#') {
            return Err(ValueError::InvalidFormat);
        }

        // Check if all characters after # are valid hex digits
        for c in hex[1..].chars() {
            if !c.is_ascii_hexdigit() {
                return Err(ValueError::InvalidFormat);
            }
        }

        Ok(ColorHex(hex))
    }

    /// Get the hex string value
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ColorHex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A validated URL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidUrl(String);

impl ValidUrl {
    /// Create a new ValidUrl, validating the format
    pub fn new(url: String) -> Result<Self, ValueError> {
        // Basic URL validation - check if it starts with http or https
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(ValueError::InvalidFormat);
        }

        // Check for basic URL structure
        if url.len() < 10 { // Minimum length for a valid URL
            return Err(ValueError::InvalidFormat);
        }

        Ok(ValidUrl(url))
    }

    /// Get the URL string value
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ValidUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A validated content identifier for p2p storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentId(String);

impl ContentId {
    /// Create a new ContentId, validating the format
    pub fn new(cid: String) -> Result<Self, ValueError> {
        // Basic validation - check if it's not empty and has reasonable length
        if cid.is_empty() || cid.len() > 100 {
            return Err(ValueError::InvalidFormat);
        }

        Ok(ContentId(cid))
    }

    /// Get the CID string value
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ContentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Custom error type for value validation
#[derive(Debug, thiserror::Error)]
pub enum ValueError {
    #[error("Invalid format")]
    InvalidFormat,
    #[error("Value too long")]
    TooLong,
    #[error("Value too short")]
    TooShort,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_hex_valid() {
        let color = ColorHex::new("#FF0000".to_string()).unwrap();
        assert_eq!(color.as_str(), "#FF0000");
    }

    #[test]
    fn test_color_hex_invalid_format() {
        assert!(ColorHex::new("FF0000".to_string()).is_err());
        assert!(ColorHex::new("#FF000".to_string()).is_err());
        assert!(ColorHex::new("#FF00000".to_string()).is_err());
        assert!(ColorHex::new("#FF00GG".to_string()).is_err());
    }

    #[test]
    fn test_valid_url_valid() {
        let url = ValidUrl::new("https://example.com".to_string()).unwrap();
        assert_eq!(url.as_str(), "https://example.com");
    }

    #[test]
    fn test_valid_url_invalid_format() {
        assert!(ValidUrl::new("example.com".to_string()).is_err());
        assert!(ValidUrl::new("ftp://example.com".to_string()).is_err());
        assert!(ValidUrl::new("http://".to_string()).is_err());
    }

    #[test]
    fn test_content_id_valid() {
        let cid = ContentId::new("bafybeig6xv5nwphfmvcnektpnojts33jqcuam7bmye2pb54adnrtccjlsu".to_string()).unwrap();
        assert_eq!(cid.as_str(), "bafybeig6xv5nwphfmvcnektpnojts33jqcuam7bmye2pb54adnrtccjlsu");
    }

    #[test]
    fn test_content_id_invalid_format() {
        assert!(ContentId::new("".to_string()).is_err());
        let long_cid = "a".repeat(101);
        assert!(ContentId::new(long_cid).is_err());
    }
}