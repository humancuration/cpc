//! Main application component using Yew and Stylist

use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::ui::{
    navigation::Navigation,
    stream_player::{StreamPlayer, StreamPlayerProps},
    chat::{Chat, ChatProps, ChatMessage},
    channel_list::{ChannelList, ChannelListProps, ChannelInfo},
};

/// Main application component
#[styled_component(App)]
pub fn app() -> Html {
    let style = style!(
        r#"
        .app {
            display: flex;
            flex-direction: column;
            min-height: 100vh;
            background-color: #0f0f0f;
            color: #ffffff;
        }
        
        .main-content {
            display: flex;
            flex: 1;
            padding: 1rem;
            gap: 1rem;
        }
        
        .stream-section {
            flex: 3;
        }
        
        .sidebar {
            flex: 1;
            display: flex;
            flex-direction: column;
            gap: 1rem;
        }
        
        .chat-section {
            flex: 1;
        }
        
        .channel-list-section {
            flex: 1;
        }
        
        .footer {
            background-color: #1f1f1f;
            padding: 1rem;
            text-align: center;
            color: #aaaaaa;
        }
        
        /* Responsive design */
        @media (max-width: 768px) {
            .main-content {
                flex-direction: column;
            }
            
            .stream-section, .sidebar {
                flex: none;
            }
        }
    "#
    ).expect("Failed to create style");
    
    // State for the application
    let active_route = use_state(|| "browse".to_string());
    let chat_messages = use_state(|| vec![
        ChatMessage {
            username: "Streamer".to_string(),
            content: "Welcome to my stream!".to_string(),
            timestamp: "10:00".to_string(),
        },
        ChatMessage {
            username: "Viewer123".to_string(),
            content: "Great stream!".to_string(),
            timestamp: "10:01".to_string(),
        },
    ]);
    let channels = use_state(|| vec![
        ChannelInfo {
            id: "1".to_string(),
            name: "GamerPro".to_string(),
            description: "Playing the latest RPG games".to_string(),
            viewer_count: 1250,
            profile_image_url: "https://example.com/avatar1.jpg".to_string(),
        },
        ChannelInfo {
            id: "2".to_string(),
            name: "MusicMaster".to_string(),
            description: "Live music performances".to_string(),
            viewer_count: 842,
            profile_image_url: "https://example.com/avatar2.jpg".to_string(),
        },
    ]);
    
    // Callbacks
    let on_navigate = {
        let active_route = active_route.clone();
        Callback::from(move |route: String| active_route.set(route))
    };
    
    let on_send_message = {
        let chat_messages = chat_messages.clone();
        Callback::from(move |message: String| {
            let new_message = ChatMessage {
                username: "You".to_string(),
                content: message,
                timestamp: "Just now".to_string(),
            };
            chat_messages.set({
                let mut msgs = (*chat_messages).clone();
                msgs.push(new_message);
                msgs
            });
        })
    };
    
    let on_select_channel = {
        Callback::from(move |channel_id: String| {
            log::info!("Selected channel: {}", channel_id);
        })
    };

    html! {
        <div class={style}>
            <div class="app">
                <Navigation 
                    active_route={(*active_route).clone()} 
                    on_navigate={on_navigate} 
                />
                
                <main class="main-content">
                    <section class="stream-section">
                        <StreamPlayer 
                            title="Playing the latest RPG game".to_string()
                            streamer_name="GamerPro".to_string()
                            viewer_count=1250
                            stream_url="https://example.com/stream.webm".to_string()
                        />
                    </section>
                    
                    <section class="sidebar">
                        <div class="chat-section">
                            <Chat 
                                messages={(*chat_messages).clone()} 
                                on_send={on_send_message} 
                            />
                        </div>
                        
                        <div class="channel-list-section">
                            <ChannelList 
                                channels={(*channels).clone()} 
                                on_select={on_select_channel} 
                            />
                        </div>
                    </section>
                </main>
                
                <footer class="footer">
                    <p>{"© 2025 Live Streaming Platform. All rights reserved."}</p>
                </footer>
            </div>
        </div>
    }
}