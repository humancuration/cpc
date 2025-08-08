use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::types::Course;
use skill_development::ml::LearnerProfile;
use std::collections::HashMap;

#[derive(Properties, PartialEq)]
pub struct PersonalRelevanceProps {
    pub course: Course,
    pub profile: LearnerProfile,
    pub current_volunteer_activities: Vec<String>,
    pub expressed_values: Vec<String>,
}

#[styled_component(PersonalRelevanceIndicator)]
pub fn personal_relevance_indicator(props: &PersonalRelevanceProps) -> Html {
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

    let relevance_score_style = style!(
        r#"
        display: flex;
        align-items: center;
        justify-content: center;
        width: 100px;
        height: 100px;
        border-radius: 50%;
        background: var(--background-secondary);
        margin: 0 auto 1rem auto;
        font-size: 1.5rem;
        font-weight: bold;
        border: 4px solid var(--primary);
    "#
    ).unwrap();

    let alignment_item_style = style!(
        r#"
        display: flex;
        align-items: center;
        margin: 0.5rem 0;
        padding: 0.5rem;
        background: var(--background-secondary);
        border-radius: 4px;
    "#
    ).unwrap();

    let icon_style = style!(
        r#"
        margin-right: 0.5rem;
        color: var(--primary);
        font-weight: bold;
    "#
    ).unwrap();

    html! {
        <div class={container_style}>
            <div class={title_style.clone()}>
                <span>{"Personal Relevance"}</span>
                <span style="font-size: 0.9rem; color: var(--text-secondary);">{"Match Score"}</span>
            </div>
            
            <div class={relevance_score_style}>
                {"92%"}
            </div>
            
            <div style="text-align: center; margin-bottom: 1rem;">
                <h3>{"Excellent Match!"}</h3>
                <p>{"This course aligns strongly with your goals and community needs"}</p>
            </div>
            
            <div style="margin: 1rem 0;">
                <h4>{"Alignment Factors"}</h4>
                
                <div class={alignment_item_style.clone()}>
                    <span class={icon_style.clone()}>{"✓"}</span>
                    <span>{"Matches your learning goals: Master Data Science"}</span>
                </div>
                
                <div class={alignment_item_style.clone()}>
                    <span class={icon_style.clone()}>{"✓"}</span>
                    <span>{"Connects to your current volunteer work: Community Data Project"}</span>
                </div>
                
                <div class={alignment_item_style.clone()}>
                    <span class={icon_style.clone()}>{"✓"}</span>
                    <span>{"Aligns with your values: Community Service, Education"}</span>
                </div>
                
                <div class={alignment_item_style}>
                    <span class={icon_style}>{"✓"}</span>
                    <span>{"Addresses high community demand skills"}</span>
                </div>
            </div>
            
            <div style="margin-top: 1rem; padding: 1rem; background: var(--background-secondary); border-radius: 4px;">
                <h4>{"Pathway to Community Roles"}</h4>
                <p>{"Completing this course will prepare you for:"}</p>
                <ul>
                    <li>{"Community Data Analyst (Ready in 40 hours)"}</li>
                    <li>{"Project Lead for Local Non-Profit Analytics (Ready in 80 hours)"}</li>
                    <li>{"Data Science Mentor for New Learners (Ready immediately)"}</li>
                </ul>
            </div>
        </div>
    }
}