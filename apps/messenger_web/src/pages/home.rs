//! Home page for the Messenger web application

use yew::prelude::*;
use stylist::yew::styled_component;
use stylist::Style;

#[styled_component(Home)]
pub fn home() -> Html {
    let css = Style::new(r#"
        .home {
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            text-align: center;
        }
        
        .hero {
            background: linear-gradient(135deg, #007bff, #0056b3);
            color: white;
            padding: 60px 20px;
            border-radius: 10px;
            margin-bottom: 40px;
        }
        
        .hero h1 {
            font-size: 2.5rem;
            margin-bottom: 20px;
        }
        
        .hero p {
            font-size: 1.2rem;
            margin-bottom: 30px;
        }
        
        .cta-button {
            background: white;
            color: #007bff;
            border: none;
            padding: 12px 24px;
            font-size: 1.1rem;
            border-radius: 30px;
            cursor: pointer;
            font-weight: bold;
            transition: all 0.3s ease;
        }
        
        .cta-button:hover {
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(0,0,0,0.1);
        }
        
        .features {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 30px;
            margin-top: 40px;
        }
        
        .feature {
            background: #f8f9fa;
            padding: 30px;
            border-radius: 8px;
            text-align: center;
        }
        
        .feature-icon {
            font-size: 3rem;
            margin-bottom: 20px;
        }
        
        .feature h3 {
            margin-bottom: 15px;
            color: #333;
        }
        
        .feature p {
            color: #666;
            line-height: 1.6;
        }
    "#).expect("style");

    html! {
        <div class={css}>
            <div class="home">
                <div class="hero">
                    <h1>{"Welcome to Messenger"}</h1>
                    <p>{"Connect with people around the world through secure, private messaging"}</p>
                    <button class="cta-button">{"Get Started"}</button>
                </div>
                
                <div class="features">
                    <div class="feature">
                        <div class="feature-icon">{"💬"}</div>
                        <h3>{"Real-time Messaging"}</h3>
                        <p>{"Send messages instantly with our fast and reliable infrastructure"}</p>
                    </div>
                    
                    <div class="feature">
                        <div class="feature-icon">{"🔒"}</div>
                        <h3>{"End-to-End Encryption"}</h3>
                        <p>{"Your conversations are secured with military-grade encryption"}</p>
                    </div>
                    
                    <div class="feature">
                        <div class="feature-icon">{"👥"}</div>
                        <h3>{"Group Chats"}</h3>
                        <p>{"Create groups with up to 1000 participants for team collaboration"}</p>
                    </div>
                    
                    <div class="feature">
                        <div class="feature-icon">{"📎"}</div>
                        <h3>{"Media Sharing"}</h3>
                        <p>{"Share photos, videos, documents and more with ease"}</p>
                    </div>
                </div>
            </div>
        </div>
    }
}