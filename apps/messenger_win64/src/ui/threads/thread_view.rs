//! Component for viewing a thread of messages

use yew::prelude::*;
use stylist::yew::styled_component;
use stylist::Style;
use uuid::Uuid;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Properties for the ThreadView component
#[derive(Properties, PartialEq)]
pub struct ThreadViewProps {
    /// The ID of the thread to display
    pub thread_id: String,
}

/// A message in a thread
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreadMessage {
    pub id: Uuid,
    pub content: String,
    pub sender_id: Uuid,
    pub sent_at: DateTime<Utc>,
    pub delivery_status: String,
}

/// A thread of messages
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thread {
    pub id: String,
    pub parent_message_id: Uuid,
    pub root_message_id: Option<Uuid>,
    pub conversation_id: Uuid,
    pub created_at: DateTime<Utc>,
}

/// A component that displays a threaded conversation
#[styled_component(ThreadView)]
pub fn thread_view(props: &ThreadViewProps) -> Html {
    let css = Style::new(r#"
        .thread-view {
            background: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            padding: 16px;
            max-width: 600px;
            margin: 16px auto;
        }
        
        .thread-header {
            border-bottom: 1px solid #eee;
            padding-bottom: 12px;
            margin-bottom: 12px;
        }
        
        .thread-title {
            font-size: 18px;
            font-weight: bold;
            margin: 0;
        }
        
        .thread-info {
            font-size: 12px;
            color: #666;
            margin-top: 4px;
        }
        
        .messages-container {
            max-height: 400px;
            overflow-y: auto;
        }
        
        .message {
            padding: 12px 0;
            border-bottom: 1px solid #f5f5f5;
        }
        
        .message:last-child {
            border-bottom: none;
        }
        
        .message-header {
            display: flex;
            justify-content: space-between;
            margin-bottom: 4px;
        }
        
        .sender-name {
            font-weight: bold;
            color: #333;
        }
        
        .timestamp {
            font-size: 12px;
            color: #999;
        }
        
        .message-content {
            font-size: 14px;
            color: #333;
            line-height: 1.4;
        }
        
        .loading, .error {
            text-align: center;
            padding: 20px;
        }
        
        .error {
            color: #d32f2f;
        }
    "#).expect("style");

    let thread = use_state(|| Option::<Thread>::None);
    let messages = use_state(|| Vec::<ThreadMessage>::new());
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);

    // Fetch thread data when the component mounts or thread_id changes
    {
        let thread = thread.clone();
        let messages = messages.clone();
        let loading = loading.clone();
        let error = error.clone();
        let thread_id = props.thread_id.clone();
        
        use_effect_with(thread_id.clone(), move |_| {
            let thread = thread.clone();
            let messages = messages.clone();
            let loading = loading.clone();
            let error = error.clone();
            let thread_id = thread_id.clone();
            
            spawn_local(async move {
                loading.set(true);
                error.set(None);
                
                // First, fetch the thread information
                let query = format!(r#"
                    query {{
                        thread(threadId: "{}") {{
                            id
                            parentMessageId
                            rootMessageId
                            conversationId
                            createdAt
                        }}
                    }}
                "#, thread_id);
                
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
                                // For now, we'll just set placeholder data
                                thread.set(Some(Thread {
                                    id: thread_id.clone(),
                                    parent_message_id: Uuid::nil(),
                                    root_message_id: None,
                                    conversation_id: Uuid::nil(),
                                    created_at: Utc::now(),
                                }));
                                
                                // Then fetch messages in the thread
                                // In a real implementation, we would call a query like getThreadMessages
                                messages.set(Vec::new());
                                loading.set(false);
                            }
                            Err(e) => {
                                error.set(Some(format!("Failed to read response: {:?}", e)));
                                loading.set(false);
                            }
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to fetch thread: {:?}", e)));
                        loading.set(false);
                    }
                }
            });
            
            || ()
        });
    }

    if *loading {
        return html! { 
            <div class={css}>
                <div class="loading">{"Loading thread..."}</div>
            </div>
        };
    }

    if let Some(err) = &*error {
        return html! { 
            <div class={css}>
                <div class="error">{"Error: "}{err}</div>
            </div>
        };
    }

    let thread_data = match &*thread {
        Some(t) => t,
        None => return html! { 
            <div class={css}>
                <div class="error">{"Thread not found"}</div>
            </div>
        },
    };

    html! {
        <div class={css}>
            <div class="thread-view">
                <div class="thread-header">
                    <h3 class="thread-title">{"Thread"}</h3>
                    <div class="thread-info">
                        {format!("Created: {}", thread_data.created_at.format("%Y-%m-%d %H:%M"))}
                    </div>
                </div>
                
                <div class="messages-container">
                    {messages.iter().map(|message| {
                        html! {
                            <div class="message">
                                <div class="message-header">
                                    <div class="sender-name">{format!("User {}", message.sender_id.to_string()[0..8].to_uppercase())}</div>
                                    <div class="timestamp">{message.sent_at.format("%H:%M")}</div>
                                </div>
                                <div class="message-content">{&message.content}</div>
                            </div>
                        }
                    }).collect::<Html>()}
                    
                    {if messages.is_empty() {
                        html! { <div class="message">{"No messages in this thread yet."}</div> }
                    } else {
                        html! {}
                    }}
                </div>
            </div>
        </div>
    }
}