use yew::prelude::*;
use stylist::{style, yew::styled_component};
use yew_router::prelude::*;

use crate::main::Route;

#[styled_component(Navigation)]
pub fn navigation() -> Html {
    let nav_style = style!(
        r#"
        .nav {
            display: flex;
            list-style: none;
        }
        
        .nav-item {
            margin-right: 20px;
        }
        
        .nav-link {
            color: rgba(255, 255, 255, 0.8);
            text-decoration: none;
            padding: 8px 12px;
            border-radius: 4px;
            transition: all 0.2s;
        }
        
        .nav-link:hover {
            color: white;
            background-color: rgba(255, 255, 255, 0.1);
        }
        
        .nav-link.active {
            color: white;
            background-color: rgba(255, 255, 255, 0.2);
        }
    "#
    ).unwrap();

    html! {
        <nav>
            <ul class={nav_style}>
                <li class="nav-item">
                    <Link<Route> to={Route::Dashboard} classes="nav-link">
                        {"Dashboard"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Analytics} classes="nav-link">
                        {"Analytics"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Settings} classes="nav-link">
                        {"Settings"}
                    </Link<Route>>
                </li>
            </ul>
        </nav>
    }
}