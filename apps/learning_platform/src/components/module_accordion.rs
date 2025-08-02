use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::types::Module;

#[derive(Properties, PartialEq)]
pub struct ModuleAccordionProps {
    pub module: Module,
}

#[styled_component(ModuleAccordion)]
pub fn module_accordion(props: &ModuleAccordionProps) -> Html {
    let is_open = use_state(|| false);

    let toggle = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    let accordion_style = style!(
        r#"
        border: 1px solid var(--border);
        border-radius: 8px;
        margin-bottom: 1rem;
        overflow: hidden;
    "#
    ).unwrap();

    let header_style = style!(
        r#"
        background: var(--surface);
        padding: 1rem 1.5rem;
        cursor: pointer;
        display: flex;
        justify-content: space-between;
        align-items: center;
        font-weight: bold;

        &:hover {
            background: rgba(0,0,0,0.05);
        }
    "#
    ).unwrap();

    let content_style = style!(
        r#"
        padding: 0 1.5rem;
        max-height: 0;
        overflow: hidden;
        transition: max-height 0.3s ease, padding 0.3s ease;

        &[data-open="true"] {
            padding: 1rem 1.5rem;
            max-height: 1000px;
        }
    "#
    ).unwrap();

    let lesson_style = style!(
        r#"
        padding: 0.75rem 0;
        border-bottom: 1px solid var(--border);

        &:last-child {
            border-bottom: none;
        }
    "#
    ).unwrap();

    let lessons_list: Vec<Html> = props.module.lessons.iter().map(|lesson| {
        html! {
            <div class={lesson_style.clone()}>
                <h4>{&lesson.title}</h4>
                <p>{&lesson.content}</p>
            </div>
        }
    }).collect();

    html! {
        <div class={accordion_style}>
            <div class={header_style} onclick={toggle}>
                <span>{&props.module.title}</span>
                <span>{if *is_open { "▲" } else { "▼" }}</span>
            </div>
            <div class={content_style} data-open={is_open.to_string()}>
                {lessons_list}
            </div>
        </div>
    }
}