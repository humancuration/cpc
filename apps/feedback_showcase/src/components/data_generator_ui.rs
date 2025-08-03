//! Main UI component for the data generator

use yew::prelude::*;
use crate::components::{ConfigPanel, ActionBar, MetricsPanel, DemoControlPanel, Playground, PresetSelector};
use crate::components::visualization::{Summary, RatingsChart, WordCloud, Sentiment};
use crate::components::demo_control_panel::{AppState, ComponentType, LayoutConfig, ComponentSize};
use crate::data_generator::config::DataGeneratorConfig;
use crate::data_generator::generators::products::Product;
use crate::services::generator_service::{GeneratorService, GenerationMetrics};
use crate::services::data_generator::DataGeneratorService;
use gloo_timers::callback::Interval;
use wasm_bindgen_futures::spawn_local;
use reviews::Review;

#[derive(Properties, PartialEq)]
pub struct DataGeneratorUIProps {
    // No props needed for root component
}

#[function_component(DataGeneratorUI)]
pub fn data_generator_ui(_props: &DataGeneratorUIProps) -> Html {
    let config = use_state(|| crate::data_generator::utils::create_default_config());
    let app_state = use_state(|| AppState::default());
    let metrics = use_state(|| GenerationMetrics::default());
    let generating = use_state(|| false);
    let generated_data = use_state(|| None::<Vec<Review<Product>>>);
    
    // Update metrics periodically when generating
    let metrics_clone = metrics.clone();
    let generating_clone = generating.clone();
    use_effect_with((*generating).clone(), move |_| {
        if *generating_clone {
            let interval = Interval::new(100, move || {
                // In a real implementation, this would fetch actual metrics
                // For now, we'll just simulate progress
                metrics_clone.set(GenerationMetrics {
                    items_processed: 0,
                    items_per_second: 0.0,
                    memory_usage: 0,
                    progress: 0,
                });
            });
            || drop(interval)
        } else {
            || ()
        }
    });

    let on_config_update = {
        let config = config.clone();
        Callback::from(move |new_config: DataGeneratorConfig| {
            config.set(new_config);
        })
    };
    
    let on_preset_selected = {
        let config = config.clone();
        Callback::from(move |new_config: DataGeneratorConfig| {
            config.set(new_config);
        })
    };

    let on_generate = {
        let config = config.clone();
        let app_state = app_state.clone();
        let generating = generating.clone();
        let metrics = metrics.clone();
        let generated_data = generated_data.clone();
        Callback::from(move |_| {
            let config = (*config).clone();
            let app_state = app_state.clone();
            let generating = generating.clone();
            let metrics = metrics.clone();
            let generated_data = generated_data.clone();
            
            // Set generating state to true
            generating.set(true);
            
            // Start generation in background
            spawn_local(async move {
                // Use the new data generator service
                DataGeneratorService::generate_data_async(config, move |result| {
                    match result {
                        Ok(data) => {
                            // Update app state with generated data
                            let mut new_app_state = (*app_state).clone();
                            new_app_state.reviews = data.reviews.clone();
                            app_state.set(new_app_state);
                            
                            // Store generated data
                            generated_data.set(Some(data.reviews));
                            
                            // Generation completed
                            metrics.set(GenerationMetrics {
                                items_processed: 0,
                                items_per_second: 0.0,
                                memory_usage: 0,
                                progress: 100,
                            });
                        }
                        Err(_) => {
                            // Handle error
                        }
                    }
                    generating.set(false);
                });
            });
        })
    };

    let on_reset = {
        let config = config.clone();
        let app_state = app_state.clone();
        let generated_data = generated_data.clone();
        Callback::from(move |_| {
            config.set(crate::data_generator::utils::create_default_config());
            app_state.set(AppState::default());
            generated_data.set(None);
        })
    };
    
    let on_export = {
        let generated_data = generated_data.clone();
        Callback::from(move |_| {
            if let Some(data) = generated_data.as_ref() {
                GeneratorService::export_data(data);
            }
        })
    };

let on_show_sample_data = {
    let app_state = app_state.clone();
    let generated_data = generated_data.clone();
    Callback::from(move |_| {
        // Create sample data for preview
        let sample_reviews = crate::data_generator::utils::create_sample_reviews();
        
        // Update app state with sample data
        let mut new_app_state = (*app_state).clone();
        new_app_state.reviews = sample_reviews.clone();
        app_state.set(new_app_state);
        
        generated_data.set(Some(sample_reviews));
    })
};

let on_component_toggle = {
    let app_state = app_state.clone();
    Callback::from(move |component_type: ComponentType| {
        let mut new_app_state = (*app_state).clone();
        let visibility = new_app_state.visibility.get_mut(&component_type).unwrap();
        *visibility = !*visibility;
        app_state.set(new_app_state);
    })
};

    html! {
        <div class="data-generator-ui">
            <h1>{"Feedback Data Generator"}</h1>
            
            <div class="control-section">
                <PresetSelector on_preset_selected={on_preset_selected} />
                <DemoControlPanel
                    config={(*config).clone()}
                    on_config_update={on_config_update}
                    on_generate={on_generate}
                />
            </div>
            
            <div class="action-buttons">
                <ActionBar
                    on_generate={on_generate}
                    on_reset={on_reset}
                    on_export={on_export}
                    generating={*generating}
                    has_data={generated_data.is_some()}
                />
                <button onclick={on_show_sample_data} class="preview-button">
                    {"Preview Visualizations"}
                </button>
            </div>
            <MetricsPanel metrics={(*metrics).clone()} />
            
            <Playground
                app_state={(*app_state).clone()}
                on_component_toggle={on_component_toggle}
            />
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_data_generator_ui_render() {
        let props = DataGeneratorUIProps {};
        let html = yew::Renderer::<DataGeneratorUI>::with_props(props).render();
        assert!(html.contains("Feedback Data Generator"));
    }
}