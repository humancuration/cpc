use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::types::Course;
use yew_router::prelude::*;
use crate::routes::AppRoute;

#[derive(Properties, PartialEq)]
pub struct CourseCardProps {
    pub course: Course,
}

#[styled_component(CourseCard)]
pub fn course_card(props: &CourseCardProps) -> Html {
    let card_style = style!(
        r#"
        background: var(--surface);
        border-radius: 8px;
        overflow: hidden;
        box-shadow: 0 2px 8px rgba(0,0,0,0.1);
        transition: transform 0.2s, box-shadow 0.2s;
        height: 100%;
        display: flex;
        flex-direction: column;

        &:hover {
            transform: translateY(-4px);
            box-shadow: 0 4px 16px rgba(0,0,0,0.15);
        }
    "#
    ).unwrap();

    let content_style = style!(
        r#"
        padding: 1.5rem;
        flex-grow: 1;
        display: flex;
        flex-direction: column;
    "#
    ).unwrap();

    let title_style = style!(
        r#"
        margin-top: 0;
        margin-bottom: 0.5rem;
        font-size: 1.25rem;
    "#
    ).unwrap();

    let description_style = style!(
        r#"
        color: var(--text-secondary);
        margin-bottom: 1rem;
        flex-grow: 1;
    "#
    ).unwrap();

    let button_style = style!(
        r#"
        background: var(--primary);
        color: white;
        border: none;
        padding: 0.75rem;
        border-radius: 4px;
        cursor: pointer;
        font-weight: bold;
        text-align: center;
        text-decoration: none;
        display: block;

        &:hover {
            background: var(--secondary);
        }
    "#
    ).unwrap();
let lessons_count = props.course.modules.iter().map(|m| m.lessons.len()).sum::<usize>();

html! {
    <div class={card_style}>
        <div class={content_style}>
            <h3 class={title_style}>{&props.course.title}</h3>
            <p class={description_style}>{&props.course.description}</p>
            <p>{"Lessons: "}{lessons_count}</p>
            <div style="display: flex; justify-content: space-between; align-items: center; margin-top: 1rem;">
                <span style="background: var(--primary); color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                    {"High Community Impact"}
                </span>
                <span style="font-size: 0.9rem; color: var(--text-secondary);">
                    {"85% Match"}
                </span>
            </div>
        </div>
        <Link<AppRoute> to={AppRoute::CourseDetail(props.course.id.clone())} classes={button_style}>
            {"View Course"}
        </Link<AppRoute>>
    </div>
}
}
}