//! Share button group component for visualization data

use yew::prelude::*;
use web_sys::HtmlCanvasElement;
use wasm_bindgen::JsCast;
use crate::components::visualization::types::{VisualizationComponent, ShareAction};
use reviews::Review;
use crate::data_generator::generators::products::Product;
use crate::components::social_sharing::image_exporter::export_as_image;
use crate::components::social_sharing::embed_code_generator::generate_embed_code;
use crate::services::federation::share_visualization;
use crate::components::social_sharing::social_sharing_dialog::SocialSharingDialog;
use crate::components::social_sharing::embed_code_dialog::EmbedCodeDialog;
use crate::utils::accessibility::ensure_social_button_accessibility;

#[derive(Properties, PartialEq)]
pub struct ShareButtonGroupProps {
    pub visualization_type: VisualizationComponent,
    pub reviews: Vec<Review<Product>>,
    pub canvas_ref: NodeRef,
    #[prop_or_default]
    pub on_share: Callback<ShareAction>,
}

#[function_component(ShareButtonGroup)]
pub fn share_button_group(props: &ShareButtonGroupProps) -> Html {
    let show_social_dialog = use_state(|| false);
    let show_embed_dialog = use_state(|| false);
    let share_id = use_state(|| None);
    
    let on_federation_share = {
        let visualization_type = props.visualization_type.clone();
        let reviews = props.reviews.clone();
        let on_share = props.on_share.clone();
        let share_id = share_id.clone();
        Callback::from(move |_| {
            let visualization_type = visualization_type.clone();
            let reviews = reviews.clone();
            let on_share = on_share.clone();
            let share_id = share_id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match share_visualization(reviews, &format!("{:?}", visualization_type)).await {
                    Ok(id) => {
                        share_id.set(Some(id.clone()));
                        web_sys::console::log_1(&format!("Successfully shared {:?} visualization to federation with ID: {}", visualization_type, id).into());
                        on_share.emit(ShareAction::Federation);
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to share visualization: {}", e).into());
                    }
                }
            });
        })
    };

    let on_embed_code = {
        let show_embed_dialog = show_embed_dialog.clone();
        let on_share = props.on_share.clone();
        let share_id = share_id.clone();
        Callback::from(move |_| {
            on_share.emit(ShareAction::Embed);
            // Generate a temporary share ID for the embed dialog if none exists
            if share_id.is_none() {
                let temp_id = uuid::Uuid::new_v4().to_string();
                share_id.set(Some(temp_id));
            }
            show_embed_dialog.set(true);
        })
    };

    let on_image_export = {
        let canvas_ref = props.canvas_ref.clone();
        let on_share = props.on_share.clone();
        Callback::from(move |_| {
            on_share.emit(ShareAction::Image);
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                export_as_image(&canvas, "visualization.png");
            }
        })
    };

    let on_social_share = {
        let show_social_dialog = show_social_dialog.clone();
        let on_share = props.on_share.clone();
        Callback::from(move |_| {
            on_share.emit(ShareAction::Social);
            show_social_dialog.set(true);
        })
    };
    
    let on_close_social_dialog = {
        let show_social_dialog = show_social_dialog.clone();
        Callback::from(move |_| {
            show_social_dialog.set(false);
        })
    };
    
    let on_close_embed_dialog = {
        let show_embed_dialog = show_embed_dialog.clone();
        Callback::from(move |_| {
            show_embed_dialog.set(false);
        })
    };

    // Get accessibility attributes
    let accessibility_attrs = ensure_social_button_accessibility();
    
    html! {
        <>
            <div class="share-button-group">
                <button
                    class="share-btn federation"
                    onclick={on_federation_share}
                    title="Share to Federation"
                    role={accessibility_attrs[0].1}
                    aria-label={accessibility_attrs[1].1}
                    tabindex={accessibility_attrs[2].1.parse::<i32>().unwrap_or(0)}
                >
                    {"🌐"}
                </button>
                <button
                    class="share-btn embed"
                    onclick={on_embed_code}
                    title="Generate Embed Code"
                    role={accessibility_attrs[0].1}
                    aria-label={accessibility_attrs[1].1}
                    tabindex={accessibility_attrs[2].1.parse::<i32>().unwrap_or(0)}
                >
                    {"</>"}
                </button>
                <button
                    class="share-btn image"
                    onclick={on_image_export}
                    title="Export as Image"
                    role={accessibility_attrs[0].1}
                    aria-label={accessibility_attrs[1].1}
                    tabindex={accessibility_attrs[2].1.parse::<i32>().unwrap_or(0)}
                >
                    {"📷"}
                </button>
                <button
                    class="share-btn social"
                    onclick={on_social_share}
                    title="Share to Social Platforms"
                    role={accessibility_attrs[0].1}
                    aria-label={accessibility_attrs[1].1}
                    tabindex={accessibility_attrs[2].1.parse::<i32>().unwrap_or(0)}
                >
                    {"🔗"}
                </button>
            </div>
            
            if *show_social_dialog {
                <SocialSharingDialog
                    visualization_type={props.visualization_type.clone()}
                    reviews={props.reviews.clone()}
                    on_close={on_close_social_dialog}
                />
            }
            
            if *show_embed_dialog {
                if let Some(id) = share_id.as_ref() {
                    <EmbedCodeDialog
                        share_id={id.clone()}
                        visualization_type={props.visualization_type.clone()}
                        on_close={on_close_embed_dialog}
                    />
                }
            }
        </>
    }
}