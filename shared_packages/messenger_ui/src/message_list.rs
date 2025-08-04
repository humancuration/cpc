//! Message list component for the CPC Messenger application

use yew::prelude::*;
use crate::models::UIMessage;
use crate::message_item::MessageItem;

/// Properties for the MessageList component
#[derive(Properties, PartialEq)]
pub struct MessageListProps {
    /// The messages to display
    pub messages: Vec<UIMessage>,
    
    /// Callback for when a message is edited
    pub on_edit_message: Callback<(Uuid, String)>,
    
    /// Callback for when a message is deleted
    pub on_delete_message: Callback<Uuid>,
    
    /// Callback for when a reaction is added
    pub on_add_reaction: Callback<(Uuid, String)>,
    
    /// Callback for when a reaction is removed
    pub on_remove_reaction: Callback<(Uuid, String)>,
}

/// Message list component
#[function_component(MessageList)]
pub fn message_list(props: &MessageListProps) -> Html {
    let messages = props.messages.clone();
    
    html! {
        <div class="message-list">
            {for messages.iter().map(|message| {
                html! {
                    <MessageItem 
                        message={message.clone()}
                        on_edit_message={props.on_edit_message.clone()}
                        on_delete_message={props.on_delete_message.clone()}
                        on_add_reaction={props.on_add_reaction.clone()}
                        on_remove_reaction={props.on_remove_reaction.clone()}
                    />
                }
            })}
        </div>
    }
}