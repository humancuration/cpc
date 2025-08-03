//! Navigation component using Yew and Stylist

use yew::prelude::*;
use stylist::{style, yew::styled_component};

/// Properties for the Navigation component
#[derive(Properties, PartialEq)]
pub struct NavigationProps {
    /// Current active route
    pub active_route: String,
    
    /// Callback for when a navigation item is clicked
    #[prop_or_default]
    pub on_navigate: Callback<String>,
}

/// Navigation component
#[styled_component(Navigation)]
pub fn navigation(props: &NavigationProps) -> Html {
    let style = style!(
        r#"
        .navigation {
            background-color: #1f1f1f;
            padding: 1rem;
            box-shadow: 0 2px 5px rgba(0, 0, 0, 0.3);
        }
        
        .app-title {
            color: #9146ff;
            margin-bottom: 0.5rem;
            font-size: 1.8rem;
        }
        
        .nav-list {
            display: flex;
            list-style: none;
        }
        
        .nav-item {
            margin-right: 1rem;
        }
        
        .nav-link {
            color: #ffffff;
            text-decoration: none;
            padding: 0.5rem 1rem;
            border-radius: 4px;
            transition: background-color 0.3s;
        }
        
        .nav-link:hover {
            background-color: #333333;
        }
        
        .nav-link.active {
            background-color: #9146ff;
        }
    "#
    ).expect("Failed to create style");
    
    let nav_items = vec![
        ("browse", "Browse"),
        ("following", "Following"),
        ("subscriptions", "Subscriptions"),
        ("profile", "Profile"),
    ];

    html! {
        <nav class={style}>
            <div class="navigation">
                <h1 class="app-title">{"Live Streaming Platform"}</h1>
                <ul class="nav-list">
                    {for nav_items.iter().map(|(route, label)| {
                        let is_active = props.active_route == *route;
                        let class = if is_active { "nav-link active" } else { "nav-link" };
                        let on_navigate = props.on_navigate.clone();
                        let route = route.to_string();
                        let onclick = Callback::from(move |_| on_navigate.emit(route.clone()));
                        
                        html! {
                            <li class="nav-item">
                                <a class={class} {onclick}>{label}</a>
                            </li>
                        }
                    })}
                </ul>
            </div>
        </nav>
    }
}