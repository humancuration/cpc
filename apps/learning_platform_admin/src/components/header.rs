use yew::prelude::*;
use yew_router::prelude::*;
use stylist::{style, yew::styled_component};
use crate::Route;

#[styled_component(Header)]
pub fn header() -> Html {
    let header_style = style!(
        r#"
        .app-header {
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            background-color: #2c3e50;
            color: white;
            padding: 0 20px;
            height: 60px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            z-index: 1000;
        }
        
        .logo {
            display: flex;
            align-items: center;
            gap: 10px;
        }
        
        .logo h1 {
            margin: 0;
            font-size: 1.5rem;
        }
        
        .main-nav ul {
            display: flex;
            list-style: none;
            gap: 20px;
        }
        
        .main-nav a {
            color: white;
            text-decoration: none;
            padding: 8px 12px;
            border-radius: 4px;
            transition: background-color 0.2s;
        }
        
        .main-nav a:hover {
            background-color: #34495e;
        }
        
        .main-nav a.active {
            background-color: #3498db;
        }
        
        .user-profile {
            display: flex;
            align-items: center;
            gap: 10px;
        }
        
        .user-avatar {
            width: 32px;
            height: 32px;
            border-radius: 50%;
            background-color: #3498db;
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: bold;
        }
    "#
    ).unwrap();

    html! {
        <header class={header_style}>
            <div class="logo">
                <h1>{"Learning Platform Admin"}</h1>
            </div>
            <nav class="main-nav">
                <ul>
                    <li><Link<Route> to={Route::Dashboard}>{"Dashboard"}</Link<Route>></li>
                    <li><Link<Route> to={Route::Analytics}>{"Analytics"}</Link<Route>></li>
                    <li><Link<Route> to={Route::Settings}>{"Settings"}</Link<Route>></li>
                </ul>
            </nav>
            <div class="user-profile">
                <span class="user-name">{"Admin User"}</span>
                <div class="user-avatar">{"AU"}</div>
            </div>
        </header>
    }
}