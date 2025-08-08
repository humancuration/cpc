use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::types::{SkillProgress, Certification};
use std::collections::HashMap;

#[derive(Properties, PartialEq)]
pub struct ValidationBadgesProps {
    pub skill_progress: SkillProgress,
    pub community_validations: Vec<String>,
    pub validators: HashMap<String, String>, // validator_id -> validator_name
}

#[styled_component(ValidationBadges)]
pub fn validation_badges(props: &ValidationBadgesProps) -> Html {
    let container_style = style!(
        r#"
        background: var(--surface);
        border-radius: 8px;
        padding: 1.5rem;
        box-shadow: 0 2px 8px rgba(0,0,0,0.1);
        margin-bottom: 2rem;
    "#
    ).unwrap();

    let title_style = style!(
        r#"
        margin-top: 0;
        margin-bottom: 1rem;
        font-size: 1.25rem;
        color: var(--text-primary);
        display: flex;
        justify-content: space-between;
        align-items: center;
    "#
    ).unwrap();

    let badge_container_style = style!(
        r#"
        display: flex;
        flex-wrap: wrap;
        gap: 1rem;
        margin: 1rem 0;
    "#
    ).unwrap();

    let badge_style = style!(
        r#"
        background: var(--background-secondary);
        border-radius: 8px;
        padding: 1rem;
        text-align: center;
        min-width: 150px;
        position: relative;
        border: 2px solid var(--border-color);
        
        &.validated {
            border-color: var(--primary);
            background: linear-gradient(135deg, var(--background-secondary) 0%, var(--surface) 100%);
        }
        
        &.pending {
            border-color: var(--secondary);
            background: linear-gradient(135deg, var(--background-secondary) 0%, var(--surface) 100%);
        }
    "#
    ).unwrap();

    let validator_network_style = style!(
        r#"
        display: flex;
        flex-wrap: wrap;
        gap: 0.5rem;
        margin: 1rem 0;
    "#
    ).unwrap();

    let validator_style = style!(
        r#"
        background: var(--background-secondary);
        border-radius: 20px;
        padding: 0.5rem 1rem;
        font-size: 0.8rem;
        
        &.you {
            background: var(--primary);
            color: white;
        }
    "#
    ).unwrap();

    html! {
        <div class={container_style}>
            <div class={title_style.clone()}>
                <span>{"Community Validation"}</span>
                <span style="font-size: 0.9rem; color: var(--text-secondary);">{"Skill Recognition"}</span>
            </div>
            
            <div class={badge_container_style}>
                <div class={badge_style.clone() + " validated"}>
                    <div style="font-size: 2rem; margin-bottom: 0.5rem;">{"🏆"}</div>
                    <div style="font-weight: bold; color: var(--primary);">{"Community Validated"}</div>
                    <div style="font-size: 0.8rem;">{"Rust Programming"}</div>
                    <div style="font-size: 0.7rem; color: var(--text-secondary); margin-top: 0.5rem;">{"Validated by 12 community members"}</div>
                </div>
                
                <div class={badge_style.clone() + " pending"}>
                    <div style="font-size: 2rem; margin-bottom: 0.5rem;">{"⏳"}</div>
                    <div style="font-weight: bold; color: var(--secondary);">{"Pending Validation"}</div>
                    <div style="font-size: 0.8rem;">{"Data Analysis"}</div>
                    <div style="font-size: 0.7rem; color: var(--text-secondary); margin-top: 0.5rem;">{"2 validations received"}</div>
                </div>
                
                <div class={badge_style.clone()}>
                    <div style="font-size: 2rem; margin-bottom: 0.5rem;">{"🔓"}</div>
                    <div style="font-weight: bold;">{"Available for Validation"}</div>
                    <div style="font-size: 0.8rem;">{"Project Management"}</div>
                    <div style="font-size: 0.7rem; color: var(--text-secondary); margin-top: 0.5rem;">{"Ready for community review"}</div>
                </div>
            </div>
            
            <div style="margin: 1rem 0;">
                <h4>{"Validation Network"}</h4>
                <div class={validator_network_style}>
                    <div class={validator_style.clone() + " you"}>{"You"}</div>
                    <div class={validator_style.clone()}>{"Alex M."}</div>
                    <div class={validator_style.clone()}>{"Sam T."}</div>
                    <div class={validator_style.clone()}>{"Jordan K."}</div>
                    <div class={validator_style.clone()}>{"Taylor R."}</div>
                    <div class={validator_style.clone()}>{"Casey L."}</div>
                    <div class={validator_style}>{"+7 more"}</div>
                </div>
            </div>
            
            <div style="margin-top: 1rem; padding: 1rem; background: var(--background-secondary); border-radius: 4px;">
                <h4>{"Get Your Skills Validated"}</h4>
                <p>{"Share your work with the community to get validation:"}</p>
                <button style="background: var(--primary); color: white; border: none; padding: 0.75rem 1.5rem; border-radius: 4px; cursor: pointer; margin-top: 0.5rem;">
                    {"Request Validation for Data Analysis"}
                </button>
            </div>
        </div>
    }
}