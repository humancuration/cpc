use yew::prelude::*;
use stylist::{style, yew::styled_component};

#[derive(Properties, PartialEq)]
pub struct MetricsCardProps {
    pub title: String,
    pub value: String,
    pub description: String,
    #[prop_or_default]
    pub trend: Option<String>,
    #[prop_or_default]
    pub color: Option<String>,
}

#[styled_component(MetricsCard)]
pub fn metrics_card(props: &MetricsCardProps) -> Html {
    let card_style = style!(
        r#"
        .metrics-card {
            background: var(--surface);
            border-radius: 8px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
            padding: 20px;
            height: 100%;
            transition: transform 0.2s, box-shadow 0.2s;
        }
        
        .metrics-card:hover {
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(0,0,0,0.15);
        }
        
        .card-title {
            font-size: 1rem;
            color: var(--text-secondary);
            margin: 0 0 10px 0;
        }
        
        .card-value {
            font-size: 2rem;
            font-weight: 700;
            margin: 0 0 10px 0;
            color: var(--primary);
        }
        
        .card-value.blue {
            color: var(--secondary);
        }
        
        .card-value.green {
            color: var(--success);
        }
        
        .card-value.purple {
            color: #9b59b6;
        }
        
        .card-value.orange {
            color: var(--warning);
        }
        
        .card-value.red {
            color: var(--accent);
        }
        
        .card-description {
            font-size: 0.9rem;
            color: var(--text-secondary);
            margin: 0 0 15px 0;
        }
        
        .card-trend {
            font-size: 0.85rem;
            font-weight: 600;
            padding: 4px 8px;
            border-radius: 12px;
            display: inline-block;
        }
        
        .card-trend.positive {
            background-color: rgba(46, 204, 113, 0.1);
            color: var(--success);
        }
        
        .card-trend.negative {
            background-color: rgba(231, 76, 60, 0.1);
            color: var(--accent);
        }
    "#
    ).unwrap();

    let value_color_class = match props.color.as_deref() {
        Some("blue") => "blue",
        Some("green") => "green",
        Some("purple") => "purple",
        Some("orange") => "orange",
        Some("red") => "red",
        _ => "",
    };

    let trend_class = if let Some(trend) = &props.trend {
        if trend.starts_with('+') {
            "card-trend positive"
        } else if trend.starts_with('-') {
            "card-trend negative"
        } else {
            "card-trend"
        }
    } else {
        "card-trend"
    };

    html! {
        <div class={card_style}>
            <h3 class="card-title">{ &props.title }</h3>
            <div class={format!("card-value {}", value_color_class)}>{ &props.value }</div>
            <p class="card-description">{ &props.description }</p>
            if let Some(trend) = &props.trend {
                <div class={trend_class}>{ trend }</div>
            }
        </div>
    }
}