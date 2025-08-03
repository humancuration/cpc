//! Sentiment analysis visualization component

use yew::prelude::*;
use reviews::Review;
use crate::data_generator::generators::products::Product;
use crate::components::visualization::types::{VisualizationProps, VisualizationComponent};
use reviews::analytics::AnalyticsEngine;
use web_sys::HtmlCanvasElement;
use wasm_bindgen::JsCast;
use crate::utils::accessibility::{generate_sentiment_alt_text, check_color_contrast};
use web_sys::ResizeObserver;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use crate::components::social_sharing::ShareButtonGroup;

#[function_component(Sentiment)]
pub fn sentiment(props: &VisualizationProps) -> Html {
    let canvas_ref = use_node_ref();
    let container_ref = use_node_ref();
    let reviews = &props.reviews;
    
    // Render sentiment chart when reviews change
    // Render sentiment chart when reviews change
    {
        let canvas_ref = canvas_ref.clone();
        let reviews = reviews.clone();
        use_effect_with(reviews.clone(), move |_| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let _ = render_sentiment_chart(&canvas, &reviews);
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
                    let _ = render_sentiment_chart(&canvas, &reviews);
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
        <div ref={container_ref} class="visualization-sentiment">
            <div class="visualization-header">
                <h2>{"Sentiment Analysis"}</h2>
                if props.enable_sharing {
                    <ShareButtonGroup
                        visualization_type={VisualizationComponent::Sentiment}
                        reviews={reviews.clone()}
                        canvas_ref={canvas_ref.clone()}
                        on_share={props.on_share.clone()}
                    />
                }
            </div>
            <canvas
                ref={canvas_ref}
                width="400"
                height="400"
                aria-label={generate_sentiment_alt_text(reviews)}
                role="img"
            ></canvas>
            
            <div class="sentiment-legend">
                <div class="legend-item">
                    <div class="legend-color positive"></div>
                    <span>{"Positive"}</span>
                </div>
                <div class="legend-item">
                    <div class="legend-color neutral"></div>
                    <span>{"Neutral"}</span>
                </div>
                <div class="legend-item">
                    <div class="legend-color negative"></div>
                    <span>{"Negative"}</span>
                </div>
            </div>
        </div>
    }
}
}

/// Render sentiment distribution radial chart
fn render_sentiment_chart(canvas: &HtmlCanvasElement, reviews: &[Review<Product>]) -> Result<(), Box<dyn std::error::Error>> {
    let analytics_engine = AnalyticsEngine::new();
    
    // Calculate sentiment distribution
    let mut positive = 0;
    let mut neutral = 0;
    let mut negative = 0;
    
    for review in reviews {
        let sentiment = analytics_engine.sentiment_analysis(&review.content);
        if sentiment > 0.1 {
            positive += 1;
        } else if sentiment < -0.1 {
            negative += 1;
        } else {
            neutral += 1;
        }
    }
    
    let total = positive + neutral + negative;
    if total == 0 {
        return Ok(());
    }
    
    // Calculate percentages
    let positive_pct = positive as f64 / total as f64;
    let neutral_pct = neutral as f64 / total as f64;
    let negative_pct = negative as f64 / total as f64;
    
    // Create drawing context
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    
    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    let center_x = width / 2.0;
    let center_y = height / 2.0;
    let radius = (width.min(height) / 2.0) * 0.8;
    
    // Clear canvas
    context.clear_rect(0.0, 0.0, width, height);
    
    // Draw radial chart
    let mut start_angle = -std::f64::consts::PI / 2.0; // Start from top
    
    // Draw positive segment (green)
    let positive_angle = positive_pct * 2.0 * std::f64::consts::PI;
    context.begin_path();
    context.set_fill_style(&"green".into());
    context.move_to(center_x, center_y);
    context.arc(center_x, center_y, radius, start_angle, start_angle + positive_angle).unwrap();
    context.fill();
    start_angle += positive_angle;
    
    // Draw neutral segment (yellow)
    let neutral_angle = neutral_pct * 2.0 * std::f64::consts::PI;
    context.begin_path();
    context.set_fill_style(&"yellow".into());
    context.move_to(center_x, center_y);
    context.arc(center_x, center_y, radius, start_angle, start_angle + neutral_angle).unwrap();
    context.fill();
    start_angle += neutral_angle;
    
    // Draw negative segment (red)
    let negative_angle = negative_pct * 2.0 * std::f64::consts::PI;
    context.begin_path();
    context.set_fill_style(&"red".into());
    context.move_to(center_x, center_y);
    context.arc(center_x, center_y, radius, start_angle, start_angle + negative_angle).unwrap();
    context.fill();
    
    // Draw center circle
    context.begin_path();
    context.set_fill_style(&"white".into());
    context.arc(center_x, center_y, radius * 0.3, 0.0, 2.0 * std::f64::consts::PI).unwrap();
    context.fill();
    
    // Draw text in center
    context.set_fill_style(&"black".into());
    context.set_text_align("center");
    context.set_text_baseline("middle");
    context.set_font("bold 16px sans-serif");
    context.fill_text(&format!("{}", total), center_x, center_y - 10.0).unwrap();
    context.set_font("12px sans-serif");
    context.fill_text("reviews", center_x, center_y + 10.0).unwrap();
    
    // Draw percentage labels
    draw_percentage_label(&context, center_x, center_y, radius, positive_pct, 0.0, "green", &format!("{}%", (positive_pct * 100.0) as u32));
    draw_percentage_label(&context, center_x, center_y, radius, neutral_pct, positive_pct, "black", &format!("{}%", (neutral_pct * 100.0) as u32));
    draw_percentage_label(&context, center_x, center_y, radius, negative_pct, positive_pct + neutral_pct, "black", &format!("{}%", (negative_pct * 100.0) as u32));
    
    Ok(())
}

/// Draw percentage label at appropriate position
fn draw_percentage_label(
    context: &web_sys::CanvasRenderingContext2d,
    center_x: f64,
    center_y: f64,
    radius: f64,
    segment_pct: f64,
    start_pct: f64,
    color: &str,
    text: &str,
) {
    if segment_pct < 0.05 {
        return; // Don't draw labels for very small segments
    }
    
    let angle = (start_pct + segment_pct / 2.0) * 2.0 * std::f64::consts::PI - std::f64::consts::PI / 2.0;
    let label_radius = radius * 0.7;
    let x = center_x + label_radius * angle.cos();
    let y = center_y + label_radius * angle.sin();
    
    context.set_fill_style(&color.into());
    context.set_text_align("center");
    context.set_text_baseline("middle");
    context.set_font("12px sans-serif");
    context.fill_text(text, x, y).unwrap();
}