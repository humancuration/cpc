//! Embed code generator for visualization data

/// Generate embed code for a shared visualization
pub fn generate_embed_code(share_id: &str) -> String {
    generate_custom_embed_code(share_id, Some(600), Some(400), Some("light"), Some(true), Some(true))
}

/// Generate markdown embed code for a shared visualization
pub fn generate_markdown_embed(share_id: &str) -> String {
    format!(
        r#"[![Visualization](https://federation.example.com/embed/{}/thumbnail)](https://federation.example.com/embed/{})"#,
        share_id, share_id
    )
}

/// Generate HTML embed code with customization options
pub fn generate_custom_embed_code(
    share_id: &str,
    width: Option<u32>,
    height: Option<u32>,
    theme: Option<&str>,
    show_title: Option<bool>,
    interactive: Option<bool>,
) -> String {
    let width_attr = width.unwrap_or(600);
    let height_attr = height.unwrap_or(400);
    let theme_param = theme.unwrap_or("light");
    let show_title_param = if show_title.unwrap_or(true) { "true" } else { "false" };
    let interactive_param = if interactive.unwrap_or(true) { "true" } else { "false" };
    
    format!(
        r#"<iframe src="https://federation.example.com/embed/{}?theme={}&show_title={}&interactive={}" width="{}" height="{}" frameborder="0" allowfullscreen></iframe>"#,
        share_id, theme_param, show_title_param, interactive_param, width_attr, height_attr
    )
}