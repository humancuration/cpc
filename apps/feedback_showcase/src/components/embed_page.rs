//! Embed page component for displaying embedded visualizations

use yew::prelude::*;
use crate::components::visualization::types::{VisualizationComponent, Annotation};
use crate::components::social_sharing::embedded_visualization::EmbeddedVisualization;
use crate::services::federation::{get_shared_visualization, SharedVisualization};
use reviews::Review;
use crate::data_generator::generators::products::Product;
use uuid::Uuid;

#[derive(Properties, PartialEq)]
pub struct EmbedPageProps {
    #[prop_or_default]
    pub share_id: Option<String>,
    #[prop_or_default]
    pub width: Option<u32>,
    #[prop_or_default]
    pub height: Option<u32>,
    #[prop_or_default]
    pub theme: Option<String>,
    #[prop_or_default]
    pub show_title: Option<bool>,
    #[prop_or_default]
    pub interactive: Option<bool>,
}

#[function_component(EmbedPage)]
pub fn embed_page(props: &EmbedPageProps) -> Html {
    let share_id = props.share_id.clone().unwrap_or_default();
    let width = props.width.unwrap_or(600);
    let height = props.height.unwrap_or(400);
    let theme = props.theme.clone().unwrap_or("light".to_string());
    let show_title = props.show_title.unwrap_or(true);
    let interactive = props.interactive.unwrap_or(true);
    
    let annotations = use_state(|| vec![]);
    let loading = use_state(|| !share_id.is_empty());
    let error = use_state(|| None);
    let shared_visualization = use_state(|| None::<SharedVisualization>);
    
    // Load shared visualization data
    {
        let share_id = share_id.clone();
        let loading = loading.clone();
        let error = error.clone();
        let shared_visualization = shared_visualization.clone();
        use_effect_with(share_id.clone(), move |_| {
            if !share_id.is_empty() {
                wasm_bindgen_futures::spawn_local(async move {
                    match get_shared_visualization(&share_id).await {
                        Ok(shared_vis) => {
                            web_sys::console::log_1(&format!("Loaded shared visualization: {:?}", shared_vis).into());
                            shared_visualization.set(Some(shared_vis));
                            loading.set(false);
                        }
                        Err(e) => {
                            error.set(Some(format!("Failed to load visualization: {}", e)));
                            loading.set(false);
                        }
                    }
                });
            } else {
                loading.set(false);
            }
            || ()
        });
    }
    
    let on_add_annotation = {
        let annotations = annotations.clone();
        Callback::from(move |annotation: Annotation| {
            let mut new_annotations = (*annotations).clone();
            new_annotations.push(annotation);
            annotations.set(new_annotations);
        })
    };
    
    // Determine visualization type from shared data or default
    let visualization_type = shared_visualization.as_ref()
        .map(|shared_vis| {
            match shared_vis.visualization_type.as_str() {
                "Summary" => VisualizationComponent::Summary,
                "Ratings" => VisualizationComponent::Ratings,
                "WordCloud" => VisualizationComponent::WordCloud,
                "Sentiment" => VisualizationComponent::Sentiment,
                _ => VisualizationComponent::Summary,
            }
        })
        .unwrap_or(VisualizationComponent::Summary);
    
    // Extract data from shared visualization
    let data = shared_visualization.as_ref().map(|shared_vis| shared_vis.data.clone());
    
    html! {
        <div class="embed-page">
            if *loading {
                <div class="loading">{"Loading visualization..."}</div>
            } else if let Some(err) = &*error {
                <div class="error">{"Error: "}{err}</div>
            } else {
                <EmbeddedVisualization
                    share_id={share_id.clone()}
                    visualization_type={visualization_type}
                    width={width}
                    height={height}
                    theme={theme.clone()}
                    show_title={show_title}
                    interactive={interactive}
                    on_add_annotation={on_add_annotation}
                    annotations={(*annotations).clone()}
                    data={data}
                    loading={*loading}
                />
            }
        </div>
    }
}
    }
}