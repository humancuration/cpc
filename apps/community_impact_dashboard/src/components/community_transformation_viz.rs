//! Community Transformation Metrics Visualization
//!
//! This component visualizes community wellbeing indicators and transformation metrics
//! showing collective progress toward cooperative goals.

use yew::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use chrono::{DateTime, Utc};

use impact_viz::core::VisualizationStyle;
use crate::models::{CommunityWellbeing, CooperativeGoalProgress, WellbeingProgressPoint};

/// Properties for the CommunityTransformationMetrics component
#[derive(Properties, PartialEq)]
pub struct CommunityTransformationMetricsProps {
    /// Community wellbeing data to visualize
    pub wellbeing: CommunityWellbeing,
    
    /// Visualization style
    pub style: VisualizationStyle,
}

/// State for the CommunityTransformationMetrics component
#[derive(Clone, PartialEq)]
pub struct CommunityTransformationMetricsState {
    /// Canvas reference
    canvas_ref: NodeRef,
}

/// Community Transformation Metrics Component
#[function_component(CommunityTransformationMetrics)]
pub fn community_transformation_metrics(props: &CommunityTransformationMetricsProps) -> Html {
    let state = use_state(|| CommunityTransformationMetricsState {
        canvas_ref: NodeRef::default(),
    });
    
    // Draw the visualization when data or style changes
    {
        let state = state.clone();
        let wellbeing = props.wellbeing.clone();
        let style = props.style.clone();
        
        use_effect_with((wellbeing, style), move |_| {
            let canvas_ref = state.canvas_ref.clone();
            
            // Draw the visualization
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                draw_transformation_viz(&canvas, &props.wellbeing, &props.style);
            }
            
            || ()
        });
    }
    
    html! {
        <div class="community-transformation-metrics">
            <h3>{"Community Transformation Metrics"}</h3>
            <div class="transformation-summary">
                <div class="overall-score">
                    <h4>{"Overall Community Wellbeing"}</h4>
                    <div class="score-display">
                        <span class="score-value">{format!("{:.1}%", props.wellbeing.overall_score * 100.0)}</span>
                    </div>
                </div>
                <div class="goals-progress">
                    <h4>{"Cooperative Goals Progress"}</h4>
                    {for props.wellbeing.cooperative_goals_progress.iter().map(|goal| {
                        html! {
                            <div class="goal-progress">
                                <div class="goal-header">
                                    <span class="goal-title">{&goal.title}</span>
                                    <span class="goal-progress-value">{format!("{:.0}%", goal.progress * 100.0)}</span>
                                </div>
                                <div class="progress-bar">
                                    <div 
                                        class="progress-fill" 
                                        style={format!("width: {}%", goal.progress * 100.0)}
                                    ></div>
                                </div>
                            </div>
                        }
                    })}
                </div>
            </div>
            <canvas 
                ref={state.canvas_ref.clone()} 
                id="transformation-canvas" 
                width="800" 
                height="400"
            />
            <div class="domain-wellbeing">
                <h4>{"Wellbeing by Domain"}</h4>
                <div class="wellbeing-grid">
                    <WellbeingCard 
                        domain="Learning" 
                        score={props.wellbeing.domain_indicators.learning.knowledge_sharing_rate}
                        icon="📚"
                    />
                    <WellbeingCard 
                        domain="Volunteer" 
                        score={props.wellbeing.domain_indicators.volunteer.participation_rate}
                        icon="🤝"
                    />
                    <WellbeingCard 
                        domain="Financial" 
                        score={props.wellbeing.domain_indicators.financial.financial_health}
                        icon="💰"
                    />
                    <WellbeingCard 
                        domain="Cause" 
                        score={props.wellbeing.domain_indicators.cause.engagement_rate}
                        icon="🌍"
                    />
                </div>
            </div>
        </div>
    }
}

/// Properties for the WellbeingCard component
#[derive(Properties, PartialEq)]
pub struct WellbeingCardProps {
    pub domain: String,
    pub score: f64,
    pub icon: String,
}

