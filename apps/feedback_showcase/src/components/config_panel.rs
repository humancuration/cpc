//! Configuration panel component for the data generator

use yew::prelude::*;
use crate::data_generator::config::{DataGeneratorConfig, RatingDistributionConfig, ProductTypeConfig};
use crate::styles::data_generator::*;

#[derive(Properties, PartialEq)]
pub struct ConfigPanelProps {
    pub config: DataGeneratorConfig,
    pub on_update: Callback<DataGeneratorConfig>,
}

#[function_component(ConfigPanel)]
pub fn config_panel(props: &ConfigPanelProps) -> Html {
    let config = use_state(|| props.config.clone());
    
    // Update parent when config changes
    let on_update = props.on_update.clone();
    let config_clone = config.clone();
    use_effect_with((), move |_| {
        on_update.emit((*config_clone).clone());
        || ()
    });

    let on_review_count_change = {
        let config = config.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = input.value().parse::<usize>().unwrap_or(100);
            let mut new_config = (*config).clone();
            new_config.review_count = value;
            config.set(new_config);
        })
    };

    let on_survey_response_rate_change = {
        let config = config.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = input.value().parse::<f32>().unwrap_or(0.8) / 100.0;
            let mut new_config = (*config).clone();
            new_config.survey_response_rate = value;
            config.set(new_config);
        })
    };

    let on_rating_mean_change = {
        let config = config.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = input.value().parse::<f32>().unwrap_or(0.75);
            let mut new_config = (*config).clone();
            new_config.rating_distribution.mean = value;
            config.set(new_config);
        })
    };

    let on_rating_std_dev_change = {
        let config = config.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = input.value().parse::<f32>().unwrap_or(0.15);
            let mut new_config = (*config).clone();
            new_config.rating_distribution.std_dev = value;
            config.set(new_config);
        })
    };

    let on_rating_min_change = {
        let config = config.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = input.value().parse::<f32>().unwrap_or(0.0);
            let mut new_config = (*config).clone();
            new_config.rating_distribution.min = value;
            config.set(new_config);
        })
    };

    let on_rating_max_change = {
        let config = config.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let value = input.value().parse::<f32>().unwrap_or(1.0);
            let mut new_config = (*config).clone();
            new_config.rating_distribution.max = value;
            config.set(new_config);
        })
    };

    let on_add_product_type = {
        let config = config.clone();
        Callback::from(move |_| {
            let mut new_config = (*config).clone();
            new_config.product_types.push(ProductTypeConfig {
                name: "New Product".to_string(),
                description: "Product description".to_string(),
                weight: 0.1,
                common_attributes: vec![],
            });
            config.set(new_config);
        })
    };

    let on_remove_product_type = {
        let config = config.clone();
        Callback::from(move |index: usize| {
            let mut new_config = (*config).clone();
            if new_config.product_types.len() > 1 {
                new_config.product_types.remove(index);
                config.set(new_config);
            }
        })
    };

    let on_product_type_name_change = {
        let config = config.clone();
        Callback::from(move |(index, name): (usize, String)| {
            let mut new_config = (*config).clone();
            if let Some(product_type) = new_config.product_types.get_mut(index) {
                product_type.name = name;
            }
            config.set(new_config);
        })
    };

    let on_product_type_weight_change = {
        let config = config.clone();
        Callback::from(move |(index, weight): (usize, f32)| {
            let mut new_config = (*config).clone();
            if let Some(product_type) = new_config.product_types.get_mut(index) {
                product_type.weight = weight;
            }
            config.set(new_config);
        })
    };
let config_panel_style = config_panel();
let config_panel_h2_style = config_panel_h2();
let config_section_style = config_section();
let config_section_h3_style = config_section_h3();
let form_group_style = form_group();
let form_group_label_style = form_group_label();
let form_group_input_number_style = form_group_input_number();
let form_group_input_range_style = form_group_input_range();
let form_group_span_style = form_group_span();
let product_type_config_style = product_type_config();
let product_type_config_button_style = product_type_config_button();

