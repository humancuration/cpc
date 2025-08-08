use yew::prelude::*;
use stylist::{style, yew::styled_component};

#[derive(Properties, PartialEq)]
pub struct ChartProps {
    pub title: String,
    pub data: Vec<ChartDataPoint>,
    pub chart_type: ChartType,
    pub height: Option<String>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ChartDataPoint {
    pub label: String,
    pub value: f64,
    pub color: Option<String>,
}

#[derive(PartialEq, Clone)]
pub enum ChartType {
    Bar,
    Line,
    Pie,
}

#[styled_component(Chart)]
pub fn chart(props: &ChartProps) -> Html {
    let chart_style = style!(
        r#"
        .chart-container {
            background: white;
            border-radius: 8px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
            padding: 20px;
            margin-bottom: 20px;
        }
        
        .chart-header {
            margin-bottom: 20px;
        }
        
        .chart-title {
            font-size: 1.2rem;
            color: #2c3e50;
            margin: 0 0 10px 0;
        }
        
        .chart-description {
            color: #7f8c8d;
            margin: 0;
            font-size: 0.9rem;
        }
        
        .chart-content {
            height: 300px;
            display: flex;
            align-items: center;
            justify-content: center;
            background-color: #f8f9fa;
            border-radius: 4px;
            position: relative;
        }
        
        .chart-placeholder {
            text-align: center;
            color: #95a5a6;
        }
        
        .chart-icon {
            font-size: 3rem;
            margin-bottom: 10px;
        }
    "#
    ).unwrap();

    let height = props.height.clone().unwrap_or("300px".to_string());

    html! {
        <div class={chart_style}>
            <div class="chart-header">
                <h3 class="chart-title">{&props.title}</h3>
                <p class="chart-description">{"Interactive visualization"}</p>
            </div>
            <div class="chart-content" style={format!("height: {}", height)}>
                <div class="chart-placeholder">
                    <div class="chart-icon">{"📊"}</div>
                    <p>{"Chart visualization would appear here"}</p>
                    <p class="chart-description">
                        {format!("{} chart with {} data points", 
                                 match props.chart_type {
                                     ChartType::Bar => "Bar",
                                     ChartType::Line => "Line",
                                     ChartType::Pie => "Pie",
                                 },
                                 props.data.len())}
                    </p>
                </div>
            </div>
        </div>
    }
}