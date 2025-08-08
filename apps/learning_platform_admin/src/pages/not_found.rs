use yew::prelude::*;
use yew_router::prelude::*;
use stylist::{style, yew::styled_component};
use crate::Route;

#[styled_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    let page_style = style!(
        r#"
        .not-found-page {
            max-width: 600px;
            margin: 0 auto;
            text-align: center;
            padding: 50px 20px;
        }
        
        .error-icon {
            font-size: 4rem;
            margin-bottom: 20px;
            color: #e74c3c;
        }
        
        .page-title {
            font-size: 2.5rem;
            color: #2c3e50;
            margin: 0 0 20px 0;
        }
        
        .page-description {
            color: #7f8c8d;
            font-size: 1.2rem;
            margin: 0 0 30px 0;
        }
        
        .btn {
            padding: 12px 24px;
            background-color: #3498db;
            color: white;
            text-decoration: none;
            border-radius: 4px;
            font-size: 1rem;
            transition: background-color 0.2s;
        }
        
        .btn:hover {
            background-color: #2980b9;
        }
    "#
    ).unwrap();

    html! {
        <div class={page_style}>
            <div class="error-icon">{"⚠️"}</div>
            <h1 class="page-title">{"404 - Page Not Found"}</h1>
            <p class="page-description">{"The page you are looking for does not exist or has been moved."}</p>
            <Link<Route> classes="btn" to={Route::Dashboard}>
                {"Go to Dashboard"}
            </Link<Route>>
        </div>
    }
}