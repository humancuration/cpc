//! Reaction bar component for the CPC Messenger application

use yew::prelude::*;
use uuid::Uuid;
use crate::models::UIReaction;

/// Properties for the ReactionBar component
#[derive(Properties, PartialEq)]
pub struct ReactionBarProps {
    /// The reactions to display
    pub reactions: Vec<UIReaction>,
    
    /// The ID of the message these reactions belong to
    pub message_id: Uuid,
    
    /// Callback for when a reaction is added
    pub on_add_reaction: Callback<(Uuid, String)>,
    
    /// Callback for when a reaction is removed
    pub on_remove_reaction: Callback<(Uuid, String)>,
}

/// Reaction bar component
#[function_component(ReactionBar)]
pub fn reaction_bar(props: &ReactionBarProps) -> Html {
    let grouped_reactions = group_reactions_by_type(&props.reactions);
    
    html! {
        <div class="reaction-bar">
            <div class="reactions">
                {for grouped_reactions.iter().map(|(reaction_type, reactions)| {
                    let count = reactions.len();
                    let user_reacted = reactions.iter().any(|r| r.user_id == get_current_user_id());
                    
                    let onclick = if user_reacted {
                        let on_remove_reaction = props.on_remove_reaction.clone();
                        let message_id = props.message_id;
                        let reaction_type = reaction_type.clone();
                        Callback::from(move |_| {
                            on_remove_reaction.emit((message_id, reaction_type.clone()));
                        })
                    } else {
                        let on_add_reaction = props.on_add_reaction.clone();
                        let message_id = props.message_id;
                        let reaction_type = reaction_type.clone();
                        Callback::from(move |_| {
                            on_add_reaction.emit((message_id, reaction_type.clone()));
                        })
                    };
                    
                    html! {
                        <button 
                            class={classes!("reaction", if user_reacted { "user-reacted" } else { "" })}
                            onclick={onclick}
                        >
                            <span class="reaction-emoji">{reaction_type}</span>
                            <span class="reaction-count">{count}</span>
                        </button>
                    }
                })}
            </div>
            <button class="add-reaction-button">{"+"}</button>
        </div>
    }
}

/// Group reactions by type
fn group_reactions_by_type(reactions: &[UIReaction]) -> Vec<(String, Vec<&UIReaction>)> {
    let mut grouped: std::collections::HashMap<String, Vec<&UIReaction>> = std::collections::HashMap::new();
    
    for reaction in reactions {
        grouped.entry(reaction.reaction_type.clone())
            .or_insert_with(Vec::new)
            .push(reaction);
    }
    
    grouped.into_iter().collect()
}

/// Get the current user ID (placeholder implementation)
fn get_current_user_id() -> Uuid {
    // In a real implementation, this would come from the authentication context
    Uuid::nil()
}