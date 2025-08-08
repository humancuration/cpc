use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::types::SkillProgress;
use chrono::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProgressNarrativeProps {
    pub skill_progress: Vec<SkillProgress>,
    pub community_impact_stories: Vec<String>,
}

#[styled_component(ProgressNarrative)]
pub fn progress_narrative(props: &ProgressNarrativeProps) -> Html {
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

    let timeline_style = style!(
        r#"
        position: relative;
        margin: 2rem 0;
        padding-left: 30px;
        
        &::before {
            content: '';
            position: absolute;
            left: 15px;
            top: 0;
            bottom: 0;
            width: 2px;
            background: var(--border-color);
        }
    "#
    ).unwrap();

    let timeline_item_style = style!(
        r#"
        position: relative;
        margin-bottom: 2rem;
        
        &::before {
            content: '';
            position: absolute;
            left: -35px;
            top: 5px;
            width: 12px;
            height: 12px;
            border-radius: 50%;
            background: var(--primary);
            border: 3px solid white;
            box-shadow: 0 0 0 2px var(--primary);
        }
    "#
    ).unwrap();

    let story_card_style = style!(
        r#"
        background: var(--background-secondary);
        border-radius: 4px;
        padding: 1rem;
        margin: 1rem 0;
        border-left: 4px solid var(--primary);
    "#
    ).unwrap();

    let progress_visualization_style = style!(
        r#"
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
        gap: 1rem;
        margin: 1rem 0;
    "#
    ).unwrap();

    let progress_item_style = style!(
        r#"
        background: var(--background-secondary);
        border-radius: 4px;
        padding: 1rem;
        text-align: center;
    "#
    ).unwrap();

    html! {
        <div class={container_style}>
            <div class={title_style.clone()}>
                <span>{"Your Learning Impact Journey"}</span>
                <button style="background: var(--primary); color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer;">
                    {"Share Your Story"}
                </button>
            </div>
            
            <div class={progress_visualization_style}>
                <div class={progress_item_style.clone()}>
                    <div style="font-size: 2rem; font-weight: bold; color: var(--primary);">{"85%"}</div>
                    <div>{"Overall Progress"}</div>
                </div>
                
                <div class={progress_item_style.clone()}>
                    <div style="font-size: 2rem; font-weight: bold; color: var(--primary);">{"42"}</div>
                    <div>{"Hours Invested"}</div>
                </div>
                
                <div class={progress_item_style}>
                    <div style="font-size: 2rem; font-weight: bold; color: var(--primary);">{"3"}</div>
                    <div>{"Community Projects"}</div>
                </div>
            </div>
            
            <div class={timeline_style}>
                <div class={timeline_item_style.clone()}>
                    <h3>{"Week 1-2: Foundations"}</h3>
                    <p>{"Started learning Rust programming basics. Completed 12 lessons."}</p>
                    <div class={story_card_style.clone()}>
                        <p><strong>{"Community Impact:"}</strong>{" Your first project helped analyze data for the local food bank, identifying patterns in donation needs."}</p>
                    </div>
                </div>
                
                <div class={timeline_item_style.clone()}>
                    <h3>{"Week 3-4: Intermediate Skills"}</h3>
                    <p>{"Mastered data structures and began working on real projects. Completed 18 lessons."}</p>
                    <div class={story_card_style.clone()}>
                        <p><strong>{"Community Impact:"}</strong>{" Your analysis skills helped optimize volunteer scheduling for the community center, saving 10 hours per week."}</p>
                    </div>
                </div>
                
                <div class={timeline_item_style}>
                    <h3>{"Week 5-6: Advanced Applications"}</h3>
                    <p>{"Started applying skills to community projects. Completed 15 lessons."}</p>
                    <div class={story_card_style}>
                        <p><strong>{"Community Impact:"}</strong>{" Your work on the education data project helped identify learning gaps affecting 200+ students."}</p>
                    </div>
                </div>
            </div>
            
            <div style="margin-top: 1rem; padding: 1rem; background: var(--background-secondary); border-radius: 4px;">
                <h4>{"Your Next Impact Opportunity"}</h4>
                <p>{"Complete 10 more hours to unlock the 'Data Visualization Specialist' community role."}</p>
                <p>{"This role will allow you to help 3 ongoing projects with data storytelling."}</p>
            </div>
        </div>
    }
}