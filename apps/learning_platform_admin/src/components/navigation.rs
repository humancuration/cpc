use yew::prelude::*;
use yew_router::prelude::*;
use stylist::{style, yew::styled_component};
use crate::Route;

#[styled_component(Navigation)]
pub fn navigation() -> Html {
    let nav_style = style!(
        r#"
        .sidebar-nav {
            width: 250px;
            background-color: #34495e;
            color: white;
            height: calc(100vh - 60px); /* Account for header height */
            position: fixed;
            top: 60px;
            left: 0;
            overflow-y: auto;
            padding: 20px 0;
        }
        
        .nav-item {
            padding: 12px 20px;
            display: block;
            color: #ecf0f1;
            text-decoration: none;
            transition: background-color 0.2s;
        }
        
        .nav-item:hover {
            background-color: #3d566e;
        }
        
        .nav-item.active {
            background-color: #3498db;
        }
        
        .nav-section {
            margin: 20px 0 10px 0;
            padding: 0 20px;
            font-size: 0.9rem;
            color: #bdc3c7;
            text-transform: uppercase;
            letter-spacing: 1px;
        }
    "#
    ).unwrap();

    html! {
        <nav class={nav_style}>
            <div class="nav-section">{"Dashboard"}</div>
            <Link<Route> classes="nav-item" to={Route::Dashboard}>
                {"Overview"}
            </Link<Route>>
            
            <div class="nav-section">{"Analytics"}</div>
            <Link<Route> classes="nav-item" to={Route::Analytics}>
                {"Impact Metrics"}
            </Link<Route>>
            
            <div class="nav-section">{"Settings"}</div>
            <Link<Route> classes="nav-item" to={Route::Settings}>
                {"Preferences"}
            </Link<Route>>
        </nav>
    }
}