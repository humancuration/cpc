//! Playground component for interactive visualization layout

use yew::prelude::*;
use crate::components::visualization::{Summary, RatingsChart, WordCloud, Sentiment};
use crate::components::demo_control_panel::AppState;
use crate::components::demo_control_panel::ComponentType;
use reviews::Review;
use crate::data_generator::generators::products::Product;

#[derive(Properties, PartialEq)]
pub struct PlaygroundProps {
    pub app_state: AppState,
    pub on_component_toggle: Callback<ComponentType>,
}

#[function_component(Playground)]
pub fn playground(props: &PlaygroundProps) -> Html {
    let app_state = &props.app_state;
    let reviews = &app_state.reviews;
    let visibility = &app_state.visibility;
    
    // Calculate grid classes based on layout config
    let grid_class = match app_state.layout.columns {
        1 => "playground-grid cols-1",
        2 => "playground-grid cols-2",
        3 => "playground-grid cols-3",
        _ => "playground-grid cols-2",
    };
    
    let loading = reviews.is_empty();
    
    html! {
        <div class="playground">
            <h2>{"Visualization Playground"}</h2>
            
            <div class={grid_class}>
                if *visibility.get(&ComponentType::Summary).unwrap_or(&true) {
                    <div class="playground-item">
                        <div class="component-header">
                            <h3>{"Summary"}</h3>
                            <button 
                                class="toggle-btn"
                                onclick={
                                    let on_component_toggle = props.on_component_toggle.clone();
                                    Callback::from(move |_| on_component_toggle.emit(ComponentType::Summary))
                                }
                            >
                                {"Hide"}
                            </button>
                        </div>
                        <Summary reviews={reviews.clone()} loading={loading} enable_sharing={true} />
                    </div>
                }
                
                if *visibility.get(&ComponentType::Ratings).unwrap_or(&true) {
                    <div class="playground-item">
                        <div class="component-header">
                            <h3>{"Ratings Distribution"}</h3>
                            <button 
                                class="toggle-btn"
                                onclick={
                                    let on_component_toggle = props.on_component_toggle.clone();
                                    Callback::from(move |_| on_component_toggle.emit(ComponentType::Ratings))
                                }
                            >
                                {"Hide"}
                            </button>
                        </div>
                        <RatingsChart reviews={reviews.clone()} loading={loading} enable_sharing={true} />
                    </div>
                }
                
                if *visibility.get(&ComponentType::WordCloud).unwrap_or(&true) {
                    <div class="playground-item">
                        <div class="component-header">
                            <h3>{"Word Cloud"}</h3>
                            <button 
                                class="toggle-btn"
                                onclick={
                                    let on_component_toggle = props.on_component_toggle.clone();
                                    Callback::from(move |_| on_component_toggle.emit(ComponentType::WordCloud))
                                }
                            >
                                {"Hide"}
                            </button>
                        </div>
                        <WordCloud reviews={reviews.clone()} loading={loading} enable_sharing={true} />
                    </div>
                }
                
                if *visibility.get(&ComponentType::Sentiment).unwrap_or(&true) {
                    <div class="playground-item">
                        <div class="component-header">
                            <h3>{"Sentiment Analysis"}</h3>
                            <button 
                                class="toggle-btn"
                                onclick={
                                    let on_component_toggle = props.on_component_toggle.clone();
                                    Callback::from(move |_| on_component_toggle.emit(ComponentType::Sentiment))
                                }
                            >
                                {"Hide"}
                            </button>
                        </div>
                        <Sentiment reviews={reviews.clone()} loading={loading} enable_sharing={true} />
                    </div>
                }
            </div>
            
            if reviews.is_empty() {
                <div class="placeholder">
                    <p>{"Generate sample data to see visualizations"}</p>
                </div>
            }
        </div>
    }
}