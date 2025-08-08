use yew::prelude::*;
use stylist::{style, yew::styled_component};

#[derive(Properties, PartialEq)]
pub struct MetricsCardProps {
    pub title: String,
    pub value: String,
    pub description: String,
    pub trend: Option<String>, // e.g., "+12% from last month"
    pub color: Option<String>, // e.g., "green", "red", "blue"
}

#[styled_component(MetricsCard)]
pub fn metrics_card(props: &MetricsCardProps) -> Html {
    let card_style = style!(
        r#"
        .metrics-card {
            background: white;
            border-radius: 8px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
            padding: 20px;
            margin-bottom: 20px;
            transition: transform 0.2s, box-shadow 0.2s;
        }
        
        .metrics-card:hover {
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(0,0,0,0.15);
        }
        
        .card-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 10px;
        }
        
        .card-title {
            font-size: 1rem;
            color: #7f8c8d;
            margin: 0;
        }
        
        .card-value {
            font-size: 2rem;
            font-weight: bold;
            margin: 10px 0;
            color: #2c3e50;
        }
        
        .card-description {
            color: #95a5a6;
            margin: 0;
            font-size: 0.9rem;
        }
        
        .card-trend {
            font-size: 0.8rem;
            padding: 4px 8px;
            border-radius: 4px;
            font-weight: bold;
        }
        
        .trend-positive {
            background-color: #e8f5e9;
            color: #2e7d32;
        }
        
        .trend-negative {
            background-color: #ffebee;
            color: #c62828;
        }
    "#
    ).unwrap();

    let trend_class = match props.trend.as_deref() {
        Some(t) if t.starts_with('+') => "card-trend trend-positive",
        Some(t) if t.starts_with('-') => "card-trend trend-negative",
        _ => "card-trend",
    };

    html! {
        <div class={card_style}>
            <div class="card-header">
                <h3 class="card-title">{&props.title}</h3>
                if let Some(trend) = &props.trend {
                    <span class={trend_class}>{trend}</span>
                }
            </div>
            <div class="card-value">{&props.value}</div>
            <p class="card-description">{&props.description}</p>
        </div>
    }
}