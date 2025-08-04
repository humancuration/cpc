//! Message input component for the CPC Messenger application

use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::models::MessageInputState;

/// Properties for the MessageInput component
#[derive(Properties, PartialEq)]
pub struct MessageInputProps {
    /// The current input state
    pub state: MessageInputState,
    
    /// Callback for when the input content changes
    pub on_input_change: Callback<String>,
    
    /// Callback for when the message is submitted
    pub on_submit: Callback<String>,
}

/// Message input component
#[function_component(MessageInput)]
pub fn message_input(props: &MessageInputProps) -> Html {
    let on_input = {
        let on_input_change = props.on_input_change.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            on_input_change.emit(input.value());
        })
    };
    
    let on_key_down = {
        let on_submit = props.on_submit.clone();
        let content = props.state.content.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" && !e.shift_key() {
                e.prevent_default();
                if !content.trim().is_empty() {
                    on_submit.emit(content.trim().to_string());
                }
            }
        })
    };
    
    let on_submit = {
        let on_submit = props.on_submit.clone();
        let content = props.state.content.clone();
        Callback::from(move |_| {
            if !content.trim().is_empty() {
                on_submit.emit(content.trim().to_string());
            }
        })
    };
    
    html! {
        <div class="message-input-container">
            if let Some(error) = &props.state.error {
                <div class="error-message">{error}</div>
            }
            <div class="message-input-wrapper">
                <textarea
                    class="message-input"
                    placeholder="Type a message..."
                    value={props.state.content.clone()}
                    oninput={on_input}
                    onkeydown={on_key_down}
                    disabled={props.state.is_sending}
                />
                <button 
                    class="send-button"
                    onclick={on_submit}
                    disabled={props.state.is_sending || props.state.content.trim().is_empty()}
                >
                    if props.state.is_sending {
                        {"Sending..."}
                    } else {
                        {"Send"}
                    }
                </button>
            </div>
        </div>
    }
}