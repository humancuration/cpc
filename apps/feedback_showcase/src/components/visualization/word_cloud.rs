//! Word cloud visualization component for review content

use yew::prelude::*;
use reviews::Review;
use crate::data_generator::generators::products::Product;
use crate::components::visualization::types::{VisualizationProps, VisualizationComponent};
use std::collections::HashMap;
use web_sys::HtmlCanvasElement;
use wasm_bindgen::JsCast;
use crate::utils::accessibility::{generate_word_cloud_alt_text, check_color_contrast};
use web_sys::ResizeObserver;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use crate::components::social_sharing::ShareButtonGroup;

#[function_component(WordCloud)]
pub fn word_cloud(props: &VisualizationProps) -> Html {
    let canvas_ref = use_node_ref();
    let container_ref = use_node_ref();
    let reviews = &props.reviews;
    
    // Render word cloud when reviews change
    // Render word cloud when reviews change
    {
        let canvas_ref = canvas_ref.clone();
        let reviews = reviews.clone();
        use_effect_with(reviews.clone(), move |_| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let _ = render_word_cloud(&canvas, &reviews);
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
                    let _ = render_word_cloud(&canvas, &reviews);
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
        <div ref={container_ref} class="visualization-word-cloud">
            <div class="visualization-header">
                <h2>{"Common Words in Reviews"}</h2>
                if props.enable_sharing {
                    <ShareButtonGroup
                        visualization_type={VisualizationComponent::WordCloud}
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
                aria-label={generate_word_cloud_alt_text(reviews)}
                role="img"
            ></canvas>
        </div>
    }
}
}

/// Process review content and calculate word frequencies
fn calculate_word_frequencies(reviews: &[Review<Product>]) -> HashMap<String, u32> {
    let mut frequencies = HashMap::new();
    
    // Common stop words to filter out
    let stop_words = [
        "the", "and", "for", "are", "but", "not", "you", "all", "can", "had", "her", "was", 
        "one", "our", "out", "day", "get", "has", "him", "his", "how", "its", "may", "new", 
        "now", "old", "see", "two", "who", "boy", "did", "man", "men", "put", "too", "use", 
        "any", "big", "end", "far", "got", "hot", "let", "lot", "run", "set", "sit", "way",
        "will", "with", "this", "that", "have", "from", "were", "been", "have", "they", "them",
        "than", "then", "what", "when", "where", "which", "who", "why", "would", "could", "should",
        "might", "must", "shall", "does", "done", "very", "just", "only", "even", "also", "well",
        "here", "there", "about", "into", "over", "after", "before", "under", "above", "below"
    ];
    
    for review in reviews {
        // Split content into words and process
        for word in review.content.split_whitespace() {
            // Clean word (remove punctuation, convert to lowercase)
            let clean_word = word
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>()
                .to_lowercase();
            
            // Skip empty words and stop words
            if clean_word.is_empty() || clean_word.len() < 3 || stop_words.contains(&clean_word.as_str()) {
                continue;
            }
            
            // Increment frequency count
            *frequencies.entry(clean_word).or_insert(0) += 1;
        }
    }
    
    frequencies
}

/// Render word cloud using spiral placement algorithm
fn render_word_cloud(canvas: &HtmlCanvasElement, reviews: &[Review<Product>]) -> Result<(), Box<dyn std::error::Error>> {
    let frequencies = calculate_word_frequencies(reviews);
    
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
    
    // Find max frequency for scaling
    let max_frequency = frequencies.values().max().copied().unwrap_or(1);
    
    // Center of canvas
    let center_x = width / 2.0;
    let center_y = height / 2.0;
    
    // Spiral placement parameters
    let mut angle = 0.0;
    let mut radius = 0.0;
    let spiral_spacing = 2.0;
    
    // Place words in spiral pattern
    let mut placed_words = Vec::new();
    
    // Sort words by frequency (descending)
    let mut sorted_words: Vec<(&String, &u32)> = frequencies.iter().collect();
    sorted_words.sort_by(|a, b| b.1.cmp(a.1));
    
    // Place top 50 words
    for (word, &frequency) in sorted_words.iter().take(50) {
        // Calculate font size based on frequency
        let font_size = 12.0 + (frequency as f64 / max_frequency as f64) * 30.0;
        
        // Set font for measuring text
        context.set_font(&format!("{}px sans-serif", font_size));
        
        // Measure text width
        let text_metrics = context.measure_text(word).unwrap();
        let text_width = text_metrics.width();
        let text_height = font_size;
        
        // Find non-overlapping position using spiral
        let (x, y) = find_non_overlapping_position(
            center_x, center_y,
            text_width, text_height,
            &placed_words,
            &mut angle, &mut radius,
            spiral_spacing,
            width, height
        );
        
        // Store placed word for collision detection
        placed_words.push((x, y, text_width, text_height));
        
        // Draw word
        context.set_fill_style(&get_word_color(frequency, max_frequency).into());
        context.fill_text(word, x, y).unwrap();
    }
    
    Ok(())
}

/// Find a non-overlapping position for text using spiral algorithm
fn find_non_overlapping_position(
    center_x: f64, center_y: f64,
    width: f64, height: f64,
    placed_words: &[(f64, f64, f64, f64)],
    angle: &mut f64,
    radius: &mut f64,
    spiral_spacing: f64,
    canvas_width: f64,
    canvas_height: f64,
) -> (f64, f64) {
    loop {
        // Calculate spiral position
        let x = center_x + *radius * angle.cos() - width / 2.0;
        let y = center_y + *radius * angle.sin() - height / 2.0;
        
        // Check if position is within canvas bounds
        if x >= 0.0 && x + width <= canvas_width && y >= 0.0 && y + height <= canvas_height {
            // Check for collisions with placed words
            let mut collision = false;
            for &(px, py, pw, ph) in placed_words {
                if x < px + pw && x + width > px && y < py + ph && y + height > py {
                    collision = true;
                    break;
                }
            }
            
            if !collision {
                return (x, y);
            }
        }
        
        // Move along spiral
        *angle += 0.5;
        *radius = spiral_spacing * *angle.sqrt();
    }
}

/// Get color for word based on frequency
fn get_word_color(frequency: u32, max_frequency: u32) -> String {
    let intensity = frequency as f64 / max_frequency as f64;
    
    // Color gradient from blue (low frequency) to red (high frequency)
    let red = (intensity * 255.0) as u8;
    let blue = ((1.0 - intensity) * 255.0) as u8;
    
    format!("rgb({}, 0, {})", red, blue)
}