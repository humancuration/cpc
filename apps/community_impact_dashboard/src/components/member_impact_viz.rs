//! Member Impact Visualization
//!
//! This component visualizes individual member impact within the community ecosystem
//! and provides personalized suggestions for optimizing community impact.

use yew::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

use impact_viz::core::VisualizationStyle;
use learning_impact_tracker::tracker::ImpactMetrics as LearningMetrics;
use volunteer_impact_tracker::tracker::ImpactMetrics as VolunteerMetrics;
use financial_impact_tracker::tracker::FinancialImpactRecord;
use cause_impact_tracker::tracker::ImpactMetrics as CauseMetrics;
use crate::models::impact_data::{MemberImpactData, EcosystemPosition, ContributionImpact, ImpactSuggestion, ImpactEvolution};

/// Properties for the MemberImpactView component
#[derive(Properties, PartialEq)]
pub struct MemberImpactViewProps {
    /// Learning metrics
    pub learning_metrics: LearningMetrics,
    
    /// Volunteer metrics
    pub volunteer_metrics: VolunteerMetrics,
    
    /// Financial metrics
    pub financial_metrics: Vec<FinancialImpactRecord>,
    
    /// Cause metrics
    pub cause_metrics: CauseMetrics,
    
    /// Visualization style
    pub style: VisualizationStyle,
}

/// State for the MemberImpactView component
#[derive(Clone, PartialEq)]
pub struct MemberImpactViewState {
    /// Canvas reference
    canvas_ref: NodeRef,
}

/// Member Impact View Component
#[function_component(MemberImpactView)]
pub fn member_impact_view(props: &MemberImpactViewProps) -> Html {
    let state = use_state(|| MemberImpactViewState {
        canvas_ref: NodeRef::default(),
    });
    
    // For demonstration purposes, we'll create mock member data
    // In a real implementation, this would come from the backend
    let member_data = create_mock_member_data();
    
    // Draw the visualization when data or style changes
    {
        let state = state.clone();
        let member_data = member_data.clone();
        let style = props.style.clone();
        
        use_effect_with((member_data, style), move |_| {
            let canvas_ref = state.canvas_ref.clone();
            
            // Draw the visualization
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                draw_member_impact_viz(&canvas, &member_data, &props.style);
            }
            
            || ()
        });
    }
    
    html! {
        <div class="member-impact-view">
            <h3>{"Your Position in the Community Ecosystem"}</h3>
            <EcosystemPositionVisualization position={member_data.ecosystem_position.clone()} />
            
            <h3>{"Your Contribution Impact"}</h3>
            <ContributionImpactVisualization contribution={member_data.contribution_impact.clone()} />
            
            <h3>{"Personalized Impact Suggestions"}</h3>
            <ImpactSuggestionsList suggestions={member_data.impact_suggestions.clone()} />
            
            <h3>{"Your Impact Evolution"}</h3>
            <ImpactEvolutionVisualization 
                evolution={member_data.impact_evolution.clone()} 
                canvas_ref={state.canvas_ref.clone()}
            />
        </div>
    }
}

/// Properties for the EcosystemPositionVisualization component
#[derive(Properties, PartialEq)]
pub struct EcosystemPositionVisualizationProps {
    pub position: EcosystemPosition,
}

/// Ecosystem Position Visualization Component
#[function_component(EcosystemPositionVisualization)]
fn ecosystem_position_visualization(props: &EcosystemPositionVisualizationProps) -> Html {
    html! {
        <div class="ecosystem-position">
            <div class="position-grid">
                <PositionCard 
                    domain="Learning" 
                    level={props.position.learning_engagement}
                    color="#4CAF50"
                />
                <PositionCard 
                    domain="Volunteer" 
                    level={props.position.volunteer_participation}
                    color="#2196F3"
                />
                <PositionCard 
                    domain="Financial" 
                    level={props.position.financial_participation}
                    color="#FF9800"
                />
                <PositionCard 
                    domain="Cause" 
                    level={props.position.cause_engagement}
                    color="#9C27B0"
                />
            </div>
            <div class="community-connectivity">
                <h4>{"Community Connectivity"}</h4>
                <div class="connectivity-meter">
                    <div class="connectivity-fill" style={format!("width: {}%", props.position.community_connectivity * 100.0)}></div>
                </div>
                <div class="connectivity-value">
                    {format!("{:.1}%", props.position.community_connectivity * 100.0)}
                </div>
            </div>
        </div>
    }
}

