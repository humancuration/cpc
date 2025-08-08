//! Collaborative Interpreter Component
//!
//! This component enables collaborative interpretation of community data
//! and facilitates community-wide reflection sessions.

use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::models::{UnifiedImpactData, community_wellbeing::CooperativeGoalProgress};
use social_interactions::domain::{Comment, Reaction, ReactionType};
use uuid::Uuid;
use chrono::Utc;

/// Properties for the CollaborativeInterpreter component
#[derive(Properties, PartialEq)]
pub struct CollaborativeInterpreterProps {
    /// Impact data to interpret
    pub impact_data: UnifiedImpactData,
    
    /// Callback when interpretation is submitted
    pub on_interpret: Callback<CommunityInterpretation>,
}

/// State for the CollaborativeInterpreter component
#[derive(Clone, PartialEq)]
pub struct CollaborativeInterpreterState {
    /// Current interpretation step
    current_step: InterpretationStep,
    
    /// Community interpretation
    interpretation: CommunityInterpretation,
    
    /// Discussion comments
    comments: Vec<InterpretationComment>,
}

/// Interpretation steps
#[derive(Clone, PartialEq)]
pub enum InterpretationStep {
    ReviewData,
    SharePerspectives,
    IdentifyPatterns,
    FormConsensus,
    DocumentInsights,
}

/// Community interpretation structure
#[derive(Clone, PartialEq)]
pub struct CommunityInterpretation {
    /// Key insights from the community
    pub insights: Vec<CommunityInsight>,
    
    /// Emerging patterns
    pub patterns: Vec<EmergingPattern>,
    
    /// Consensus areas
    pub consensus_areas: Vec<ConsensusArea>,
    
    /// Divergent perspectives
    pub divergent_views: Vec<DivergentView>,
    
    /// Actionable recommendations
    pub recommendations: Vec<ActionableRecommendation>,
}

/// Community insight structure
#[derive(Clone, PartialEq)]
pub struct CommunityInsight {
    /// Unique identifier
    pub id: Uuid,
    
    /// Insight description
    pub description: String,
    
    /// Supporting evidence
    pub evidence: String,
    
    /// Community members who contributed
    pub contributors: Vec<String>,
    
    /// Timestamp
    pub timestamp: chrono::DateTime<Utc>,
}

/// Emerging pattern structure
#[derive(Clone, PartialEq)]
pub struct EmergingPattern {
    /// Pattern description
    pub description: String,
    
    /// Domains involved
    pub domains: Vec<String>,
    
    /// Strength of pattern
    pub strength: PatternStrength,
    
    /// Supporting data points
    pub data_points: Vec<String>,
}

/// Pattern strength levels
#[derive(Clone, PartialEq)]
pub enum PatternStrength {
    Weak,
    Moderate,
    Strong,
}

/// Consensus area structure
#[derive(Clone, PartialEq)]
pub struct ConsensusArea {
    /// Topic of consensus
    pub topic: String,
    
    /// Consensus statement
    pub statement: String,
    
    /// Percentage agreement
    pub agreement_percentage: f64,
    
    /// Supporting perspectives
    pub perspectives: Vec<String>,
}

/// Divergent view structure
#[derive(Clone, PartialEq)]
pub struct DivergentView {
    /// Topic of divergence
    pub topic: String,
    
    /// Different perspectives
    pub perspectives: Vec<Perspective>,
    
    /// Areas of common ground
    pub common_ground: Vec<String>,
}

/// Perspective structure
#[derive(Clone, PartialEq)]
pub struct Perspective {
    /// Description of perspective
    pub description: String,
    
    /// Community members holding this view
    pub holders: Vec<String>,
    
    /// Supporting rationale
    pub rationale: String,
}

/// Actionable recommendation structure
#[derive(Clone, PartialEq)]
pub struct ActionableRecommendation {
    /// Recommendation description
    pub description: String,
    
    /// Priority level
    pub priority: RecommendationPriority,
    
    /// Expected impact
    pub expected_impact: String,
    
    /// Resources needed
    pub resources_needed: Vec<String>,
    
    /// Responsible parties
    pub responsible_parties: Vec<String>,
}

