//! Not found page for the Messenger web application

use yew::prelude::*;
use stylist::yew::styled_component;
use stylist::Style;

#[styled_component(NotFound)]
pub fn not_found() -> Html {
    let css = Style::new(r#"
        .not-found {
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            height: 100vh;
            text-align: center;
            padding: 20px;
        }
        
        .error-icon {
            font-size: 4rem;
            margin-bottom: 20px;
        }
        
        h1 {
            font-size: 2.5rem;
            margin-bottom: 20px;
            color: #333;
        }
        
        p {
            font-size: 1.2rem;
            color: #666;
            margin-bottom: 30px;
            max-width: 600px;
            line-height: 1.6;
        }
        
        .home-link {
            background: #007bff;
            color: white;
            border: none;
            padding: 12px 24px;
            font-size: 1rem;
            border-radius: 4px;
            cursor: pointer;
            text-decoration: none;
        }
        
        .home-link:hover {
            background: #0056b3;
        }
    "#).expect("style");

    html! {
        <div class={css}>
            <div class="not-found">
                <div class="error-icon">{"🔍"}</div>
                <h1>{"Page Not Found"}</h1>
                <p>{"Sorry, the page you're looking for doesn't exist or has been moved."}</p>
                <a href="/" class="home-link">{"Go to Homepage"}</a>
            </div>
        </div>
    }
}