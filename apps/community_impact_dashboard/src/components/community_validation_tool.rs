//! Community Validation Tool Component
//!
//! This component provides a structured validation workflow for collective impact metrics
//! and enables community discussion threads tied to specific impact visualizations.

use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::models::{UnifiedImpactData, impact_data::ImpactDomain};
use social_interactions::domain::{Comment, Reaction, ReactionType, TargetType};
use uuid::Uuid;
use chrono::Utc;

/// Properties for the CommunityValidationTool component
#[derive(Properties, PartialEq)]
pub struct CommunityValidationToolProps {
    /// Impact data to validate
    pub impact_data: UnifiedImpactData,
    
    /// Callback when validation is submitted
    pub on_validate: Callback<ValidationResult>,
}

/// State for the CommunityValidationTool component
#[derive(Clone, PartialEq)]
pub struct CommunityValidationToolState {
    /// Current validation step
    current_step: ValidationStep,
    
    /// User's validation feedback
    feedback: ValidationFeedback,
    
    /// Comments on specific metrics
    comments: Vec<MetricComment>,
}

/// Validation steps
#[derive(Clone, PartialEq)]
pub enum ValidationStep {
    ReviewMetrics,
    ProvideFeedback,
    AddComments,
    Submit,
}

/// Validation feedback structure
#[derive(Clone, PartialEq)]
pub struct ValidationFeedback {
    /// Overall accuracy rating (1-5 stars)
    pub accuracy_rating: u8,
    
    /// Confidence in the metrics
    pub confidence_level: ConfidenceLevel,
    
    /// Additional feedback text
    pub feedback_text: String,
    
    /// Areas that need improvement
    pub improvement_areas: Vec<String>,
}

/// Confidence levels
#[derive(Clone, PartialEq)]
pub enum ConfidenceLevel {
    Low,
    Medium,
    High,
}

/// Comment on a specific metric
#[derive(Clone, PartialEq)]
pub struct MetricComment {
    /// Unique identifier
    pub id: Uuid,
    
    /// The metric this comment is about
    pub metric_name: String,
    
    /// The domain this metric belongs to
    pub domain: ImpactDomain,
    
    /// Comment text
    pub comment: String,
    
    /// Timestamp
    pub timestamp: chrono::DateTime<Utc>,
    
    /// Reactions to this comment
    pub reactions: Vec<Reaction>,
}

/// Validation result structure
#[derive(Clone, PartialEq)]
pub struct ValidationResult {
    /// User's feedback
    pub feedback: ValidationFeedback,
    
    /// Comments on metrics
    pub comments: Vec<MetricComment>,
    
    /// Timestamp of validation
    pub timestamp: chrono::DateTime<Utc>,
}

