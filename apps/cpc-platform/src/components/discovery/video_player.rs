use yew::prelude::*;
use web_sys::{HtmlVideoElement, MediaSource, URL};
use wasm_bindgen_futures::spawn_local;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Properties, PartialEq, Clone)]
pub struct VideoPlayerProps {
    pub src: String,
    pub poster: String,
    pub autoplay: bool,
    pub muted: bool,
    pub on_ended: Callback<()>,
    pub on_time_update: Callback<f64>,
    pub on_loaded: Callback<()>,
}

#[function_component(VideoPlayer)]
pub fn video_player(props: &VideoPlayerProps) -> Html {
    let video_ref = use_node_ref();
    let is_playing = use_state(|| false);
    let duration = use_state(|| 0.0);
    let current_time = use_state(|| 0.0);
    let is_muted = use_state(|| props.muted);
    let volume = use_state(|| 1.0);

    let video_element = {
        let video_ref = video_ref.clone();
        use_state(|| None::<HtmlVideoElement>)
    };

    // Setup video element
    {
        let video_ref = video_ref.clone();
        let video_element = video_element.clone();
        let src = props.src.clone();
        let poster = props.poster.clone();
        let autoplay = props.autoplay;
        let is_playing = is_playing.clone();
        let duration = duration.clone();
        let current_time = current_time.clone();
        let on_ended = props.on_ended.clone();
        let on_time_update = props.on_time_update.clone();
        let on_loaded = props.on_loaded.clone();
        
        use_effect_with_deps(move |_| {
            if let Some(video) = video_ref.cast::<HtmlVideoElement>() {
                video.set_src(&src);
                video.set_poster(&poster);
                video.set_autoplay(autoplay);
                video.set_muted(*is_muted);
                
                // Setup event listeners
                let ended_closure = Closure::wrap(Box::new(move || {
                    is_playing.set(false);
                    on_ended.emit(());
                }) as Box<dyn FnMut()>);
                
                let loaded_closure = Closure::wrap(Box::new(move || {
                    duration.set(video.duration().unwrap_or(0.0));
                    on_loaded.emit(());
                }) as Box<dyn FnMut()>);
                
                let time_update_closure = Closure::wrap(Box::new(move || {
                    let time = video.current_time().unwrap_or(0.0);
                    current_time.set(time);
                    on_time_update.emit(time);
                }) as Box<dyn FnMut()>);
                
                video.add_event_listener_with_callback("ended", ended_closure.as_ref().unchecked_ref());
                video.add_event_listener_with_callback("loadedmetadata", loaded_closure.as_ref().unchecked_ref());
                video.add_event_listener_with_callback("timeupdate", time_update_closure.as_ref().unchecked_ref());
                
                ended_closure.forget();
                loaded_closure.forget();
                time_update_closure.forget();
                
                video_element.set(Some(video));
            }
            
            || ()
        }, (src, poster, autoplay));
    }

    // Control playback
    let toggle_play = {
        let video_element = video_element.clone();
        let is_playing = is_playing.clone();
        
        Callback::from(move |_| {
            if let Some(video) = &*video_element {
                if *is_playing {
                    let _ = video.pause();
                } else {
                    let _ = video.play();
                }
                is_playing.set(!*is_playing);
            }
        })
    };

    // Toggle mute
    let toggle_mute = {
        let video_element = video_element.clone();
        let is_muted = is_muted.clone();
        
        Callback::from(move |_| {
            if let Some(video) = &*video_element {
                let new_muted = !*is_muted;
                video.set_muted(new_muted);
                is_muted.set(new_muted);
            }
        })
    };

    // Seek to position
    let seek_to = {
        let video_element = video_element.clone();
        
        Callback::from(move |position: f64| {
            if let Some(video) = &*video_element {
                video.set_current_time(position);
            }
        })
    };

    // Format time for display
    let format_time = |time: f64| -> String {
        let minutes = (time / 60.0).floor() as i32;
        let seconds = (time % 60.0).floor() as i32;
        format!("{:02}:{:02}", minutes, seconds)
    };

    html! {
        <div class="relative w-full h-full bg-black">
            <video
                ref={video_ref}
                class="w-full h-full object-cover"
                playsinline={true}
                webkit_playsinline={true}
                x5_video_player_type="h5"
                x5_video_player_fullscreen="false"
                preload="metadata"
            />
            
            // Video controls overlay
            <div class="absolute inset-0 flex items-center justify-center opacity-0 hover:opacity-100 transition-opacity duration-300">
                <button
                    onclick={toggle_play}
                    class="bg-black/50 text-white rounded-full p-4 hover:bg-black/70 transition-colors"
                >
                    if *is_playing {
                        <svg class="w-8 h-8" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zM7 8a1 1 0 012 0v4a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v4a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd" />
                        </svg>
                    } else {
                        <svg class="w-8 h-8" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd" />
                        </svg>
                    }
                </button>
            </div>
            
            // Progress bar and time
            <div class="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black/80 to-transparent p-4">
                <div class="flex items-center justify-between text-white text-sm mb-2">
                    <span>{ format_time(*current_time) }</span>
                    <span>{ format_time(*duration) }</span>
                </div>
                
                <div class="bg-white/30 rounded-full h-1">
                    <div 
                        class="bg-white h-1 rounded-full transition-all duration-100"
                        style={format!("width: {}%", (*current_time / *duration * 100.0).min(100.0))}
                    />
                </div>
            </div>
            
            // Mute button
            <button
                onclick={toggle_mute}
                class="absolute top-4 right-4 bg-black/50 text-white rounded-full p-2 hover:bg-black/70 transition-colors"
            >
                if *is_muted {
                    <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M9.383 3.076A1 1 0 0110 4v12a1 1 0 01-1.617.816L4.5 14H2a1 1 0 01-1-1V7a1 1 0 011-1h2.5l3.883-2.816zM16 8a1 1 0 10-2 0v4a1 1 0 102 0V8z" clip-rule="evenodd" />
                    </svg>
                } else {
                    <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M9.383 3.076A1 1 0 0110 4v12a1 1 0 01-1.617.816L4.5 14H2a1 1 0 01-1-1V7a1 1 0 011-1h2.5l3.883-2.816zM15.536 6.464a1 1 0 011.414 0 5 5 0 010 7.072 1 1 0 01-1.414-1.414 3 3 0 000-4.242 1 1 0 010-1.414zM17.657 4.343a1 1 0 011.414 0 9 9 0 010 12.728 1 1 0 11-1.414-1.414 7 7 0 000-9.9 1 1 0 010-1.414z" clip-rule="evenodd" />
                    </svg>
                }
            </button>
        </div>
    }
}