use yew::prelude::*;
use stylist::{style, yew::styled_component};

#[styled_component(Footer)]
pub fn footer() -> Html {
    let footer_style = style!(
        r#"
        .app-footer {
            background-color: #2c3e50;
            color: white;
            padding: 20px;
            text-align: center;
        }
        
        .footer-content {
            display: flex;
            justify-content: space-between;
            align-items: center;
            max-width: 1200px;
            margin: 0 auto;
        }
        
        .footer-links {
            display: flex;
            gap: 20px;
        }
        
        .footer-links a {
            color: #ecf0f1;
            text-decoration: none;
        }
        
        .footer-links a:hover {
            text-decoration: underline;
        }
        
        @media (max-width: 768px) {
            .footer-content {
                flex-direction: column;
                gap: 10px;
            }
        }
    "#
    ).unwrap();

    html! {
        <footer class={footer_style}>
            <div class="footer-content">
                <p>{"© 2023 CPC Cooperative. All rights reserved."}</p>
                <div class="footer-links">
                    <a href="/privacy">{"Privacy Policy"}</a>
                    <a href="/terms">{"Terms of Service"}</a>
                    <a href="/help">{"Help"}</a>
                </div>
            </div>
        </footer>
    }
}