/// Community Validation Tool Component
#[styled_component(CommunityValidationTool)]
pub fn community_validation_tool(props: &CommunityValidationToolProps) -> Html {
    let state = use_state(|| CommunityValidationToolState {
        current_step: ValidationStep::ReviewMetrics,
        feedback: ValidationFeedback {
            accuracy_rating: 3,
            confidence_level: ConfidenceLevel::Medium,
            feedback_text: String::new(),
            improvement_areas: Vec::new(),
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
                ValidationStep::ReviewMetrics => ValidationStep::ProvideFeedback,
                ValidationStep::ProvideFeedback => ValidationStep::AddComments,
                ValidationStep::AddComments => ValidationStep::Submit,
                ValidationStep::Submit => ValidationStep::Submit,
            };
            
            state.set(CommunityValidationToolState {
                current_step: next_step,
                ..(*state).clone()
            });
        })
    };
    
    let on_previous = {
        let state = state.clone();
        Callback::from(move |_| {
            let prev_step = match state.current_step {
                ValidationStep::ReviewMetrics => ValidationStep::ReviewMetrics,
                ValidationStep::ProvideFeedback => ValidationStep::ReviewMetrics,
                ValidationStep::AddComments => ValidationStep::ProvideFeedback,
                ValidationStep::Submit => ValidationStep::AddComments,
            };
            
            state.set(CommunityValidationToolState {
                current_step: prev_step,
                ..(*state).clone()
            });
        })
    };
    
    let on_submit = {
        let state = state.clone();
        let on_validate = props.on_validate.clone();
        
        Callback::from(move |_| {
            let validation_result = ValidationResult {
                feedback: state.feedback.clone(),
                comments: state.comments.clone(),
                timestamp: Utc::now(),
            };
            
            on_validate.emit(validation_result);
        })
    };
    
    html! {
        <div class={container_style}>
            <h2>{"Community Validation of Impact Metrics"}</h2>
            <p>{"Help us ensure our impact measurements accurately reflect community reality."}</p>
            
            <div style="position: relative;">
                <div class={step_indicator_style}>
                    <div class={step_connector_style}></div>
                    <div class={step_style}>
                        <div class={classes!(step_number_style.clone(), 
                            if matches!(state.current_step, ValidationStep::ReviewMetrics) { 
                                active_step_style.clone() 
                            } else { 
                                style!("").unwrap() 
                            })}>
                            {"1"}
                        </div>
                        <div>{"Review Metrics"}</div>
                    </div>
                    <div class={step_style}>
                        <div class={classes!(step_number_style.clone(), 
                            if matches!(state.current_step, ValidationStep::ProvideFeedback) { 
                                active_step_style.clone() 
                            } else { 
                                style!("").unwrap() 
                            })}>
                            {"2"}
                        </div>
                        <div>{"Provide Feedback"}</div>
                    </div>
                    <div class={step_style}>
                        <div class={classes!(step_number_style.clone(), 
                            if matches!(state.current_step, ValidationStep::AddComments) { 
                                active_step_style.clone() 
                            } else { 
                                style!("").unwrap() 
                            })}>
                            {"3"}
                        </div>
                        <div>{"Add Comments"}</div>
                    </div>
                    <div class={step_style}>
                        <div class={classes!(step_number_style.clone(), 
                            if matches!(state.current_step, ValidationStep::Submit) { 
                                active_step_style.clone() 
                            } else { 
                                style!("").unwrap() 
                            })}>
                            {"4"}
                        </div>
                        <div>{"Submit"}</div>
                    </div>
                </div>
            </div>
            
            <div style="margin: 2rem 0;">
                {render_validation_step(&state.current_step, &props.impact_data, &state.feedback, &state.comments)}
            </div>
            
            <div style="display: flex; justify-content: space-between; margin-top: 2rem;">
                <div>
                    {if !matches!(state.current_step, ValidationStep::ReviewMetrics) {
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
                        ValidationStep::ReviewMetrics => html! {
                            <button 
                                class={classes!(button_style.clone(), primary_button_style.clone())}
                                onclick={on_next}
                            >
                                {"Continue to Feedback"}
                            </button>
                        },
                        ValidationStep::ProvideFeedback => html! {
                            <button 
                                class={classes!(button_style.clone(), primary_button_style.clone())}
                                onclick={on_next}
                            >
                                {"Continue to Comments"}
                            </button>
                        },
                        ValidationStep::AddComments => html! {
                            <button 
                                class={classes!(button_style.clone(), primary_button_style.clone())}
                                onclick={on_next}
                            >
                                {"Review & Submit"}
                            </button>
                        },
                        ValidationStep::Submit => html! {
                            <button 
                                class={classes!(button_style.clone(), primary_button_style.clone())}
                                onclick={on_submit}
                            >
                                {"Submit Validation"}
                            </button>
                        },
                    }}
                </div>
            </div>
        </div>
    }
}

/// Render the appropriate validation step
fn render_validation_step(
    step: &ValidationStep, 
    impact_data: &UnifiedImpactData,
    feedback: &ValidationFeedback,
    comments: &Vec<MetricComment>
) -> Html {
    match step {
        ValidationStep::ReviewMetrics => render_review_metrics_step(impact_data),
        ValidationStep::ProvideFeedback => render_provide_feedback_step(feedback),
        ValidationStep::AddComments => render_add_comments_step(impact_data, comments),
        ValidationStep::Submit => render_submit_step(feedback, comments),
    }
}

