//! Thread page for the Messenger web application

use yew::prelude::*;
use stylist::yew::styled_component;
use stylist::Style;
use crate::components::ThreadView;

#[derive(Properties, PartialEq)]
pub struct ThreadProps {
    pub id: String,
}

#[styled_component(Thread)]
pub fn thread(props: &ThreadProps) -> Html {
    let css = Style::new(r#"
        .thread-page {
            display: flex;
            flex-direction: column;
            height: 100vh;
            max-width: 800px;
            margin: 0 auto;
        }
        
        .header {
            background: white;
            border-bottom: 1px solid #eee;
            padding: 16px 20px;
            display: flex;
            align-items: center;
        }
        
        .back-button {
            background: none;
            border: none;
            color: #007bff;
            cursor: pointer;
            font-size: 18px;
            margin-right: 16px;
        }
        
        .header-text h2 {
            margin: 0;
            font-size: 18px;
            color: #333;
        }
        
        .header-text p {
            margin: 4px 0 0 0;
            font-size: 12px;
            color: #666;
        }
    "#).expect("style");

    html! {
        <div class={css}>
            <div class="thread-page">
                <div class="header">
                    <button class="back-button">{"←"}</button>
                    <div class="header-text">
                        <h2>{"Thread"}</h2>
                        <p>{"Reply to message"}</p>
                    </div>
                </div>
                <ThreadView thread_id={props.id.clone()} />
            </div>
        </div>
    }
}