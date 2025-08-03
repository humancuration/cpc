//! Chat component using Yew and Stylist

use yew::prelude::*;
use stylist::{style, yew::styled_component};
use std::collections::HashMap;

/// Properties for the Chat component
#[derive(Properties, PartialEq)]
pub struct ChatProps {
    /// List of chat messages
    pub messages: Vec<ChatMessage>,
    
    /// Callback for sending a new message
    #[prop_or_default]
    pub on_send: Callback<String>,
}

/// Represents a chat message
#[derive(Debug, Clone, PartialEq)]
pub struct ChatMessage {
    /// Username of the sender
    pub username: String,
    
    /// Message content
    pub content: String,
    
    /// Timestamp of the message
    pub timestamp: String,
}

/// Chat component
#[styled_component(Chat)]
pub fn chat(props: &ChatProps) -> Html {
    let style = style!(
        r#"
        .chat-container {
            display: flex;
            flex-direction: column;
            background-color: #1f1f1f;
            border-radius: 8px;
            overflow: hidden;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
            height: 100%;
        }
        
        .chat-messages {
            flex: 1;
            padding: 1rem;
            overflow-y: auto;
            max-height: 500px;
        }
        
        .chat-message {
            margin-bottom: 0.75rem;
            padding-bottom: 0.75rem;
            border-bottom: 1px solid #333333;
        }
        
        .chat-message:last-child {
            border-bottom: none;
            margin-bottom: 0;
            padding-bottom: 0;
        }
        
        .message-header {
            display: flex;
            justify-content: space-between;
            margin-bottom: 0.25rem;
        }
        
        .chat-username {
            font-weight: bold;
            color: #9146ff;
        }
        
        .chat-timestamp {
            color: #888888;
            font-size: 0.8rem;
        }
        
        .chat-text {
            color: #ffffff;
        }
        
        .chat-form {
            display: flex;
            padding: 1rem;
            background-color: #2d2d2d;
        }
        
        .chat-input {
            flex: 1;
            padding: 0.5rem;
            border: none;
            border-radius: 4px 0 0 4px;
            background-color: #333333;
            color: #ffffff;
        }
        
        .send-button {
            padding: 0.5rem 1rem;
            border: none;
            border-radius: 0 4px 4px 0;
            background-color: #9146ff;
            color: #ffffff;
            cursor: pointer;
            transition: background-color 0.3s;
        }
        
        .send-button:hover {
            background-color: #772ce8;
        }
    "#
    ).expect("Failed to create style");
    
    let message_input_ref = use_state(|| String::new());
    let on_input_change = {
        let message_input_ref = message_input_ref.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            message_input_ref.set(input.value());
        })
    };
    
    let on_send = {
        let on_send = props.on_send.clone();
        let message_input_ref = message_input_ref.clone();
        Callback::from(move |_| {
            let message = (*message_input_ref).clone();
            if !message.is_empty() {
                on_send.emit(message);
                message_input_ref.set(String::new());
            }
        })
    };

    html! {
        <div class={style}>
            <div class="chat-container">
                <div class="chat-messages">
                    {for props.messages.iter().map(|msg| {
                        html! {
                            <div class="chat-message">
                                <div class="message-header">
                                    <span class="chat-username">{&msg.username}</span>
                                    <span class="chat-timestamp">{&msg.timestamp}</span>
                                </div>
                                <div class="chat-text">{&msg.content}</div>
                            </div>
                        }
                    })}
                </div>
                <form class="chat-form" onsubmit={on_send}>
                    <input 
                        type="text" 
                        class="chat-input" 
                        placeholder="Type your message..." 
                        value={(*message_input_ref).clone()}
                        oninput={on_input_change}
                    />
                    <button type="submit" class="send-button">{"Send"}</button>
                </form>
            </div>
        </div>
    }
}