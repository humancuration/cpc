use yew::prelude::*;
use plotters::prelude::*;
use web_sys::HtmlCanvasElement;
use crate::types::PricingDataPoint;

#[derive(Properties, PartialEq, Clone)]
pub struct PriceHistoryChartProps {
    pub data: Vec<PricingDataPoint>,
}

#[function_component(PriceHistoryChart)]
pub fn price_history_chart(props: &PriceHistoryChartProps) -> Html {
    let canvas_ref = use_node_ref();

    use_effect_with_deps({
        let data = props.data.clone();
        let canvas_ref = canvas_ref.clone();
        move |_| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let ctx = canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
                
                // Clear canvas
                ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
                
                if data.is_empty() {
                    return;
                }
                
                // Find min/max values
                let min_price = data.iter().map(|p| p.price).fold(f64::INFINITY, f64::min);
                let max_price = data.iter().map(|p| p.price).fold(f64::NEG_INFINITY, f64::max);
                let min_cost = data.iter().map(|p| p.cost).fold(f64::INFINITY, f64::min);
                let max_cost = data.iter().map(|p| p.cost).fold(f64::NEG_INFINITY, f64::max);
                
                let y_min = min_price.min(min_cost) * 0.9;
                let y_max = max_price.max(max_cost) * 1.1;
                
                // Set up chart dimensions
                let width = canvas.width() as f64;
                let height = canvas.height() as f64;
                let margin = 40.0;
                let chart_width = width - 2.0 * margin;
                let chart_height = height - 2.0 * margin;
                
                // Draw axes
                ctx.begin_path();
                ctx.move_to(margin, margin);
                ctx.line_to(margin, height - margin);
                ctx.line_to(width - margin, height - margin);
                ctx.stroke();
                
                // Draw data points
                for (i, point) in data.iter().enumerate() {
                    let x = margin + (i as f64 * chart_width / (data.len() as f64 - 1.0));
                    let y_price = height - margin - ((point.price - y_min) / (y_max - y_min)) * chart_height;
                    let y_cost = height - margin - ((point.cost - y_min) / (y_max - y_min)) * chart_height;
                    
                    // Draw price point
                    ctx.begin_path();
                    ctx.arc(x, y_price, 3.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
                    ctx.fill_style(&"blue".into());
                    ctx.fill();
                    
                    // Draw cost point
                    ctx.begin_path();
                    ctx.arc(x, y_cost, 3.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
                    ctx.fill_style(&"red".into());
                    ctx.fill();
                }
            }
            || {}
        }
    }, (props.data.clone(),));

    html! {
        <div class="price-history-chart">
            <canvas ref={canvas_ref} width="400" height="300"></canvas>
        </div>
    }
}