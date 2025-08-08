//! Community Reflection Component
//!
//! This component facilitates community-wide reflection sessions
//! and documents collective insights.

use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::models::{UnifiedImpactData, community_wellbeing::CooperativeGoalProgress};
use uuid::Uuid;
use chrono::{Utc, DateTime};
use std::collections::HashMap;

/// Properties for the CommunityReflection component
#[derive(Properties, PartialEq)]
pub struct CommunityReflectionProps {
    /// Impact data to reflect on
    pub impact_data: UnifiedImpactData,
    
    /// Callback when reflection is submitted
    pub on_reflect: Callback<CommunityReflectionOutcome>,
}

/// State for the CommunityReflection component
#[derive(Clone, PartialEq)]
pub struct CommunityReflectionState {
    /// Current reflection phase
    current_phase: ReflectionPhase,
    
    /// Reflection responses
    responses: HashMap<String, Vec<ReflectionResponse>>,
    
    /// Collective insights
    insights: Vec<CollectiveInsight>,
    
    /// Action items
    action_items: Vec<ReflectionActionItem>,
}

/// Reflection phases
#[derive(Clone, PartialEq)]
pub enum ReflectionPhase {
    IndividualReflection,
    SmallGroupSharing,
    CommunitySynthesis,
    ActionPlanning,
}

/// Reflection response structure
#[derive(Clone, PartialEq)]
pub struct ReflectionResponse {
    /// Unique identifier
    pub id: Uuid,
    
    /// Respondent
    pub respondent: String,
    
    /// Response content
    pub content: String,
    
    /// Emotional tone
    pub emotional_tone: EmotionalTone,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Emotional tone of a response
#[derive(Clone, PartialEq)]
pub enum EmotionalTone {
    Joy,
    Gratitude,
    Concern,
    Challenge,
    Hope,
    Reflection,
}

/// Collective insight structure
#[derive(Clone, PartialEq)]
pub struct CollectiveInsight {
    /// Unique identifier
    pub id: Uuid,
    
    /// Insight description
    pub description: String,
    
    /// Supporting responses
    pub supporting_responses: Vec<Uuid>,
    
    /// Emergence pattern
    pub emergence_pattern: EmergencePattern,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Emergence patterns
#[derive(Clone, PartialEq)]
pub enum EmergencePattern {
    Consensus,
    Tension,
    Innovation,
    Healing,
    Transformation,
}

/// Action item from reflection
#[derive(Clone, PartialEq)]
pub struct ReflectionActionItem {
    /// Unique identifier
    pub id: Uuid,
    
    /// Action description
    pub description: String,
    
    /// Priority level
    pub priority: ActionPriority,
    
    /// Assigned to
    pub assigned_to: Vec<String>,
    
    /// Timeline
    pub timeline: String,
    
    /// Resources needed
    pub resources_needed: Vec<String>,
    
    /// Status
    pub status: ActionStatus,
}

/// Action priority levels
#[derive(Clone, PartialEq)]
pub enum ActionPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Action status
#[derive(Clone, PartialEq)]
pub enum ActionStatus {
    Proposed,
    Approved,
    InProgress,
    Completed,
    Deferred,
}

/// Community reflection outcome structure
#[derive(Clone, PartialEq)]
pub struct CommunityReflectionOutcome {
    /// Key insights
    pub insights: Vec<CollectiveInsight>,
    
    /// Action items
    pub action_items: Vec<ReflectionActionItem>,
    
    /// Participation metrics
    pub participation_metrics: ParticipationMetrics,
    
    /// Emotional climate
    pub emotional_climate: EmotionalClimate,
}

/// Participation metrics
#[derive(Clone, PartialEq)]
pub struct ParticipationMetrics {
    /// Total participants
    pub total_participants: usize,
    
    /// Active contributors
    pub active_contributors: usize,
    
    /// Response diversity
    pub response_diversity: f64,
}

/// Emotional climate
#[derive(Clone, PartialEq)]
pub struct EmotionalClimate {
    /// Dominant emotions
    pub dominant_emotions: Vec<EmotionalTone>,
    
    /// Emotional balance
    pub emotional_balance: f64,
    
