//! Component for creating a new thread from a message

use yew::prelude::*;
use stylist::yew::styled_component;
use stylist::Style;
use uuid::Uuid;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

/// Properties for the ThreadCreateButton component
#[derive(Properties, PartialEq)]
pub struct ThreadCreateButtonProps {
    /// The ID of the parent message to create a thread from
    pub parent_message_id: Uuid,
    
    /// The ID of the conversation the message belongs to
    pub conversation_id: Uuid,
    
    /// Callback when a thread is successfully created
    pub on_create: Callback<String>,
}

/// A component that provides a button to create a new thread from a message
#[styled_component(ThreadCreateButton)]
pub fn thread_create_button(props: &ThreadCreateButtonProps) -> Html {
    let css = Style::new(r#"
        .thread-button {
            background: none;
            border: none;
            color: #666;
            cursor: pointer;
            font-size: 14px;
            padding: 4px 8px;
            border-radius: 4px;
            display: inline-flex;
            align-items: center;
            gap: 4px;
        }
        
        .thread-button:hover {
            background: #f0f0f0;
            color: #333;
        }
        
        .thread-button:focus {
            outline: 2px solid #007bff;
        }
        
        .thread-icon {
            font-size: 16px;
        }
        
        .loading {
            color: #666;
            font-style: italic;
        }
        
        .error {
            color: #d32f2f;
            font-size: 12px;
            margin-top: 4px;
        }
    "#).expect("style");

    let creating = use_state(|| false);
    let error = use_state(|| Option::<String>::None);

    let on_click = {
        let parent_message_id = props.parent_message_id;
        let conversation_id = props.conversation_id;
        let on_create = props.on_create.clone();
        let creating = creating.clone();
        let error = error.clone();
        
        Callback::from(move |_| {
            let parent_message_id = parent_message_id;
            let conversation_id = conversation_id;
            let on_create = on_create.clone();
            let creating = creating.clone();
            let error = error.clone();
            
            spawn_local(async move {
                if *creating {
                    return;
                }
                
                creating.set(true);
                error.set(None);
                
                // Call the GraphQL mutation to create a thread
                let query = format!(r#"
                    mutation {{
                        createThread(parentMessageId: "{}", conversationId: "{}") {{
                            id
                            parentMessageId
                            rootMessageId
                            conversationId
                            createdAt
                        }}
                    }}
                "#, parent_message_id, conversation_id);
                
                let response = Request::post("/graphql")
                    .header("Content-Type", "application/json")
                    .body(query)
                    .send()
                    .await;
                
                match response {
                    Ok(resp) => {
                        match resp.text().await {
                            Ok(_text) => {
                                // In a real implementation, we would parse the GraphQL response properly
                                // For now, we'll just emit a placeholder thread ID
                                on_create.emit(Uuid::new_v4().to_string());
                                creating.set(false);
                            }
                            Err(e) => {
                                error.set(Some(format!("Failed to read response: {:?}", e)));
                                creating.set(false);
                            }
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to create thread: {:?}", e)));
                        creating.set(false);
                    }
                }
            });
        })
    };

    html! {
        <div class={css}>
            <button 
                class="thread-button" 
                onclick={on_click}
                disabled={*creating}
                aria-label="Start a thread"
            >
                if *creating {
                    <span class="thread-icon">{"🔄"}</span>
                    <span>{"Creating..."}</span>
                } else {
                    <span class="thread-icon">{"💬"}</span>
                    <span>{"Thread"}</span>
                }
            </button>
            if let Some(err) = &*error {
                <div class="error">{err}</div>
            }
        </div>
    }
}