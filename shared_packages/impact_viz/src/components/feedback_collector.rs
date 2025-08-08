//! Feedback collector component for impact visualizations
//!
//! This component provides a reusable feedback collection interface that can be
//! embedded in any impact visualization to collect user feedback.

use yew::prelude::*;
use stylist::{style, yew::styled_component};
use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;

/// Properties for the FeedbackCollector component
#[derive(Properties, PartialEq)]
pub struct FeedbackCollectorProps {
    /// Unique identifier for the visualization component
    pub component_id: String,
    
    /// Callback function to handle feedback submission
    #[prop_or_default]
    pub on_feedback_submit: Callback<FeedbackData>,
}

/// Data structure for feedback submission
#[derive(Clone, Debug)]
pub struct FeedbackData {
    /// Component identifier
    pub component_id: String,
    
    /// Whether the visualization was helpful
    pub helpful: bool,
    
    /// Optional rating (1-5 stars)
    pub rating: Option<u8>,
    
    /// Optional comment
    pub comment: Option<String>,
    
    /// How the visualization affected volunteer decisions
    pub decision_impact: Option<String>,
}

/// Feedback collector component for impact visualizations
#[styled_component(FeedbackCollector)]
pub fn feedback_collector(props: &FeedbackCollectorProps) -> Html {
    let container_style = style!(
        r#"
        background: var(--surface);
        border-radius: 8px;
        padding: 1rem;
        box-shadow: 0 2px 8px rgba(0,0,0,0.1);
        margin-top: 1rem;
        font-family: inherit;
    "#
    ).unwrap();

    let title_style = style!(
        r#"
        margin-top: 0;
        margin-bottom: 1rem;
        font-size: 1rem;
        color: var(--text-primary);
    "#
    ).unwrap();

    let button_group_style = style!(
        r#"
        display: flex;
        gap: 0.5rem;
        margin-bottom: 1rem;
        flex-wrap: wrap;
    "#
    ).unwrap();

    let feedback_button_style = style!(
        r#"
        background: var(--background-secondary);
        border: 1px solid var(--border-color);
        border-radius: 4px;
        padding: 0.5rem 1rem;
        cursor: pointer;
        font-size: 0.9rem;
        font-family: inherit;
        
        &:hover {
            background: var(--primary);
            color: white;
        }
        
        &.selected {
            background: var(--primary);
            color: white;
        }
    "#
    ).unwrap();

    let comment_style = style!(
        r#"
        width: 100%;
        padding: 0.5rem;
        border: 1px solid var(--border-color);
        border-radius: 4px;
        margin-bottom: 1rem;
        font-family: inherit;
        min-height: 80px;
    "#
    ).unwrap();

    let submit_button_style = style!(
        r#"
        background: var(--primary);
        color: white;
        border: none;
        border-radius: 4px;
        padding: 0.5rem 1rem;
        cursor: pointer;
        font-size: 0.9rem;
        font-family: inherit;
        
        &:hover {
            background: #2980b9;
        }
        
        &:disabled {
            background: #bdc3c7;
            cursor: not-allowed;
        }
    "#
    ).unwrap();

    let helpful = use_state(|| None::<bool>);
    let rating = use_state(|| None::<u8>);
    let comment = use_state(|| String::new());
    let decision_impact = use_state(|| String::new());

    let on_helpful_click = {
        let helpful = helpful.clone();
        Callback::from(move |_| helpful.set(Some(true)))
    };

    let on_not_helpful_click = {
        let helpful = helpful.clone();
        Callback::from(move |_| helpful.set(Some(false)))
    };

    let on_rating_click = {
        let rating = rating.clone();
        Callback::from(move |value: u8| rating.set(Some(value)))
    };

    let on_comment_input = {
        let comment = comment.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            comment.set(input.value());
        })
    };

    let on_decision_impact_input = {
        let decision_impact = decision_impact.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            decision_impact.set(input.value());
        })
    };

    let on_submit = {
        let helpful = helpful.clone();
        let rating = rating.clone();
        let comment = comment.clone();
        let decision_impact = decision_impact.clone();
        let component_id = props.component_id.clone();
        let on_feedback_submit = props.on_feedback_submit.clone();
        
        Callback::from(move |_| {
            let feedback_data = FeedbackData {
                component_id: component_id.clone(),
                helpful: helpful.as_ref().copied().unwrap_or(false),
                rating: *rating,
                comment: if comment.is_empty() { None } else { Some(comment.to_string()) },
                decision_impact: if decision_impact.is_empty() { None } else { Some(decision_impact.to_string()) },
            };
            on_feedback_submit.emit(feedback_data);
            
            // Reset form
            helpful.set(None);
            rating.set(None);
            comment.set(String::new());
            decision_impact.set(String::new());
        })
    };

    let is_submit_disabled = helpful.is_none();

    html! {
        <div class={container_style}>
            <h4 class={title_style}>{"Was this visualization helpful for your volunteer work?"}</h4>
            
            <div class={button_group_style.clone()}>
                <button 
                    class={feedback_button_style.clone() + if *helpful == Some(true) { " selected" } else { "" }}
                    onclick={on_helpful_click}
                >
                    {"Yes"}
                </button>
                <button 
                    class={feedback_button_style.clone() + if *helpful == Some(false) { " selected" } else { "" }}
                    onclick={on_not_helpful_click}
                >
                    {"No"}
                </button>
            </div>
            
            if helpful.is_some() {
                <div>
                    <h4 class={title_style.clone()}>{"Rate this visualization (1-5 stars)"}</h4>
                    <div class={button_group_style.clone()}>
                        {(1..=5).map(|i| {
                            let rating_callback = on_rating_click.clone();
                            let rating_class = feedback_button_style.clone() + if *rating == Some(i) { " selected" } else { "" };
                            html! {
                                <button 
                                    class={rating_class}
                                    onclick={move |_| rating_callback.emit(i)}
                                >
                                    {i}
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                    
                    <h4 class={title_style.clone()}>{"Additional comments (optional)"}</h4>
                    <textarea
                        class={comment_style.clone()}
                        placeholder="How could this visualization be improved?"
                        value={(*comment).clone()}
                        oninput={on_comment_input}
                    />
                    
                    <h4 class={title_style.clone()}>{"How did this visualization affect your volunteer decisions? (optional)"}</h4>
                    <textarea
                        class={comment_style}
                        placeholder="Did this visualization help you choose volunteer opportunities or understand your impact?"
                        value={(*decision_impact).clone()}
                        oninput={on_decision_impact_input}
                    />
                    
                    <button 
                        class={submit_button_style} 
                        onclick={on_submit}
                        disabled={is_submit_disabled}
                    >
                        {"Submit Feedback"}
                    </button>
                </div>
            }
        </div>
    }
}