/// Properties for the PositionCard component
#[derive(Properties, PartialEq)]
pub struct PositionCardProps {
    pub domain: String,
    pub level: f64,
    pub color: String,
}

/// Position Card Component
#[function_component(PositionCard)]
fn position_card(props: &PositionCardProps) -> Html {
    html! {
        <div class="position-card">
            <div class="domain-header">
                <h5>{&props.domain}</h5>
            </div>
            <div class="level-meter">
                <div 
                    class="level-fill" 
                    style={format!("width: {}%; background-color: {}", props.level * 100.0, props.color)}
                ></div>
            </div>
            <div class="level-value">
                {format!("{:.1}%", props.level * 100.0)}
            </div>
        </div>
    }
}

/// Properties for the ContributionImpactVisualization component
#[derive(Properties, PartialEq)]
pub struct ContributionImpactVisualizationProps {
    pub contribution: ContributionImpact,
}

/// Contribution Impact Visualization Component
#[function_component(ContributionImpactVisualization)]
fn contribution_impact_visualization(props: &ContributionImpactVisualizationProps) -> Html {
    html! {
        <div class="contribution-impact">
            <div class="contribution-grid">
                <ContributionCard 
                    domain="Learning" 
                    impact={props.contribution.learning_contribution}
                    description="Knowledge shared with community"
                />
                <ContributionCard 
                    domain="Volunteer" 
                    impact={props.contribution.volunteer_contribution}
                    description="Service provided to community"
                />
                <ContributionCard 
                    domain="Financial" 
                    impact={props.contribution.financial_contribution}
                    description="Resources contributed"
                />
                <ContributionCard 
                    domain="Cause" 
                    impact={props.contribution.cause_contribution}
                    description="Social impact created"
                />
            </div>
            <div class="multiplier-effect">
                <h4>{"Overall Impact Multiplier"}</h4>
                <div class="multiplier-display">
                    <span class="multiplier-value">
                        {format!("{:.1}x", props.contribution.multiplier_effect * 5.0 + 1.0)}
                    </span>
                    <span class="multiplier-description">
                        {"Your actions amplify community impact"}
                    </span>
                </div>
            </div>
        </div>
    }
}

/// Properties for the ContributionCard component
#[derive(Properties, PartialEq)]
pub struct ContributionCardProps {
    pub domain: String,
    pub impact: f64,
    pub description: String,
}

/// Contribution Card Component
#[function_component(ContributionCard)]
fn contribution_card(props: &ContributionCardProps) -> Html {
    html! {
        <div class="contribution-card">
            <div class="contribution-header">
                <h5>{&props.domain}</h5>
            </div>
            <div class="impact-meter">
                <div class="impact-fill" style={format!("width: {}%", props.impact * 100.0)}></div>
            </div>
            <div class="impact-value">
                {format!("{:.1}%", props.impact * 100.0)}
            </div>
            <div class="impact-description">
                {&props.description}
            </div>
        </div>
    }
}

/// Properties for the ImpactSuggestionsList component
#[derive(Properties, PartialEq)]
pub struct ImpactSuggestionsListProps {
    pub suggestions: Vec<ImpactSuggestion>,
}

/// Impact Suggestions List Component
#[function_component(ImpactSuggestionsList)]
fn impact_suggestions_list(props: &ImpactSuggestionsListProps) -> Html {
    html! {
        <div class="impact-suggestions">
            {for props.suggestions.iter().map(|suggestion| {
                html! {
                    <div class="suggestion-card">
                        <div class="suggestion-header">
                            <h4>{&suggestion.title}</h4>
                            <div class="suggestion-priority">
                                {match suggestion.priority {
                                    crate::models::impact_data::PriorityLevel::Low => "Low Priority",
                                    crate::models::impact_data::PriorityLevel::Medium => "Medium Priority",
                                    crate::models::impact_data::PriorityLevel::High => "High Priority",
                                    crate::models::impact_data::PriorityLevel::Critical => "Critical Priority",
                                }}
                            </div>
                        </div>
                        <div class="suggestion-domain">
                            {format!("Domain: {:?}", suggestion.domain)}
                        </div>
                        <div class="suggestion-description">
                            {&suggestion.description}
                        </div>
                        <div class="suggestion-details">
                            <span class="expected-impact">
                                {format!("Expected Impact: {:.1}%", suggestion.expected_impact * 100.0)}
                            </span>
                            <span class="difficulty">
                                {match suggestion.difficulty {
                                    crate::models::impact_data::DifficultyLevel::Easy => "Easy",
                                    crate::models::impact_data::DifficultyLevel::Medium => "Medium",
                                    crate::models::impact_data::DifficultyLevel::Hard => "Hard",
                                }}
                            </span>
                        </div>
                        <button class="btn btn-primary">{"Explore This Suggestion"}</button>
                    </div>
                }
            })}
        </div>
    }
}

