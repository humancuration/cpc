//! Ratings distribution chart visualization component

use yew::prelude::*;
use reviews::Review;
use crate::data_generator::generators::products::Product;
use crate::components::visualization::types::{VisualizationProps, VisualizationComponent};
use reviews::analytics::AnalyticsEngine;
use web_sys::HtmlCanvasElement;
use wasm_bindgen::JsCast;
use crate::utils::accessibility::{generate_ratings_chart_alt_text, check_color_contrast};
use web_sys::ResizeObserver;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use crate::components::social_sharing::ShareButtonGroup;

#[function_component(RatingsChart)]
pub fn ratings_chart(props: &VisualizationProps) -> Html {
    let canvas_ref = use_node_ref();
    let container_ref = use_node_ref();
    let reviews = &props.reviews;
    
    // Render chart when reviews change
    // Render chart when reviews change
    {
        let canvas_ref = canvas_ref.clone();
        let reviews = reviews.clone();
        use_effect_with(reviews.clone(), move |_| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let _ = render_ratings_chart(&canvas, &reviews);
            }
            || ()
        });
    }
    
    // Add resize observer for responsive design
    {
        let canvas_ref = canvas_ref.clone();
        let reviews = reviews.clone();
        use_effect_with((), move |_| {
            let canvas_ref = canvas_ref.clone();
            let reviews = reviews.clone();
            
            // Create closure for resize observer
            let closure = Closure::wrap(Box::new(move |_entries: js_sys::Array| {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    // Update canvas size based on container
                    let _ = render_ratings_chart(&canvas, &reviews);
                }
            }) as Box<dyn FnMut(js_sys::Array)>);
            
            // Create resize observer
            let observer = ResizeObserver::new(closure.as_ref().unchecked_ref()).unwrap();
            // Note: In a real implementation, we would observe the container element
            // For now, we'll just create the observer without observing
            
            // Cleanup function
            move || {
                observer.disconnect();
                drop(closure);
            }
        });
    
    html! {
        <div ref={container_ref} class="visualization-ratings-chart">
            <div class="visualization-header">
                <h2>{"Ratings Distribution"}</h2>
                if props.enable_sharing {
                    <ShareButtonGroup
                        visualization_type={VisualizationComponent::Ratings}
                        reviews={reviews.clone()}
                        canvas_ref={canvas_ref.clone()}
                        on_share={props.on_share.clone()}
                    />
                }
            </div>
            <canvas
                ref={canvas_ref}
                width="600"
                height="400"
                aria-label={generate_ratings_chart_alt_text(reviews)}
                role="img"
            ></canvas>
        </div>
    }
}
}

/// Render ratings distribution bar chart
fn render_ratings_chart(canvas: &HtmlCanvasElement, reviews: &[Review<Product>]) -> Result<(), Box<dyn std::error::Error>> {
    let analytics_engine = AnalyticsEngine::new();
    let distribution = analytics_engine.rating_distribution(reviews, "overall");
    
    // Create drawing context
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    
    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    
    // Clear canvas
    context.clear_rect(0.0, 0.0, width, height);
    
    // Set styles
    context.set_font("12px sans-serif");
    context.set_text_align("center");
    context.set_text_baseline("top");
    
    // Chart dimensions and padding
    let padding = 40.0;
    let chart_width = width - 2.0 * padding;
    let chart_height = height - 2.0 * padding;
    
    // Get data points
    let sorted_values = distribution.sorted_values();
    if sorted_values.is_empty() {
        return Ok(());
    }
    
    let max_count = sorted_values.iter().map(|&(_, count)| count).max().unwrap_or(1) as f64;
    let bar_width = chart_width / sorted_values.len() as f64 * 0.8;
    let bar_spacing = chart_width / sorted_values.len() as f64 * 0.2;
    
    // Draw bars
    for (i, &(rating, count)) in sorted_values.iter().enumerate() {
        let x = padding + (i as f64) * (bar_width + bar_spacing);
        let bar_height = (count as f64 / max_count) * chart_height;
        let y = padding + chart_height - bar_height;
        
        // Color gradient from red (1-star) to green (5-star)
        let rating_ratio = rating as f64 / 100.0;
        let red = ((1.0 - rating_ratio) * 255.0) as u8;
        let green = (rating_ratio * 255.0) as u8;
        let color = format!("rgb({}, {}, 0)", red, green);
        
        context.set_fill_style(&color.into());
        context.fill_rect(x, y, bar_width, bar_height);
        
        // Draw rating label
        context.set_fill_style(&"black".into());
        context.fill_text(&format!("{}", rating), x + bar_width / 2.0, padding + chart_height + 5.0).unwrap();
        
        // Draw count label
        context.fill_text(&format!("{}", count), x + bar_width / 2.0, y - 20.0).unwrap();
    }
    
    // Draw axes
    context.set_stroke_style(&"black".into());
    context.begin_path();
    context.move_to(padding, padding);
    context.line_to(padding, padding + chart_height);
    context.line_to(padding + chart_width, padding + chart_height);
    context.stroke();
    
    // Draw axis labels
    context.set_fill_style(&"black".into());
    context.fill_text("Rating", padding + chart_width / 2.0, padding + chart_height + 25.0).unwrap();
    context.save();
    context.translate(padding - 25.0, padding + chart_height / 2.0).unwrap();
    context.rotate(-std::f64::consts::PI / 2.0).unwrap();
    context.fill_text("Count", 0.0, 0.0).unwrap();
    context.restore();
    
    Ok(())
}