/// Wellbeing Card Component
#[function_component(WellbeingCard)]
fn wellbeing_card(props: &WellbeingCardProps) -> Html {
    html! {
        <div class="wellbeing-card">
            <div class="wellbeing-icon">{&props.icon}</div>
            <div class="wellbeing-content">
                <h5>{&props.domain}</h5>
                <div class="wellbeing-score">
                    {format!("{:.1}%", props.score * 100.0)}
                </div>
            </div>
        </div>
    }
}

/// Draw the transformation visualization on the canvas
fn draw_transformation_viz(
    canvas: &HtmlCanvasElement,
    wellbeing: &CommunityWellbeing,
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
            draw_narrative_transformation_viz(&context, wellbeing, width, height);
        }
        VisualizationStyle::Comparative => {
            draw_comparative_transformation_viz(&context, wellbeing, width, height);
        }
        VisualizationStyle::TrendBased => {
            draw_trend_transformation_viz(&context, wellbeing, width, height);
        }
        VisualizationStyle::Quantitative => {
            draw_quantitative_transformation_viz(&context, wellbeing, width, height);
        }
        VisualizationStyle::Qualitative => {
            draw_qualitative_transformation_viz(&context, wellbeing, width, height);
        }
    }
}

/// Draw narrative-style transformation visualization
fn draw_narrative_transformation_viz(
    context: &CanvasRenderingContext2d,
    wellbeing: &CommunityWellbeing,
    width: f64,
    height: f64,
) {
    // Draw a community tree visualization
    let center_x = width / 2.0;
    let ground_y = height - 50.0;
    
    // Draw ground
    context.set_fill_style(&"#8BC34A".into());
    context.fill_rect(0.0, ground_y, width, 50.0);
    
    // Draw tree trunk
    let trunk_width = 30.0;
    let trunk_height = 150.0;
    context.set_fill_style(&"#795548".into());
    context.fill_rect(center_x - trunk_width / 2.0, ground_y - trunk_height, trunk_width, trunk_height);
    
    // Draw tree canopy based on overall wellbeing
    let canopy_radius = 80.0 + (wellbeing.overall_score * 70.0);
    let gradient = context.create_radial_gradient(
        center_x, ground_y - trunk_height - 20.0, 0.0,
        center_x, ground_y - trunk_height - 20.0, canopy_radius
    ).unwrap();
    
    gradient.add_color_stop(0.0, "#4CAF50").unwrap();
    gradient.add_color_stop(1.0, "#2E7D32").unwrap();
    context.set_fill_style(&gradient);
    context.begin_path();
    context.arc(
        center_x, 
        ground_y - trunk_height - 20.0, 
        canopy_radius, 
        0.0, 
        2.0 * std::f64::consts::PI
    ).unwrap();
    context.fill();
    
    // Draw domain fruits on the tree
    let domains = [
        ("Learning", "#FFC107", wellbeing.domain_indicators.learning.knowledge_sharing_rate),
        ("Volunteer", "#2196F3", wellbeing.domain_indicators.volunteer.participation_rate),
        ("Financial", "#FF9800", wellbeing.domain_indicators.financial.financial_health),
        ("Cause", "#9C27B0", wellbeing.domain_indicators.cause.engagement_rate),
    ];
    
    for (i, (domain, color, score)) in domains.iter().enumerate() {
        let angle = (i as f64) * (2.0 * std::f64::consts::PI / 4.0) - std::f64::consts::PI / 4.0;
        let fruit_radius = 10.0 + (score * 15.0);
        let fruit_x = center_x + (canopy_radius - 30.0) * angle.cos();
        let fruit_y = ground_y - trunk_height - 20.0 + (canopy_radius - 30.0) * angle.sin();
        
        context.set_fill_style(&(*color).into());
        context.begin_path();
        context.arc(fruit_x, fruit_y, fruit_radius, 0.0, 2.0 * std::f64::consts::PI).unwrap();
        context.fill();
        
        // Draw domain initial
        context.set_fill_style(&"#FFFFFF".into());
        context.set_font("bold 12px Arial");
        context.set_text_align("center");
        context.set_text_baseline("middle");
        context.fill_text(&domain[..1], fruit_x, fruit_y).unwrap();
    }
    
    // Draw title
    context.set_fill_style(&"#000000".into());
    context.set_font("bold 18px Arial");
    context.set_text_align("center");
    context.set_text_baseline("top");
    context.fill_text("Community Transformation Tree", width / 2.0, 20.0).unwrap();
    
    // Draw wellbeing score
    context.set_font("16px Arial");
    context.fill_text(
        &format!("Overall Wellbeing: {:.1}%", wellbeing.overall_score * 100.0),
        width / 2.0,
        50.0
    ).unwrap();
}

