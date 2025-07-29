use yew::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use gloo_console;

#[derive(Properties, PartialEq, Clone)]
pub struct CostBreakdownProps {
    pub product: ProductData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductData {
    pub material_cost: Option<f64>,
    pub labor_cost: Option<f64>,
    pub cost: Option<MoneyData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoneyData {
    pub amount: f64,
    pub currency: String,
}

#[function_component(CostBreakdown)]
pub fn cost_breakdown(props: &CostBreakdownProps) -> Html {
    let canvas_ref = use_node_ref();
    let product = props.product.clone();
    
    use_effect_with_deps(
        move |(canvas_ref, data)| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                // drawing is now handled by a dedicated, testable function
                if let Err(e) = draw_chart_with_plotters(&canvas, &data) {
                    // Log the error
                    gloo_console::error!(format!("Failed to draw chart: {:?}", e));
                }
            }
            || ()
        },
        (canvas_ref.clone(), product),
    );
    
    let calculate_total_cost = move || {
        let material = product.material_cost.unwrap_or(0.0);
        let labor = product.labor_cost.unwrap_or(0.0);
        material + labor
    };
    
    let calculate_profit_margin = move || {
        let total_cost = calculate_total_cost();
        if total_cost > 0.0 && product.cost.is_some() {
            let selling_price = product.cost.as_ref().unwrap().amount;
            ((selling_price - total_cost) / selling_price) * 100.0
        } else {
            0.0
        }
    };
    
    html! {
        <div class="bg-white shadow rounded-lg">
            <div class="px-6 py-4 border-b border-gray-200">
                <h2 class="text-lg font-medium text-gray-900">{ "Cost Breakdown" }</h2>
            </div>
            
            <div class="px-6 py-4">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                    {/* Chart */}
                    <div>
                        <h3 class="text-sm font-medium text-gray-900 mb-3">{ "Cost Distribution" }</h3>
                        <canvas
                            ref={canvas_ref}
                            width="400"
                            height="300"
                            class="w-full h-64 border border-gray-200 rounded"
                        />
                    </div>
                    
                    {/* Cost Summary */}
                    <div>
                        <h3 class="text-sm font-medium text-gray-900 mb-3">{ "Cost Summary" }</h3>
                        <div class="space-y-4">
                            <div class="flex justify-between items-center">
                                <span class="text-sm text-gray-600">{ "Material Cost" }</span>
                                <span class="text-sm font-medium text-gray-900">
                                    { format!("${:.2}", product.material_cost.unwrap_or(0.0)) }
                                </span>
                            </div>
                            
                            <div class="flex justify-between items-center">
                                <span class="text-sm text-gray-600">{ "Labor Cost" }</span>
                                <span class="text-sm font-medium text-gray-900">
                                    { format!("${:.2}", product.labor_cost.unwrap_or(0.0)) }
                                </span>
                            </div>
                            
                            <div class="border-t pt-4">
                                <div class="flex justify-between items-center">
                                    <span class="text-sm font-medium text-gray-600">{ "Total Cost" }</span>
                                    <span class="text-sm font-bold text-gray-900">
                                        { format!("${:.2}", calculate_total_cost()) }
                                    </span>
                                </div>
                            </div>
                            
                            if let Some(cost) = &product.cost {
                                <div class="border-t pt-4">
                                    <div class="flex justify-between items-center">
                                        <span class="text-sm font-medium text-gray-600">{ "Selling Price" }</span>
                                        <span class="text-sm font-bold text-green-600">
                                            { format!("${:.2} {}", cost.amount, cost.currency) }
                                        </span>
                                    </div>
                                    
                                    <div class="mt-2 flex justify-between items-center">
                                        <span class="text-sm text-gray-600">{ "Profit Margin" }</span>
                                        <span class={`text-sm font-medium ${
                                            if calculate_profit_margin() > 0.0 { "text-green-600" } else { "text-red-600" }
                                        }`}>
                                            { format!("{:.1}%", calculate_profit_margin()) }
                                        </span>
                                    </div>
                                </div>
                            }
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Draws a pie chart onto the given canvas element using plotters.
fn draw_chart_with_plotters(
    canvas: &HtmlCanvasElement,
    product: &ProductData,
) -> Result<(), Box<dyn std::error::Error>> {
    let backend = CanvasBackend::with_canvas_object(canvas.clone())
        .ok_or("Failed to create canvas backend")?;
    let root = backend.into_drawing_area();
    root.fill(&WHITE)?;

    let (material_cost, labor_cost) = (
        product.material_cost.unwrap_or(0.0),
        product.labor_cost.unwrap_or(0.0),
    );

    if material_cost + labor_cost == 0.0 {
        // No data to plot, maybe draw a message
        return Ok(());
    }

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .build_pie();

    chart.draw_series(
        Pie::new(
            &[
                ("Material", material_cost),
                ("Labor", labor_cost),
            ],
            &[BLUE, GREEN], // Use plotters colors
            |&_name, &value| format!("{:.0}%", value / (material_cost + labor_cost) * 100.0),
        )
    )?;

    Ok(())
}