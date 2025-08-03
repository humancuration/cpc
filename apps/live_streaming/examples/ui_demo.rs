//! UI components demonstration

use yew::prelude::*;
use cpc_live_streaming::ui::{
    app::App,
    stream_player::{StreamPlayer, StreamPlayerProps},
    chat::{Chat, ChatProps, ChatMessage},
    channel_list::{ChannelList, ChannelListProps, ChannelInfo},
    navigation::{Navigation, NavigationProps},
};

/// Demo component that shows all UI components
#[function_component(UiDemo)]
fn ui_demo() -> Html {
    let messages = vec![
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
    ];
    
    let channels = vec![
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
    ];

    html! {
        <div style="padding: 20px; background-color: #0f0f0f; color: #ffffff;">
            <h1>{"UI Components Demo"}</h1>
            
            <h2>{"Navigation Component"}</h2>
            <Navigation 
                active_route="browse".to_string() 
                on_navigate={Callback::from(|_| {})} 
            />
            
            <h2>{"Stream Player Component"}</h2>
            <StreamPlayer 
                title="Playing the latest RPG game".to_string()
                streamer_name="GamerPro".to_string()
                viewer_count=1250
                stream_url="https://example.com/stream.webm".to_string()
            />
            
            <h2>{"Chat Component"}</h2>
            <div style="width: 400px;">
                <Chat 
                    messages={messages} 
                    on_send={Callback::from(|_| {})} 
                />
            </div>
            
            <h2>{"Channel List Component"}</h2>
            <div style="width: 400px;">
                <ChannelList 
                    channels={channels} 
                    on_select={Callback::from(|_| {})} 
                />
            </div>
            
            <h2>{"Full Application"}</h2>
            <div style="height: 600px;">
                <App />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<UiDemo>::new().render();
}