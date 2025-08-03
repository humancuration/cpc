//! Social sharing dialog component for sharing visualizations to social platforms

use yew::prelude::*;
use web_sys::HtmlTextAreaElement;
use crate::components::visualization::types::VisualizationComponent;
use reviews::Review;
use crate::data_generator::generators::products::Product;
use crate::utils::accessibility::generate_sharing_alt_text;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct SocialSharingDialogProps {
    pub visualization_type: VisualizationComponent,
    pub reviews: Vec<Review<Product>>,
    pub on_close: Callback<()>,
}

#[function_component(SocialSharingDialog)]
pub fn social_sharing_dialog(props: &SocialSharingDialogProps) -> Html {
    let selected_platform = use_state(|| "twitter".to_string());
    let message = use_state(|| "".to_string());
    let is_sharing = use_state(|| false);
    let focus_ref = use_node_ref();
    
    // Focus the dialog when it opens
    {
        let focus_ref = focus_ref.clone();
        use_effect_with((), move |_| {
            if let Some(element) = focus_ref.cast::<web_sys::HtmlElement>() {
                let _ = element.focus();
            }
            || ()
        });
    }
    
    let on_platform_change = {
        let selected_platform = selected_platform.clone();
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<web_sys::HtmlSelectElement>().unwrap();
            selected_platform.set(target.value());
        })
    };
    
    let on_message_input = {
        let message = message.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target_dyn_into::<HtmlTextAreaElement>().unwrap();
            message.set(target.value());
        })
    };
    
    let on_share = {
        let selected_platform = selected_platform.clone();
        let message = message.clone();
        let is_sharing = is_sharing.clone();
        let reviews = props.reviews.clone();
        let visualization_type = props.visualization_type.clone();
        Callback::from(move |_| {
            is_sharing.set(true);
            
            // In a real implementation, we would:
            // 1. Generate image data from canvas
            // 2. Use OAuth2 client to authenticate with selected platform
            // 3. Post the content to the platform
            
            let platform = selected_platform.as_str();
            let msg = message.as_str();
            let alt_text = generate_sharing_alt_text(
                &format!("{:?}", visualization_type),
                &reviews
            );
            
            web_sys::console::log_1(
                &format!(
                    "Sharing to {}: {}\nAlt text: {}",
                    platform, msg, alt_text
                )
                .into()
            );
            
            // Simulate sharing process
            wasm_bindgen_futures::spawn_local({
                let is_sharing = is_sharing.clone();
                async move {
                    // Simulate network delay
                    gloo_timers::future::TimeoutFuture::new(2000).await;
                    is_sharing.set(false);
                    web_sys::console::log_1(&"Sharing completed!".into());
                }
            });
        })
    };
    
    let on_close = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };
    
    let platform_options = vec![
        ("twitter", "Twitter"),
        ("facebook", "Facebook"),
        ("linkedin", "LinkedIn"),
    ];
    
    html! {
        <div class="social-sharing-dialog-overlay" onclick={on_close.clone()}>
            <div
                class="social-sharing-dialog"
                onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
                ref={focus_ref.clone()}
                tabindex="0"
            >
                <div class="dialog-header">
                    <h2>{"Share to Social Media"}</h2>
                    <button
                        class="close-button"
                        onclick={on_close.clone()}
                        aria-label="Close dialog"
                    >
                        {"×"}
                    </button>
                </div>
                
                <div class="dialog-content">
                    <div class="platform-selector">
                        <label for="platform-select">{"Select Platform:"}</label>
                        <select
                            id="platform-select"
                            onchange={on_platform_change}
                            aria-label="Select social media platform"
                        >
                            {for platform_options.iter().map(|(value, label)| {
                                let is_selected = *value == *selected_platform;
                                html! {
                                    <option value={value.to_string()} selected={is_selected}>
                                        {label}
                                    </option>
                                }
                            })}
                        </select>
                    </div>
                    
                    <div class="message-editor">
                        <label for="message-textarea">{"Message:"}</label>
                        <textarea
                            id="message-textarea"
                            value={(*message).clone()}
                            oninput={on_message_input}
                            placeholder="Add your message here..."
                            rows="4"
                            aria-label="Message to share"
                        />
                    </div>
                    
                    <div class="preview-panel">
                        <h3>{"Preview"}</h3>
                        <div class="preview-content">
                            <p>{"Your visualization will appear here with the message:"}</p>
                            <p class="preview-message"><strong>{&*message}</strong></p>
                            <p class="preview-alt-text">
                                {"Alt text: "} 
                                {generate_sharing_alt_text(&format!("{:?}", props.visualization_type), &props.reviews)}
                            </p>
                        </div>
                    </div>
                </div>
                
                <div class="dialog-footer">
                    <button
                        class="cancel-button"
                        onclick={on_close.clone()}
                        aria-label="Cancel sharing"
                    >
                        {"Cancel"}
                    </button>
                    <button
                        class="share-button"
                        onclick={on_share}
                        disabled={*is_sharing}
                        aria-label="Share to selected platform"
                    >
                        if *is_sharing {
                            {"Sharing..."}
                        } else {
                            {"Share"}
                        }
                    </button>
                </div>
            </div>
        </div>
    }
}