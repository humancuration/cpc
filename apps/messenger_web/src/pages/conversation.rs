//! Conversation page for the Messenger web application

use yew::prelude::*;
use stylist::yew::styled_component;
use stylist::Style;
use yew_router::prelude::*;
use crate::components::{ReactionPicker, ReactionList, ThreadCreateButton, MediaUpload, MediaPreview};
use crate::Route;
use uuid::Uuid;

#[derive(Properties, PartialEq)]
pub struct ConversationProps {
    pub id: String,
}

#[styled_component(Conversation)]
pub fn conversation(props: &ConversationProps) -> Html {
    let css = Style::new(r#"
        .conversation {
            display: flex;
            flex-direction: column;
            height: 100vh;
            max-width: 1200px;
            margin: 0 auto;
        }
        
        .header {
            background: white;
            border-bottom: 1px solid #eee;
            padding: 16px 20px;
            display: flex;
            align-items: center;
            justify-content: space-between;
        }
        
        .header-info {
            display: flex;
            align-items: center;
            gap: 12px;
        }
        
        .avatar {
            width: 40px;
            height: 40px;
            border-radius: 50%;
            background: #007bff;
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
            font-weight: bold;
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
        
        .header-actions {
            display: flex;
            gap: 12px;
        }
        
        .action-button {
            background: none;
            border: none;
            color: #666;
            cursor: pointer;
            font-size: 18px;
            width: 36px;
            height: 36px;
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
        }
        
        .action-button:hover {
            background: #f0f0f0;
        }
        
        .messages-container {
            flex: 1;
            overflow-y: auto;
            padding: 20px;
            background: #f5f5f5;
        }
        
        .message {
            background: white;
            border-radius: 8px;
            padding: 12px 16px;
            margin-bottom: 12px;
            box-shadow: 0 1px 3px rgba(0,0,0,0.05);
            max-width: 70%;
        }
        
        .message.own {
            background: #007bff;
            color: white;
            margin-left: auto;
        }
        
        .message-header {
            display: flex;
            justify-content: space-between;
            margin-bottom: 6px;
        }
        
        .sender-name {
            font-weight: bold;
            font-size: 14px;
        }
        
        .timestamp {
            font-size: 11px;
            opacity: 0.8;
        }
        
        .message-content {
            font-size: 14px;
            line-height: 1.4;
        }
        
        .message-footer {
            display: flex;
            justify-content: flex-end;
            margin-top: 8px;
        }
        
        .input-area {
            background: white;
            border-top: 1px solid #eee;
            padding: 16px 20px;
        }
        
        .input-container {
            display: flex;
            gap: 12px;
        }
        
        .message-input {
            flex: 1;
            padding: 12px 16px;
            border: 1px solid #ddd;
            border-radius: 20px;
            font-size: 14px;
            resize: none;
            min-height: 20px;
            max-height: 120px;
        }
        
        .message-input:focus {
            outline: none;
            border-color: #007bff;
        }
        
        .send-button {
            background: #007bff;
            color: white;
            border: none;
            border-radius: 50%;
            width: 40px;
            height: 40px;
            display: flex;
            align-items: center;
            justify-content: center;
            cursor: pointer;
        }
        
        .send-button:hover {
            background: #0056b3;
        }
        
        .send-button:disabled {
            background: #ccc;
            cursor: not-allowed;
        }
    "#).expect("style");

    let navigator = use_navigator().unwrap();
    let message_input_ref = use_node_ref();

    let on_send_message = {
        let message_input_ref = message_input_ref.clone();
        Callback::from(move |_| {
            if let Some(input) = message_input_ref.cast::<web_sys::HtmlTextAreaElement>() {
                let message = input.value();
                if !message.trim().is_empty() {
                    // In a real implementation, we would send the message
                    input.set_value("");
                }
            }
        })
    };

    let on_create_thread = {
        let navigator = navigator.clone();
        Callback::from(move |thread_id: String| {
            navigator.push(&Route::Thread { id: thread_id });
        })
    };

    html! {
        <div class={css}>
            <div class="conversation">
                <div class="header">
                    <div class="header-info">
                        <div class="avatar">{"JD"}</div>
                        <div class="header-text">
                            <h2>{"John Doe"}</h2>
                            <p>{"Online"}</p>
                        </div>
                    </div>
                    <div class="header-actions">
                        <button class="action-button">{"📞"}</button>
                        <button class="action-button">{"⚙️"}</button>
                    </div>
                </div>
                
                <div class="messages-container">
                    <div class="message">
                        <div class="message-header">
                            <div class="sender-name">{"John Doe"}</div>
                            <div class="timestamp">{"10:30 AM"}</div>
                        </div>
                        <div class="message-content">
                            {"Hey, how are you doing today?"}
                        </div>
                        <ReactionList message_id={Uuid::nil()} />
                        <div class="message-footer">
                            <ThreadCreateButton 
                                parent_message_id={Uuid::nil()} 
                                conversation_id={Uuid::nil()} 
                                on_create={on_create_thread.clone()}
                            />
                        </div>
                    </div>
                    
                    <div class="message own">
                        <div class="message-header">
                            <div class="sender-name">{"You"}</div>
                            <div class="timestamp">{"10:32 AM"}</div>
                        </div>
                        <div class="message-content">
                            {"I'm doing great! Just working on some new features for the messenger app."}
                        </div>
                        <ReactionList message_id={Uuid::nil()} />
                    </div>
                    
                    <div class="message">
                        <div class="message-header">
                            <div class="sender-name">{"John Doe"}</div>
                            <div class="timestamp">{"10:35 AM"}</div>
                        </div>
                        <div class="message-content">
                            {"That sounds exciting! What kind of features?"}
                        </div>
                        <ReactionList message_id={Uuid::nil()} />
                    </div>
                </div>
                
                <div class="input-area">
                    <MediaUpload 
                        conversation_id={Uuid::nil()} 
                        on_upload={Callback::from(|_| ())}
                    />
                    <div class="input-container">
                        <textarea
                            ref={message_input_ref}
                            class="message-input"
                            placeholder="Type a message..."
                            onkeypress={
                                let on_send_message = on_send_message.clone();
                                Callback::from(move |e: KeyboardEvent| {
                                    if e.key() == "Enter" && !e.shift_key() {
                                        e.prevent_default();
                                        on_send_message.emit(());
                                    }
                                })
                            }
                        />
                        <button 
                            class="send-button" 
                            onclick={on_send_message}
                        >
                            {"➤"}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}