/// Draw comparative-style transformation visualization
fn draw_comparative_transformation_viz(
    context: &CanvasRenderingContext2d,
    wellbeing: &CommunityWellbeing,
    width: f64,
    height: f64,
) {
    // Draw radar chart of domain wellbeing
    let center_x = width / 2.0;
    let center_y = height / 2.0;
    let max_radius = f64::min(width, height) / 3.0;
    
    let domains = [
        ("Learning", wellbeing.domain_indicators.learning.knowledge_sharing_rate),
        ("Volunteer", wellbeing.domain_indicators.volunteer.participation_rate),
        ("Financial", wellbeing.domain_indicators.financial.financial_health),
        ("Cause", wellbeing.domain_indicators.cause.engagement_rate),
    ];
    
    // Draw grid circles
    context.set_stroke_style(&"#E0E0E0".into());
    context.set_line_width(1.0);
    for i in 1..=5 {
        let radius = (i as f64) * max_radius / 5.0;
        context.begin_path();
        context.arc(center_x, center_y, radius, 0.0, 2.0 * std::f64::consts::PI).unwrap();
        context.stroke();
    }
    
    // Draw axes
    context.set_stroke_style(&"#9E9E9E".into());
    for (i, _) in domains.iter().enumerate() {
        let angle = (i as f64) * (2.0 * std::f64::consts::PI / 4.0) - std::f64::consts::PI / 2.0;
        let x = center_x + max_radius * angle.cos();
        let y = center_y + max_radius * angle.sin();
        
        context.begin_path();
        context.move_to(center_x, center_y);
        context.line_to(x, y);
        context.stroke();
    }
    
    // Draw wellbeing polygon
    context.set_fill_style(&"rgba(33, 150, 243, 0.3)".into());
    context.set_stroke_style(&"#2196F3".into());
    context.set_line_width(2.0);
    context.begin_path();
    
    for (i, (_, score)) in domains.iter().enumerate() {
        let angle = (i as f64) * (2.0 * std::f64::consts::PI / 4.0) - std::f64::consts::PI / 2.0;
        let radius = score * max_radius;
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        
        if i == 0 {
            context.move_to(x, y);
        } else {
            context.line_to(x, y);
        }
    }
    
    context.close_path();
    context.fill();
    context.stroke();
    
    // Draw domain labels
    context.set_fill_style(&"#000000".into());
    context.set_font("12px Arial");
    context.set_text_align("center");
    context.set_text_baseline("middle");
    
    for (i, (domain, _)) in domains.iter().enumerate() {
        let angle = (i as f64) * (2.0 * std::f64::consts::PI / 4.0) - std::f64::consts::PI / 2.0;
        let x = center_x + (max_radius + 30.0) * angle.cos();
        let y = center_y + (max_radius + 30.0) * angle.sin();
        
        context.fill_text(domain, x, y).unwrap();
    }
    
    // Draw title
    context.set_font("bold 16px Arial");
    context.set_text_align("center");
    context.set_text_baseline("top");
    context.fill_text("Community Wellbeing by Domain", width / 2.0, 20.0).unwrap();
}

