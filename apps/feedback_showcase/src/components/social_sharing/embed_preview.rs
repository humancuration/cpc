//! Embed preview component for visualizing embedded visualizations

use yew::prelude::*;
use crate::components::visualization::types::VisualizationComponent;

#[derive(Properties, PartialEq)]
pub struct EmbedPreviewProps {
    pub share_id: String,
    pub visualization_type: VisualizationComponent,
    pub width: u32,
    pub height: u32,
    pub theme: String,
    pub show_title: bool,
    pub interactive: bool,
}

#[function_component(EmbedPreview)]
pub fn embed_preview(props: &EmbedPreviewProps) -> Html {
    // In a real implementation, this would render an actual preview of the embedded visualization
    // For now, we'll create a placeholder that shows the configuration
    
    let preview_style = format!(
        "width: {}px; height: {}px; border: 1px solid #ddd; border-radius: 4px; display: flex; flex-direction: column;",
        props.width, props.height
    );
    
    let title = match props.visualization_type {
        VisualizationComponent::Summary => "Feedback Summary",
        VisualizationComponent::Ratings => "Ratings Distribution",
        VisualizationComponent::WordCloud => "Word Cloud",
        VisualizationComponent::Sentiment => "Sentiment Analysis",
    };
    
    html! {
        <div class="embed-preview-container">
            <h3>{"Embed Preview"}</h3>
            <div class="embed-preview" style={preview_style}>
                if props.show_title {
                    <div class="embed-preview-header">
                        <h4>{title}</h4>
                    </div>
                }
                <div class="embed-preview-content">
                    <p>{"Embedded visualization would appear here"}</p>
                    <p><small>{"Share ID: "}{&props.share_id}</small></p>
                </div>
                <div class="embed-preview-footer">
                    <small>{"Powered by Feedback Showcase"}</small>
                </div>
            </div>
            <div class="embed-config-info">
                <p>{"Configuration:"}</p>
                <ul>
                    <li>{"Width: "}{props.width}{"px"}</li>
                    <li>{"Height: "}{props.height}{"px"}</li>
                    <li>{"Theme: "}{&props.theme}</li>
                    <li>{"Show Title: "}{if props.show_title { "Yes" } else { "No" }}</li>
                    <li>{"Interactive: "}{if props.interactive { "Yes" } else { "No" }}</li>
                </ul>
            </div>
        </div>
    }
}