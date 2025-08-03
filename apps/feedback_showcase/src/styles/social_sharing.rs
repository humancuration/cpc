use stylist::{css, Style};

pub fn visualization_header() -> Style {
    Style::new(css!(
        r#"
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 15px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn share_button_group() -> Style {
    Style::new(css!(
        r#"
            display: flex;
            gap: 5px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn share_btn() -> Style {
    Style::new(css!(
        r#"
            background-color: #f0f0f0;
            border: 1px solid #ddd;
            border-radius: 4px;
            padding: 5px 10px;
            cursor: pointer;
            font-size: 16px;
            transition: background-color 0.2s;
        "#
    ))
    .expect("Failed to create style")
}

pub fn share_btn_hover() -> Style {
    Style::new(css!(
        r#"
            background-color: #e0e0e0;
        "#
    ))
    .expect("Failed to create style")
}

pub fn share_btn_federation() -> Style {
    Style::new(css!(
        r#"
            color: #1da1f2;
        "#
    ))
    .expect("Failed to create style")
}

pub fn share_btn_embed() -> Style {
    Style::new(css!(
        r#"
            color: #ff6b6b;
        "#
    ))
    .expect("Failed to create style")
}

pub fn share_btn_image() -> Style {
    Style::new(css!(
        r#"
            color: #4caf50;
        "#
    ))
    .expect("Failed to create style")
}

pub fn share_btn_social() -> Style {
    Style::new(css!(
        r#"
            color: #9c27b0;
        "#
    ))
    .expect("Failed to create style")
}

pub fn social_sharing_dialog_overlay() -> Style {
    Style::new(css!(
        r#"
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background-color: rgba(0, 0, 0, 0.5);
            display: flex;
            justify-content: center;
            align-items: center;
            z-index: 1000;
        "#
    ))
    .expect("Failed to create style")
}

pub fn social_sharing_dialog() -> Style {
    Style::new(css!(
        r#"
            background-color: white;
            border-radius: 8px;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
            width: 90%;
            max-width: 500px;
            max-height: 90vh;
            overflow-y: auto;
            outline: none;
        "#
    ))
    .expect("Failed to create style")
}

pub fn dialog_header() -> Style {
    Style::new(css!(
        r#"
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 20px;
            border-bottom: 1px solid #eee;
        "#
    ))
    .expect("Failed to create style")
}

pub fn dialog_header_h2() -> Style {
    Style::new(css!(
        r#"
            margin: 0;
            color: #333;
        "#
    ))
    .expect("Failed to create style")
}

pub fn close_button() -> Style {
    Style::new(css!(
        r#"
            background: none;
            border: none;
            font-size: 24px;
            cursor: pointer;
            color: #999;
            padding: 0;
            width: 30px;
            height: 30px;
            display: flex;
            justify-content: center;
            align-items: center;
        "#
    ))
    .expect("Failed to create style")
}

pub fn close_button_hover() -> Style {
    Style::new(css!(
        r#"
            color: #333;
        "#
    ))
    .expect("Failed to create style")
}

pub fn dialog_content() -> Style {
    Style::new(css!(
        r#"
            padding: 20px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn platform_selector() -> Style {
    Style::new(css!(
        r#"
            margin-bottom: 20px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn platform_selector_label() -> Style {
    Style::new(css!(
        r#"
            display: block;
            margin-bottom: 8px;
            font-weight: bold;
            color: #555;
        "#
    ))
    .expect("Failed to create style")
}

pub fn platform_selector_select() -> Style {
    Style::new(css!(
        r#"
            width: 100%;
            padding: 10px;
            border: 1px solid #ddd;
            border-radius: 4px;
            font-size: 16px;
            background-color: white;
        "#
    ))
    .expect("Failed to create style")
}

pub fn message_editor() -> Style {
    Style::new(css!(
        r#"
            margin-bottom: 20px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn message_editor_label() -> Style {
    Style::new(css!(
        r#"
            display: block;
            margin-bottom: 8px;
            font-weight: bold;
            color: #555;
        "#
    ))
    .expect("Failed to create style")
}

pub fn message_editor_textarea() -> Style {
    Style::new(css!(
        r#"
            width: 100%;
            padding: 10px;
            border: 1px solid #ddd;
            border-radius: 4px;
            font-family: Arial, sans-serif;
            font-size: 14px;
            resize: vertical;
        "#
    ))
    .expect("Failed to create style")
}

pub fn preview_panel() -> Style {
    Style::new(css!(
        r#"
            background-color: #f9f9f9;
            border-radius: 4px;
            padding: 15px;
            margin-bottom: 20px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn preview_panel_h3() -> Style {
    Style::new(css!(
        r#"
            margin-top: 0;
            color: #555;
        "#
    ))
    .expect("Failed to create style")
}

pub fn preview_content() -> Style {
    Style::new(css!(
        r#"
            font-size: 14px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn preview_message() -> Style {
    Style::new(css!(
        r#"
            background-color: #e3f2fd;
            padding: 10px;
            border-radius: 4px;
            margin: 10px 0;
            word-wrap: break-word;
        "#
    ))
    .expect("Failed to create style")
}

pub fn preview_alt_text() -> Style {
    Style::new(css!(
        r#"
            font-size: 12px;
            color: #666;
            font-style: italic;
        "#
    ))
    .expect("Failed to create style")
}

pub fn dialog_footer() -> Style {
    Style::new(css!(
        r#"
            display: flex;
            justify-content: flex-end;
            padding: 20px;
            border-top: 1px solid #eee;
            gap: 10px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn cancel_button() -> Style {
    Style::new(css!(
        r#"
            background-color: #f5f5f5;
            color: #333;
            padding: 10px 20px;
            border: none;
            border-radius: 4px;
            font-size: 16px;
            cursor: pointer;
        "#
    ))
    .expect("Failed to create style")
}

pub fn cancel_button_hover() -> Style {
    Style::new(css!(
        r#"
            background-color: #e0e0e0;
        "#
    ))
    .expect("Failed to create style")
}

pub fn share_button() -> Style {
    Style::new(css!(
        r#"
            background-color: #1da1f2;
            color: white;
            padding: 10px 20px;
            border: none;
            border-radius: 4px;
            font-size: 16px;
            cursor: pointer;
        "#
    ))
    .expect("Failed to create style")
}

pub fn share_button_hover_not_disabled() -> Style {
    Style::new(css!(
        r#"
            background-color: #0d8bd9;
        "#
    ))
    .expect("Failed to create style")
}

pub fn share_button_disabled() -> Style {
    Style::new(css!(
        r#"
            background-color: #cccccc;
            cursor: not-allowed;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_code_dialog_overlay() -> Style {
    Style::new(css!(
        r#"
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background-color: rgba(0, 0, 0, 0.5);
            display: flex;
            justify-content: center;
            align-items: center;
            z-index: 1000;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_code_dialog() -> Style {
    Style::new(css!(
        r#"
            background-color: white;
            border-radius: 8px;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
            width: 90%;
            max-width: 700px;
            max-height: 90vh;
            overflow-y: auto;
            outline: none;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_customization() -> Style {
    Style::new(css!(
        r#"
            background-color: #f9f9f9;
            border-radius: 4px;
            padding: 15px;
            margin-bottom: 20px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_customization_form_group() -> Style {
    Style::new(css!(
        r#"
            margin-bottom: 15px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_customization_form_group_label() -> Style {
    Style::new(css!(
        r#"
            display: block;
            margin-bottom: 5px;
            font-weight: bold;
            color: #555;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_customization_form_group_input_number() -> Style {
    Style::new(css!(
        r#"
            width: 100%;
            padding: 8px;
            border: 1px solid #ddd;
            border-radius: 4px;
            box-sizing: border-box;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_customization_form_group_select() -> Style {
    Style::new(css!(
        r#"
            width: 100%;
            padding: 8px;
            border: 1px solid #ddd;
            border-radius: 4px;
            box-sizing: border-box;
            background-color: white;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_customization_checkbox_group() -> Style {
    Style::new(css!(
        r#"
            display: flex;
            align-items: center;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_customization_checkbox_group_label() -> Style {
    Style::new(css!(
        r#"
            font-weight: normal;
            display: flex;
            align-items: center;
            cursor: pointer;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_customization_checkbox_group_input_checkbox() -> Style {
    Style::new(css!(
        r#"
            margin-right: 8px;
            width: auto;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_code_section() -> Style {
    Style::new(css!(
        r#"
            margin-top: 20px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_code_section_label() -> Style {
    Style::new(css!(
        r#"
            display: block;
            margin-bottom: 8px;
            font-weight: bold;
            color: #555;
        "#
    ))
    .expect("Failed to create style")
}

pub fn embed_code_section_textarea() -> Style {
    Style::new(css!(
        r#"
            width: 100%;
            padding: 10px;
            border: 1px solid #ddd;
            border-radius: 4px;
            font-family: monospace;
            font-size: 12px;
            resize: vertical;
            margin-bottom: 10px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn copy_button() -> Style {
    Style::new(css!(
        r#"
            background-color: #4CAF50;
            color: white;
            border: none;
            padding: 10px 15px;
            border-radius: 4px;
            cursor: pointer;
            font-size: 14px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn copy_button_hover_not_disabled() -> Style {
    Style::new(css!(
        r#"
            background-color: #45a049;
        "#
    ))
    .expect("Failed to create style")
}

pub fn copy_button_disabled() -> Style {
    Style::new(css!(
        r#"
            background-color: #cccccc;
            cursor: not-allowed;
        "#
    ))
    .expect("Failed to create style")
}

pub fn close_button_bottom() -> Style {
    Style::new(css!(
        r#"
            background-color: #f5f5f5;
            color: #333;
            border: none;
            padding: 10px 20px;
            border-radius: 4px;
            cursor: pointer;
            font-size: 16px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn close_button_bottom_hover() -> Style {
    Style::new(css!(
        r#"
            background-color: #e0e0e0;
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotation_manager() -> Style {
    Style::new(css!(
        r#"
            margin: 20px 0;
            border-top: 1px solid #eee;
            padding-top: 20px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotation_header() -> Style {
    Style::new(css!(
        r#"
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 15px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotation_header_h3() -> Style {
    Style::new(css!(
        r#"
            margin: 0;
            color: #555;
        "#
    ))
    .expect("Failed to create style")
}

pub fn add_annotation_btn() -> Style {
    Style::new(css!(
        r#"
            background-color: #9c27b0;
            color: white;
            border: none;
            padding: 8px 15px;
            border-radius: 4px;
            cursor: pointer;
            font-size: 14px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn add_annotation_btn_hover() -> Style {
    Style::new(css!(
        r#"
            background-color: #7b1fa2;
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotation_form() -> Style {
    Style::new(css!(
        r#"
            background-color: #f9f9f9;
            border-radius: 4px;
            padding: 15px;
            margin-bottom: 20px;
            position: relative;
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotation_form_positioned() -> Style {
    Style::new(css!(
        r#"
            position: absolute;
            z-index: 100;
            width: 300px;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotation_form_textarea() -> Style {
    Style::new(css!(
        r#"
            width: 100%;
            padding: 10px;
            border: 1px solid #ddd;
            border-radius: 4px;
            font-family: Arial, sans-serif;
            font-size: 14px;
            resize: vertical;
            margin-bottom: 10px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotation_form_actions() -> Style {
    Style::new(css!(
        r#"
            display: flex;
            justify-content: flex-end;
            gap: 10px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn cancel_annotation_btn() -> Style {
    Style::new(css!(
        r#"
            background-color: #f5f5f5;
            color: #333;
            padding: 8px 15px;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            font-size: 14px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn cancel_annotation_btn_hover() -> Style {
    Style::new(css!(
        r#"
            background-color: #e0e0e0;
        "#
    ))
    .expect("Failed to create style")
}

pub fn submit_annotation_btn() -> Style {
    Style::new(css!(
        r#"
            background-color: #9c27b0;
            color: white;
            padding: 8px 15px;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            font-size: 14px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn submit_annotation_btn_hover_not_disabled() -> Style {
    Style::new(css!(
        r#"
            background-color: #7b1fa2;
        "#
    ))
    .expect("Failed to create style")
}

pub fn submit_annotation_btn_disabled() -> Style {
    Style::new(css!(
        r#"
            background-color: #cccccc;
            cursor: not-allowed;
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotations_list() -> Style {
    Style::new(css!(
        r#"
            max-height: 300px;
            overflow-y: auto;
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotation_item() -> Style {
    Style::new(css!(
        r#"
            background-color: #f5f5f5;
            border-radius: 4px;
            padding: 15px;
            margin-bottom: 15px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotation_content() -> Style {
    Style::new(css!(
        r#"
            margin-bottom: 10px;
            word-wrap: break-word;
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotation_meta() -> Style {
    Style::new(css!(
        r#"
            display: flex;
            justify-content: space-between;
            font-size: 12px;
            color: #666;
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotation_user() -> Style {
    Style::new(css!(
        r#"
            font-weight: bold;
        "#
    ))
    .expect("Failed to create style")
}

pub fn no_annotations() -> Style {
    Style::new(css!(
        r#"
            text-align: center;
            color: #999;
            font-style: italic;
            padding: 20px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn annotation_position_indicator() -> Style {
    Style::new(css!(
        r#"
            position: absolute;
            width: 12px;
            height: 12px;
            background-color: #9c27b0;
            border-radius: 50%;
            transform: translate(-50%, -50%);
            z-index: 99;
            pointer-events: none;
        "#
    ))
    .expect("Failed to create style")
}