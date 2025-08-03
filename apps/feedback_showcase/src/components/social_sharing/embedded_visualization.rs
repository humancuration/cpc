//! Embedded visualization component for displaying shared visualizations with annotations

use yew::prelude::*;
use web_sys::{HtmlElement, MouseEvent};
use crate::components::visualization::types::{VisualizationComponent, Annotation, VisualizationProps};
use crate::components::social_sharing::annotation_manager::AnnotationManager;
use crate::components::social_sharing::utils::normalize_coordinates;
use crate::services::federation::{get_shared_visualization, SharedVisualization};
use reviews::Review;
use crate::data_generator::generators::products::Product;
use crate::components::visualization::{Summary, RatingsChart, WordCloud, Sentiment};

#[derive(Properties, PartialEq)]
pub struct EmbeddedVisualizationProps {
    pub share_id: String,
    #[prop_or(VisualizationComponent::Summary)]
    pub visualization_type: VisualizationComponent,
    pub width: u32,
    pub height: u32,
    pub theme: String,
    #[prop_or(true)]
    pub show_title: bool,
    #[prop_or(true)]
    pub interactive: bool,
    pub on_add_annotation: Callback<Annotation>,
    #[prop_or_default]
    pub annotations: Vec<Annotation>,
    #[prop_or_default]
    pub data: Option<Vec<Review<Product>>>,
    #[prop_or(true)]
    pub loading: bool,
}

#[function_component(EmbeddedVisualization)]
pub fn embedded_visualization(props: &EmbeddedVisualizationProps) -> Html {
    let container_ref = use_node_ref();
    let annotation_position = use_state(|| None);
    let show_annotation_form = use_state(|| false);
    
    let title = match props.visualization_type {
        VisualizationComponent::Summary => "Feedback Summary",
        VisualizationComponent::Ratings => "Ratings Distribution",
        VisualizationComponent::WordCloud => "Word Cloud",
        VisualizationComponent::Sentiment => "Sentiment Analysis",
    };
    
    let container_style = format!(
        "width: {}px; height: {}px; border: 1px solid #ddd; border-radius: 4px; display: flex; flex-direction: column;",
        props.width, props.height
    );
    
    let content_style = "flex: 1; display: flex; flex-direction: column; justify-content: center; align-items: center; padding: 20px; text-align: center;".to_string();
    
    let render_visualization = || -> Html {
        if props.loading {
            return html! {
                <div class="embedded-visualization-content" style={content_style.clone()}>
                    <p>{"Loading visualization..."}</p>
                </div>
            };
        }
        
        if let Some(ref data) = props.data {
            let viz_props = VisualizationProps {
                reviews: data.clone(),
                loading: props.loading,
                on_share: Callback::noop(),
                enable_sharing: false,
            };
            
            return match props.visualization_type {
                VisualizationComponent::Summary => {
                    html! { <Summary ..viz_props /> }
                }
                VisualizationComponent::Ratings => {
                    html! { <RatingsChart ..viz_props /> }
                }
                VisualizationComponent::WordCloud => {
                    html! { <WordCloud ..viz_props /> }
                }
                VisualizationComponent::Sentiment => {
                    html! { <Sentiment ..viz_props /> }
                }
            };
        }
        
        // Fallback to placeholder if no data
        html! {
            <div class="embedded-visualization-content" style={content_style.clone()}>
                <p>{"Embedded visualization would appear here"}</p>
                <p><small>{"Share ID: "}{&props.share_id}</small></p>
                if !props.interactive {
                    <p><small>{"Interactivity disabled"}</small></p>
                }
            </div>
        }
    };
    
    let on_click = {
        let container_ref = container_ref.clone();
        let annotation_position = annotation_position.clone();
        let show_annotation_form = show_annotation_form.clone();
        Callback::from(move |e: MouseEvent| {
            if let Some(container) = container_ref.cast::<HtmlElement>() {
                let x = e.client_x() as f32;
                let y = e.client_y() as f32;
                let (norm_x, norm_y) = normalize_coordinates(x, y, &container);
                
                annotation_position.set(Some((norm_x, norm_y)));
                show_annotation_form.set(true);
            }
        })
    };
    
    html! {
        <div class="embedded-visualization-container">
            <div
                ref={container_ref}
                onclick={on_click}
                class="embedded-visualization"
                style={container_style}
            >
                if props.show_title {
                    <div class="embedded-visualization-header">
                        <h4>{title}</h4>
                    </div>
                }
                {render_visualization()}
                <div class="embedded-visualization-footer">
                    <small>{"Powered by Feedback Showcase"}</small>
                </div>
                if let Some((x, y)) = *annotation_position {
                    <div
                        class="annotation-position-indicator"
                        style={format!("left: {}%; top: {}%;", x * 100.0, y * 100.0)}
                    />
                }
            </div>
            
            <AnnotationManager
                share_id={props.share_id.clone()}
                on_add_annotation={props.on_add_annotation.clone()}
                annotations={props.annotations.clone()}
                initial_position={*annotation_position}
                container_dimensions={(props.width, props.height)}
                show_form={*show_annotation_form}
                on_form_toggle={Callback::from(move |_| {
                    show_annotation_form.set(!*show_annotation_form);
                })}
            />
        </div>
    }
}