/// Render the review metrics step
fn render_review_metrics_step(impact_data: &UnifiedImpactData) -> Html {
    html! {
        <div>
            <h3>{"Review Community Impact Metrics"}</h3>
            <p>{"Please review the following metrics and consider if they accurately represent community impact:"}</p>
            
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 1rem; margin-top: 1rem;">
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px;">
                    <h4>{"Learning Impact"}</h4>
                    <p>{"Knowledge Sharing Rate: "}{format!("{:.1}%", impact_data.community_wellbeing.domain_indicators.learning.knowledge_sharing_rate * 100.0)}</p>
                    <p>{"Skill Development Progress: "}{format!("{:.1}%", impact_data.community_wellbeing.domain_indicators.learning.skill_development_progress * 100.0)}</p>
                </div>
                
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px;">
                    <h4>{"Volunteer Impact"}</h4>
                    <p>{"Participation Rate: "}{format!("{:.1}%", impact_data.community_wellbeing.domain_indicators.volunteer.participation_rate * 100.0)}</p>
                    <p>{"Satisfaction Index: "}{format!("{:.1}%", impact_data.community_wellbeing.domain_indicators.volunteer.satisfaction_index * 100.0)}</p>
                </div>
                
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px;">
                    <h4>{"Financial Impact"}</h4>
                    <p>{"Financial Health: "}{format!("{:.1}%", impact_data.community_wellbeing.domain_indicators.financial.financial_health * 100.0)}</p>
                    <p>{"Resource Equity: "}{format!("{:.1}%", impact_data.community_wellbeing.domain_indicators.financial.resource_equity * 100.0)}</p>
                </div>
                
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px;">
                    <h4>{"Cause Impact"}</h4>
                    <p>{"Engagement Rate: "}{format!("{:.1}%", impact_data.community_wellbeing.domain_indicators.cause.engagement_rate * 100.0)}</p>
                    <p>{"Solidarity Index: "}{format!("{:.1}%", impact_data.community_wellbeing.domain_indicators.cause.solidarity_index * 100.0)}</p>
                </div>
            </div>
            
            <div style="margin-top: 1rem; padding: 1rem; background-color: #e8f4fc; border-radius: 8px;">
                <h4>{"Overall Community Wellbeing"}</h4>
                <p>{"Score: "}{format!("{:.1}%", impact_data.community_wellbeing.overall_score * 100.0)}</p>
            </div>
        </div>
    }
}

/// Render the provide feedback step
fn render_provide_feedback_step(feedback: &ValidationFeedback) -> Html {
    html! {
        <div>
            <h3>{"Provide Your Feedback"}</h3>
            <p>{"How accurate do you think these metrics are in representing community impact?"}</p>
            
            <div style="margin: 1rem 0;">
                <label>{"Accuracy Rating (1-5 stars):"}</label>
                <div style="margin: 0.5rem 0;">
                    {for (1..=5).map(|i| {
                        let star_style = if i <= feedback.accuracy_rating {
                            "color: #ffc107; font-size: 1.5rem; cursor: pointer;"
                        } else {
                            "color: #ddd; font-size: 1.5rem; cursor: pointer;"
                        };
                        
                        html! {
                            <span 
                                style={star_style}
                                onclick={Callback::from(move |_| {
                                    // In a real implementation, this would update the state
                                })}
                            >
                                {"★"}
                            </span>
                        }
                    })}
                </div>
            </div>
            
            <div style="margin: 1rem 0;">
                <label>{"Confidence in these metrics:"}</label>
                <div style="margin: 0.5rem 0;">
                    <label style="margin-right: 1rem;">
                        <input 
                            type="radio" 
                            name="confidence" 
                            value="low" 
                            checked={matches!(feedback.confidence_level, ConfidenceLevel::Low)}
                        />
                        {"Low"}
                    </label>
                    <label style="margin-right: 1rem;">
                        <input 
                            type="radio" 
                            name="confidence" 
                            value="medium" 
                            checked={matches!(feedback.confidence_level, ConfidenceLevel::Medium)}
                        />
                        {"Medium"}
                    </label>
                    <label>
                        <input 
                            type="radio" 
                            name="confidence" 
                            value="high" 
                            checked={matches!(feedback.confidence_level, ConfidenceLevel::High)}
                        />
                        {"High"}
                    </label>
                </div>
            </div>
            
            <div style="margin: 1rem 0;">
                <label>{"Additional Feedback:"}</label>
                <textarea 
                    style="width: 100%; height: 100px; padding: 0.5rem; margin-top: 0.5rem;"
                    placeholder="What do you think about these metrics? Are there important aspects missing?"
                >
                    {&feedback.feedback_text}
                </textarea>
            </div>
            
            <div style="margin: 1rem 0;">
                <label>{"Areas for Improvement:"}</label>
                <textarea 
                    style="width: 100%; height: 100px; padding: 0.5rem; margin-top: 0.5rem;"
                    placeholder="What could be measured better or differently to more accurately represent community impact?"
                >
                </textarea>
            </div>
        </div>
    }
}

