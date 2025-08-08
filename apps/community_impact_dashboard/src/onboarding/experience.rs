//! Onboarding experience for the Unified Community Impact Dashboard
//!
//! This module provides a guided onboarding experience that introduces members
//! to the interconnected nature of impact in the cooperative ecosystem.

use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::models::impact_data::{ImpactDomain, MemberImpactData, EcosystemPosition};
use uuid::Uuid;

/// Properties for the OnboardingExperience component
#[derive(Properties, PartialEq)]
pub struct OnboardingExperienceProps {
    /// Member's engagement history
    pub member_data: MemberImpactData,
    
    /// Callback when onboarding is completed
    pub on_complete: Callback<()>,
}

/// State for the OnboardingExperience component
#[derive(Clone, PartialEq)]
pub struct OnboardingExperienceState {
    /// Current step in the onboarding process
    current_step: usize,
    
    /// Total number of steps
    total_steps: usize,
}

/// Onboarding Experience Component
#[styled_component(OnboardingExperience)]
pub fn onboarding_experience(props: &OnboardingExperienceProps) -> Html {
    let state = use_state(|| OnboardingExperienceState {
        current_step: 0,
        total_steps: 6, // Total number of onboarding steps
    });
    
    let container_style = style!(
        r#"
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: rgba(0, 0, 0, 0.8);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 1000;
    "#
    ).unwrap();
    
    let modal_style = style!(
        r#"
        background-color: white;
        border-radius: 8px;
        padding: 2rem;
        max-width: 800px;
        width: 90%;
        max-height: 90vh;
        overflow-y: auto;
        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
    "#
    ).unwrap();
    
    let progress_style = style!(
        r#"
        width: 100%;
        height: 10px;
        background-color: #eee;
        border-radius: 5px;
        margin-bottom: 2rem;
        overflow: hidden;
    "#
    ).unwrap();
    
    let progress_fill_style = style!(
        r#"
        height: 100%;
        background-color: #3498db;
        border-radius: 5px;
        width: ${percentage}%;
        transition: width 0.3s ease;
    "#,
        percentage = (state.current_step as f64 / state.total_steps as f64) * 100.0
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
        let total_steps = state.total_steps;
        let on_complete = props.on_complete.clone();
        
        Callback::from(move |_| {
            let current = state.current_step;
            if current < total_steps - 1 {
                state.set(OnboardingExperienceState {
                    current_step: current + 1,
                    ..(*state).clone()
                });
            } else {
                on_complete.emit(());
            }
        })
    };
    
    let on_previous = {
        let state = state.clone();
        Callback::from(move |_| {
            let current = state.current_step;
            if current > 0 {
                state.set(OnboardingExperienceState {
                    current_step: current - 1,
                    ..(*state).clone()
                });
            }
        })
    };
    
    let on_skip = {
        let on_complete = props.on_complete.clone();
        Callback::from(move |_| {
            on_complete.emit(());
        })
    };
    
    html! {
        <div class={container_style}>
            <div class={modal_style}>
                <div class={progress_style}>
                    <div class={progress_fill_style}></div>
                </div>
                
                {render_step(state.current_step, &props.member_data)}
                
                <div style="display: flex; justify-content: space-between; margin-top: 2rem;">
                    <div>
                        {if state.current_step > 0 {
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
                        <button 
                            class={classes!(button_style.clone(), secondary_button_style.clone())}
                            onclick={on_skip}
                        >
                            {"Skip Onboarding"}
                        </button>
                        
                        <button 
                            class={classes!(button_style.clone(), primary_button_style.clone())}
                            onclick={on_next}
                        >
                            {if state.current_step < state.total_steps - 1 { "Next" } else { "Get Started" }}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Render the appropriate step based on the current step index
fn render_step(step: usize, member_data: &MemberImpactData) -> Html {
    match step {
        0 => render_welcome_step(),
        1 => render_interconnected_impact_step(member_data),
        2 => render_your_position_step(member_data),
        3 => render_visualization_explanation_step(),
        4 => render_community_validation_step(),
        5 => render_personal_pathway_step(member_data),
        _ => html! { <div>{"Unknown step"}</div> }
    }
}

/// Render the welcome step
fn render_welcome_step() -> Html {
    html! {
        <div>
            <h2>{"Welcome to Your Community Impact Dashboard"}</h2>
            <p>{"Discover how your actions across learning, volunteering, financial participation, and cause engagement all interconnect to create meaningful community transformation."}</p>
            <div style="text-align: center; margin: 2rem 0;">
                <div style="font-size: 4rem; margin-bottom: 1rem;">{"🌍"}</div>
                <p>{"Together, we're building a stronger, more connected community."}</p>
            </div>
        </div>
    }
}

/// Render the interconnected impact step
fn render_interconnected_impact_step(member_data: &MemberImpactData) -> Html {
    html! {
        <div>
            <h2>{"The Interconnected Nature of Impact"}</h2>
            <p>{"In our cooperative community, all forms of impact are interconnected. Your engagement in one area strengthens the entire ecosystem:"}</p>
            
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem; margin: 2rem 0;">
                <div style="background-color: #e8f4fc; padding: 1rem; border-radius: 8px; text-align: center;">
                    <div style="font-size: 2rem; margin-bottom: 0.5rem;">{"📚"}</div>
                    <h3>{"Learning"}</h3>
                    <p>{"Builds skills that enhance volunteer effectiveness"}</p>
                </div>
                
                <div style="background-color: #e8f4fc; padding: 1rem; border-radius: 8px; text-align: center;">
                    <div style="font-size: 2rem; margin-bottom: 0.5rem;">{"🤝"}</div>
                    <h3>{"Volunteering"}</h3>
                    <p>{"Builds trust that leads to financial participation"}</p>
                </div>
                
                <div style="background-color: #e8f4fc; padding: 1rem; border-radius: 8px; text-align: center;">
                    <div style="font-size: 2rem; margin-bottom: 0.5rem;">{"💰"}</div>
                    <h3>{"Financial"}</h3>
                    <p>{"Enables more effective cause engagement"}</p>
                </div>
                
                <div style="background-color: #e8f4fc; padding: 1rem; border-radius: 8px; text-align: center;">
                    <div style="font-size: 2rem; margin-bottom: 0.5rem;">{"🌱"}</div>
                    <h3>{"Cause"}</h3>
                    <p>{"Inspires new learning paths and knowledge sharing"}</p>
                </div>
            </div>
            
            <p>{"This circular flow means your actions in any domain amplify impact across all others."}</p>
        </div>
    }
}

/// Render the "your position" step
fn render_your_position_step(member_data: &MemberImpactData) -> Html {
    let position = &member_data.ecosystem_position;
    
    html! {
        <div>
            <h2>{"Your Position in the Community Ecosystem"}</h2>
            <p>{"Based on your engagement history, here's how you're contributing to our community ecosystem:"}</p>
            
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem; margin: 2rem 0;">
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px;">
                    <h3>{"Learning Engagement"}</h3>
                    <div style="width: 100%; height: 10px; background-color: #eee; border-radius: 5px; margin: 0.5rem 0; overflow: hidden;">
                        <div style={format!("height: 100%; background-color: #4CAF50; border-radius: 5px; width: {}%;", position.learning_engagement * 100.0)}></div>
                    </div>
                    <p style="text-align: right; font-weight: bold;">{format!("{:.0}%", position.learning_engagement * 100.0)}</p>
                </div>
                
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px;">
                    <h3>{"Volunteer Participation"}</h3>
                    <div style="width: 100%; height: 10px; background-color: #eee; border-radius: 5px; margin: 0.5rem 0; overflow: hidden;">
                        <div style={format!("height: 100%; background-color: #2196F3; border-radius: 5px; width: {}%;", position.volunteer_participation * 100.0)}></div>
                    </div>
                    <p style="text-align: right; font-weight: bold;">{format!("{:.0}%", position.volunteer_participation * 100.0)}</p>
                </div>
                
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px;">
                    <h3>{"Financial Participation"}</h3>
                    <div style="width: 100%; height: 10px; background-color: #eee; border-radius: 5px; margin: 0.5rem 0; overflow: hidden;">
                        <div style={format!("height: 100%; background-color: #FF9800; border-radius: 5px; width: {}%;", position.financial_participation * 100.0)}></div>
                    </div>
                    <p style="text-align: right; font-weight: bold;">{format!("{:.0}%", position.financial_participation * 100.0)}</p>
                </div>
                
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px;">
                    <h3>{"Cause Engagement"}</h3>
                    <div style="width: 100%; height: 10px; background-color: #eee; border-radius: 5px; margin: 0.5rem 0; overflow: hidden;">
                        <div style={format!("height: 100%; background-color: #9C27B0; border-radius: 5px; width: {}%;", position.cause_engagement * 100.0)}></div>
                    </div>
                    <p style="text-align: right; font-weight: bold;">{format!("{:.0}%", position.cause_engagement * 100.0)}</p>
                </div>
            </div>
            
            <div style="background-color: #e8f4fc; padding: 1rem; border-radius: 8px; margin-top: 1rem;">
                <h3>{"Community Connectivity"}</h3>
                <p>{"Your overall connection to the community ecosystem:"}</p>
                <div style="width: 100%; height: 15px; background-color: #eee; border-radius: 7.5px; margin: 0.5rem 0; overflow: hidden;">
                    <div style={format!("height: 100%; background-color: #3498db; border-radius: 7.5px; width: {}%;", position.community_connectivity * 100.0)}></div>
                </div>
                <p style="text-align: right; font-weight: bold; font-size: 1.2rem;">{format!("{:.0}% Connected", position.community_connectivity * 100.0)}</p>
            </div>
        </div>
    }
}

/// Render the visualization explanation step
fn render_visualization_explanation_step() -> Html {
    html! {
        <div>
            <h2>{"Understanding Your Impact Visualizations"}</h2>
            <p>{"Our dashboard uses several visualization styles to help you understand your impact. Here's what to look for:"}</p>
            
            <div style="margin: 2rem 0;">
                <h3>{"Narrative View"}</h3>
                <p>{"Tells the story of your impact through connected explanations and examples."}</p>
                
                <h3>{"Comparative View"}</h3>
                <p>{"Shows how your impact compares across different domains and over time."}</p>
                
                <h3>{"Trend-Based View"}</h3>
                <p>{"Visualizes how your impact has evolved and where you're heading."}</p>
                
                <h3>{"Quantitative View"}</h3>
                <p>{"Presents precise metrics and data points for detailed analysis."}</p>
                
                <h3>{"Qualitative View"}</h3>
                <p>{"Provides contextual explanations and community perspectives on your impact."}</p>
            </div>
            
            <div style="background-color: #fff8e1; padding: 1rem; border-radius: 8px; border-left: 4px solid #ffc107;">
                <p>{"Tip: Try different visualization styles to find what helps you understand your impact best!"}</p>
            </div>
        </div>
    }
}

/// Render the community validation step
fn render_community_validation_step() -> Html {
    html! {
        <div>
            <h2>{"Community Validation of Impact"}</h2>
            <p>{"Your impact becomes even more meaningful when validated and interpreted by the community. Here's how you can participate:"}</p>
            
            <div style="margin: 2rem 0;">
                <h3>{"Collaborative Interpretation"}</h3>
                <p>{"Join structured sessions where community members make sense of impact data together, identifying patterns and insights."}</p>
                <div style="background-color: #e8f4fc; padding: 1rem; border-radius: 8px; margin: 1rem 0;">
                    <h4>{"How it works:"}</h4>
                    <ul>
                        <li>{"Review community impact data across all domains"}</li>
                        <li>{"Share your perspectives and observations"}</li>
                        <li>{"Identify emerging patterns together"}</li>
                        <li>{"Build consensus on key insights"}</li>
                        <li>{"Document actionable recommendations"}</li>
                    </ul>
                </div>
            </div>
            
            <div style="margin: 2rem 0;">
                <h3>{"Community Reflection Sessions"}</h3>
                <p>{"Participate in facilitated reflection sessions to deepen understanding of community transformation."}</p>
                <div style="background-color: #fff3e0; padding: 1rem; border-radius: 8px; margin: 1rem 0;">
                    <h4>{"Session phases:"}</h4>
                    <ul>
                        <li>{"Individual reflection on your impact journey"}</li>
                        <li>{"Small group sharing of experiences"}</li>
                        <li>{"Community synthesis of insights"}</li>
                        <li>{"Action planning for collective improvement"}</li>
                    </ul>
                </div>
            </div>
            
            <div style="margin: 2rem 0;">
                <h3>{"Documentation Center"}</h3>
                <p>{"Contribute to and access our community knowledge base of validated insights and best practices."}</p>
                <div style="background-color: #e8f5e9; padding: 1rem; border-radius: 8px; margin: 1rem 0;">
                    <h4>{"What you'll find:"}</h4>
                    <ul>
                        <li>{"Community interpretations of impact data"}</li>
                        <li>{"Reflection outcomes and insights"}</li>
                        <li>{"Actionable recommendations"}</li>
                        <li>{"Learning resources and best practices"}</li>
                    </ul>
                </div>
            </div>
            
            <div style="background-color: #e3f2fd; padding: 1rem; border-radius: 8px; border-left: 4px solid #2196f3;">
                <p>{"Community validation transforms individual actions into collective understanding and coordinated improvement."}</p>
            </div>
        </div>
    }
}

/// Render the personal pathway step
fn render_personal_pathway_step(member_data: &MemberImpactData) -> Html {
    html! {
        <div>
            <h2>{"Your Personal Impact Pathway"}</h2>
            <p>{"Based on your engagement history, here are insights about how your specific actions connect across domains:"}</p>
            
            <div style="background-color: #e8f5e9; padding: 1rem; border-radius: 8px; margin: 1rem 0;">
                <h3>{"Strengths & Opportunities"}</h3>
                <p>{"Your learning engagement is strong, which positions you well to take on leadership roles in volunteer activities."}</p>
            </div>
            
            <div style="background-color: #e3f2fd; padding: 1rem; border-radius: 8px; margin: 1rem 0;">
                <h3>{"Suggested Next Steps"}</h3>
                <p>{"Consider exploring volunteer opportunities that align with your learning interests to maximize your impact multiplier."}</p>
            </div>
            
            <div style="background-color: #f3e5f5; padding: 1rem; border-radius: 8px; margin: 1rem 0;">
                <h3>{"Community Connection"}</h3>
                <p>{"Your balanced engagement across domains makes you a valuable bridge between different community initiatives."}</p>
            </div>
            
            <div style="text-align: center; margin-top: 2rem;">
                <div style="font-size: 3rem; margin-bottom: 1rem;">{"🚀"}</div>
                <p>{"You're ready to explore your full impact potential!"}</p>
            </div>
        </div>
    }
}