use stylist::{css, Style};

pub fn visualization_summary() -> Style {
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

pub fn visualization_summary_h2() -> Style {
    Style::new(css!(
        r#"
            margin-top: 0;
            color: #333;
        "#
    ))
    .expect("Failed to create style")
}

pub fn summary_metrics() -> Style {
    Style::new(css!(
        r#"
            display: flex;
            justify-content: space-between;
            flex-wrap: wrap;
        "#
    ))
    .expect("Failed to create style")
}

pub fn metric_card() -> Style {
    Style::new(css!(
        r#"
            background-color: white;
            border-radius: 8px;
            padding: 20px;
            margin: 10px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
            flex: 1;
            min-width: 200px;
            text-align: center;
        "#
    ))
    .expect("Failed to create style")
}

pub fn metric_card_h3() -> Style {
    Style::new(css!(
        r#"
            margin-top: 0;
            color: #555;
        "#
    ))
    .expect("Failed to create style")
}

pub fn rating_display() -> Style {
    Style::new(css!(
        r#"
            font-size: 24px;
            font-weight: bold;
            margin: 10px 0;
        "#
    ))
    .expect("Failed to create style")
}

pub fn rating_value() -> Style {
    Style::new(css!(
        r#"
            color: #4CAF50;
        "#
    ))
    .expect("Failed to create style")
}

pub fn rating_max() -> Style {
    Style::new(css!(
        r#"
            color: #999;
            font-size: 16px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn stars() -> Style {
    Style::new(css!(
        r#"
            margin: 10px 0;
        "#
    ))
    .expect("Failed to create style")
}

pub fn star() -> Style {
    Style::new(css!(
        r#"
            font-size: 24px;
            color: #ddd;
        "#
    ))
    .expect("Failed to create style")
}

pub fn star_full() -> Style {
    Style::new(css!(
        r#"
            color: #FFD700;
        "#
    ))
    .expect("Failed to create style")
}

pub fn star_half() -> Style {
    Style::new(css!(
        r#"
            color: #FFD700;
        "#
    ))
    .expect("Failed to create style")
}

pub fn review_count() -> Style {
    Style::new(css!(
        r#"
            font-size: 36px;
            font-weight: bold;
            color: #2196F3;
        "#
    ))
    .expect("Failed to create style")
}

pub fn visualization_ratings_chart() -> Style {
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

pub fn visualization_word_cloud() -> Style {
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

pub fn visualization_sentiment() -> Style {
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

pub fn visualization_ratings_chart_h2() -> Style {
    Style::new(css!(
        r#"
            margin-top: 0;
            color: #333;
        "#
    ))
    .expect("Failed to create style")
}

pub fn visualization_word_cloud_h2() -> Style {
    Style::new(css!(
        r#"
            margin-top: 0;
            color: #333;
        "#
    ))
    .expect("Failed to create style")
}

pub fn visualization_sentiment_h2() -> Style {
    Style::new(css!(
        r#"
            margin-top: 0;
            color: #333;
        "#
    ))
    .expect("Failed to create style")
}

pub fn visualization_sentiment_center() -> Style {
    Style::new(css!(
        r#"
            text-align: center;
        "#
    ))
    .expect("Failed to create style")
}

pub fn sentiment_legend() -> Style {
    Style::new(css!(
        r#"
            display: flex;
            justify-content: center;
            margin-top: 20px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn legend_item() -> Style {
    Style::new(css!(
        r#"
            display: flex;
            align-items: center;
            margin: 0 15px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn legend_color() -> Style {
    Style::new(css!(
        r#"
            width: 20px;
            height: 20px;
            margin-right: 8px;
            border-radius: 3px;
        "#
    ))
    .expect("Failed to create style")
}

pub fn legend_color_positive() -> Style {
    Style::new(css!(
        r#"
            background-color: green;
        "#
    ))
    .expect("Failed to create style")
}

pub fn legend_color_neutral() -> Style {
    Style::new(css!(
        r#"
            background-color: yellow;
        "#
    ))
    .expect("Failed to create style")
}

pub fn legend_color_negative() -> Style {
    Style::new(css!(
        r#"
            background-color: red;
        "#
    ))
    .expect("Failed to create style")
}