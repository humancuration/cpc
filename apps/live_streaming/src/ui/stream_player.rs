//! Stream player component using Yew and Stylist

use yew::prelude::*;
use stylist::{style, yew::styled_component};

/// Properties for the StreamPlayer component
#[derive(Properties, PartialEq)]
pub struct StreamPlayerProps {
    /// Title of the stream
    pub title: String,
    
    /// Name of the streamer
    pub streamer_name: String,
    
    /// Current viewer count
    pub viewer_count: u32,
    
    /// Stream URL
    pub stream_url: String,
}

/// Stream player component
#[styled_component(StreamPlayer)]
pub fn stream_player(props: &StreamPlayerProps) -> Html {
    let style = style!(
        r#"
        .video-container {
            background-color: #000000;
            border-radius: 8px;
            overflow: hidden;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
            margin-bottom: 1rem;
        }
        
        .video-player {
            width: 100%;
            height: auto;
            aspect-ratio: 16 / 9;
            background-color: #000000;
        }
        
        .stream-info {
            padding: 1rem;
        }
        
        .stream-title {
            color: #ffffff;
            margin-bottom: 0.5rem;
            font-size: 1.5rem;
        }
        
        .streamer-name {
            color: #9146ff;
            margin-bottom: 0.25rem;
            font-size: 1.1rem;
        }
        
        .viewer-count {
            color: #aaaaaa;
            font-size: 1rem;
        }
    "#
    ).expect("Failed to create style");

    html! {
        <div class={style}>
            <div class="video-container">
                <video class="video-player" controls=true>
                    <source src={props.stream_url.clone()} type="video/webm" />
                    {"Your browser does not support the video tag."}
                </video>
                <div class="stream-info">
                    <h2 class="stream-title">{&props.title}</h2>
                    <p class="streamer-name">{&props.streamer_name}</p>
                    <p class="viewer-count">{format!("{} viewers", props.viewer_count)}</p>
                </div>
            </div>
        </div>
    }
}