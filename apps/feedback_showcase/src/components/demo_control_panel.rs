//! Demo control panel component for adjusting data generation parameters

use yew::prelude::*;
use crate::data_generator::config::{DataGeneratorConfig, RatingDistributionConfig};
use std::collections::HashMap;

/// Component types that can be toggled in the playground
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComponentType {
    Summary,
    Ratings,
    WordCloud,
    Sentiment,
}

/// Layout configuration for responsive design
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutConfig {
    pub columns: u8,
    pub component_size: ComponentSize,
}

/// Component size options
#[derive(Debug, Clone, PartialEq)]
pub enum ComponentSize {
    Small,
    Medium,
    Large,
}

/// Application state structure
#[derive(Debug, Clone, PartialEq)]
pub struct AppState {
    pub reviews: Vec<reviews::Review<crate::data_generator::generators::products::Product>>,
    pub layout: LayoutConfig,
    pub visibility: HashMap<ComponentType, bool>,
}

impl Default for AppState {
    fn default() -> Self {
        let mut visibility = HashMap::new();
        visibility.insert(ComponentType::Summary, true);
        visibility.insert(ComponentType::Ratings, true);
        visibility.insert(ComponentType::WordCloud, true);
        visibility.insert(ComponentType::Sentiment, true);

        Self {
            reviews: vec![],
            layout: LayoutConfig {
                columns: 2,
                component_size: ComponentSize::Medium,
            },
            visibility,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct DemoControlPanelProps {
    pub config: DataGeneratorConfig,
    pub on_config_update: Callback<DataGeneratorConfig>,
    pub on_generate: Callback<()>,
}

#[function_component(DemoControlPanel)]
pub fn demo_control_panel(props: &DemoControlPanelProps) -> Html {
    let config = use_state(|| props.config.clone());
    
    // Update parent when config changes
    let on_config_update = props.on_config_update.clone();
    let config_clone = config.clone();
    use_effect_with(config_clone.clone(), move |_| {
        on_config_update.emit((*config_clone).clone());
        || ()
    });

    // Rating distribution controls
    let on_rating_change = {
        let config = config.clone();
        Callback::from(move |(index, value): (usize, f32)| {
            let mut new_config = (*config).clone();
            match index {
                0 => new_config.rating_distribution.mean = value,
                1 => new_config.rating_distribution.std_dev = value,
                2 => new_config.rating_distribution.min = value,
                3 => new_config.rating_distribution.max = value,
                _ => {}
            }
            config.set(new_config);
        })
    };

    // Sentiment mix controls
    let on_sentiment_change = {
        let config = config.clone();
        Callback::from(move |(index, value): (usize, f32)| {
            // For now, we'll just log the changes
            // In a real implementation, this would affect the sentiment distribution
            web_sys::console::log_1(&format!("Sentiment change: {} = {}", index, value).into());
        })
    };

    let on_generate = props.on_generate.clone();
    let on_generate_click = Callback::from(move |_| on_generate.emit(()));

    html! {
        <div class="demo-control-panel">
            <h2>{"Demo Control Panel"}</h2>
            
            <div class="control-section">
                <h3>{"Rating Distribution"}</h3>
                <div class="slider-group">
                    <label>{"Mean: "}
                        <input 
                            type="range" 
                            min="0" 
                            max="100" 
                            value={(config.rating_distribution.mean * 100.0) as i32} 
                            onchange={
                                let on_rating_change = on_rating_change.clone();
                                Callback::from(move |e: Event| {
                                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                    let value = input.value().parse::<i32>().unwrap_or(75) as f32 / 100.0;
                                    on_rating_change.emit((0, value));
                                })
                            }
                        />
                        <span>{format!("{:.2}", config.rating_distribution.mean)}</span>
                    </label>
                </div>
                
                <div class="slider-group">
                    <label>{"Std Dev: "}
                        <input 
                            type="range" 
                            min="1" 
                            max="50" 
                            value={(config.rating_distribution.std_dev * 100.0) as i32} 
                            onchange={
                                let on_rating_change = on_rating_change.clone();
                                Callback::from(move |e: Event| {
                                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                    let value = input.value().parse::<i32>().unwrap_or(15) as f32 / 100.0;
                                    on_rating_change.emit((1, value));
                                })
                            }
                        />
                        <span>{format!("{:.2}", config.rating_distribution.std_dev)}</span>
                    </label>
                </div>
                
                <div class="slider-group">
                    <label>{"Min: "}
                        <input 
                            type="range" 
                            min="0" 
                            max="100" 
                            value={(config.rating_distribution.min * 100.0) as i32} 
                            onchange={
                                let on_rating_change = on_rating_change.clone();
                                Callback::from(move |e: Event| {
                                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                    let value = input.value().parse::<i32>().unwrap_or(0) as f32 / 100.0;
                                    on_rating_change.emit((2, value));
                                })
                            }
                        />
                        <span>{format!("{:.2}", config.rating_distribution.min)}</span>
                    </label>
                </div>
                
                <div class="slider-group">
                    <label>{"Max: "}
                        <input 
                            type="range" 
                            min="0" 
                            max="100" 
                            value={(config.rating_distribution.max * 100.0) as i32} 
                            onchange={
                                let on_rating_change = on_rating_change.clone();
                                Callback::from(move |e: Event| {
                                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                    let value = input.value().parse::<i32>().unwrap_or(100) as f32 / 100.0;
                                    on_rating_change.emit((3, value));
                                })
                            }
                        />
                        <span>{format!("{:.2}", config.rating_distribution.max)}</span>
                    </label>
                </div>
            </div>
            
            <div class="control-section">
                <h3>{"Sentiment Mix"}</h3>
                <div class="slider-group">
                    <label>{"Positive: "}
                        <input 
                            type="range" 
                            min="0" 
                            max="100" 
                            value="60" 
                            onchange={
                                let on_sentiment_change = on_sentiment_change.clone();
                                Callback::from(move |e: Event| {
                                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                    let value = input.value().parse::<i32>().unwrap_or(60) as f32 / 100.0;
                                    on_sentiment_change.emit((0, value));
                                })
                            }
                        />
                        <span>{"60%"}</span>
                    </label>
                </div>
                
                <div class="slider-group">
                    <label>{"Neutral: "}
                        <input 
                            type="range" 
                            min="0" 
                            max="100" 
                            value="30" 
                            onchange={
                                let on_sentiment_change = on_sentiment_change.clone();
                                Callback::from(move |e: Event| {
                                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                    let value = input.value().parse::<i32>().unwrap_or(30) as f32 / 100.0;
                                    on_sentiment_change.emit((1, value));
                                })
                            }
                        />
                        <span>{"30%"}</span>
                    </label>
                </div>
                
                <div class="slider-group">
                    <label>{"Negative: "}
                        <input 
                            type="range" 
                            min="0" 
                            max="100" 
                            value="10" 
                            onchange={
                                let on_sentiment_change = on_sentiment_change.clone();
                                Callback::from(move |e: Event| {
                                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                    let value = input.value().parse::<i32>().unwrap_or(10) as f32 / 100.0;
                                    on_sentiment_change.emit((2, value));
                                })
                            }
                        />
                        <span>{"10%"}</span>
                    </label>
                </div>
            </div>
            
            <button onclick={on_generate_click} class="generate-btn">
                {"Generate Sample Data"}
            </button>
        </div>
    }
}