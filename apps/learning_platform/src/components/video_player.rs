use yew::prelude::*;
use stylist::{style, yew::styled_component};

#[derive(Properties, PartialEq)]
pub struct VideoPlayerProps {
    pub media_url: String,
    #[prop_or(true)]
    pub autoplay: bool,
}

#[styled_component(VideoPlayer)]
pub fn video_player(props: &VideoPlayerProps) -> Html {
    let container_style = style!(
        r#"
        position: relative;
        width: 100%;
        padding-top: 56.25%; /* 16:9 Aspect Ratio */
        background: #000;
        border-radius: 8px;
        overflow: hidden;
    "#
    ).unwrap();

    let video_style = style!(
        r#"
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        outline: none;
    "#
    ).unwrap();

    let placeholder_style = style!(
        r#"
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        color: white;
        background: linear-gradient(45deg, #4361ee, #3a0ca3);
        text-align: center;
        padding: 1rem;
    "#
    ).unwrap();

    let button_style = style!(
        r#"
        background: rgba(255, 255, 255, 0.2);
        backdrop-filter: blur(10px);
        border: 2px solid white;
        color: white;
        padding: 0.75rem 1.5rem;
        border-radius: 50px;
        cursor: pointer;
        font-weight: bold;
        margin-top: 1rem;
        transition: background 0.2s;

        &:hover {
            background: rgba(255, 255, 255, 0.3);
        }
    "#
    ).unwrap();

// were no longer using ffmpeg
    // For now, we'll show a placeholder with a play button
    
    let on_play_click = Callback::from(|_| {
        // In a real implementation, this would initialize the video player
        web_sys::console::log_1(&"Play button clicked".into());
    });

    html! {
        <div class={container_style}>
            <div class={placeholder_style}>
                <h3>{"Video Content"}</h3>
                <p>{"AV1/Opus media would play here using ffmpeg.wasm"}</p>
                <p>{"Media URL: "}{&props.media_url}</p>
                <button class={button_style} onclick={on_play_click}>
                    {"Play Video"}
                </button>
            </div>
        </div>
    }
}