html! {
    <div class={config_panel_style}>
        <h2 class={config_panel_h2_style}>{"Configuration"}</h2>
        
        <div class={config_section_style}>
            <h3 class={config_section_h3_style}>{"Basic Settings"}</h3>
            <div class={form_group_style}>
                <label class={form_group_label_style} for="review-count">{"Review Count: "}</label>
                <input
                    type="number"
                    id="review-count"
                    value={config.review_count.to_string()}
                    onchange={on_review_count_change}
                    min="1"
                    class={form_group_input_number_style}
                />
            </div>
            
            <div class={form_group_style}>
                <label class={form_group_label_style} for="survey-response-rate">{"Survey Response Rate: "}</label>
                <input
                    type="range"
                    id="survey-response-rate"
                    min="0"
                    max="100"
                    value={(config.survey_response_rate * 100.0) as i32}
                    onchange={on_survey_response_rate_change}
                    class={form_group_input_range_style}
                />
                <span class={form_group_span_style}>{format!("{}%", (config.survey_response_rate * 100.0) as i32)}</span>
            </div>
        </div>
        
        <div class={config_section_style}>
            <h3 class={config_section_h3_style}>{"Rating Distribution"}</h3>
            <div class={form_group_style}>
                <label class={form_group_label_style} for="rating-mean">{"Mean: "}</label>
                <input
                    type="number"
                    id="rating-mean"
                    step="0.01"
                    min="0"
                    max="1"
                    value={config.rating_distribution.mean.to_string()}
                    onchange={on_rating_mean_change}
                    class={form_group_input_number_style}
                />
            </div>
            
            <div class={form_group_style}>
                <label class={form_group_label_style} for="rating-std-dev">{"Standard Deviation: "}</label>
                <input
                    type="number"
                    id="rating-std-dev"
                    step="0.01"
                    min="0"
                    max="1"
                    value={config.rating_distribution.std_dev.to_string()}
                    onchange={on_rating_std_dev_change}
                    class={form_group_input_number_style}
                />
            </div>
            
            <div class={form_group_style}>
                <label class={form_group_label_style} for="rating-min">{"Minimum: "}</label>
                <input
                    type="number"
                    id="rating-min"
                    step="0.01"
                    min="0"
                    max="1"
                    value={config.rating_distribution.min.to_string()}
                    onchange={on_rating_min_change}
                    class={form_group_input_number_style}
                />
            </div>
            
            <div class={form_group_style}>
                <label class={form_group_label_style} for="rating-max">{"Maximum: "}</label>
                <input
                    type="number"
                    id="rating-max"
                    step="0.01"
                    min="0"
                    max="1"
                    value={config.rating_distribution.max.to_string()}
                    onchange={on_rating_max_change}
                    class={form_group_input_number_style}
                />
            </div>
        </div>
        
        <div class={config_section_style}>
            <h3 class={config_section_h3_style}>{"Product Types"}
                <button onclick={on_add_product_type} class={product_type_config_button_style}>{"Add Product Type"}</button>
            </h3>
            {for config.product_types.iter().enumerate().map(|(index, product_type)| {
                let on_name_change = {
                    let on_product_type_name_change = on_product_type_name_change.clone();
                    Callback::from(move |e: Event| {
                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                        on_product_type_name_change.emit((index, input.value()));
                    })
                };
                
                let on_weight_change = {
                    let on_product_type_weight_change = on_product_type_weight_change.clone();
                    Callback::from(move |e: Event| {
                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                        let value = input.value().parse::<f32>().unwrap_or(0.1);
                        on_product_type_weight_change.emit((index, value));
                    })
                };
                
                let on_remove = {
                    let on_remove_product_type = on_remove_product_type.clone();
                    Callback::from(move |_| on_remove_product_type.emit(index))
                };
                
                html! {
                    <div class={product_type_config_style} key={index.to_string()}>
                        <div class={form_group_style}>
                            <label class={form_group_label_style} for={format!("product-name-{}", index)}>{"Name: "}</label>
                            <input
                                type="text"
                                id={format!("product-name-{}", index)}
                                value={product_type.name.clone()}
                                onchange={on_name_change}
                                class={form_group_input_text()}
                            />
                        </div>
                        
                        <div class={form_group_style}>
                            <label class={form_group_label_style} for={format!("product-weight-{}", index)}>{"Weight: "}</label>
                            <input
                                type="number"
                                id={format!("product-weight-{}", index)}
                                step="0.01"
                                min="0"
                                max="1"
                                value={product_type.weight.to_string()}
                                onchange={on_weight_change}
                                class={form_group_input_number_style}
                            />
                        </div>
                        
                        <button onclick={on_remove} class={product_type_config_button_style}>{"Remove"}</button>
                    </div>
                }
            })}
        </div>
    </div>
}
    }
}