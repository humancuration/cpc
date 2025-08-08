//! Interconnection Visualization Component
//!
//! This component visualizes the interconnections between the four impact domains
//! showing how engagement in one area strengthens the community across all areas.

use yew::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, Path2d};
use gloo_timers::callback::AnimationFrame;
use gloo_utils::window;

use impact_viz::core::VisualizationStyle;
use crate::models::ImpactInterconnection;

/// Properties for the InterconnectionVisualization component
#[derive(Properties, PartialEq)]
pub struct InterconnectionVisualizationProps {
    /// Interconnection data to visualize
    pub data: Vec<ImpactInterconnection>,
    
    /// Visualization style
    pub style: VisualizationStyle,
}

/// State for the InterconnectionVisualization component
#[derive(Clone, PartialEq)]
pub struct InterconnectionVisualizationState {
    /// Canvas reference
    canvas_ref: NodeRef,
    
    /// Animation frame handle
    #[allow(dead_code)]
    animation_frame: Option<AnimationFrame>,
}

/// Interconnection Visualization Component
#[function_component(InterconnectionVisualization)]
pub fn interconnection_visualization(props: &InterconnectionVisualizationProps) -> Html {
    let state = use_state(|| InterconnectionVisualizationState {
        canvas_ref: NodeRef::default(),
        animation_frame: None,
    });
    
    // Draw the visualization when data or style changes
    {
        let state = state.clone();
        let data = props.data.clone();
        let style = props.style.clone();
        
        use_effect_with((data, style), move |_| {
            let canvas_ref = state.canvas_ref.clone();
            
            // Draw the visualization
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                draw_interconnection_viz(&canvas, &props.data, &props.style);
            }
            
            || ()
        });
    }
    
    html! {
        <div class="interconnection-visualization">
            <h3>{"Impact Domain Interconnections"}</h3>
            <canvas 
                ref={state.canvas_ref.clone()} 
                id="interconnection-canvas" 
                width="800" 
                height="600"
            />
            <div class="interconnection-key">
                <div class="key-item">
                    <div class="key-color learning"></div>
                    <span>{"Learning"}</span>
                </div>
                <div class="key-item">
                    <div class="key-color volunteer"></div>
                    <span>{"Volunteer"}</span>
                </div>
                <div class="key-item">
                    <div class="key-color financial"></div>
                    <span>{"Financial"}</span>
                </div>
                <div class="key-item">
                    <div class="key-color cause"></div>
                    <span>{"Cause"}</span>
                </div>
            </div>
            <div class="interconnection-details">
                {for props.data.iter().map(|interconnection| {
                    html! {
                        <div class="interconnection-detail">
                            <strong>
                                {format!("{:?} → {:?}", interconnection.source_domain, interconnection.target_domain)}
                            </strong>
                            <p>{&interconnection.description}</p>
                            <div class="strength-bar">
                                <div 
                                    class="strength-fill" 
                                    style={format!("width: {}%", interconnection.strength * 100.0)}
                                ></div>
                            </div>
                            <span class="strength-value">
                                {format!("Strength: {:.1}%", interconnection.strength * 100.0)}
                            </span>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}

/// Draw the interconnection visualization on the canvas
fn draw_interconnection_viz(
    canvas: &HtmlCanvasElement,
    data: &Vec<ImpactInterconnection>,
    style: &VisualizationStyle,
) {
    let context: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();
    
    // Clear the canvas
    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    
    // Set canvas dimensions
    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    
    // Draw based on visualization style
    match style {
        VisualizationStyle::Narrative => {
            draw_narrative_viz(&context, data, width, height);
        }
        VisualizationStyle::Comparative => {
            draw_comparative_viz(&context, data, width, height);
        }
        VisualizationStyle::TrendBased => {
            draw_trend_viz(&context, data, width, height);
        }
        VisualizationStyle::Quantitative => {
            draw_quantitative_viz(&context, data, width, height);
        }
        VisualizationStyle::Qualitative => {
            draw_qualitative_viz(&context, data, width, height);
        }
    }
}

/// Draw narrative-style visualization
fn draw_narrative_viz(
    context: &CanvasRenderingContext2d,
    data: &Vec<ImpactInterconnection>,
    width: f64,
    height: f64,
) {
    // Draw circular flow diagram
    let center_x = width / 2.0;
    let center_y = height / 2.0;
    let radius = f64::min(width, height) / 3.0;
    
    // Draw the circular background
    context.set_fill_style(&"rgba(240, 240, 240, 0.5)".into());
    let circle = Path2d::new().unwrap();
    circle.arc(center_x, center_y, radius, 0.0, 2.0 * std::f64::consts::PI).unwrap();
    context.fill_with_path_2d(&circle);
    
    // Draw domain nodes
    let domains = ["Learning", "Volunteer", "Financial", "Cause"];
    let colors = ["#4CAF50", "#2196F3", "#FF9800", "#9C27B0"];
    
    for (i, (domain, color)) in domains.iter().zip(colors.iter()).enumerate() {
        let angle = (i as f64) * (2.0 * std::f64::consts::PI / 4.0) - std::f64::consts::PI / 4.0;
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        
        // Draw node
        context.set_fill_style(&(*color).into());
        context.begin_path();
        context.arc(x, y, 40.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
        context.fill();
        
        // Draw node label
        context.set_fill_style(&"#000000".into());
        context.set_font("bold 14px Arial");
        context.set_text_align("center");
        context.set_text_baseline("middle");
        context.fill_text(domain, x, y).unwrap();
    }
    
    // Draw connections
    for interconnection in data {
        let source_index = match interconnection.source_domain {
            crate::models::impact_data::ImpactDomain::Learning => 0,
            crate::models::impact_data::ImpactDomain::Volunteer => 1,
            crate::models::impact_data::ImpactDomain::Financial => 2,
            crate::models::impact_data::ImpactDomain::Cause => 3,
        };
        
        let target_index = match interconnection.target_domain {
            crate::models::impact_data::ImpactDomain::Learning => 0,
            crate::models::impact_data::ImpactDomain::Volunteer => 1,
            crate::models::impact_data::ImpactDomain::Financial => 2,
            crate::models::impact_data::ImpactDomain::Cause => 3,
        };
        
        let source_angle = (source_index as f64) * (2.0 * std::f64::consts::PI / 4.0) - std::f64::consts::PI / 4.0;
        let target_angle = (target_index as f64) * (2.0 * std::f64::consts::PI / 4.0) - std::f64::consts::PI / 4.0;
        
        let source_x = center_x + radius * source_angle.cos();
        let source_y = center_y + radius * source_angle.sin();
        let target_x = center_x + radius * target_angle.cos();
        let target_y = center_y + radius * target_angle.sin();
        
        // Draw arrow
        context.set_stroke_style(&format!("rgba(0, 0, 0, {})", interconnection.strength).into());
        context.set_line_width(2.0 + interconnection.strength * 8.0);
        context.begin_path();
        context.move_to(source_x, source_y);
        context.line_to(target_x, target_y);
        context.stroke();
        
        // Draw arrowhead
        let angle = (target_y - source_y).atan2(target_x - source_x);
        let arrow_size = 10.0 + interconnection.strength * 20.0;
        context.begin_path();
        context.move_to(target_x, target_y);
        context.line_to(
            target_x - arrow_size * (angle - std::f64::consts::PI / 6.0).cos(),
            target_y - arrow_size * (angle - std::f64::consts::PI / 6.0).sin()
        );
        context.line_to(
            target_x - arrow_size * (angle + std::f64::consts::PI / 6.0).cos(),
            target_y - arrow_size * (angle + std::f64::consts::PI / 6.0).sin()
        );
        context.close_path();
        context.fill();
    }
}

/// Draw comparative-style visualization
fn draw_comparative_viz(
    context: &CanvasRenderingContext2d,
    data: &Vec<ImpactInterconnection>,
    width: f64,
    height: f64,
) {
    // Draw bar chart of interconnection strengths
    let bar_width = width / (data.len() as f64 + 1.0);
    let max_height = height - 100.0;
    
    context.set_font("12px Arial");
    context.set_text_align("center");
    context.set_text_baseline("top");
    
    for (i, interconnection) in data.iter().enumerate() {
        let x = (i as f64 + 1.0) * bar_width;
        let bar_height = interconnection.strength * max_height;
        let y = height - bar_height - 50.0;
        
        // Draw bar
        let gradient = context.create_linear_gradient(x, y, x, y + bar_height).unwrap();
        gradient.add_color_stop(0.0, "rgba(76, 175, 80, 0.8)").unwrap();
        gradient.add_color_stop(1.0, "rgba(76, 175, 80, 0.4)").unwrap();
        context.set_fill_style(&gradient);
        context.fill_rect(x - 20.0, y, 40.0, bar_height);
        
        // Draw label
        context.set_fill_style(&"#000000".into());
        context.fill_text(
            &format!("{:?}→{:?}", interconnection.source_domain, interconnection.target_domain),
            x,
            height - 40.0
        ).unwrap();
        
        // Draw value
        context.fill_text(
            &format!("{:.1}%", interconnection.strength * 100.0),
            x,
            y - 20.0
        ).unwrap();
    }
}

/// Draw trend-style visualization (placeholder)
fn draw_trend_viz(
    context: &CanvasRenderingContext2d,
    _data: &Vec<ImpactInterconnection>,
    width: f64,
    height: f64,
) {
    // Draw a simple trend line
    context.set_stroke_style(&"#2196F3".into());
    context.set_line_width(3.0);
    context.begin_path();
    context.move_to(50.0, height - 50.0);
    
    let points = [(100.0, height - 100.0), (200.0, height - 150.0), (300.0, height - 120.0), (400.0, height - 180.0)];
    for (x, y) in points.iter() {
        context.line_to(*x, *y);
    }
    
    context.stroke();
    
    // Draw title
    context.set_fill_style(&"#000000".into());
    context.set_font("bold 16px Arial");
    context.set_text_align("center");
    context.set_text_baseline("top");
    context.fill_text("Interconnection Trend Over Time", width / 2.0, 20.0).unwrap();
}

/// Draw quantitative-style visualization (placeholder)
fn draw_quantitative_viz(
    context: &CanvasRenderingContext2d,
    data: &Vec<ImpactInterconnection>,
    width: f64,
    height: f64,
) {
    // Draw a data table representation
    context.set_fill_style(&"#000000".into());
    context.set_font("14px Arial");
    context.set_text_align("left");
    context.set_text_baseline("middle");
    
    let row_height = 30.0;
    let start_y = 50.0;
    
    // Draw headers
    context.fill_text("Source", 50.0, start_y).unwrap();
    context.fill_text("Target", 200.0, start_y).unwrap();
    context.fill_text("Strength", 350.0, start_y).unwrap();
    
    // Draw data rows
    for (i, interconnection) in data.iter().enumerate() {
        let y = start_y + ((i + 1) as f64) * row_height;
        
        context.fill_text(
            &format!("{:?}", interconnection.source_domain),
            50.0,
            y
        ).unwrap();
        
        context.fill_text(
            &format!("{:?}", interconnection.target_domain),
            200.0,
            y
        ).unwrap();
        
        context.fill_text(
            &format!("{:.1}%", interconnection.strength * 100.0),
            350.0,
            y
        ).unwrap();
    }
}

/// Draw qualitative-style visualization (placeholder)
fn draw_qualitative_viz(
    context: &CanvasRenderingContext2d,
    data: &Vec<ImpactInterconnection>,
    width: f64,
    height: f64,
) {
    // Draw narrative descriptions
    context.set_fill_style(&"#000000".into());
    context.set_font("14px Arial");
    context.set_text_align("left");
    context.set_text_baseline("top");
    
    let line_height = 25.0;
    let start_y = 50.0;
    
    for (i, interconnection) in data.iter().enumerate() {
        let y = start_y + (i as f64) * line_height * 3.0;
        
        // Draw connection description
        context.fill_text(
            &format!("{:?} → {:?}: {}", 
                    interconnection.source_domain, 
                    interconnection.target_domain,
                    interconnection.description),
            50.0,
            y
        ).unwrap();
        
        // Draw strength indicator
        context.fill_text(
            &format!("Strength: {:.1}%", interconnection.strength * 100.0),
            70.0,
            y + line_height
        ).unwrap();
    }
}