use yew::prelude::*;
use stylist::{style, yew::styled_component};
use yew_router::prelude::*;
use crate::routes::AppRoute;

#[styled_component(Navigation)]
pub fn navigation() -> Html {
    let nav_style = style!(
        r#"
        background: var(--surface);
        padding: 1rem;
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        margin-bottom: 2rem;
    "#
    ).unwrap();

    let nav_list_style = style!(
        r#"
        display: flex;
        list-style: none;
        margin: 0;
        padding: 0;
        gap: 1rem;
    "#
    ).unwrap();

    let nav_item_style = style!(
        r#"
        margin: 0;
    "#
    ).unwrap();

    let link_style = style!(
        r#"
        text-decoration: none;
        color: var(--text-primary);
        padding: 0.5rem 1rem;
        border-radius: 4px;
        transition: background-color 0.2s;

        &:hover {
            background-color: var(--background-secondary);
        }

        &.active {
            background-color: var(--primary);
            color: white;
        }
    "#
    ).unwrap();

    html! {
        <nav class={nav_style}>
            <ul class={nav_list_style}>
                <li class={nav_item_style.clone()}>
                    <Link<AppRoute> to={AppRoute::CourseCatalog} classes={link_style.clone()}>
                        {"Courses"}
                    </Link<AppRoute>>
                </li>
                <li class={nav_item_style.clone()}>
                    <Link<AppRoute> to={AppRoute::Enrollments} classes={link_style.clone()}>
                        {"Enrollments"}
                    </Link<AppRoute>>
                </li>
                <li class={nav_item_style.clone()}>
                    <Link<AppRoute> to={AppRoute::Credentials} classes={link_style.clone()}>
                        {"Credentials"}
                    </Link<AppRoute>>
                </li>
                <li class={nav_item_style.clone()}>
                    <Link<AppRoute> to={AppRoute::Skills} classes={link_style.clone()}>
                        {"Skills"}
                    </Link<AppRoute>>
                </li>
                <li class={nav_item_style}>
                    <Link<AppRoute> to={AppRoute::Tip} classes={link_style}>
                        {"Tip"}
                    </Link<AppRoute>>
                </li>
            </ul>
        </nav>
    }
}