use yew::prelude::*;
use stylist::{style, yew::styled_component};
use yew_router::prelude::*;

use crate::main::Route;

#[styled_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    let page_style = style!(
        r#"
        .not-found-page {
            max-width: 800px;
            margin: 0 auto;
            text-align: center;
            padding: 60px 20px;
        }
        
        .error-code {
            font-size: 6rem;
            font-weight: 700;
            color: #e74c3c;
            margin: 0 0 20px 0;
        }
        
        .error-title {
            font-size: 2rem;
            color: #2c3e50;
            margin: 0 0 20px 0;
        }
        
        .error-message {
            font-size: 1.25rem;
            color: #7f8c8d;
            margin: 0 0 30px 0;
        }
        
        .btn {
            display: inline-block;
            padding: 12px 24px;
            background-color: #3498db;
            color: white;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            font-size: 1rem;
            text-decoration: none;
            transition: background-color 0.2s;
        }
        
        .btn:hover {
            background-color: #2980b9;
        }
    "#
    ).unwrap();

    html! {
        <div class={page_style}>
            <div class="error-code">{"404"}</div>
            <h1 class="error-title">{"Page Not Found"}</h1>
            <p class="error-message">{"The page you're looking for doesn't exist or has been moved."}</p>
            <Link<Route> to={Route::Dashboard} classes="btn">
                {"Return to Dashboard"}
            </Link<Route>>
        </div>
    }
}