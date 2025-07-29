use cpc_core::supply_chain::models::*;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SupplyChainChartsProps {
    pub supply_chain: SupplyChain,
}

#[function_component(SupplyChainCharts)]
pub fn supply_chain_charts(props: &SupplyChainChartsProps) -> Html {
    let transport_canvas_ref = use_node_ref();
    let cooperative_canvas_ref = use_node_ref();
    let supply_chain = props.supply_chain.clone();

    use_effect_with_deps(
        move |(transport_ref, cooperative_ref, chain)| {
            if let Some(canvas) = transport_ref.cast::<HtmlCanvasElement>() {
                if let Err(e) = draw_transport_chart(&canvas, &chain) {
                    web_sys::console::error_1(&format!("Failed to draw transport chart: {:?}", e).into());
                }
            }
            if let Some(canvas) = cooperative_ref.cast::<HtmlCanvasElement>() {
                if let Err(e) = draw_cooperative_chart(&canvas, &chain) {
                    web_sys::console::error_1(&format!("Failed to draw cooperative chart: {:?}", e).into());
                }
            }
            || ()
        },
        (
            transport_canvas_ref.clone(),
            cooperative_canvas_ref.clone(),
            supply_chain,
        ),
    );

    html! {
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-6">
            <div class="bg-gray-50 p-4 rounded-lg">
                <h3 class="text-sm font-medium text-gray-900 mb-2">{ "Transport Breakdown" }</h3>
                <canvas ref={transport_canvas_ref} width="400" height="300"></canvas>
            </div>
            <div class="bg-gray-50 p-4 rounded-lg">
                <h3 class="text-sm font-medium text-gray-900 mb-2">{ "Cooperative Impact Share" }</h3>
                <canvas ref={cooperative_canvas_ref} width="400" height="300"></canvas>
            </div>
        </div>
    }
}

/// Draws a bar chart for transport cost and carbon footprint breakdown.
fn draw_transport_chart(
    canvas: &HtmlCanvasElement,
    chain: &SupplyChain,
) -> Result<(), Box<dyn std::error::Error>> {
    let backend = CanvasBackend::with_canvas_object(canvas.clone())
        .ok_or("Failed to create canvas backend")?;
    let root = backend.into_drawing_area();
    root.fill(&WHITE)?;

    // Create mock data for transport segments since we need cost and carbon data
    let segments = &chain.segments;
    if segments.is_empty() { return Ok(()); }

    // Calculate total cost and carbon for each transport method
    let mut method_data: std::collections::HashMap<TransportMethod, (f64, f64)> = std::collections::HashMap::new();
    
    for segment in segments {
        let entry = method_data.entry(segment.method.clone()).or_insert((0.0, 0.0));
        let actual_cost = segment.cost.amount;
        let actual_carbon = segment.environmental_impact.carbon_footprint_kg_co2;
        entry.0 += actual_cost;
        entry.1 += actual_carbon;
    }

    let methods: Vec<String> = method_data.keys()
        .map(|m| format!("{:?}", m))
        .collect();
    let costs: Vec<f64> = method_data.values().map(|(cost, _)| *cost).collect();
    let carbons: Vec<f64> = method_data.values().map(|(_, carbon)| *carbon).collect();

    let max_cost = costs.iter().fold(0.0, f64::max);
    let max_carbon = carbons.iter().fold(0.0, f64::max);

    // Create chart with dual axis
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .margin(10)
        .caption("Transport Cost & Carbon Impact", ("sans-serif", 16))
        .build_cartesian_2d(methods.into_segmented(), 0.0..max_cost * 1.2)?;

    chart.configure_mesh()
        .y_desc("Cost ($)")
        .draw()?;

    // Draw cost bars
    chart.draw_series(
        method_data.iter().enumerate().map(|(i, (method, (cost, _)))| {
            let method_str = format!("{:?}", method);
            let mut bar = Rectangle::new(
                [(method_str.clone(), 0.0), (method_str, *cost)],
                BLUE.filled(),
            );
            bar.set_margin(5);
            bar
        })
    )?;

    Ok(())
}

/// Draws a pie chart for cooperative metrics.
fn draw_cooperative_chart(
    canvas: &HtmlCanvasElement,
    chain: &SupplyChain,
) -> Result<(), Box<dyn std::error::Error>> {
    let backend = CanvasBackend::with_canvas_object(canvas.clone())
        .ok_or("Failed to create canvas backend")?;
    let root = backend.into_drawing_area();
    root.fill(&WHITE)?;
    
    // Calculate cooperative impact metrics
    let total_nodes = chain.nodes.len() as f64;
    let cooperative_nodes = chain.nodes.iter()
        .filter(|node| node.cooperative_metrics.is_some())
        .count() as f64;
    
    let cooperative_share = if total_nodes > 0.0 {
        (cooperative_nodes / total_nodes) * 100.0
    } else {
        0.0
    };
    
    let other_share = 100.0 - cooperative_share;
    
    let data = [
        ("Co-op Nodes", cooperative_share),
        ("Other Nodes", other_share),
    ];

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .build_pie();

    chart.draw_series(
        Pie::new(
            &data,
            &[BLUE.mix(0.8), RED.mix(0.5)],
            |&(_name, value)| format!("{:.1}%", value),
        )
    )?;
    
    Ok(())
}