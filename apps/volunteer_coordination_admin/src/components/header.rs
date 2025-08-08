use yew::prelude::*;
use stylist::{style, yew::styled_component};
use yew_router::prelude::*;

use crate::components::navigation::Navigation;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    #[prop_or_default]
    pub title: String,
}

#[styled_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let header_style = style!(
        r#"
        .header {
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            background-color: var(--primary);
            color: white;
            padding: 0 20px;
            height: 60px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            z-index: 1000;
        }
        
        .header-title {
            font-size: 1.5rem;
            font-weight: 600;
        }
        
        .header-nav {
            display: flex;
            align-items: center;
        }
        
        .user-menu {
            margin-left: 20px;
            cursor: pointer;
        }
    "#
    ).unwrap();

    html! {
        <header class={header_style}>
            <div class="header-title">{ &props.title }</div>
            <div class="header-nav">
                <Navigation />
                <div class="user-menu">
                    <span>{"Admin"}</span>
                </div>
            </div>
        </header>
    }
}

impl Default for Header {
    fn default() -> Self {
        Self {
            props: HeaderProps {
                title: "Volunteer Coordination Admin".to_string(),
            },
            link: yew::html::Scope::new(),
        }
    }
}