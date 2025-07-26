use yew::prelude::*;
use web_sys::HtmlVideoElement;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct VideoPlayerProps {
    pub video_url: String,
    pub thumbnail_url: String,
}

#[function_component(VideoPlayer)]
pub fn video_player(props: &VideoPlayerProps) -> Html {
    let video_ref = use_node_ref();
    let is_playing = use_state(|| false);
    let is_muted = use_state(|| false);
    let progress = use_state(|| 0.0);
    
    // Handle video play/pause
    let toggle_play = {
        let video_ref = video_ref.clone();
        let is_playing = is_playing.clone();
        Callback::from(move |_| {
            if let Some(video) = video_ref.cast::<HtmlVideoElement>() {
                if *is_playing {
                    let _ = video.pause();
                } else {
                    let _ = video.play();
                }
                is_playing.set(!*is_playing);
            }
        })
    };
    
    // Handle mute/unmute
    let toggle_mute = {
        let video_ref = video_ref.clone();
        let is_muted = is_muted.clone();
        Callback::from(move |_| {
            if let Some(video) = video_ref.cast::<HtmlVideoElement>() {
                video.set_muted(!*is_muted);
                is_muted.set(!*is_muted);
            }
        })
    };
    
    // Handle video progress
    let handle_time_update = {
        let progress = progress.clone();
        Callback::from(move |event: web_sys::Event| {
            if let Some(video) = event.target()
                .and_then(|t| t.dyn_into::<HtmlVideoElement>().ok())
            {
                let current = video.current_time();
                let duration = video.duration();
                if duration > 0.0 {
                    progress.set((current / duration) * 100.0);
                }
            }
        })
    };
    
    // Auto-play when video loads
    let handle_loaded_data = {
        let video_ref = video_ref.clone();
        let is_playing = is_playing.clone();
        Callback::from(move |_| {
            if let Some(video) = video_ref.cast::<HtmlVideoElement>() {
                let _ = video.play();
                is_playing.set(true);
            }
        })
    };
    
    html! {
        <div class="video-player">
            <video
                ref={video_ref}
                class="video-element"
                src={props.video_url.clone()}
                poster={props.thumbnail_url.clone()}
                ontimeupdate={handle_time_update}
                onloadeddata={handle_loaded_data}
                muted={*is_muted}
                loop=true
                playsinline=true
            />
            
            <div class="video-controls">
                <button 
                    class="play-pause-btn"
                    onclick={toggle_play}
                    aria-label="Play/Pause"
                >
                    if *is_playing {
                        { "⏸" }
                    } else {
                        { "▶" }
                    }
                </button>
                
                <button 
                    class="mute-btn"
                    onclick={toggle_mute}
                    aria-label="Mute/Unmute"
                >
                    if *is_muted {
                        { "🔇" }
                    } else {
                        { "🔊" }
                    }
                </button>
                
                <div class="progress-bar">
                    <div 
                        class="progress-fill" 
                        style={format!("width: {}%", *progress)}
                    />
                </div>
            </div>
            
            <div class="video-overlay">
                <div class="swipe-indicator">
                    <span>{ "Swipe up for next" }</span>
                </div>
            </div>
        </div>
    }
}