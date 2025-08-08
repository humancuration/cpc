use yew::prelude::*;
use stylist::{style, yew::styled_component};

#[styled_component(Footer)]
pub fn footer() -> Html {
    let footer_style = style!(
        r#"
        .footer {
            background-color: var(--primary);
            color: white;
            padding: 20px;
            text-align: center;
            margin-top: auto;
        }
        
        .footer-content {
            max-width: 1200px;
            margin: 0 auto;
        }
        
        .footer-links {
            margin-bottom: 10px;
        }
        
        .footer-link {
            color: rgba(255, 255, 255, 0.8);
            margin: 0 10px;
            text-decoration: none;
        }
        
        .footer-link:hover {
            color: white;
            text-decoration: underline;
        }
        
        .footer-copyright {
            font-size: 0.9rem;
            color: rgba(255, 255, 255, 0.6);
        }
    "#
    ).unwrap();

    html! {
        <footer class={footer_style}>
            <div class="footer-content">
                <div class="footer-links">
                    <a href="#" class="footer-link">{"Documentation"}</a>
                    <a href="#" class="footer-link">{"Support"}</a>
                    <a href="#" class="footer-link">{"Privacy Policy"}</a>
                    <a href="#" class="footer-link">{"Terms of Service"}</a>
                </div>
                <div class="footer-copyright">
                    {"© 2025 CPC Cooperative. All rights reserved."}
                </div>
            </div>
        </footer>
    }
}