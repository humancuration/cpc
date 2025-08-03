use stylist::{css, Style};

pub fn data_generator_ui() -> Style {
    Style::new(css!(
        r#"
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            font-family: Arial, sans-serif;
        "#
    ))
    .expect("Failed to create style")
}

pub fn data_generator_ui_h1() -> Style {
    Style::new(css!(
        r#"
            text-align: center;
            color: #333;
            margin-bottom: 30px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn config_panel() -> Style {
    Style::new(css!(
        r#"
            background-color: #f5f5f5;
            border-radius: 8px;
            padding: 20px;
            margin-bottom: 20px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn config_panel_h2() -> Style {
    Style::new(css!(
        r#"
            margin-top: 0;
            color: #444;
        "#
    ))
    .expect("Failed to create style")
}

pub fn config_section() -> Style {
    Style::new(css!(
        r#"
            margin-bottom: 20px;
            padding: 15px;
            background-color: white;
            border-radius: 5px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        "#
    ))
    .expect("Failed to create style")
}

pub fn config_section_h3() -> Style {
    Style::new(css!(
        r#"
            margin-top: 0;
            color: #555;
            display: flex;
            justify-content: space-between;
            align-items: center;
        "#
    ))
    .expect("Failed to create style")
}

pub fn form_group() -> Style {
    Style::new(css!(
        r#"
            margin-bottom: 15px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn form_group_label() -> Style {
    Style::new(css!(
        r#"
            display: block;
            margin-bottom: 5px;
            font-weight: bold;
            color: #666;
        "#
    ))
    .expect("Failed to create style")
}

pub fn form_group_input_number() -> Style {
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

pub fn form_group_input_text() -> Style {
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

pub fn form_group_input_range() -> Style {
    Style::new(css!(
        r#"
            width: 70%;
            vertical-align: middle;
        "#
    ))
    .expect("Failed to create style")
}

pub fn form_group_span() -> Style {
    Style::new(css!(
        r#"
            display: inline-block;
            width: 25%;
            text-align: right;
            vertical-align: middle;
        "#
    ))
    .expect("Failed to create style")
}

pub fn product_type_config() -> Style {
    Style::new(css!(
        r#"
            background-color: #e9e9e9;
            padding: 15px;
            margin-bottom: 15px;
            border-radius: 5px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn product_type_config_button() -> Style {
    Style::new(css!(
        r#"
            background-color: #ff6b6b;
            color: white;
            border: none;
            padding: 5px 10px;
            border-radius: 3px;
            cursor: pointer;
        "#
    ))
    .expect("Failed to create style")
}

pub fn product_type_config_button_hover() -> Style {
    Style::new(css!(
        r#"
            background-color: #ff5252;
        "#
    ))
    .expect("Failed to create style")
}

pub fn action_bar() -> Style {
    Style::new(css!(
        r#"
            text-align: center;
            margin-bottom: 20px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn action_bar_button() -> Style {
    Style::new(css!(
        r#"
            padding: 12px 20px;
            margin: 0 10px;
            border: none;
            border-radius: 5px;
            font-size: 16px;
            cursor: pointer;
        "#
    ))
    .expect("Failed to create style")
}

pub fn generate_btn() -> Style {
    Style::new(css!(
        r#"
            background-color: #4CAF50;
            color: white;
        "#
    ))
    .expect("Failed to create style")
}

pub fn generate_btn_hover_not_disabled() -> Style {
    Style::new(css!(
        r#"
            background-color: #45a049;
        "#
    ))
    .expect("Failed to create style")
}

pub fn generate_btn_disabled() -> Style {
    Style::new(css!(
        r#"
            background-color: #cccccc;
            cursor: not-allowed;
        "#
    ))
    .expect("Failed to create style")
}

pub fn export_btn() -> Style {
    Style::new(css!(
        r#"
            background-color: #2196F3;
            color: white;
        "#
    ))
    .expect("Failed to create style")
}

pub fn export_btn_hover_not_disabled() -> Style {
    Style::new(css!(
        r#"
            background-color: #1976D2;
        "#
    ))
    .expect("Failed to create style")
}

pub fn export_btn_disabled() -> Style {
    Style::new(css!(
        r#"
            background-color: #cccccc;
            cursor: not-allowed;
        "#
    ))
    .expect("Failed to create style")
}

pub fn reset_btn() -> Style {
    Style::new(css!(
        r#"
            background-color: #f44336;
            color: white;
        "#
    ))
    .expect("Failed to create style")
}

pub fn reset_btn_hover() -> Style {
    Style::new(css!(
        r#"
            background-color: #d32f2f;
        "#
    ))
    .expect("Failed to create style")
}

pub fn action_buttons() -> Style {
    Style::new(css!(
        r#"
            display: flex;
            justify-content: center;
            align-items: center;
            gap: 20px;
            margin-bottom: 20px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn preview_button() -> Style {
    Style::new(css!(
        r#"
            padding: 12px 20px;
            border: none;
            border-radius: 5px;
            font-size: 16px;
            cursor: pointer;
            background-color: #9c27b0;
            color: white;
        "#
    ))
    .expect("Failed to create style")
}

pub fn preview_button_hover() -> Style {
    Style::new(css!(
        r#"
            background-color: #7b1fa2;
        "#
    ))
    .expect("Failed to create style")
}

pub fn metrics_panel() -> Style {
    Style::new(css!(
        r#"
            background-color: #e3f2fd;
            border-radius: 8px;
            padding: 20px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn metrics_panel_h2() -> Style {
    Style::new(css!(
        r#"
            margin-top: 0;
            color: #1976D2;
        "#
    ))
    .expect("Failed to create style")
}

pub fn metric() -> Style {
    Style::new(css!(
        r#"
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 15px;
            padding: 10px;
            background-color: white;
            border-radius: 5px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn metric_label() -> Style {
    Style::new(css!(
        r#"
            font-weight: bold;
            color: #333;
        "#
    ))
    .expect("Failed to create style")
}

pub fn metric_progress() -> Style {
    Style::new(css!(
        r#"
            width: 60%;
            height: 20px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn metric_span() -> Style {
    Style::new(css!(
        r#"
            width: 20%;
            text-align: right;
            font-weight: bold;
        "#
    ))
    .expect("Failed to create style")
}

pub fn demo_control_panel() -> Style {
    Style::new(css!(
        r#"
            background-color: #f5f5f5;
            border-radius: 8px;
            padding: 20px;
            margin-bottom: 20px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn demo_control_panel_h2() -> Style {
    Style::new(css!(
        r#"
            margin-top: 0;
            color: #444;
        "#
    ))
    .expect("Failed to create style")
}

pub fn control_section() -> Style {
    Style::new(css!(
        r#"
            margin-bottom: 20px;
            padding: 15px;
            background-color: white;
            border-radius: 5px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        "#
    ))
    .expect("Failed to create style")
}

pub fn control_section_h3() -> Style {
    Style::new(css!(
        r#"
            margin-top: 0;
            color: #555;
        "#
    ))
    .expect("Failed to create style")
}

pub fn slider_group() -> Style {
    Style::new(css!(
        r#"
            margin-bottom: 15px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn slider_group_label() -> Style {
    Style::new(css!(
        r#"
            display: block;
            margin-bottom: 5px;
            font-weight: bold;
            color: #666;
        "#
    ))
    .expect("Failed to create style")
}

pub fn slider_group_input_range() -> Style {
    Style::new(css!(
        r#"
            width: 70%;
            vertical-align: middle;
        "#
    ))
    .expect("Failed to create style")
}

pub fn slider_group_span() -> Style {
    Style::new(css!(
        r#"
            display: inline-block;
            width: 25%;
            text-align: right;
            vertical-align: middle;
        "#
    ))
    .expect("Failed to create style")
}

pub fn preset_selector() -> Style {
    Style::new(css!(
        r#"
            background-color: #f5f5f5;
            border-radius: 8px;
            padding: 20px;
            margin-bottom: 20px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn preset_selector_h3() -> Style {
    Style::new(css!(
        r#"
            margin-top: 0;
            color: #444;
        "#
    ))
    .expect("Failed to create style")
}

pub fn preset_buttons() -> Style {
    Style::new(css!(
        r#"
            display: flex;
            flex-wrap: wrap;
            gap: 10px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn preset_btn() -> Style {
    Style::new(css!(
        r#"
            background-color: #673ab7;
            color: white;
            border: none;
            padding: 10px 15px;
            border-radius: 5px;
            cursor: pointer;
            flex: 1;
            min-width: 120px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn preset_btn_hover() -> Style {
    Style::new(css!(
        r#"
            background-color: #5e35b1;
        "#
    ))
    .expect("Failed to create style")
}

pub fn playground() -> Style {
    Style::new(css!(
        r#"
            background-color: #f9f9f9;
            border-radius: 8px;
            padding: 20px;
            margin-bottom: 20px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn playground_h2() -> Style {
    Style::new(css!(
        r#"
            margin-top: 0;
            color: #333;
        "#
    ))
    .expect("Failed to create style")
}

pub fn playground_grid() -> Style {
    Style::new(css!(
        r#"
            display: grid;
            gap: 20px;
            margin-top: 20px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn playground_grid_cols_1() -> Style {
    Style::new(css!(
        r#"
            grid-template-columns: 1fr;
        "#
    ))
    .expect("Failed to create style")
}

pub fn playground_grid_cols_2() -> Style {
    Style::new(css!(
        r#"
            grid-template-columns: 1fr 1fr;
        "#
    ))
    .expect("Failed to create style")
}

pub fn playground_grid_cols_3() -> Style {
    Style::new(css!(
        r#"
            grid-template-columns: 1fr 1fr 1fr;
        "#
    ))
    .expect("Failed to create style")
}

pub fn playground_item() -> Style {
    Style::new(css!(
        r#"
            background-color: white;
            border-radius: 8px;
            padding: 15px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
        "#
    ))
    .expect("Failed to create style")
}

pub fn component_header() -> Style {
    Style::new(css!(
        r#"
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 15px;
            padding-bottom: 10px;
            border-bottom: 1px solid #eee;
        "#
    ))
    .expect("Failed to create style")
}

pub fn component_header_h3() -> Style {
    Style::new(css!(
        r#"
            margin: 0;
            color: #555;
        "#
    ))
    .expect("Failed to create style")
}

pub fn toggle_btn() -> Style {
    Style::new(css!(
        r#"
            background-color: #ff9800;
            color: white;
            border: none;
            padding: 5px 10px;
            border-radius: 3px;
            cursor: pointer;
            font-size: 12px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn toggle_btn_hover() -> Style {
    Style::new(css!(
        r#"
            background-color: #f57c00;
        "#
    ))
    .expect("Failed to create style")
}

pub fn placeholder() -> Style {
    Style::new(css!(
        r#"
            text-align: center;
            padding: 40px;
            color: #999;
        "#
    ))
    .expect("Failed to create style")
}

pub fn placeholder_p() -> Style {
    Style::new(css!(
        r#"
            font-size: 18px;
            margin: 0;
        "#
    ))
    .expect("Failed to create style")
}