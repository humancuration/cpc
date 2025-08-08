use yew::prelude::*;
use stylist::{style, yew::styled_component};
use plotters::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, Window};
use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub struct ChartDataPoint {
    pub label: String,
    pub value: f64,
    pub color: Option<String>,
}

#[derive(Clone, PartialEq)]
pub enum ChartType {
    Bar,
    Line,
    Pie,
}

#[derive(Properties, PartialEq)]
pub struct ChartProps {
    pub title: String,
    pub data: Vec<ChartDataPoint>,
    pub chart_type: ChartType,
    #[prop_or(Some(400))]
    pub height: Option<i32>,
}

#[styled_component(Chart)]
pub fn chart(props: &ChartProps) -> Html {
    let chart_style = style!(
        r#"
        .chart-card {
            background: var(--surface);
            border-radius: 8px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
            padding: 20px;
        }
        
        .chart-title {
            font-size: 1.25rem;
            margin: 0 0 20px 0;
            color: var(--text-primary);
        }
        
        .chart-container {
            position: relative;
            width: 100%;
            height: 400px;
        }
        
        .chart-canvas {
            width: 100%;
            height: 100%;
        }
    "#
    ).unwrap();

    let height = props.height.unwrap_or(400);

    html! {
        <div class={chart_style}>
            <h3 class="chart-title">{ &props.title }</h3>
            <div class="chart-container" style={format!("height: {}px", height)}>
                // In a real implementation, this would render a chart using canvas or SVG
                // For now, we'll show a placeholder
                <div class="chart-placeholder">
                    <p>{"Chart visualization would appear here"}</p>
                    <ul>
                        { for props.data.iter().map(|point| {
                            html! {
                                <li>{ format!("{}: {}", point.label, point.value) }</li>
                            }
                        }) }
                    </ul>
                </div>
            </div>
        </div>
    }
}