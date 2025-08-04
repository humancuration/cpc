//! Message item component for the CPC Messenger application

use yew::prelude::*;
use uuid::Uuid;
use crate::models::UIMessage;
use crate::reaction_bar::ReactionBar;

/// Properties for the MessageItem component
#[derive(Properties, PartialEq)]
pub struct MessageItemProps {
    /// The message to display
    pub message: UIMessage,
    
    /// Callback for when a message is edited
    pub on_edit_message: Callback<(Uuid, String)>,
    
    /// Callback for when a message is deleted
    pub on_delete_message: Callback<Uuid>,
    
    /// Callback for when a reaction is added
    pub on_add_reaction: Callback<(Uuid, String)>,
    
    /// Callback for when a reaction is removed
    pub on_remove_reaction: Callback<(Uuid, String)>,
}

/// State for the MessageItem component
#[derive(Debug, Clone)]
pub struct MessageItemState {
    is_editing: bool,
    edit_content: String,
}

/// Message item component
#[function_component(MessageItem)]
pub fn message_item(props: &MessageItemProps) -> Html {
    let state = use_state(|| MessageItemState {
        is_editing: false,
        edit_content: props.message.content.clone(),
    });
    
    let on_edit_click = {
        let state = state.clone();
        let message = props.message.clone();
        Callback::from(move |_| {
            state.set(MessageItemState {
                is_editing: true,
                edit_content: message.content.clone(),
            });
        })
    };
    
    let on_cancel_edit = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(MessageItemState {
                is_editing: false,
                edit_content: String::new(),
            });
        })
    };
    
    let on_save_edit = {
        let state = state.clone();
        let on_edit_message = props.on_edit_message.clone();
        let message_id = props.message.id;
        Callback::from(move |_| {
            on_edit_message.emit((message_id, state.edit_content.clone()));
        })
    };
    
    let on_content_change = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            state.set(MessageItemState {
                is_editing: state.is_editing,
                edit_content: input.value(),
            });
        })
    };
    
    let on_delete_click = {
        let on_delete_message = props.on_delete_message.clone();
        let message_id = props.message.id;
        Callback::from(move |_| {
            on_delete_message.emit(message_id);
        })
    };
    
    let message = &props.message;
    
    if state.is_editing {
        html! {
            <div class="message-item editing">
                <input 
                    type="text" 
                    value={state.edit_content.clone()}
                    oninput={on_content_change}
                />
                <button onclick={on_save_edit}>{"Save"}</button>
                <button onclick={on_cancel_edit}>{"Cancel"}</button>
            </div>
        }
    } else {
        html! {
            <div class="message-item">
                <div class="message-header">
                    <span class="sender">{format!("User: {}", message.sender_id)}</span>
                    <span class="timestamp">{message.sent_at.to_rfc3339()}</span>
                </div>
                <div class="message-content">
                    if message.is_deleted {
                        <em>{"[Deleted]"}</em>
                    } else {
                        {&message.content}
                    }
                </div>
                <ReactionBar 
                    reactions={message.reactions.clone()}
                    message_id={message.id}
                    on_add_reaction={props.on_add_reaction.clone()}
                    on_remove_reaction={props.on_remove_reaction.clone()}
                />
                <div class="message-actions">
                    <button onclick={on_edit_click}>{"Edit"}</button>
                    <button onclick={on_delete_click}>{"Delete"}</button>
                </div>
            </div>
        }
    }
}