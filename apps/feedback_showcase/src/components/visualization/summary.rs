//! Summary visualization component showing key metrics

use yew::prelude::*;
use reviews::Review;
use crate::data_generator::generators::products::Product;
use crate::components::visualization::types::{VisualizationProps, VisualizationComponent};
use reviews::analytics::AnalyticsEngine;
use web_sys::HtmlCanvasElement;
use wasm_bindgen::JsCast;
use crate::utils::accessibility::{generate_summary_alt_text, check_color_contrast};
use web_sys::ResizeObserver;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use crate::components::social_sharing::ShareButtonGroup;

#[function_component(Summary)]
pub fn summary(props: &VisualizationProps) -> Html {
    let canvas_ref = use_node_ref();
    let container_ref = use_node_ref();
    let reviews = &props.reviews;
    
    // Calculate summary statistics
    let analytics_engine = AnalyticsEngine::new();
    let avg_rating = if !reviews.is_empty() {
        analytics_engine.average_rating(reviews, "overall")
    } else {
        0.0
    };
    
    let total_reviews = reviews.len();
    
    // Render pie chart when reviews change
    // Render pie chart when reviews change
    {
        let canvas_ref = canvas_ref.clone();
        let reviews = reviews.clone();
        use_effect_with(reviews.clone(), move |_| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let _ = render_sentiment_pie_chart(&canvas, &reviews);
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
                    let _ = render_sentiment_pie_chart(&canvas, &reviews);
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
    }
    
    html! {
        <div ref={container_ref} class="visualization-summary">
            <div class="visualization-header">
                <h2>{"Feedback Summary"}</h2>
                if props.enable_sharing {
                    <ShareButtonGroup
                        visualization_type={VisualizationComponent::Summary}
                        reviews={reviews.clone()}
                        canvas_ref={canvas_ref.clone()}
                        on_share={props.on_share.clone()}
                    />
                }
            </div>
            
            <div class="summary-metrics">
                <div class="metric-card">
                    <h3>{"Average Rating"}</h3>
                    <div class="rating-display">
                        <span class="rating-value">{format!("{:.2}", avg_rating * 5.0)}</span>
                        <span class="rating-max">{" / 5.0"}</span>
                    </div>
                    <div class="star-rating">
                        {render_stars(avg_rating)}
                    </div>
                </div>
                
                <div class="metric-card">
                    <h3>{"Total Reviews"}</h3>
                    <div class="review-count">
                        {total_reviews}
                    </div>
                </div>
                
                <div class="metric-card">
                    <h3>{"Sentiment Distribution"}</h3>
                    <canvas
                        ref={canvas_ref}
                        width="200"
                        height="200"
                        aria-label={generate_summary_alt_text(reviews)}
                        role="img"
                    ></canvas>
                </div>
            </div>
        </div>
    }
}

/// Render star rating visualization
fn render_stars(rating: f32) -> Html {
    let full_stars = (rating * 5.0).floor() as u32;
    let has_half_star = (rating * 5.0 * 2.0).fract() >= 0.5;
    
    let mut stars = Vec::new();
    
    for i in 0..5 {
        if i < full_stars {
            stars.push(html! { <span class="star full">{"★"}</span> });
        } else if i == full_stars && has_half_star {
            stars.push(html! { <span class="star half">{"★"}</span> });
        } else {
            stars.push(html! { <span class="star empty">{"★"}</span> });
        }
    }
    
    html! {
        <div class="stars">
            {stars}
        </div>
    }
}

/// Render sentiment distribution pie chart
fn render_sentiment_pie_chart(canvas: &HtmlCanvasElement, reviews: &[Review<Product>]) -> Result<(), Box<dyn std::error::Error>> {
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
    
    // Draw pie chart
    let total = positive + neutral + negative;
    if total == 0 {
        return Ok(());
    }
    
    let positive_angle = (positive as f64 / total as f64) * 2.0 * std::f64::consts::PI;
    let neutral_angle = (neutral as f64 / total as f64) * 2.0 * std::f64::consts::PI;
    
    // Draw positive segment (green)
    context.begin_path();
    context.set_fill_style(&"green".into());
    context.move_to(center_x, center_y);
    context.arc(center_x, center_y, radius, 0.0, positive_angle).unwrap();
    context.fill();
    
    // Draw neutral segment (yellow)
    context.begin_path();
    context.set_fill_style(&"yellow".into());
    context.move_to(center_x, center_y);
    context.arc(center_x, center_y, radius, positive_angle, positive_angle + neutral_angle).unwrap();
    context.fill();
    
    // Draw negative segment (red)
    context.begin_path();
    context.set_fill_style(&"red".into());
    context.move_to(center_x, center_y);
    context.arc(center_x, center_y, radius, positive_angle + neutral_angle, 2.0 * std::f64::consts::PI).unwrap();
    context.fill();
    
    // Draw legend
    let legend_x = radius + 20.0;
    let mut legend_y = center_y - 30.0;
    
    // Positive legend
    context.set_fill_style(&"black".into());
    context.set_font("12px sans-serif");
    context.fill_rect(legend_x, legend_y, 15.0, 15.0);
    context.set_fill_style(&"green".into());
    context.fill_rect(legend_x + 2.0, legend_y + 2.0, 11.0, 11.0);
    context.set_fill_style(&"black".into());
    context.fill_text(&format!("Positive ({})", positive), legend_x + 20.0, legend_y + 12.0).unwrap();
    
    legend_y += 25.0;
    
    // Neutral legend
    context.set_fill_style(&"black".into());
    context.fill_rect(legend_x, legend_y, 15.0, 15.0);
    context.set_fill_style(&"yellow".into());
    context.fill_rect(legend_x + 2.0, legend_y + 2.0, 11.0, 11.0);
    context.set_fill_style(&"black".into());
    context.fill_text(&format!("Neutral ({})", neutral), legend_x + 20.0, legend_y + 12.0).unwrap();
    
    legend_y += 25.0;
    
    // Negative legend
    context.set_fill_style(&"black".into());
    context.fill_rect(legend_x, legend_y, 15.0, 15.0);
    context.set_fill_style(&"red".into());
    context.fill_rect(legend_x + 2.0, legend_y + 2.0, 11.0, 11.0);
    context.set_fill_style(&"black".into());
    context.fill_text(&format!("Negative ({})", negative), legend_x + 20.0, legend_y + 12.0).unwrap();
    
    Ok(())
}