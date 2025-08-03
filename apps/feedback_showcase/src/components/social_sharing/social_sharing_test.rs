//! Tests for social sharing components

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_embed_code_generation() {
        // Test basic embed code generation
        let share_id = "test-id-123";
        let embed_code = crate::components::social_sharing::embed_code_generator::generate_embed_code(share_id);
        assert!(embed_code.contains(share_id));
        assert!(embed_code.contains("iframe"));
        assert!(embed_code.contains("width=\"600\""));
        assert!(embed_code.contains("height=\"400\""));
    }
    
    #[test]
    fn test_custom_embed_code_generation() {
        // Test custom embed code generation
        let share_id = "test-id-456";
        let embed_code = crate::components::social_sharing::embed_code_generator::generate_custom_embed_code(
            share_id,
            Some(800),
            Some(600),
            Some("dark"),
            Some(false),
            Some(true),
        );
        assert!(embed_code.contains(share_id));
        assert!(embed_code.contains("iframe"));
        assert!(embed_code.contains("width=\"800\""));
        assert!(embed_code.contains("height=\"600\""));
        assert!(embed_code.contains("theme=dark"));
        assert!(embed_code.contains("show_title=false"));
        assert!(embed_code.contains("interactive=true"));
    }
    
    #[test]
    fn test_markdown_embed_generation() {
        // Test markdown embed code generation
        let share_id = "test-id-789";
        let markdown_embed = crate::components::social_sharing::embed_code_generator::generate_markdown_embed(share_id);
        assert!(markdown_embed.contains(share_id));
        assert!(markdown_embed.contains("![Visualization]"));
        assert!(markdown_embed.contains("thumbnail"));
    }
}