/// Recommendation priority levels
#[derive(Clone, PartialEq)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Comment on an interpretation
#[derive(Clone, PartialEq)]
pub struct InterpretationComment {
    /// Unique identifier
    pub id: Uuid,
    
    /// Comment text
    pub comment: String,
    
    /// Author
    pub author: String,
    
    /// Timestamp
    pub timestamp: chrono::DateTime<Utc>,
    
    /// Reactions
    pub reactions: Vec<Reaction>,
    
    /// Replies
    pub replies: Vec<InterpretationComment>,
}

/// Collaborative Interpreter Component
#[styled_component(CollaborativeInterpreter)]
pub fn collaborative_interpreter(props: &CollaborativeInterpreterProps) -> Html {
    let state = use_state(|| CollaborativeInterpreterState {
        current_step: InterpretationStep::ReviewData,
        interpretation: CommunityInterpretation {
            insights: Vec::new(),
            patterns: Vec::new(),
            consensus_areas: Vec::new(),
            divergent_views: Vec::new(),
            recommendations: Vec::new(),
        },
        comments: Vec::new(),
    });
    
    let container_style = style!(
        r#"
        background-color: white;
        border-radius: 8px;
        padding: 2rem;
        box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        margin-bottom: 2rem;
    "#
    ).unwrap();
    
    let step_indicator_style = style!(
        r#"
        display: flex;
        justify-content: space-between;
        margin-bottom: 2rem;
        position: relative;
    "#
    ).unwrap();
    
    let step_style = style!(
        r#"
        flex: 1;
        text-align: center;
        position: relative;
        z-index: 1;
    "#
    ).unwrap();
    
    let step_number_style = style!(
        r#"
        width: 30px;
        height: 30px;
        border-radius: 50%;
        background-color: #eee;
        display: flex;
        align-items: center;
        justify-content: center;
        margin: 0 auto 0.5rem;
        font-weight: bold;
    "#
    ).unwrap();
    
    let active_step_style = style!(
        r#"
        background-color: #3498db;
        color: white;
    "#
    ).unwrap();
    
    let step_connector_style = style!(
        r#"
        position: absolute;
        top: 15px;
        left: 50px;
        right: 50px;
        height: 2px;
        background-color: #eee;
        z-index: 0;
    "#
    ).unwrap();
    
    let button_style = style!(
        r#"
        padding: 0.75rem 1.5rem;
        border: none;
        border-radius: 4px;
        font-size: 1rem;
        cursor: pointer;
        margin: 0 0.5rem;
        transition: all 0.3s;
    "#
    ).unwrap();
    
    let primary_button_style = style!(
        r#"
        background-color: #3498db;
        color: white;
    "#
    ).unwrap();
    
    let secondary_button_style = style!(
        r#"
        background-color: #95a5a6;
        color: white;
    "#
    ).unwrap();
    
    let on_next = {
        let state = state.clone();
        Callback::from(move |_| {
            let next_step = match state.current_step {
                InterpretationStep::ReviewData => InterpretationStep::SharePerspectives,
                InterpretationStep::SharePerspectives => InterpretationStep::IdentifyPatterns,
                InterpretationStep::IdentifyPatterns => InterpretationStep::FormConsensus,
                InterpretationStep::FormConsensus => InterpretationStep::DocumentInsights,
                InterpretationStep::DocumentInsights => InterpretationStep::DocumentInsights,
            };
            
            state.set(CollaborativeInterpreterState {
                current_step: next_step,
                ..(*state).clone()
            });
        })
    };
    
    let on_previous = {
        let state = state.clone();
        Callback::from(move |_| {
            let prev_step = match state.current_step {
                InterpretationStep::ReviewData => InterpretationStep::ReviewData,
                InterpretationStep::SharePerspectives => InterpretationStep::ReviewData,
                InterpretationStep::IdentifyPatterns => InterpretationStep::SharePerspectives,
                InterpretationStep::FormConsensus => InterpretationStep::IdentifyPatterns,
                InterpretationStep::DocumentInsights => InterpretationStep::FormConsensus,
            };
            
            state.set(CollaborativeInterpreterState {
                current_step: prev_step,
                ..(*state).clone()
            });
        })
    };
    
    let on_submit = {
        let state = state.clone();
        let on_interpret = props.on_interpret.clone();
        
        Callback::from(move |_| {
            on_interpret.emit(state.interpretation.clone());
        })
    };
    
    html! {
        <div class={container_style}>
            <h2>{"Collaborative Interpretation of Community Impact"}</h2>
            <p>{"Work together to understand what the data means for our community and identify actionable insights."}</p>
            
            <div style="position: relative;">
                <div class={step_indicator_style}>
                    <div class={step_connector_style}></div>
                    <div class={step_style}>
                        <div class={classes!(step_number_style.clone(), 
                            if matches!(state.current_step, InterpretationStep::ReviewData) { 
                                active_step_style.clone() 
                            } else { 
                                style!("").unwrap() 
                            })}>
                            {"1"}
                        </div>
                        <div>{"Review Data"}</div>
                    </div>
                    <div class={step_style}>
                        <div class={classes!(step_number_style.clone(), 
                            if matches!(state.current_step, InterpretationStep::SharePerspectives) { 
                                active_step_style.clone() 
                            } else { 
                                style!("").unwrap() 
                            })}>
                            {"2"}
                        </div>
                        <div>{"Share Perspectives"}</div>
                    </div>
                    <div class={step_style}>
                        <div class={classes!(step_number_style.clone(), 
                            if matches!(state.current_step, InterpretationStep::IdentifyPatterns) { 
                                active_step_style.clone() 
                            } else { 
                                style!("").unwrap() 
                            })}>
                            {"3"}
                        </div>
                        <div>{"Identify Patterns"}</div>
                    </div>
                    <div class={step_style}>
                        <div class={classes!(step_number_style.clone(), 
                            if matches!(state.current_step, InterpretationStep::FormConsensus) { 
                                active_step_style.clone() 
                            } else { 
                                style!("").unwrap() 
                            })}>
                            {"4"}
                        </div>
                        <div>{"Form Consensus"}</div>
                    </div>
                    <div class={step_style}>
                        <div class={classes!(step_number_style.clone(), 
                            if matches!(state.current_step, InterpretationStep::DocumentInsights) { 
                                active_step_style.clone() 
                            } else { 
                                style!("").unwrap() 
                            })}>
                            {"5"}
                        </div>
                        <div>{"Document Insights"}</div>
                    </div>
                </div>
            </div>
            
            <div style="margin: 2rem 0;">
                {render_interpretation_step(&state.current_step, &props.impact_data, &state.interpretation, &state.comments)}
            </div>
            
            <div style="display: flex; justify-content: space-between; margin-top: 2rem;">
                <div>
                    {if !matches!(state.current_step, InterpretationStep::ReviewData) {
                        html! {
                            <button 
                                class={classes!(button_style.clone(), secondary_button_style.clone())}
                                onclick={on_previous}
                            >
                                {"Previous"}
                            </button>
                        }
                    } else {
                        html! {}
                    }}
                </div>
                
                <div>
                    {match state.current_step {
                        InterpretationStep::ReviewData => html! {
                            <button 
                                class={classes!(button_style.clone(), primary_button_style.clone())}
                                onclick={on_next}
                            >
                                {"Share Perspectives"}
                            </button>
                        },
                        InterpretationStep::SharePerspectives => html! {
                            <button 
                                class={classes!(button_style.clone(), primary_button_style.clone())}
                                onclick={on_next}
                            >
                                {"Identify Patterns"}
                            </button>
                        },
                        InterpretationStep::IdentifyPatterns => html! {
                            <button 
                                class={classes!(button_style.clone(), primary_button_style.clone())}
                                onclick={on_next}
                            >
                                {"Form Consensus"}
                            </button>
                        },
                        InterpretationStep::FormConsensus => html! {
                            <button 
                                class={classes!(button_style.clone(), primary_button_style.clone())}
                                onclick={on_next}
                            >
                                {"Document Insights"}
                            </button>
                        },
                        InterpretationStep::DocumentInsights => html! {
                            <button 
                                class={classes!(button_style.clone(), primary_button_style.clone())}
                                onclick={on_submit}
                            >
                                {"Submit Interpretation"}
                            </button>
                        },
                    }}
                </div>
            </div>
        </div>
    }
}