    /// Emergent themes
    pub emergent_themes: Vec<String>,
}

/// Community Reflection Component
#[styled_component(CommunityReflection)]
pub fn community_reflection(props: &CommunityReflectionProps) -> Html {
    let state = use_state(|| CommunityReflectionState {
        current_phase: ReflectionPhase::IndividualReflection,
        responses: HashMap::new(),
        insights: Vec::new(),
        action_items: Vec::new(),
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
    
    let phase_indicator_style = style!(
        r#"
        display: flex;
        justify-content: space-between;
        margin-bottom: 2rem;
        position: relative;
    "#
    ).unwrap();
    
    let phase_style = style!(
        r#"
        flex: 1;
        text-align: center;
        position: relative;
        z-index: 1;
    "#
    ).unwrap();
    
    let phase_number_style = style!(
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
    
    let active_phase_style = style!(
        r#"
        background-color: #9b59b6;
        color: white;
    "#
    ).unwrap();
    
    let phase_connector_style = style!(
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
        background-color: #9b59b6;
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
            let next_phase = match state.current_phase {
                ReflectionPhase::IndividualReflection => ReflectionPhase::SmallGroupSharing,
                ReflectionPhase::SmallGroupSharing => ReflectionPhase::CommunitySynthesis,
                ReflectionPhase::CommunitySynthesis => ReflectionPhase::ActionPlanning,
                ReflectionPhase::ActionPlanning => ReflectionPhase::ActionPlanning,
            };
            
            state.set(CommunityReflectionState {
                current_phase: next_phase,
                ..(*state).clone()
            });
        })
    };
    
    let on_previous = {
        let state = state.clone();
        Callback::from(move |_| {
            let prev_phase = match state.current_phase {
                ReflectionPhase::IndividualReflection => ReflectionPhase::IndividualReflection,
                ReflectionPhase::SmallGroupSharing => ReflectionPhase::IndividualReflection,
                ReflectionPhase::CommunitySynthesis => ReflectionPhase::SmallGroupSharing,
                ReflectionPhase::ActionPlanning => ReflectionPhase::CommunitySynthesis,
            };
            
            state.set(CommunityReflectionState {
                current_phase: prev_phase,
                ..(*state).clone()
            });
        })
    };
    
    let on_submit = {
        let state = state.clone();
        let on_reflect = props.on_reflect.clone();
        
        Callback::from(move |_| {
            // Create participation metrics
            let participation_metrics = ParticipationMetrics {
                total_participants: 42,
                active_contributors: 31,
                response_diversity: 0.74,
            };
            
            // Create emotional climate
            let emotional_climate = EmotionalClimate {
                dominant_emotions: vec![EmotionalTone::Hope, EmotionalTone::Gratitude, EmotionalTone::Challenge],
                emotional_balance: 0.68,
                emergent_themes: vec![
                    "Community Resilience".to_string(),
                    "Need for Connection".to_string(),
                    "Excitement for Future".to_string(),
                ],
            };
            
            let outcome = CommunityReflectionOutcome {
                insights: state.insights.clone(),
                action_items: state.action_items.clone(),
                participation_metrics,
                emotional_climate,
            };
            
            on_reflect.emit(outcome);
        })
    };
    
    html! {
        <div class={container_style}>
            <h2>{"Community Reflection Session"}</h2>
            <p>{"Take time to reflect on our collective impact journey and envision our future together."}</p>
            
            <div style="position: relative;">
                <div class={phase_indicator_style}>
                    <div class={phase_connector_style}></div>
                    <div class={phase_style}>
                        <div class={classes!(phase_number_style.clone(), 
                            if matches!(state.current_phase, ReflectionPhase::IndividualReflection) { 
                                active_phase_style.clone() 
                            } else { 
                                style!("").unwrap() 
                            })}>
                            {"1"}
                        </div>
                        <div>{"Individual Reflection"}</div>
                    </div>
                    <div class={phase_style}>
                        <div class={classes!(phase_number_style.clone(), 
                            if matches!(state.current_phase, ReflectionPhase::SmallGroupSharing) { 
                                active_phase_style.clone() 
                            } else { 
                                style!("").unwrap() 
                            })}>
                            {"2"}
                        </div>
                        <div>{"Small Group Sharing"}</div>
                    </div>
                    <div class={phase_style}>
                        <div class={classes!(phase_number_style.clone(), 
                            if matches!(state.current_phase, ReflectionPhase::CommunitySynthesis) { 
                                active_phase_style.clone() 
                            } else { 
                                style!("").unwrap() 
                            })}>
                            {"3"}
                        </div>
                        <div>{"Community Synthesis"}</div>
                    </div>
                    <div class={phase_style}>
                        <div class={classes!(phase_number_style.clone(), 
                            if matches!(state.current_phase, ReflectionPhase::ActionPlanning) { 
                                active_phase_style.clone() 
                            } else { 
                                style!("").unwrap() 
                            })}>
                            {"4"}
                        </div>
                        <div>{"Action Planning"}</div>
                    </div>
                </div>
            </div>
            
            <div style="margin: 2rem 0;">
                {render_reflection_phase(&state.current_phase, &props.impact_data, &state.responses)}
            </div>
            
            <div style="display: flex; justify-content: space-between; margin-top: 2rem;">
                <div>
                    {if !matches!(state.current_phase, ReflectionPhase::IndividualReflection) {
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
                    {match state.current_phase {
                        ReflectionPhase::IndividualReflection => html! {
                            <button 
                                class={classes!(button_style.clone(), primary_button_style.clone())}
                                onclick={on_next}
                            >
                                {"Share in Groups"}
                            </button>
                        },
                        ReflectionPhase::SmallGroupSharing => html! {
                            <button 
                                class={classes!(button_style.clone(), primary_button_style.clone())}
                                onclick={on_next}
                            >
                                {"Community Synthesis"}
                            </button>
                        },
                        ReflectionPhase::CommunitySynthesis => html! {
                            <button 
                                class={classes!(button_style.clone(), primary_button_style.clone())}
                                onclick={on_next}
                            >
                                {"Plan Actions"}
                            </button>
                        },
                        ReflectionPhase::ActionPlanning => html! {
                            <button 
                                class={classes!(button_style.clone(), primary_button_style.clone())}
                                onclick={on_submit}
                            >
                                {"Submit Reflection"}
                            </button>
                        },
                    }}
                </div>
            </div>
        </div>
    }
}

/// Render the appropriate reflection phase
fn render_reflection_phase(
    phase: &ReflectionPhase,
    impact_data: &UnifiedImpactData,
    responses: &HashMap<String, Vec<ReflectionResponse>>
) -> Html {
    match phase {
        ReflectionPhase::IndividualReflection => render_individual_reflection_phase(impact_data),
        ReflectionPhase::SmallGroupSharing => render_small_group_sharing_phase(responses),
        ReflectionPhase::CommunitySynthesis => render_community_synthesis_phase(responses),
        ReflectionPhase::ActionPlanning => render_action_planning_phase(),
    }
}

/// Render the individual reflection phase
fn render_individual_reflection_phase(impact_data: &UnifiedImpactData) -> Html {
    html! {
        <div>
            <h3>{"Individual Reflection"}</h3>
            <p>{"Take a moment to reflect on our community's impact journey. What stands out to you?"}</p>
            
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 1rem; margin-top: 1rem;">
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px;">
                    <h4>{"Personal Growth"}</h4>
                    <p>{"How have you grown through your participation in our community impact initiatives?"}</p>
                    <textarea 
                        style="width: 100%; height: 100px; padding: 0.5rem; margin-top: 0.5rem;"
                        placeholder="Share your personal reflections..."
                    >
                    </textarea>
                </div>
                
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px;">
                    <h4>{"Community Connection"}</h4>
                    <p>{"How has our work together strengthened your connection to the community?"}</p>
                    <textarea 
                        style="width: 100%; height: 100px; padding: 0.5rem; margin-top: 0.5rem;"
                        placeholder="Reflect on your community connections..."
                    >
                    </textarea>
                </div>
                
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px;">
                    <h4>{"Future Vision"}</h4>
                    <p>{"What do you envision for our community's impact in the coming year?"}</p>
                    <textarea 
                        style="width: 100%; height: 100px; padding: 0.5rem; margin-top: 0.5rem;"
                        placeholder="Share your hopes and dreams..."
                    >
                    </textarea>
                </div>
            </div>
            
            <div style="margin-top: 1rem; padding: 1rem; background-color: #e8f4fc; border-radius: 8px;">
                <h4>{"Emotional Check-in"}</h4>
                <p>{"How are you feeling about our community's impact work?"}</p>
                
                <div style="display: flex; flex-wrap: wrap; gap: 1rem; margin-top: 1rem;">
                    <button style="background-color: #27ae60; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer;">
                        {"😀 Joy"}
                    </button>
                    <button style="background-color: #2980b9; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer;">
                        {"🙏 Gratitude"}
                    </button>
                    <button style="background-color: #f39c12; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer;">
                        {"🤔 Reflection"}
                    </button>
                    <button style="background-color: #e74c3c; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer;">
                        {"😕 Concern"}
                    </button>
                    <button style="background-color: #8e44ad; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer;">
                        {"💪 Challenge"}
                    </button>
                    <button style="background-color: #3498db; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer;">
                        {"🌟 Hope"}
                    </button>
                </div>
            </div>
            
            <div style="margin-top: 1rem;">
                <button style="background-color: #9b59b6; color: white; border: none; padding: 0.75rem 1.5rem; border-radius: 4px; cursor: pointer; font-size: 1rem;">
                    {"Save My Reflection"}
                </button>
            </div>
        </div>
    }
}

/// Render the small group sharing phase
fn render_small_group_sharing_phase(responses: &HashMap<String, Vec<ReflectionResponse>>) -> Html {
    html! {
        <div>
            <h3>{"Small Group Sharing"}</h3>
            <p>{"Share your reflections with your small group and listen to others' perspectives."}</p>
            
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 1rem; margin-top: 1rem;">
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px;">
                    <h4>{"Group Discussion"}</h4>
                    <p>{"In your group of 4-6 people, take turns sharing your reflections. Listen deeply and ask curious questions."}</p>
                    
                    <div style="margin-top: 1rem;">
                        <h5>{"Discussion Prompts"}</h5>
                        <ul>
                            <li>{"What themes emerged in your group's reflections?"}</li>
                            <li>{"What surprised you about others' experiences?"}</li>
                            <li>{"What connections do you see between individual and collective impact?"}</li>
                        </ul>
                    </div>
                </div>
                
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px;">
                    <h4>{"Group Synthesis"}</h4>
                    <p>{"As a group, identify key themes and insights that emerged from your sharing."}</p>
                    
                    <textarea 
                        style="width: 100%; height: 150px; padding: 0.5rem; margin-top: 0.5rem;"
                        placeholder="Document your group's key insights..."
                    >
                    </textarea>
                    
                    <div style="margin-top: 1rem;">
                        <button style="background-color: #9b59b6; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer;">
                            {"Save Group Insights"}
                        </button>
                    </div>
                </div>
            </div>
            
            <div style="margin-top: 2rem;">
                <h4>{"Your Group Members"}</h4>
                <div style="display: flex; flex-wrap: wrap; gap: 1rem; margin-top: 1rem;">
                    <div style="background-color: #e8f4fc; padding: 1rem; border-radius: 8px; flex: 1; min-width: 200px;">
                        <strong>{"Sarah M."}</strong>
                        <p style="font-size: 0.9rem; color: #666; margin: 0.5rem 0 0;">
                            {"\"I'm grateful for how our learning programs have helped me develop new skills that I'm now using in my volunteer work.\""}
                        </p>
                    </div>
                    
                    <div style="background-color: #e8f4fc; padding: 1rem; border-radius: 8px; flex: 1; min-width: 200px;">
                        <strong>{"James T."}</strong>
                        <p style="font-size: 0.9rem; color: #666; margin: 0.5rem 0 0;">
                            {"\"I'm feeling challenged by the need to balance financial sustainability with our cooperative values.\""}
                        </p>
                    </div>
                    
                    <div style="background-color: #e8f4fc; padding: 1rem; border-radius: 8px; flex: 1; min-width: 200px;">
                        <strong>{"Maria L."}</strong>
                        <p style="font-size: 0.9rem; color: #666; margin: 0.5rem 0 0;">
                            {"\"I'm filled with hope seeing how our cause advocacy is creating real change in our community.\""}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Render the community synthesis phase
fn render_community_synthesis_phase(responses: &HashMap<String, Vec<ReflectionResponse>>) -> Html {
    html! {
        <div>
            <h3>{"Community Synthesis"}</h3>
            <p>{"Let's come together as a community to identify shared insights and emerging themes."}</p>
            
            <div style="margin: 1rem 0;">
                <h4>{"Emerging Themes"}</h4>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 1rem; margin-top: 1rem;">
                    <div style="background-color: #fff8e1; padding: 1rem; border-radius: 8px;">
                        <h5>{"Community Resilience"}</h5>
                        <p>{"Multiple groups noted how our interconnected impact systems have strengthened our community's ability to adapt and thrive through challenges."}</p>
                        <div style="display: flex; gap: 0.5rem; margin-top: 0.5rem;">
                            <span style="background-color: #ffc107; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                                {"Mentioned by 8 groups"}
                            </span>
                        </div>
                    </div>
                    
                    <div style="background-color: #e8f5e9; padding: 1rem; border-radius: 8px;">
                        <h5>{"Need for Connection"}</h5>
                        <p>{"Many participants expressed a desire for more opportunities to connect across different impact domains and share experiences."}</p>
                        <div style="display: flex; gap: 0.5rem; margin-top: 0.5rem;">
                            <span style="background-color: #4caf50; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                                {"Mentioned by 12 groups"}
                            </span>
                        </div>
                    </div>
                    
                    <div style="background-color: #e3f2fd; padding: 1rem; border-radius: 8px;">
                        <h5>{"Excitement for Future"}</h5>
                        <p>{"There's palpable excitement about new possibilities for impact, particularly around technology-enabled collaboration and expanded reach."}</p>
                        <div style="display: flex; gap: 0.5rem; margin-top: 0.5rem;">
                            <span style="background-color: #2196f3; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                                {"Mentioned by 10 groups"}
                            </span>
                        </div>
                    </div>
                </div>
            </div>
            
            <div style="margin: 2rem 0;">
                <h4>{"Collective Insights"}</h4>
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px; margin-bottom: 1rem;">
                    <h5>{"1. Our Integrated Approach Works"}</h5>
                    <p>{"The interconnected nature of our impact systems is not just theoretical - it's creating real value for community members and strengthening our collective resilience."}</p>
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
                    <h5>{"2. Connection Drives Impact"}</h5>
                    <p>{"The quality of connections between community members is as important as the quantity of activities. Deep, meaningful relationships amplify our collective impact."}</p>
                    <div style="display: flex; flex-wrap: wrap; gap: 0.5rem; margin-top: 0.5rem;">
                        <span style="background-color: #3498db; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                            {"Insight"
                        </span>
                        <span style="background-color: #3498db; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                            {"Actionable"
                        </span>
                    </div>
                </div>
            </div>
            
            <div style="background-color: #e8f4fc; padding: 1rem; border-radius: 8px;">
                <h4>{"Your Synthesis Contribution"}</h4>
                <textarea 
                    style="width: 100%; height: 100px; padding: 0.5rem; margin-top: 0.5rem;"
                    placeholder="What insights are emerging for you as we synthesize our community's reflections?"
                >
                </textarea>
                <button style="background-color: #9b59b6; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer; margin-top: 0.5rem;">
                    {"Add Synthesis Insight"}
                </button>
            </div>
        </div>
    }
}

/// Render the action planning phase
fn render_action_planning_phase() -> Html {
    html! {
        <div>
            <h3>{"Action Planning"}</h3>
            <p>{"Let's translate our insights into concrete actions for our community's continued growth."}</p>
            
            <div style="margin: 1rem 0;">
                <h4>{"Priority Action Areas"}</h4>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 1rem; margin-top: 1rem;">
                    <div style="background-color: #fff3e0; padding: 1rem; border-radius: 8px;">
                        <div style="display: flex; justify-content: space-between; align-items: center;">
                            <h5>{"Strengthen Cross-Domain Connections"}</h5>
                            <span style="background-color: #ff9800; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                                {"High Priority"}
                            </span>
                        </div>
                        <p>{"Create structured opportunities for members from different impact domains to connect and collaborate on shared initiatives."}</p>
                        <div style="display: flex; gap: 0.5rem; margin-top: 0.5rem;">
                            <button style="background-color: #9b59b6; color: white; border: none; padding: 0.25rem 0.5rem; border-radius: 4px; cursor: pointer; font-size: 0.8rem;">
                                {"Claim Action"}
                            </button>
                            <button style="background-color: #95a5a6; color: white; border: none; padding: 0.25rem 0.5rem; border-radius: 4px; cursor: pointer; font-size: 0.8rem;">
                                {"Add Details"}
                            </button>
                        </div>
                    </div>
                    
                    <div style="background-color: #e8f5e9; padding: 1rem; border-radius: 8px;">
                        <div style="display: flex; justify-content: space-between; align-items: center;">
                            <h5>{"Develop Connection-Mapping Tool"}</h5>
                            <span style="background-color: #4caf50; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                                {"Medium Priority"}
                            </span>
                        </div>
                        <p>{"Create a visualization tool that shows connections between community members, initiatives, and impact areas to help identify collaboration opportunities."}</p>
                        <div style="display: flex; gap: 0.5rem; margin-top: 0.5rem;">
                            <button style="background-color: #9b59b6; color: white; border: none; padding: 0.25rem 0.5rem; border-radius: 4px; cursor: pointer; font-size: 0.8rem;">
                                {"Claim Action"}
                            </button>
                            <button style="background-color: #95a5a6; color: white; border: none; padding: 0.25rem 0.5rem; border-radius: 4px; cursor: pointer; font-size: 0.8rem;">
                                {"Add Details"}
                            </button>
                        </div>
                    </div>
                </div>
            </div>
            
            <div style="margin: 2rem 0;">
                <h4>{"Proposed Action Items"}</h4>
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px; margin-bottom: 1rem;">
                    <div style="display: flex; justify-content: space-between; align-items: center;">
                        <div>
                            <h5>{"Monthly Cross-Domain Coffee Chats"}</h5>
                            <p>{"Monthly virtual gatherings where members from different impact domains share experiences and explore collaboration opportunities."}</p>
                        </div>
                        <div style="text-align: right;">
                            <span style="display: block; background-color: #9b59b6; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem; margin-bottom: 0.5rem;">
                                {"Proposed"}
                            </span>
                            <span style="display: block; font-size: 0.8rem; color: #666;">
                                {"Proposed by: Community Coordination Team"}
                            </span>
                        </div>
                    </div>
                    <div style="display: flex; gap: 0.5rem; margin-top: 0.5rem;">
                        <button style="background-color: #27ae60; color: white; border: none; padding: 0.25rem 0.5rem; border-radius: 4px; cursor: pointer; font-size: 0.8rem;">
                            {"Approve"}
                        </button>
                        <button style="background-color: #e74c3c; color: white; border: none; padding: 0.25rem 0.5rem; border-radius: 4px; cursor: pointer; font-size: 0.8rem;">
                            {"Request Changes"}
                        </button>
                        <button style="background-color: #95a5a6; color: white; border: none; padding: 0.25rem 0.5rem; border-radius: 4px; cursor: pointer; font-size: 0.8rem;">
                            {"Discuss"
                        </button>
                    </div>
                </div>
                
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px; margin-bottom: 1rem;">
                    <div style="display: flex; justify-content: space-between; align-items: center;">
                        <div>
                            <h5>{"Quarterly Community Reflection Retreat"}</h5>
                            <p>{"Dedicated time for the community to come together for deeper reflection, connection, and strategic planning."}</p>
                        </div>
                        <div style="text-align: right;">
                            <span style="display: block; background-color: #3498db; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem; margin-bottom: 0.5rem;">
                                {"Approved"}
                            </span>
                            <span style="display: block; font-size: 0.8rem; color: #666;">
                                {"Proposed by: Governance Committee"}
                            </span>
                        </div>
                    </div>
                    <div style="display: flex; gap: 0.5rem; margin-top: 0.5rem;">
                        <button style="background-color: #95a5a6; color: white; border: none; padding: 0.25rem 0.5rem; border-radius: 4px; cursor: pointer; font-size: 0.8rem;">
                            {"View Details"
                        </button>
                        <button style="background-color: #95a5a6; color: white; border: none; padding: 0.25rem 0.5rem; border-radius: 4px; cursor: pointer; font-size: 0.8rem;">
                            {"Track Progress"
                        </button>
                    </div>
                </div>
            </div>
            
            <div style="background-color: #e8f4fc; padding: 1rem; border-radius: 8px;">
                <h4>{"Add Your Action Item"}</h4>
                <div style="margin-top: 1rem;">
                    <input 
                        type="text" 
                        placeholder="Action item title" 
                        style="width: 100%; padding: 0.5rem; margin-bottom: 0.5rem;"
                    />
                    <textarea 
                        placeholder="Description and implementation details" 
                        style="width: 100%; height: 80px; padding: 0.5rem; margin-bottom: 0.5rem;"
                    >
                    </textarea>
                    <div style="display: flex; gap: 1rem; margin-bottom: 0.5rem;">
                        <select style="flex: 1; padding: 0.5rem;">
                            <option>{"Priority: Select"}</option>
                            <option>{"Low"}</option>
                            <option>{"Medium"}</option>
                            <option>{"High"}</option>
                            <option>{"Critical"}</option>
                        </select>
                        <input 
                            type="text" 
                            placeholder="Timeline" 
                            style="flex: 1; padding: 0.5rem;"
                        />
                    </div>
                    <button style="background-color: #9b59b6; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer;">
                        {"Propose Action Item"}
                    </button>
                </div>
            </div>
        </div>
    }
}