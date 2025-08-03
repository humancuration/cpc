//! Embed code dialog component for customizing and generating embed codes

use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::components::visualization::types::VisualizationComponent;
use crate::components::social_sharing::embed_code_generator::generate_custom_embed_code;
use crate::components::social_sharing::embed_preview::EmbedPreview;

#[derive(Properties, PartialEq)]
pub struct EmbedCodeDialogProps {
    pub share_id: String,
    pub visualization_type: VisualizationComponent,
    pub on_close: Callback<()>,
}

#[function_component(EmbedCodeDialog)]
pub fn embed_code_dialog(props: &EmbedCodeDialogProps) -> Html {
    let width = use_state(|| 600u32);
    let height = use_state(|| 400u32);
    let theme = use_state(|| "light".to_string());
    let show_title = use_state(|| true);
    let interactive = use_state(|| true);
    let copied = use_state(|| false);
    
    let on_width_change = {
        let width = width.clone();
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
            if let Ok(value) = target.value().parse::<u32>() {
                width.set(value);
            }
        })
    };
    
    let on_height_change = {
        let height = height.clone();
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
            if let Ok(value) = target.value().parse::<u32>() {
                height.set(value);
            }
        })
    };
    
    let on_theme_change = {
        let theme = theme.clone();
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<web_sys::HtmlSelectElement>().unwrap();
            theme.set(target.value());
        })
    };
    
    let on_show_title_change = {
        let show_title = show_title.clone();
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
            show_title.set(target.checked());
        })
    };
    
    let on_interactive_change = {
        let interactive = interactive.clone();
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
            interactive.set(target.checked());
        })
    };
    
    let on_copy = {
        let copied = copied.clone();
        let embed_code = generate_custom_embed_code(
            &props.share_id,
            Some(*width),
            Some(*height),
            Some(theme.as_str()),
            Some(*show_title),
            Some(*interactive),
        );
        Callback::from(move |_| {
            // Copy to clipboard
            let clipboard = web_sys::window()
                .unwrap()
                .navigator()
                .clipboard();
            
            if !clipboard.is_undefined() {
                let promise = clipboard.write_text(&embed_code);
                let copied = copied.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match wasm_bindgen_futures::JsFuture::from(promise).await {
                        Ok(_) => {
                            copied.set(true);
                            // Reset copied status after 2 seconds
                            gloo_timers::callback::Timeout::new(2000, move || {
                                copied.set(false);
                            }).forget();
                        }
                        Err(_) => {
                            web_sys::console::error_1(&"Failed to copy to clipboard".into());
                        }
                    }
                });
            } else {
                // Fallback for browsers that don't support clipboard API
                let text_area = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .create_element("textarea")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlTextAreaElement>()
                    .unwrap();
                
                text_area.set_value(&embed_code);
                text_area.set_style("position", "fixed");
                text_area.set_style("top", "0");
                text_area.set_style("left", "0");
                text_area.set_style("width", "2em");
                text_area.set_style("height", "2em");
                text_area.set_style("padding", "0");
                text_area.set_style("border", "none");
                text_area.set_style("outline", "none");
                text_area.set_style("box-shadow", "none");
                text_area.set_style("background", "transparent");
                
                web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .body()
                    .unwrap()
                    .append_child(&text_area)
                    .unwrap();
                
                text_area.select();
                let _ = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .exec_command("copy");
                text_area.remove();
                
                copied.set(true);
                // Reset copied status after 2 seconds
                gloo_timers::callback::Timeout::new(2000, move || {
                    copied.set(false);
                }).forget();
            }
        })
    };
    
    let on_close = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };
    
    let embed_code = generate_custom_embed_code(
        &props.share_id,
        Some(*width),
        Some(*height),
        Some(theme.as_str()),
        Some(*show_title),
        Some(*interactive),
    );
    
    html! {
        <div class="embed-code-dialog-overlay" onclick={on_close.clone()}>
            <div class="embed-code-dialog" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                <div class="dialog-header">
                    <h2>{"Embed Visualization"}</h2>
                    <button class="close-button" onclick={on_close.clone()} aria-label="Close dialog">{"×"}</button>
                </div>
                
                <div class="dialog-content">
                    <div class="embed-customization">
                        <div class="form-group">
                            <label for="width-input">{"Width (px):"}</label>
                            <input
                                id="width-input"
                                type="number"
                                min="200"
                                max="2000"
                                value={width.to_string()}
                                oninput={on_width_change}
                            />
                        </div>
                        
                        <div class="form-group">
                            <label for="height-input">{"Height (px):"}</label>
                            <input
                                id="height-input"
                                type="number"
                                min="100"
                                max="1500"
                                value={height.to_string()}
                                oninput={on_height_change}
                            />
                        </div>
                        
                        <div class="form-group">
                            <label for="theme-select">{"Theme:"}</label>
                            <select
                                id="theme-select"
                                onchange={on_theme_change}
                                value={theme.to_string()}
                            >
                                <option value="light">{"Light"}</option>
                                <option value="dark">{"Dark"}</option>
                            </select>
                        </div>
                        
                        <div class="form-group checkbox-group">
                            <label>
                                <input
                                    type="checkbox"
                                    checked={*show_title}
                                    onchange={on_show_title_change}
                                />
                                {"Show Title"}
                            </label>
                        </div>
                        
                        <div class="form-group checkbox-group">
                            <label>
                                <input
                                    type="checkbox"
                                    checked={*interactive}
                                    onchange={on_interactive_change}
                                />
                                {"Enable Interactivity"}
                            </label>
                        </div>
                    </div>
                    
                    <div class="embed-preview-section">
                        <EmbedPreview
                            share_id={props.share_id.clone()}
                            visualization_type={props.visualization_type.clone()}
                            width={*width}
                            height={*height}
                            theme={theme.to_string()}
                            show_title={*show_title}
                            interactive={*interactive}
                        />
                    </div>
                    
                    <div class="embed-code-section">
                        <label for="embed-code-textarea">{"Embed Code:"}</label>
                        <textarea
                            id="embed-code-textarea"
                            value={embed_code}
                            readonly=true
                            rows="4"
                        />
                        <button 
                            class="copy-button" 
                            onclick={on_copy}
                        >
                            if *copied {
                                {"Copied!"}
                            } else {
                                {"Copy to Clipboard"}
                            }
                        </button>
                    </div>
                </div>
                
                <div class="dialog-footer">
                    <button class="close-button-bottom" onclick={on_close.clone()}>
                        {"Close"}
                    </button>
                </div>
            </div>
        </div>
    }
}