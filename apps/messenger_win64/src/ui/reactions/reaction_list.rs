//! Component for displaying reactions on a message

use yew::prelude::*;
use stylist::yew::styled_component;
use stylist::Style;
use uuid::Uuid;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Properties for the ReactionList component
#[derive(Properties, PartialEq)]
pub struct ReactionListProps {
    /// The ID of the message to display reactions for
    pub message_id: Uuid,
}

/// A reaction displayed in the list
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reaction {
    pub id: Uuid,
    pub message_id: Uuid,
    pub user_id: Uuid,
    pub reaction_type: String,
    pub created_at: String,
}

/// A component that displays reactions for a message
#[styled_component(ReactionList)]
pub fn reaction_list(props: &ReactionListProps) -> Html {
    let css = Style::new(r#"
        .reaction-list {
            display: flex;
            flex-wrap: wrap;
            gap: 4px;
            margin-top: 8px;
        }
        
        .reaction-item {
            display: flex;
            align-items: center;
            background: #f0f0f0;
            border-radius: 12px;
            padding: 2px 8px;
            font-size: 14px;
            cursor: pointer;
        }
        
        .reaction-item:hover {
            background: #e0e0e0;
        }
        
        .reaction-count {
            margin-left: 4px;
            font-size: 12px;
            color: #666;
        }
        
        .user-avatars {
            display: flex;
            margin-left: 4px;
        }
        
        .user-avatar {
            width: 16px;
            height: 16px;
            border-radius: 50%;
            background: #007bff;
            margin-left: -4px;
            border: 2px solid white;
            font-size: 8px;
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
        }
    "#).expect("style");

    let reactions = use_state(|| Vec::<Reaction>::new());
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);

    // Fetch reactions when the component mounts or message_id changes
    {
        let reactions = reactions.clone();
        let loading = loading.clone();
        let error = error.clone();
        let message_id = props.message_id;
        
        use_effect_with(message_id, move |_| {
            let reactions = reactions.clone();
            let loading = loading.clone();
            let error = error.clone();
            
            spawn_local(async move {
                loading.set(true);
                error.set(None);
                
                // Call the GraphQL query to get reactions
                let query = format!(r#"
                    query {{
                        messageReactions(messageId: "{}") {{
                            id
                            messageId
                            userId
                            reactionType
                            createdAt
                        }}
                    }}
                "#, message_id);
                
                let response = Request::post("/graphql")
                    .header("Content-Type", "application/json")
                    .body(query)
                    .send()
                    .await;
                
                match response {
                    Ok(resp) => {
                        match resp.text().await {
                            Ok(text) => {
                                // In a real implementation, we would parse the GraphQL response properly
                                // For now, we'll just set empty reactions
                                reactions.set(Vec::new());
                                loading.set(false);
                            }
                            Err(e) => {
                                error.set(Some(format!("Failed to read response: {:?}", e)));
                                loading.set(false);
                            }
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to fetch reactions: {:?}", e)));
                        loading.set(false);
                    }
                }
            });
            
            || ()
        });
    }

    // Group reactions by type
    let grouped_reactions: HashMap<String, Vec<Reaction>> = {
        let mut map = HashMap::new();
        for reaction in reactions.iter() {
            map.entry(reaction.reaction_type.clone())
                .or_insert_with(Vec::new)
                .push(reaction.clone());
        }
        map
    };

    if *loading {
        return html! { <div>{"Loading reactions..."}</div> };
    }

    if let Some(err) = &*error {
        return html! { <div class="error">{"Error: "}{err}</div> };
    }

    html! {
        <div class={css}>
            <div class="reaction-list">
                {grouped_reactions.iter().map(|(reaction_type, reactions)| {
                    let count = reactions.len();
                    let first_users: Vec<Uuid> = reactions.iter().take(3).map(|r| r.user_id).collect();
                    
                    html! {
                        <div class="reaction-item" title={format!("{} users reacted with {}", count, reaction_type)}>
                            <span>{reaction_type}</span>
                            <span class="reaction-count">{count}</span>
                            <div class="user-avatars">
                                {first_users.iter().enumerate().map(|(i, user_id)| {
                                    html! {
                                        <div 
                                            class="user-avatar" 
                                            style={format!("z-index: {}", 10 - i)}
                                            title={format!("User: {}", user_id)}
                                        >
                                            {user_id.to_string()[0..2].to_uppercase()}
                                        </div>
                                    }
                                }).collect::<Html>()}
                            </div>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
}