/// Draw trend-style transformation visualization
fn draw_trend_transformation_viz(
    context: &CanvasRenderingContext2d,
    wellbeing: &CommunityWellbeing,
    width: f64,
    height: f64,
) {
    // Draw historical progress line chart
    let padding = 50.0;
    let chart_width = width - 2.0 * padding;
    let chart_height = height - 2.0 * padding - 50.0;
    
    // Draw axes
    context.set_stroke_style(&"#000000".into());
    context.set_line_width(2.0);
    context.begin_path();
    context.move_to(padding, padding);
    context.line_to(padding, height - padding);
    context.line_to(width - padding, height - padding);
    context.stroke();
    
    // Draw grid lines
    context.set_stroke_style(&"#E0E0E0".into());
    context.set_line_width(1.0);
    for i in 1..5 {
        let y = padding + (i as f64) * chart_height / 5.0;
        context.begin_path();
        context.move_to(padding, y);
        context.line_to(width - padding, y);
        context.stroke();
    }
    
    // Draw historical data points
    if !wellbeing.historical_progress.is_empty() {
        context.set_stroke_style(&"#2196F3".into());
        context.set_line_width(3.0);
        context.set_fill_style(&"#2196F3".into());
        context.begin_path();
        
        let point_count = wellbeing.historical_progress.len();
        for (i, point) in wellbeing.historical_progress.iter().enumerate() {
            let x = padding + (i as f64) * chart_width / (point_count as f64 - 1.0);
            let y = height - padding - (point.overall_score * chart_height);
            
            if i == 0 {
                context.move_to(x, y);
            } else {
                context.line_to(x, y);
            }
            
            // Draw point
            context.begin_path();
            context.arc(x, y, 5.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
            context.fill();
        }
        
        context.stroke();
    }
    
    // Draw current point
    let current_x = width - padding;
    let current_y = height - padding - (wellbeing.overall_score * chart_height);
    context.set_fill_style(&"#4CAF50".into());
    context.begin_path();
    context.arc(current_x, current_y, 8.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
    context.fill();
    
    // Draw labels
    context.set_fill_style(&"#000000".into());
    context.set_font("12px Arial");
    context.set_text_align("center");
    context.set_text_baseline("top");
    
    // Y-axis labels
    for i in 0..=5 {
        let y = height - padding - (i as f64) * chart_height / 5.0;
        let value = (i as f64) * 0.2;
        context.fill_text(&format!("{:.0}%", value * 100.0), padding - 20.0, y - 6.0).unwrap();
    }
    
    // X-axis labels
    if !wellbeing.historical_progress.is_empty() {
        let dates: Vec<DateTime<Utc>> = wellbeing.historical_progress.iter()
            .map(|p| p.timestamp)
            .collect();
        
        if dates.len() > 1 {
            context.fill_text(
                &format!("{}", dates[0].format("%m/%d")),
                padding,
                height - padding + 10.0
            ).unwrap();
            
            context.fill_text(
                &format!("{}", dates[dates.len()-1].format("%m/%d")),
                width - padding,
                height - padding + 10.0
            ).unwrap();
        }
    }
    
    // Draw title
    context.set_font("bold 16px Arial");
    context.set_text_align("center");
    context.set_text_baseline("top");
    context.fill_text("Community Wellbeing Trend", width / 2.0, 20.0).unwrap();
}

/// Draw quantitative-style transformation visualization
fn draw_quantitative_transformation_viz(
    context: &CanvasRenderingContext2d,
    wellbeing: &CommunityWellbeing,
    width: f64,
    height: f64,
) {
    // Draw a detailed metrics table
    context.set_fill_style(&"#000000".into());
    context.set_font("14px Arial");
    context.set_text_align("left");
    context.set_text_baseline("middle");
    
    let row_height = 25.0;
    let start_y = 50.0;
    let col_widths = [150.0, 100.0, 100.0, 100.0, 100.0];
    
    // Draw headers
    let headers = ["Metric", "Learning", "Volunteer", "Financial", "Cause"];
    for (i, header) in headers.iter().enumerate() {
        context.fill_text(
            header,
            50.0 + col_widths[..i].iter().sum::<f64>(),
            start_y
        ).unwrap();
    }
    
    // Draw data rows
    let metrics = [
        ("Knowledge/Participation", 
         wellbeing.domain_indicators.learning.knowledge_sharing_rate,
         wellbeing.domain_indicators.volunteer.participation_rate,
         wellbeing.domain_indicators.financial.financial_health,
         wellbeing.domain_indicators.cause.engagement_rate),
        ("Skill Development", 
         wellbeing.domain_indicators.learning.skill_development_progress,
         wellbeing.domain_indicators.volunteer.satisfaction_index,
         wellbeing.domain_indicators.financial.resource_equity,
         wellbeing.domain_indicators.cause.impact_effectiveness),
        ("Community Satisfaction", 
         wellbeing.domain_indicators.learning.community_satisfaction,
         wellbeing.domain_indicators.volunteer.satisfaction_index,
         wellbeing.domain_indicators.financial.sustainability_index,
         wellbeing.domain_indicators.cause.solidarity_index),
    ];
    
    for (i, (metric, learning, volunteer, financial, cause)) in metrics.iter().enumerate() {
        let y = start_y + ((i + 1) as f64) * row_height;
        
        context.fill_text(
            metric,
            50.0,
            y
        ).unwrap();
        
        context.fill_text(
            &format!("{:.1}%", learning * 100.0),
            50.0 + col_widths[0],
            y
        ).unwrap();
        
        context.fill_text(
            &format!("{:.1}%", volunteer * 100.0),
            50.0 + col_widths[0] + col_widths[1],
            y
        ).unwrap();
        
        context.fill_text(
            &format!("{:.1}%", financial * 100.0),
            50.0 + col_widths[0] + col_widths[1] + col_widths[2],
            y
        ).unwrap();
        
        context.fill_text(
            &format!("{:.1}%", cause * 100.0),
            50.0 + col_widths[0] + col_widths[1] + col_widths[2] + col_widths[3],
            y
        ).unwrap();
    }
    
    // Draw overall score
    let y = start_y + ((metrics.len() + 2) as f64) * row_height;
    context.set_font("bold 14px Arial");
    context.fill_text("Overall Wellbeing", 50.0, y).unwrap();
    context.fill_text(
        &format!("{:.1}%", wellbeing.overall_score * 100.0),
        50.0 + col_widths[0],
        y
    ).unwrap();
    
    // Draw title
    context.set_font("bold 16px Arial");
    context.set_text_align("center");
    context.set_text_baseline("top");
    context.fill_text("Community Wellbeing Metrics", width / 2.0, 20.0).unwrap();
}

/// Draw qualitative-style transformation visualization
fn draw_qualitative_transformation_viz(
    context: &CanvasRenderingContext2d,
    wellbeing: &CommunityWellbeing,
    width: f64,
    height: f64,
) {
    // Draw narrative descriptions of community transformation
    context.set_fill_style(&"#000000".into());
    context.set_font("14px Arial");
    context.set_text_align("left");
    context.set_text_baseline("top");
    
    let line_height = 25.0;
    let start_y = 50.0;
    
    // Draw overall wellbeing narrative
    context.fill_text(
        &format!("Our community's overall wellbeing score is {:.1}%, reflecting our collective progress across all domains.", 
                wellbeing.overall_score * 100.0),
        50.0,
        start_y
    ).unwrap();
    
    // Draw domain narratives
    let domains = [
        ("Learning", 
         wellbeing.domain_indicators.learning.knowledge_sharing_rate,
         "Our learning community is thriving with a knowledge sharing rate of {:.1}%. This reflects our commitment to education and skill development for all members."),
        ("Volunteer", 
         wellbeing.domain_indicators.volunteer.participation_rate,
         "Volunteer participation stands at {:.1}%, showing strong community engagement in service activities."),
        ("Financial", 
         wellbeing.domain_indicators.financial.financial_health,
         "Our financial health score of {:.1}% indicates sustainable resource management and equitable distribution."),
        ("Cause", 
         wellbeing.domain_indicators.cause.engagement_rate,
         "Cause engagement at {:.1}% demonstrates our community's commitment to social impact and justice."),
    ];
    
    for (i, (domain, score, description)) in domains.iter().enumerate() {
        let y = start_y + ((i + 1) as f64) * line_height * 3.0;
        
        context.set_font("bold 14px Arial");
        context.fill_text(
            &format!("{} Domain: {:.1}%", domain, score * 100.0),
            50.0,
            y
        ).unwrap();
        
        context.set_font("14px Arial");
        context.fill_text(
            &format!(description, score * 100.0),
            70.0,
            y + line_height
        ).unwrap();
    }
    
    // Draw title
    context.set_font("bold 16px Arial");
    context.set_text_align("center");
    context.set_text_baseline("top");
    context.fill_text("Community Transformation Narrative", width / 2.0, 20.0).unwrap();
}