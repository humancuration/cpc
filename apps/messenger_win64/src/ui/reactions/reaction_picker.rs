//! Component for picking reactions to add to a message

use yew::prelude::*;
use stylist::yew::styled_component;
use stylist::Style;
use uuid::Uuid;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};

/// Properties for the ReactionPicker component
#[derive(Properties, PartialEq)]
pub struct ReactionPickerProps {
    /// The ID of the message to add a reaction to
    pub message_id: Uuid,
    
    /// Callback when a reaction is selected
    pub on_select: Callback<String>,
}

/// A component that shows a grid of emoji reactions to pick from
#[styled_component(ReactionPicker)]
pub fn reaction_picker(props: &ReactionPickerProps) -> Html {
    let css = Style::new(r#"
        .reaction-picker {
            display: flex;
            flex-wrap: wrap;
            gap: 8px;
            padding: 12px;
            background: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            max-width: 300px;
        }
        
        .reaction-button {
            background: none;
            border: none;
            font-size: 24px;
            cursor: pointer;
            padding: 4px;
            border-radius: 4px;
            transition: all 0.2s ease;
        }
        
        .reaction-button:hover {
            background: #f0f0f0;
            transform: scale(1.1);
        }
        
        .reaction-button:focus {
            outline: 2px solid #007bff;
        }
    "#).expect("style");

    // Common reactions that users can quickly select
    let reactions = vec![
        "👍", "❤️", "😂", "😮", "😢", "👏", 
        "👍🏻", "👍🏼", "👍🏽", "👍🏾", "👍🏿"
    ];

    let on_reaction_click = {
        let message_id = props.message_id;
        let on_select = props.on_select.clone();
        
        Callback::from(move |reaction: String| {
            let message_id = message_id;
            let on_select = on_select.clone();
            let reaction_type = reaction.clone();
            
            spawn_local(async move {
                // Call the GraphQL mutation to add the reaction
                let query = format!(r#"
                    mutation {{
                        addReaction(messageId: "{}", reactionType: "{}") {{
                            id
                            reactionType
                            userId
                            createdAt
                        }}
                    }}
                "#, message_id, reaction_type);
                
                let response = Request::post("/graphql")
                    .header("Content-Type", "application/json")
                    .body(query)
                    .send()
                    .await;
                
                match response {
                    Ok(_) => {
                        // Notify parent component that reaction was added
                        on_select.emit(reaction_type);
                    }
                    Err(e) => {
                        // In a real implementation, we would handle the error properly
                        gloo_console::error!(format!("Failed to add reaction: {:?}", e));
                    }
                }
            });
        })
    };

    html! {
        <div class={css}>
            <div class="reaction-picker">
                {reactions.iter().map(|reaction| {
                    let reaction = reaction.to_string();
                    let on_click = on_reaction_click.clone();
                    html! {
                        <button 
                            class="reaction-button"
                            onclick={Callback::from(move |_| on_click.emit(reaction.clone()))}
                            aria-label={format!("React with {}", reaction)}
                        >
                            {reaction}
                        </button>
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
}