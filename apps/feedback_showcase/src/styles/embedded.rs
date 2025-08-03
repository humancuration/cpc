use stylist::{css, Style};

pub fn embedded_visualization_container() -> Style {
    Style::new(css!(
        r#"
            position: relative;
            margin: 20px 0;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embedded_visualization() -> Style {
    Style::new(css!(
        r#"
            background-color: #f9f9f9;
            border: 1px solid #ddd;
            border-radius: 4px;
            overflow: hidden;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embedded_visualization_header() -> Style {
    Style::new(css!(
        r#"
            background-color: #f0f0f0;
            padding: 10px 15px;
            border-bottom: 1px solid #ddd;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embedded_visualization_header_h4() -> Style {
    Style::new(css!(
        r#"
            margin: 0;
            color: #333;
            font-size: 16px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embedded_visualization_content() -> Style {
    Style::new(css!(
        r#"
            flex: 1;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            padding: 20px;
            text-align: center;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embedded_visualization_content_p() -> Style {
    Style::new(css!(
        r#"
            margin: 10px 0;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embedded_visualization_footer() -> Style {
    Style::new(css!(
        r#"
            background-color: #f0f0f0;
            padding: 8px 15px;
            border-top: 1px solid #ddd;
            text-align: right;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embedded_visualization_content_loading() -> Style {
    Style::new(css!(
        r#"
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100%;
            font-size: 16px;
            color: #666;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_page() -> Style {
    Style::new(css!(
        r#"
            font-family: Arial, sans-serif;
            background-color: #fff;
            margin: 0;
            padding: 0;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_page_loading() -> Style {
    Style::new(css!(
        r#"
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            font-size: 18px;
            color: #666;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_page_error() -> Style {
    Style::new(css!(
        r#"
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            font-size: 18px;
            color: #f44336;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_preview_container() -> Style {
    Style::new(css!(
        r#"
            margin: 20px 0;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_preview_container_h3() -> Style {
    Style::new(css!(
        r#"
            color: #555;
            margin-bottom: 15px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_preview() -> Style {
    Style::new(css!(
        r#"
            background-color: #f9f9f9;
            border: 1px solid #ddd;
            border-radius: 4px;
            overflow: hidden;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_preview_header() -> Style {
    Style::new(css!(
        r#"
            background-color: #f0f0f0;
            padding: 10px 15px;
            border-bottom: 1px solid #ddd;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_preview_header_h4() -> Style {
    Style::new(css!(
        r#"
            margin: 0;
            color: #333;
            font-size: 16px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_preview_content() -> Style {
    Style::new(css!(
        r#"
            flex: 1;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            padding: 20px;
            text-align: center;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_preview_content_p() -> Style {
    Style::new(css!(
        r#"
            margin: 10px 0;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_preview_footer() -> Style {
    Style::new(css!(
        r#"
            background-color: #f0f0f0;
            padding: 8px 15px;
            border-top: 1px solid #ddd;
            text-align: right;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_config_info() -> Style {
    Style::new(css!(
        r#"
            margin-top: 15px;
            padding: 15px;
            background-color: #f5f5f5;
            border-radius: 4px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_config_info_p() -> Style {
    Style::new(css!(
        r#"
            margin-top: 0;
            font-weight: bold;
            color: #555;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_config_info_ul() -> Style {
    Style::new(css!(
        r#"
            margin: 10px 0;
            padding-left: 20px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_config_info_li() -> Style {
    Style::new(css!(
        r#"
            margin: 5px 0;
        "#
    ))
    .expect("Failed to create style")
}

// Collaboration-specific styles
pub fn annotation_item_collab() -> Style {
    Style::new(css!(
        r#"
            border: 1px solid #e0e0e0;
            border-radius: 4px;
            padding: 12px;
            margin-bottom: 12px;
            background-color: #fafafa;
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotation_content_collab() -> Style {
    Style::new(css!(
        r#"
            margin-bottom: 8px;
            line-height: 1.4;
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotation_content_mark() -> Style {
    Style::new(css!(
        r#"
            background-color: #fff9c4;
            padding: 2px 4px;
            border-radius: 3px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotation_meta_collab() -> Style {
    Style::new(css!(
        r#"
            display: flex;
            flex-wrap: wrap;
            gap: 12px;
            font-size: 0.85em;
            color: #666;
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotation_user_collab() -> Style {
    Style::new(css!(
        r#"
            font-weight: bold;
            color: #333;
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotation_mentions() -> Style {
    Style::new(css!(
        r#"
            display: flex;
            align-items: center;
            gap: 4px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn mention_tag() -> Style {
    Style::new(css!(
        r#"
            background-color: #e3f2fd;
            color: #1976d2;
            padding: 2px 6px;
            border-radius: 12px;
            font-size: 0.8em;
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotation_actions() -> Style {
    Style::new(css!(
        r#"
            margin-top: 8px;
            display: flex;
            gap: 8px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn edit_annotation_btn() -> Style {
    Style::new(css!(
        r#"
            padding: 4px 8px;
            font-size: 0.8em;
            border: 1px solid #ccc;
            border-radius: 3px;
            background-color: white;
            cursor: pointer;
        "#
    ))
    .expect("Failed to create style")
}

pub fn edit_annotation_btn_hover() -> Style {
    Style::new(css!(
        r#"
            background-color: #f5f5f5;
        "#
    ))
    .expect("Failed to create style")
}

pub fn delete_annotation_btn() -> Style {
    Style::new(css!(
        r#"
            padding: 4px 8px;
            font-size: 0.8em;
            border: 1px solid #ccc;
            border-radius: 3px;
            background-color: white;
            cursor: pointer;
        "#
    ))
    .expect("Failed to create style")
}

pub fn delete_annotation_btn_hover() -> Style {
    Style::new(css!(
        r#"
            background-color: #ffebee;
            color: #c62828;
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotation_form_collab() -> Style {
    Style::new(css!(
        r#"
            background-color: white;
            border: 1px solid #ddd;
            border-radius: 4px;
            padding: 16px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
            margin-bottom: 16px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotation_form_positioned_collab() -> Style {
    Style::new(css!(
        r#"
            position: absolute;
            z-index: 1000;
            max-width: 300px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotation_form_textarea_collab() -> Style {
    Style::new(css!(
        r#"
            width: 100%;
            padding: 8px;
            border: 1px solid #ddd;
            border-radius: 3px;
            resize: vertical;
            font-family: inherit;
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotation_form_actions_collab() -> Style {
    Style::new(css!(
        r#"
            display: flex;
            justify-content: flex-end;
            gap: 8px;
            margin-top: 8px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn submit_annotation_btn_disabled_collab() -> Style {
    Style::new(css!(
        r#"
            opacity: 0.5;
            cursor: not-allowed;
        "#
    ))
    .expect("Failed to create style")
}