/// Render the appropriate interpretation step
fn render_interpretation_step(
    step: &InterpretationStep,
    impact_data: &UnifiedImpactData,
    interpretation: &CommunityInterpretation,
    comments: &Vec<InterpretationComment>
) -> Html {
    match step {
        InterpretationStep::ReviewData => render_review_data_step(impact_data),
        InterpretationStep::SharePerspectives => render_share_perspectives_step(),
        InterpretationStep::IdentifyPatterns => render_identify_patterns_step(interpretation),
        InterpretationStep::FormConsensus => render_form_consensus_step(interpretation),
        InterpretationStep::DocumentInsights => render_document_insights_step(interpretation),
    }
}

/// Render the review data step
fn render_review_data_step(impact_data: &UnifiedImpactData) -> Html {
    html! {
        <div>
            <h3>{"Review Community Impact Data"}</h3>
            <p>{"Let's start by reviewing the key metrics and trends in our community impact data:"}</p>
            
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 1rem; margin-top: 1rem;">
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px;">
                    <h4>{"Learning Impact"}</h4>
                    <p>{"Knowledge Sharing Rate: "}{format!("{:.1}%", impact_data.community_wellbeing.domain_indicators.learning.knowledge_sharing_rate * 100.0)}</p>
                    <p>{"Skill Development Progress: "}{format!("{:.1}%", impact_data.community_wellbeing.domain_indicators.learning.skill_development_progress * 100.0)}</p>
                    <p>{"Community Satisfaction: "}{format!("{:.1}%", impact_data.community_wellbeing.domain_indicators.learning.community_satisfaction * 100.0)}</p>
                </div>
                
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px;">
                    <h4>{"Volunteer Impact"}</h4>
                    <p>{"Participation Rate: "}{format!("{:.1}%", impact_data.community_wellbeing.domain_indicators.volunteer.participation_rate * 100.0)}</p>
                    <p>{"Retention Rate: "}{format!("{:.1}%", impact_data.community_wellbeing.domain_indicators.volunteer.retention_rate * 100.0)}</p>
                    <p>{"Satisfaction Index: "}{format!("{:.1}%", impact_data.community_wellbeing.domain_indicators.volunteer.satisfaction_index * 100.0)}</p>
                </div>
                
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px;">
                    <h4>{"Financial Impact"}</h4>
                    <p>{"Financial Health: "}{format!("{:.1}%", impact_data.community_wellbeing.domain_indicators.financial.financial_health * 100.0)}</p>
                    <p>{"Resource Equity: "}{format!("{:.1}%", impact_data.community_wellbeing.domain_indicators.financial.resource_equity * 100.0)}</p>
                    <p>{"Sustainability Index: "}{format!("{:.1}%", impact_data.community_wellbeing.domain_indicators.financial.sustainability_index * 100.0)}</p>
                </div>
                
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px;">
                    <h4>{"Cause Impact"}</h4>
                    <p>{"Engagement Rate: "}{format!("{:.1}%", impact_data.community_wellbeing.domain_indicators.cause.engagement_rate * 100.0)}</p>
                    <p>{"Impact Effectiveness: "}{format!("{:.1}%", impact_data.community_wellbeing.domain_indicators.cause.impact_effectiveness * 100.0)}</p>
                    <p>{"Solidarity Index: "}{format!("{:.1}%", impact_data.community_wellbeing.domain_indicators.cause.solidarity_index * 100.0)}</p>
                </div>
            </div>
            
            <div style="margin-top: 1rem; padding: 1rem; background-color: #e8f4fc; border-radius: 8px;">
                <h4>{"Overall Community Wellbeing"}</h4>
                <p>{"Score: "}{format!("{:.1}%", impact_data.community_wellbeing.overall_score * 100.0)}</p>
                
                <h4>{"Cooperative Goals Progress"}</h4>
                {for impact_data.community_wellbeing.cooperative_goals_progress.iter().map(|goal| {
                    html! {
                        <div style="margin: 0.5rem 0;">
                            <div style="display: flex; justify-content: space-between;">
                                <span>{&goal.title}</span>
                                <span>{format!("{:.0}%", goal.progress * 100.0)}</span>
                            </div>
                            <div style="width: 100%; height: 8px; background-color: #eee; border-radius: 4px; margin: 0.25rem 0; overflow: hidden;">
                                <div style={format!("height: 100%; background-color: #3498db; border-radius: 4px; width: {}%;", goal.progress * 100.0)}></div>
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}

/// Render the share perspectives step
fn render_share_perspectives_step() -> Html {
    html! {
        <div>
            <h3>{"Share Your Perspectives"}</h3>
            <p>{"What do these metrics mean to you? Share your observations and experiences:"}</p>
            
            <div style="margin: 1rem 0;">
                <textarea 
                    style="width: 100%; height: 150px; padding: 0.5rem; margin-top: 0.5rem;"
                    placeholder="Share your perspective on what these metrics mean for our community..."
                >
                </textarea>
                
                <div style="display: flex; gap: 1rem; margin-top: 0.5rem;">
                    <button style="background-color: #3498db; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer;">
                        {"Share Perspective"}
                    </button>
                    <button style="background-color: #95a5a6; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer;">
                        {"Add Evidence"}
                    </button>
                </div>
            </div>
            
            <div style="margin-top: 2rem;">
                <h4>{"Community Perspectives"}</h4>
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px; margin-bottom: 1rem;">
                    <div style="display: flex; justify-content: space-between; margin-bottom: 0.5rem;">
                        <strong>{"Sarah M."}</strong>
                        <span>{"2 hours ago"}</span>
                    </div>
                    <p>{"I'm encouraged by the learning metrics. The increase in knowledge sharing shows our educational initiatives are working. However, I'd like to see more connection between learning and volunteer activities."}</p>
                    <div style="display: flex; gap: 1rem; margin-top: 0.5rem;">
                        <button style="background: none; border: none; cursor: pointer; color: #666;">
                            {"👍 Like (5)"}
                        </button>
                        <button style="background: none; border: none; cursor: pointer; color: #666;">
                            {"Reply"}
                        </button>
                    </div>
                </div>
                
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px; margin-bottom: 1rem;">
                    <div style="display: flex; justify-content: space-between; margin-bottom: 0.5rem;">
                        <strong>{"James T."}</strong>
                        <span>{"4 hours ago"}</span>
                    </div>
                    <p>{"The financial health metrics are concerning. While we've improved resource equity, our sustainability index has plateaued. We need to think about long-term financial resilience."}</p>
                    <div style="display: flex; gap: 1rem; margin-top: 0.5rem;">
                        <button style="background: none; border: none; cursor: pointer; color: #666;">
                            {"👍 Like (8)"}
                        </button>
                        <button style="background: none; border: none; cursor: pointer; color: #666;">
                            {"Reply"}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Render the identify patterns step
fn render_identify_patterns_step(interpretation: &CommunityInterpretation) -> Html {
    html! {
        <div>
            <h3>{"Identify Emerging Patterns"}</h3>
            <p>{"As we review perspectives, what patterns are emerging in our community's impact?"}</p>
            
            <div style="margin: 1rem 0;">
                <div style="display: flex; gap: 1rem; margin-bottom: 1rem;">
                    <button style="background-color: #3498db; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer;">
                        {"Add Pattern"}
                    </button>
                    <button style="background-color: #95a5a6; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer;">
                        {"Suggest Connection"}
                    </button>
                </div>
                
                <div style="background-color: #fff8e1; padding: 1rem; border-radius: 8px; margin-bottom: 1rem;">
                    <div style="display: flex; justify-content: space-between; align-items: center;">
                        <h4>{"Learning → Volunteer Connection"}</h4>
                        <span style="background-color: #ffc107; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                            {"Moderate Strength"}
                        </span>
                    </div>
                    <p>{"Members who complete learning modules show 40% higher volunteer retention. This suggests our educational programs are building commitment to service."}</p>
                    <div style="display: flex; gap: 1rem; margin-top: 0.5rem;">
                        <button style="background: none; border: none; cursor: pointer; color: #666;">
                            {"👍 Support (12)"}
                        </button>
                        <button style="background: none; border: none; cursor: pointer; color: #666;">
                            {"🔍 Investigate Further"}
                        </button>
                    </div>
                </div>
                
                <div style="background-color: #e8f5e9; padding: 1rem; border-radius: 8px; margin-bottom: 1rem;">
                    <div style="display: flex; justify-content: space-between; align-items: center;">
                        <h4>{"Financial Participation Growth"}</h4>
                        <span style="background-color: #4caf50; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                            {"Strong Pattern"}
                        </span>
                    </div>
                    <p>{"Financial participation increased 25% after our volunteer recognition program. This suggests that meaningful service experiences lead to financial commitment."}</p>
                    <div style="display: flex; gap: 1rem; margin-top: 0.5rem;">
                        <button style="background: none; border: none; cursor: pointer; color: #666;">
                            {"👍 Support (18)"}
                        </button>
                        <button style="background: none; border: none; cursor: pointer; color: #666;">
                            {"📊 Request Data"}
                        </button>
                    </div>
                </div>
            </div>
            
            <div style="background-color: #e3f2fd; padding: 1rem; border-radius: 8px;">
                <h4>{"Your Pattern Insights"}</h4>
                <textarea 
                    style="width: 100%; height: 100px; padding: 0.5rem; margin-top: 0.5rem;"
                    placeholder="What patterns do you see in the data and community perspectives?"
                >
                </textarea>
                <button style="background-color: #3498db; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer; margin-top: 0.5rem;">
                    {"Add Pattern Insight"}
                </button>
            </div>
        </div>
    }
}

/// Render the form consensus step
fn render_form_consensus_step(interpretation: &CommunityInterpretation) -> Html {
    html! {
        <div>
            <h3>{"Form Community Consensus"}</h3>
            <p>{"What areas do we agree on, and where do we have different perspectives?"}</p>
            
            <div style="margin: 1rem 0;">
                <h4>{"Areas of Consensus"}</h4>
                <div style="background-color: #e8f5e9; padding: 1rem; border-radius: 8px; margin-bottom: 1rem;">
                    <div style="display: flex; justify-content: space-between; align-items: center;">
                        <h5>{"Learning Programs Are Effective"}</h5>
                        <span>{"92% Agreement"}</span>
                    </div>
                    <p>{"Our community agrees that learning programs are successfully building knowledge and skills. This is evidenced by the 75% knowledge sharing rate and positive community satisfaction scores."}</p>
                    <div style="display: flex; flex-wrap: wrap; gap: 0.5rem; margin-top: 0.5rem;">
                        <span style="background-color: #4caf50; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                            {"Continue Investment"}
                        </span>
                        <span style="background-color: #4caf50; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                            {"Expand Access"}
                        </span>
                    </div>
                </div>
                
                <h4>{"Areas of Divergent Views"}</h4>
                <div style="background-color: #fff3e0; padding: 1rem; border-radius: 8px; margin-bottom: 1rem;">
                    <div style="display: flex; justify-content: space-between; align-items: center;">
                        <h5>{"Approach to Financial Sustainability"}</h5>
                        <span>{"65% Agreement"}</span>
                    </div>
                    <p>{"While most agree financial sustainability is important, there are different views on how to achieve it:"}</p>
                    
                    <div style="margin: 1rem 0;">
                        <strong>{"Growth-Focused Perspective (40%)"}</strong>
                        <p>{"We should focus on expanding our membership and revenue streams to achieve sustainability through scale."}</p>
                    </div>
                    
                    <div style="margin: 1rem 0;">
                        <strong>{"Efficiency-Focused Perspective (35%)"}</strong>
                        <p>{"We should focus on optimizing our current operations and reducing costs to achieve sustainability through efficiency."}</p>
                    </div>
                    
                    <div style="margin: 1rem 0;">
                        <strong>{"Common Ground"}</strong>
                        <p>{"Both perspectives agree that financial sustainability is crucial and that we need to maintain our cooperative values."}</p>
                    </div>
                </div>
            </div>
            
            <div style="background-color: #e3f2fd; padding: 1rem; border-radius: 8px;">
                <h4>{"Your Consensus Input"}</h4>
                <textarea 
                    style="width: 100%; height: 100px; padding: 0.5rem; margin-top: 0.5rem;"
                    placeholder="Do you agree with these consensus areas? Do you see additional areas of agreement or divergence?"
                >
                </textarea>
                <button style="background-color: #3498db; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer; margin-top: 0.5rem;">
                    {"Add Consensus Input"}
                </button>
            </div>
        </div>
    }
}

/// Render the document insights step
fn render_document_insights_step(interpretation: &CommunityInterpretation) -> Html {
    html! {
        <div>
            <h3>{"Document Community Insights"}</h3>
            <p>{"Let's capture the key insights and recommendations from our collaborative interpretation:"}</p>
            
            <div style="margin: 1rem 0;">
                <h4>{"Key Community Insights"}</h4>
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px; margin-bottom: 1rem;">
                    <h5>{"1. Learning Programs Build Community Connection"}</h5>
                    <p>{"Our data and community feedback show that learning programs not only build skills but also strengthen community bonds and commitment to service."}</p>
                    <div style="display: flex; flex-wrap: wrap; gap: 0.5rem; margin-top: 0.5rem;">
                        <span style="background-color: #3498db; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                            {"Evidence-Based"
                        </span>
                        <span style="background-color: #3498db; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                            {"Community Validated"
                        </span>
                    </div>
                </div>
                
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px; margin-bottom: 1rem;">
                    <h5>{"2. Financial Commitment Follows Meaningful Engagement"}</h5>
                    <p>{"Members who have positive volunteer experiences are significantly more likely to contribute financially, suggesting that service opportunities are key to financial sustainability."}</p>
                    <div style="display: flex; flex-wrap: wrap; gap: 0.5rem; margin-top: 0.5rem;">
                        <span style="background-color: #3498db; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                            {"Data-Driven"
                        </span>
                        <span style="background-color: #3498db; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                            {"Actionable"
                        </span>
                    </div>
                </div>
            </div>
            
            <div style="margin: 1rem 0;">
                <h4>{"Actionable Recommendations"}</h4>
                <div style="background-color: #e8f4fc; padding: 1rem; border-radius: 8px; margin-bottom: 1rem;">
                    <div style="display: flex; justify-content: space-between; align-items: center;">
                        <h5>{"Create Integrated Learning-Volunteer Pathways"}</h5>
                        <span style="background-color: #ff9800; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                            {"High Priority"
                        </span>
                    </div>
                    <p>{"Develop structured pathways that connect learning modules directly to volunteer opportunities to maximize the connection between skill-building and service."}</p>
                    <div style="display: flex; flex-wrap: wrap; gap: 0.5rem; margin-top: 0.5rem;">
                        <span style="background-color: #9e9e9e; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                            {"Resources: 2 FTE, $5,000 budget"
                        </span>
                        <span style="background-color: #9e9e9e; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                            {"Responsible: Learning & Volunteer Coordinators"
                        </span>
                    </div>
                </div>
                
                <div style="background-color: #e8f4fc; padding: 1rem; border-radius: 8px; margin-bottom: 1rem;">
                    <div style="display: flex; justify-content: space-between; align-items: center;">
                        <h5>{"Launch Volunteer Recognition Program"}</h5>
                        <span style="background-color: #ff9800; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                            {"Medium Priority"
                        </span>
                    </div>
                    <p>{"Implement a systematic recognition program for volunteers to strengthen the connection between service and financial commitment."}</p>
                    <div style="display: flex; flex-wrap: wrap; gap: 0.5rem; margin-top: 0.5rem;">
                        <span style="background-color: #9e9e9e; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                            {"Resources: 0.5 FTE, $2,000 budget"
                        </span>
                        <span style="background-color: #9e9e9e; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                            {"Responsible: Volunteer Coordinator"
                        </span>
                    </div>
                </div>
            </div>
            
            <div style="background-color: #e8f5e9; padding: 1rem; border-radius: 8px;">
                <h4>{"Ready to Submit Community Interpretation"}</h4>
                <p>{"Your collaborative interpretation will be documented and shared with the community. This will inform our strategic planning and program development."}</p>
            </div>
        </div>
    }
}