/// Render the add comments step
fn render_add_comments_step(impact_data: &UnifiedImpactData, comments: &Vec<MetricComment>) -> Html {
    html! {
        <div>
            <h3>{"Add Comments to Specific Metrics"}</h3>
            <p>{"Share your thoughts on specific metrics or suggest improvements:"}</p>
            
            <div style="margin: 1rem 0;">
                <label>{"Select a metric to comment on:"}</label>
                <select style="width: 100%; padding: 0.5rem; margin: 0.5rem 0;">
                    <option value="">{"Select a metric..."}</option>
                    <option value="learning-knowledge-sharing">{"Learning: Knowledge Sharing Rate"}</option>
                    <option value="learning-skill-development">{"Learning: Skill Development Progress"}</option>
                    <option value="volunteer-participation">{"Volunteer: Participation Rate"}</option>
                    <option value="volunteer-satisfaction">{"Volunteer: Satisfaction Index"}</option>
                    <option value="financial-health">{"Financial: Financial Health"}</option>
                    <option value="financial-equity">{"Financial: Resource Equity"}</option>
                    <option value="cause-engagement">{"Cause: Engagement Rate"}</option>
                    <option value="cause-solidarity">{"Cause: Solidarity Index"}</option>
                    <option value="overall-wellbeing">{"Overall Community Wellbeing"}</option>
                </select>
                
                <textarea 
                    style="width: 100%; height: 100px; padding: 0.5rem; margin: 0.5rem 0;"
                    placeholder="Add your comment..."
                >
                </textarea>
                
                <button style="background-color: #3498db; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer;">
                    {"Add Comment"}
                </button>
            </div>
            
            <div style="margin-top: 2rem;">
                <h4>{"Existing Comments"}</h4>
                {if comments.is_empty() {
                    html! {
                        <p>{"No comments yet. Be the first to share your thoughts!"}</p>
                    }
                } else {
                    html! {
                        <div>
                            {for comments.iter().map(|comment| {
                                html! {
                                    <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px; margin-bottom: 1rem;">
                                        <div style="display: flex; justify-content: space-between;">
                                            <strong>{&comment.metric_name}</strong>
                                            <span>{comment.timestamp.format("%Y-%m-%d").to_string()}</span>
                                        </div>
                                        <p>{&comment.comment}</p>
                                        <div style="display: flex; gap: 1rem; margin-top: 0.5rem;">
                                            <button style="background: none; border: none; cursor: pointer; color: #666;">
                                                {"👍 Like ("}{comment.reactions.len()}{")"}
                                            </button>
                                            <button style="background: none; border: none; cursor: pointer; color: #666;">
                                                {"Reply"}
                                            </button>
                                        </div>
                                    </div>
                                }
                            })}
                        </div>
                    }
                }}
            </div>
        </div>
    }
}

/// Render the submit step
fn render_submit_step(feedback: &ValidationFeedback, comments: &Vec<MetricComment>) -> Html {
    html! {
        <div>
            <h3>{"Review Your Validation Submission"}</h3>
            <p>{"Please review your feedback before submitting:"}</p>
            
            <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px; margin: 1rem 0;">
                <h4>{"Your Accuracy Rating"}</h4>
                <p>{"Stars: "}{feedback.accuracy_rating}{" / 5"}</p>
                
                <h4>{"Confidence Level"}</h4>
                <p>{
                    match feedback.confidence_level {
                        ConfidenceLevel::Low => "Low",
                        ConfidenceLevel::Medium => "Medium",
                        ConfidenceLevel::High => "High",
                    }
                }</p>
                
                <h4>{"Additional Feedback"}</h4>
                <p>{&feedback.feedback_text}</p>
            </div>
            
            <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px; margin: 1rem 0;">
                <h4>{"Your Comments ("}{comments.len()}{")"}</h4>
                {if comments.is_empty() {
                    html! { <p>{"No comments added."}</p> }
                } else {
                    html! {
                        <div>
                            {for comments.iter().map(|comment| {
                                html! {
                                    <div style="margin-bottom: 1rem; padding-bottom: 1rem; border-bottom: 1px solid #eee;">
                                        <strong>{&comment.metric_name}</strong>
                                        <p>{&comment.comment}</p>
                                    </div>
                                }
                            })}
                        </div>
                    }
                }}
            </div>
            
            <div style="background-color: #e8f5e9; padding: 1rem; border-radius: 8px; margin: 1rem 0;">
                <h4>{"Thank You for Your Validation!"}</h4>
                <p>{"Your feedback helps us ensure our impact metrics truly reflect community reality and values."}</p>
            </div>
        </div>
    }
}