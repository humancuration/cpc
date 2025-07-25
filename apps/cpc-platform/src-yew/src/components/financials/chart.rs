use yew::prelude::*;
use web_sys::HtmlCanvasElement;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use crate::types::financial_forecasting::ForecastResult;

#[derive(Properties, PartialEq)]
pub struct ChartProps {
    pub data: ForecastResult,
}

#[function_component(ForecastChart)]
pub fn forecast_chart(props: &ChartProps) -> Html {
    let canvas_ref = use_node_ref();
    let data = props.data.clone();
    
    use_effect_with_deps(
        move |(canvas_ref, data)| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                spawn_local(async move {
                    draw_chart(&canvas, &data).await;
                });
            }
            || ()
        },
        (canvas_ref.clone(), data),
    );
    
    html! {
        <div class="forecast-chart">
            <h3>{format!("Forecast Results - Scenario {}", props.data.scenario_id)}</h3>
            <div class="chart-container">
                <canvas 
                    ref={canvas_ref}
                    width="800" 
                    height="400"
                    class="forecast-canvas"
                />
            </div>
            <div class="chart-legend">
                <div class="legend-item">
                    <span class="legend-color revenue"></span>
                    <span>{"Total Revenue"}</span>
                </div>
                <div class="legend-item">
                    <span class="legend-color expenses"></span>
                    <span>{"Total Expenses"}</span>
                </div>
                <div class="legend-item">
                    <span class="legend-color profit"></span>
                    <span>{"Profit/Loss"}</span>
                </div>
            </div>
        </div>
    }
}

async fn draw_chart(canvas: &HtmlCanvasElement, data: &ForecastResult) {
    let backend = CanvasBackend::with_canvas_object(canvas.clone())
        .expect("Failed to create canvas backend");
    
    let root = backend.into_drawing_area();
    root.fill(&WHITE).expect("Failed to fill background");
    
    let monthly_data: Vec<(i32, f64, f64, f64)> = data.monthly_projections
        .iter()
        .enumerate()
        .map(|(i, month)| {
            (i as i32, month.total_revenue, month.total_expenses, month.profit_loss)
        })
        .collect();
    
    if monthly_data.is_empty() {
        return;
    }
    
    let max_revenue = monthly_data.iter().map(|(_, r, _, _)| *r).fold(0.0, f64::max);
    let max_expenses = monthly_data.iter().map(|(_, _, e, _)| *e).fold(0.0, f64::max);
    let max_profit = monthly_data.iter().map(|(_, _, _, p)| *p).fold(0.0, f64::max);
    let min_profit = monthly_data.iter().map(|(_, _, _, p)| *p).fold(0.0, f64::min);
    
    let max_value = max_revenue.max(max_expenses).max(max_profit).max(min_profit.abs());
    let y_max = max_value * 1.1;
    let y_min = if min_profit < 0.0 { min_profit * 1.1 } else { 0.0 };
    
    let mut chart = ChartBuilder::on(&root)
        .caption("Financial Forecast", ("sans-serif", 20))
        .margin(40)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(
            0i32..(monthly_data.len() as i32 - 1),
            y_min..y_max,
        )
        .expect("Failed to build chart");
    
    chart.configure_mesh()
        .x_desc("Month")
        .y_desc("Amount ($)")
        .draw()
        .expect("Failed to draw mesh");
    
    // Draw revenue line
    chart.draw_series(
        LineSeries::new(
            monthly_data.iter().map(|(i, r, _, _)| (*i, *r)),
            &GREEN,
        )
    )
    .expect("Failed to draw revenue line")
    .label("Total Revenue")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &GREEN));
    
    // Draw expenses line
    chart.draw_series(
        LineSeries::new(
            monthly_data.iter().map(|(i, _, e, _)| (*i, *e)),
            &RED,
        )
    )
    .expect("Failed to draw expenses line")
    .label("Total Expenses")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RED));
    
    // Draw profit/loss line
    chart.draw_series(
        LineSeries::new(
            monthly_data.iter().map(|(i, _, _, p)| (*i, *p)),
            &BLUE,
        )
    )
    .expect("Failed to draw profit line")
    .label("Profit/Loss")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &BLUE));
    
    // Add legend
    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .expect("Failed to draw legend");
    
    // Add zero line if needed
    if y_min < 0.0 {
        chart.draw_series(
            std::iter::once(PathElement::new(
                vec![(0, 0), (monthly_data.len() as i32 - 1, 0)],
                &BLACK.mix(0.3),
            ))
        )
        .expect("Failed to draw zero line");
    }
}