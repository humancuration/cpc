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

    #[test]
    fn test_valid_url_valid() {
        let valid_urls = vec![
            "http://example.com",
            "https://example.com",
            "https://www.example.com/path?query=1",
            "http://localhost:8080",
            "https://subdomain.example.com/path/to/resource",
        ];
        
        for url in valid_urls {
            let valid_url = ValidUrl::new(url);
            assert!(valid_url.is_ok());
            assert_eq!(valid_url.unwrap().as_str(), url);
        }
    }

    #[test]
    fn test_valid_url_empty() {
        let empty_url = ValidUrl::new("");
        assert!(empty_url.is_err());
        assert_eq!(empty_url.unwrap_err().to_string(), "URL is empty");
    }

    #[test]
    fn test_valid_url_missing_protocol() {
        let invalid_urls = vec![
            "example.com",
            "ftp://example.com",
            "example.com/path",
            "www.example.com",
        ];
        
        for url in invalid_urls {
            let invalid_url = ValidUrl::new(url);
            assert!(invalid_url.is_err());
            assert_eq!(invalid_url.unwrap_err().to_string(), "URL must start with http:// or https://");
        }
    }

    #[test]
    fn test_valid_url_too_short() {
        let short_urls = vec![
            "http://a",
            "https://ab",
            "http://abc",
            "https://abcd",
            "http://abcde",
            "https://abcdef",
            "http://abcdefg",
            "https://abcdefgh",
            "http://a.bc",
        ];
        
        for url in short_urls {
            let invalid_url = ValidUrl::new(url);
            assert!(invalid_url.is_err());
            assert_eq!(invalid_url.unwrap_err().to_string(), format!("Invalid URL format: {}", url));
        }
    }

    #[test]
    fn test_valid_url_valid_minimum_length() {
        // Test URLs that meet the minimum length requirement (10 characters)
        let valid_urls = vec![
            "http://ab.c",
            "https://a.bc",
            "http://a.com",
            "https://ab.org",
        ];
        
        for url in valid_urls {
            let valid_url = ValidUrl::new(url);
            assert!(valid_url.is_ok());
            assert_eq!(valid_url.unwrap().as_str(), url);
        }
    }

    #[test]
    fn test_template_id() {
        let uuid = uuid::Uuid::new_v4();
        let template_id = TemplateId::new(uuid);
        
        assert_eq!(template_id.as_uuid(), &uuid);
        assert_eq!(template_id.to_string(), uuid.to_string());
    }
}