/// Properties for the ImpactEvolutionVisualization component
#[derive(Properties, PartialEq)]
pub struct ImpactEvolutionVisualizationProps {
    pub evolution: ImpactEvolution,
    pub canvas_ref: NodeRef,
}

/// Impact Evolution Visualization Component
#[function_component(ImpactEvolutionVisualization)]
fn impact_evolution_visualization(props: &ImpactEvolutionVisualizationProps) -> Html {
    html! {
        <div class="impact-evolution">
            <canvas 
                ref={props.canvas_ref.clone()} 
                id="evolution-canvas" 
                width="800" 
                height="300"
            />
            <div class="milestones">
                <h4>{"Impact Milestones"}</h4>
                {for props.evolution.milestones.iter().map(|milestone| {
                    html! {
                        <div class="milestone">
                            <div class="milestone-date">
                                {milestone.timestamp.format("%Y-%m-%d").to_string()}
                            </div>
                            <div class="milestone-content">
                                <h5>{&milestone.title}</h5>
                                <p>{&milestone.description}</p>
                                <div class="celebration">
                                    {&milestone.celebration_message}
                                </div>
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}

/// Draw the member impact visualization on the canvas
fn draw_member_impact_viz(
    canvas: &HtmlCanvasElement,
    member_data: &MemberImpactData,
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
            draw_narrative_member_viz(&context, member_data, width, height);
        }
        VisualizationStyle::Comparative => {
            draw_comparative_member_viz(&context, member_data, width, height);
        }
        VisualizationStyle::TrendBased => {
            draw_trend_member_viz(&context, member_data, width, height);
        }
        VisualizationStyle::Quantitative => {
            draw_quantitative_member_viz(&context, member_data, width, height);
        }
        VisualizationStyle::Qualitative => {
            draw_qualitative_member_viz(&context, member_data, width, height);
        }
    }
}

/// Draw narrative-style member impact visualization
fn draw_narrative_member_viz(
    context: &CanvasRenderingContext2d,
    member_data: &MemberImpactData,
    width: f64,
    height: f64,
) {
    // Draw a personal journey map
    let padding = 50.0;
    let timeline_y = height / 2.0;
    
    // Draw timeline
    context.set_stroke_style(&"#9E9E9E".into());
    context.set_line_width(2.0);
    context.begin_path();
    context.move_to(padding, timeline_y);
    context.line_to(width - padding, timeline_y);
    context.stroke();
    
    // Draw milestone markers
    for (i, milestone) in member_data.impact_evolution.milestones.iter().enumerate() {
        let x = padding + (i as f64) * (width - 2.0 * padding) / (member_data.impact_evolution.milestones.len() as f64 - 1.0);
        
        // Draw milestone point
        context.set_fill_style(&"#2196F3".into());
        context.begin_path();
        context.arc(x, timeline_y, 8.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
        context.fill();
        
        // Draw milestone label
        context.set_fill_style(&"#000000".into());
        context.set_font("12px Arial");
        context.set_text_align("center");
        context.set_text_baseline("bottom");
        context.fill_text(&milestone.title, x, timeline_y - 15.0).unwrap();
    }
    
    // Draw current position
    let current_x = width - padding;
    context.set_fill_style(&"#4CAF50".into());
    context.begin_path();
    context.arc(current_x, timeline_y, 12.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
    context.fill();
    
    context.set_fill_style(&"#000000".into());
    context.set_font("bold 14px Arial");
    context.set_text_align("center");
    context.set_text_baseline("top");
    context.fill_text("You Are Here", current_x, timeline_y + 20.0).unwrap();
    
    // Draw title
    context.set_font("bold 16px Arial");
    context.set_text_align("center");
    context.set_text_baseline("top");
    context.fill_text("Your Impact Journey", width / 2.0, 20.0).unwrap();
}

/// Draw comparative-style member impact visualization
fn draw_comparative_member_viz(
    context: &CanvasRenderingContext2d,
    member_data: &MemberImpactData,
    width: f64,
    height: f64,
) {
    // Draw radar chart of current domain levels
    let center_x = width / 2.0;
    let center_y = height / 2.0;
    let max_radius = f64::min(width, height) / 3.0;
    
    let domains = [
        ("Learning", "#4CAF50", member_data.impact_evolution.current_levels.learning),
        ("Volunteer", "#2196F3", member_data.impact_evolution.current_levels.volunteer),
        ("Financial", "#FF9800", member_data.impact_evolution.current_levels.financial),
        ("Cause", "#9C27B0", member_data.impact_evolution.current_levels.cause),
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
    
    // Draw current levels polygon
    context.set_fill_style(&"rgba(33, 150, 243, 0.3)".into());
    context.set_stroke_style(&"#2196F3".into());
    context.set_line_width(2.0);
    context.begin_path();
    
    for (i, (_, _, level)) in domains.iter().enumerate() {
        let angle = (i as f64) * (2.0 * std::f64::consts::PI / 4.0) - std::f64::consts::PI / 2.0;
        let radius = level * max_radius;
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
    
    for (i, (domain, _, _)) in domains.iter().enumerate() {
        let angle = (i as f64) * (2.0 * std::f64::consts::PI / 4.0) - std::f64::consts::PI / 2.0;
        let x = center_x + (max_radius + 30.0) * angle.cos();
        let y = center_y + (max_radius + 30.0) * angle.sin();
        
        context.fill_text(domain, x, y).unwrap();
    }
    
    // Draw title
    context.set_font("bold 16px Arial");
    context.set_text_align("center");
    context.set_text_baseline("top");
    context.fill_text("Your Current Impact Levels", width / 2.0, 20.0).unwrap();
}

/// Draw trend-style member impact visualization
fn draw_trend_member_viz(
    context: &CanvasRenderingContext2d,
    member_data: &MemberImpactData,
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
    
    // Draw historical data points for each domain
    let domains = [
        ("Learning", "#4CAF50", &member_data.impact_evolution.historical_progress.iter()
            .map(|p| p.learning_progress).collect::<Vec<f64>>()),
        ("Volunteer", "#2196F3", &member_data.impact_evolution.historical_progress.iter()
            .map(|p| p.volunteer_progress).collect::<Vec<f64>>()),
        ("Financial", "#FF9800", &member_data.impact_evolution.historical_progress.iter()
            .map(|p| p.financial_progress).collect::<Vec<f64>>()),
        ("Cause", "#9C27B0", &member_data.impact_evolution.historical_progress.iter()
            .map(|p| p.cause_progress).collect::<Vec<f64>>()),
    ];
    
    for (domain, color, progress_data) in domains.iter() {
        if !progress_data.is_empty() {
            context.set_stroke_style(&(*color).into());
            context.set_line_width(2.0);
            context.set_fill_style(&(*color).into());
            context.begin_path();
            
            let point_count = progress_data.len();
            for (i, &progress) in progress_data.iter().enumerate() {
                let x = padding + (i as f64) * chart_width / (point_count as f64 - 1.0);
                let y = height - padding - (progress * chart_height);
                
                if i == 0 {
                    context.move_to(x, y);
                } else {
                    context.line_to(x, y);
                }
                
                // Draw point
                context.begin_path();
                context.arc(x, y, 3.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
                context.fill();
            }
            
            context.stroke();
        }
    }
    
    // Draw legend
    context.set_font("12px Arial");
    context.set_text_align("left");
    context.set_text_baseline("middle");
    
    for (i, (domain, color, _)) in domains.iter().enumerate() {
        let y = 20.0 + (i as f64) * 20.0;
        
        context.set_fill_style(&(*color).into());
        context.fill_rect(width - 150.0, y - 5.0, 10.0, 10.0);
        
        context.set_fill_style(&"#000000".into());
        context.fill_text(domain, width - 135.0, y).unwrap();
    }
    
    // Draw title
    context.set_font("bold 16px Arial");
    context.set_text_align("center");
    context.set_text_baseline("top");
    context.fill_text("Your Impact Evolution", width / 2.0, 20.0).unwrap();
}

/// Draw quantitative-style member impact visualization
fn draw_quantitative_member_viz(
    context: &CanvasRenderingContext2d,
    member_data: &MemberImpactData,
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
    let col_widths = [200.0, 150.0];
    
    // Draw headers
    let headers = ["Metric", "Your Level"];
    for (i, header) in headers.iter().enumerate() {
        context.fill_text(
            header,
            50.0 + col_widths[..i].iter().sum::<f64>(),
            start_y
        ).unwrap();
    }
    
    // Draw data rows
    let metrics = [
        ("Learning Engagement", member_data.ecosystem_position.learning_engagement),
        ("Volunteer Participation", member_data.ecosystem_position.volunteer_participation),
        ("Financial Participation", member_data.ecosystem_position.financial_participation),
        ("Cause Engagement", member_data.ecosystem_position.cause_engagement),
        ("Community Connectivity", member_data.ecosystem_position.community_connectivity),
        ("Learning Contribution", member_data.contribution_impact.learning_contribution),
        ("Volunteer Contribution", member_data.contribution_impact.volunteer_contribution),
        ("Financial Contribution", member_data.contribution_impact.financial_contribution),
        ("Cause Contribution", member_data.contribution_impact.cause_contribution),
        ("Impact Multiplier", member_data.contribution_impact.multiplier_effect),
    ];
    
    for (i, (metric, value)) in metrics.iter().enumerate() {
        let y = start_y + ((i + 1) as f64) * row_height;
        
        context.fill_text(
            metric,
            50.0,
            y
        ).unwrap();
        
        context.fill_text(
            &format!("{:.1}%", value * 100.0),
            50.0 + col_widths[0],
            y
        ).unwrap();
    }
    
    // Draw title
    context.set_font("bold 16px Arial");
    context.set_text_align("center");
    context.set_text_baseline("top");
    context.fill_text("Your Impact Metrics", width / 2.0, 20.0).unwrap();
}

/// Draw qualitative-style member impact visualization
fn draw_qualitative_member_viz(
    context: &CanvasRenderingContext2d,
    member_data: &MemberImpactData,
    width: f64,
    height: f64,
) {
    // Draw narrative descriptions of member impact
    context.set_fill_style(&"#000000".into());
    context.set_font("14px Arial");
    context.set_text_align("left");
    context.set_text_baseline("top");
    
    let line_height = 25.0;
    let start_y = 50.0;
    
    // Draw position narrative
    context.fill_text(
        "Your position within the community ecosystem:",
        50.0,
        start_y
    ).unwrap();
    
    let positions = [
        ("Learning Engagement", member_data.ecosystem_position.learning_engagement, 
         "You're actively engaged in learning, which strengthens your ability to contribute across all domains."),
        ("Volunteer Participation", member_data.ecosystem_position.volunteer_participation,
         "Your volunteer work directly serves community needs and builds connections with other members."),
        ("Financial Participation", member_data.ecosystem_position.financial_participation,
         "Your financial contributions help sustain community resources and enable larger initiatives."),
        ("Cause Engagement", member_data.ecosystem_position.cause_engagement,
         "Your involvement in cause work amplifies social impact and advances justice in our community."),
    ];
    
    for (i, (domain, level, description)) in positions.iter().enumerate() {
        let y = start_y + ((i + 1) as f64) * line_height * 2.5;
        
        context.set_font("bold 14px Arial");
        context.fill_text(
            &format!("{}: {:.1}%", domain, level * 100.0),
            50.0,
            y
        ).unwrap();
        
        context.set_font("14px Arial");
        context.fill_text(
            description,
            70.0,
            y + line_height
        ).unwrap();
    }
    
    // Draw contribution narrative
    let contribution_y = start_y + (positions.len() as f64 + 1.0) * line_height * 2.5;
    context.set_font("bold 14px Arial");
    context.fill_text(
        &format!("Your overall impact multiplier is {:.1}x, meaning your actions amplify community impact!", 
                member_data.contribution_impact.multiplier_effect * 5.0 + 1.0),
        50.0,
        contribution_y
    ).unwrap();
    
    // Draw title
    context.set_font("bold 16px Arial");
    context.set_text_align("center");
    context.set_text_baseline("top");
    context.fill_text("Your Impact Narrative", width / 2.0, 20.0).unwrap();
}

/// Create mock member data for demonstration
fn create_mock_member_data() -> MemberImpactData {
    MemberImpactData {
        member_id: "member_123".to_string(),
        ecosystem_position: EcosystemPosition {
            learning_engagement: 0.80,
            volunteer_participation: 0.65,
            financial_participation: 0.55,
            cause_engagement: 0.70,
            community_connectivity: 0.72,
        },
        contribution_impact: ContributionImpact {
            learning_contribution: 0.75,
            volunteer_contribution: 0.60,
            financial_contribution: 0.50,
            cause_contribution: 0.65,
            multiplier_effect: 0.68,
        },
        impact_suggestions: vec![
            ImpactSuggestion {
                id: uuid::Uuid::new_v4(),
                domain: crate::models::impact_data::ImpactDomain::Financial,
                title: "Increase Financial Participation".to_string(),
                description: "Consider contributing to community funds to support cause initiatives you care about".to_string(),
                expected_impact: 0.30,
                difficulty: crate::models::impact_data::DifficultyLevel::Easy,
                priority: crate::models::impact_data::PriorityLevel::Medium,
            },
            ImpactSuggestion {
                id: uuid::Uuid::new_v4(),
                domain: crate::models::impact_data::ImpactDomain::Volunteer,
                title: "Explore New Volunteer Opportunities".to_string(),
                description: "Based on your learning interests, you might enjoy volunteering for our digital literacy program".to_string(),
                expected_impact: 0.25,
                difficulty: crate::models::impact_data::DifficultyLevel::Medium,
                priority: crate::models::impact_data::PriorityLevel::High,
            }
        ],
        impact_evolution: ImpactEvolution {
            milestones: vec![
                crate::models::impact_data::ImpactMilestone {
                    id: uuid::Uuid::new_v4(),
                    timestamp: chrono::Utc::now() - chrono::Duration::days(90),
                    domain: crate::models::impact_data::ImpactDomain::Learning,
                    title: "Learning Milestone".to_string(),
                    description: "Completed first learning pathway".to_string(),
                    celebration_message: "Congratulations on your first learning milestone!".to_string(),
                },
                crate::models::impact_data::ImpactMilestone {
                    id: uuid::Uuid::new_v4(),
                    timestamp: chrono::Utc::now() - chrono::Duration::days(60),
                    domain: crate::models::impact_data::ImpactDomain::Volunteer,
                    title: "Volunteer Recognition".to_string(),
                    description: "Received community recognition for volunteer work".to_string(),
                    celebration_message: "Your service is truly valued by the community!".to_string(),
                }
            ],
            current_levels: crate::models::impact_data::DomainLevels {
                learning: 0.80,
                volunteer: 0.65,
                financial: 0.55,
                cause: 0.70,
            },
            historical_progress: vec![
                crate::models::impact_data::HistoricalProgressPoint {
                    timestamp: chrono::Utc::now() - chrono::Duration::days(90),
                    learning_progress: 0.50,
                    volunteer_progress: 0.30,
                    financial_progress: 0.20,
                    cause_progress: 0.40,
                    community_progress: 0.35,
                },
                crate::models::impact_data::HistoricalProgressPoint {
                    timestamp: chrono::Utc::now() - chrono::Duration::days(60),
                    learning_progress: 0.65,
                    volunteer_progress: 0.45,
                    financial_progress: 0.35,
                    cause_progress: 0.55,
                    community_progress: 0.50,
                },
                crate::models::impact_data::HistoricalProgressPoint {
                    timestamp: chrono::Utc::now() - chrono::Duration::days(30),
                    learning_progress: 0.70,
                    volunteer_progress: 0.55,
                    financial_progress: 0.45,
                    cause_progress: 0.60,
                    community_progress: 0.57,
                },
            ],
        },
    }
}