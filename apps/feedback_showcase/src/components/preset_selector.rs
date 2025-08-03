//! Preset selector component for choosing predefined data configurations

use yew::prelude::*;
use crate::data_generator::config::{DataGeneratorConfig, RatingDistributionConfig, DemographicConfig, ProductTypeConfig};

#[derive(Properties, PartialEq)]
pub struct PresetSelectorProps {
    pub on_preset_selected: Callback<DataGeneratorConfig>,
}

#[function_component(PresetSelector)]
pub fn preset_selector(props: &PresetSelectorProps) -> Html {
    let on_preset_selected = props.on_preset_selected.clone();
    
    let presets = vec![
        ("Balanced Reviews", create_balanced_config()),
        ("Positive Reviews", create_positive_config()),
        ("Negative Reviews", create_negative_config()),
        ("Mixed Sentiment", create_mixed_config()),
    ];
    
    let on_preset_click = move |config: DataGeneratorConfig| {
        let on_preset_selected = on_preset_selected.clone();
        Callback::from(move |_| {
            on_preset_selected.emit(config.clone());
        })
    };
    
    html! {
        <div class="preset-selector">
            <h3>{"Data Presets"}</h3>
            <div class="preset-buttons">
                {for presets.into_iter().map(|(name, config)| {
                    let callback = on_preset_click(config);
                    html! {
                        <button onclick={callback} class="preset-btn">
                            {name}
                        </button>
                    }
                })}
            </div>
        </div>
    }
}

fn create_balanced_config() -> DataGeneratorConfig {
    DataGeneratorConfig {
        review_count: 100,
        survey_response_rate: 0.8,
        rating_distribution: RatingDistributionConfig {
            mean: 0.75,
            std_dev: 0.15,
            min: 0.0,
            max: 1.0,
        },
        demographic_distribution: create_default_demographic_distribution(),
        product_types: create_default_product_types(),
    }
}

fn create_positive_config() -> DataGeneratorConfig {
    DataGeneratorConfig {
        review_count: 100,
        survey_response_rate: 0.8,
        rating_distribution: RatingDistributionConfig {
            mean: 0.85,
            std_dev: 0.1,
            min: 0.3,
            max: 1.0,
        },
        demographic_distribution: create_default_demographic_distribution(),
        product_types: create_default_product_types(),
    }
}

fn create_negative_config() -> DataGeneratorConfig {
    DataGeneratorConfig {
        review_count: 100,
        survey_response_rate: 0.8,
        rating_distribution: RatingDistributionConfig {
            mean: 0.3,
            std_dev: 0.15,
            min: 0.0,
            max: 0.7,
        },
        demographic_distribution: create_default_demographic_distribution(),
        product_types: create_default_product_types(),
    }
}

fn create_mixed_config() -> DataGeneratorConfig {
    DataGeneratorConfig {
        review_count: 100,
        survey_response_rate: 0.8,
        rating_distribution: RatingDistributionConfig {
            mean: 0.65,
            std_dev: 0.25,
            min: 0.0,
            max: 1.0,
        },
        demographic_distribution: create_default_demographic_distribution(),
        product_types: create_default_product_types(),
    }
}

fn create_default_demographic_distribution() -> DemographicConfig {
    DemographicConfig {
        age_groups: vec![
            ("18-24".to_string(), 0.2),
            ("25-34".to_string(), 0.3),
            ("35-44".to_string(), 0.25),
            ("45-54".to_string(), 0.15),
            ("55+".to_string(), 0.1),
        ],
        genders: vec![
            ("male".to_string(), 0.4),
            ("female".to_string(), 0.4),
            ("non-binary".to_string(), 0.1),
            ("other".to_string(), 0.1),
        ],
        locations: vec![
            ("New York, NY".to_string(), 0.15),
            ("Los Angeles, CA".to_string(), 0.12),
            ("Chicago, IL".to_string(), 0.08),
            ("Houston, TX".to_string(), 0.07),
            ("Phoenix, AZ".to_string(), 0.05),
            ("Philadelphia, PA".to_string(), 0.05),
            ("San Antonio, TX".to_string(), 0.04),
            ("San Diego, CA".to_string(), 0.04),
            ("Dallas, TX".to_string(), 0.04),
            ("San Jose, CA".to_string(), 0.03),
        ],
        occupations: vec![
            ("Software Engineer".to_string(), 0.15),
            ("Teacher".to_string(), 0.1),
            ("Healthcare Worker".to_string(), 0.1),
            ("Student".to_string(), 0.15),
            ("Sales Representative".to_string(), 0.08),
            ("Manager".to_string(), 0.08),
            ("Designer".to_string(), 0.07),
            ("Engineer".to_string(), 0.07),
            ("Marketing Specialist".to_string(), 0.05),
            ("Other".to_string(), 0.15),
        ],
    }
}

fn create_default_product_types() -> Vec<ProductTypeConfig> {
    vec![
        ProductTypeConfig {
            name: "Water Bottle".to_string(),
            description: "A reusable water bottle designed for everyday use.".to_string(),
            weight: 0.3,
            common_attributes: vec![
                ("material".to_string(), "stainless steel".to_string()),
                ("capacity".to_string(), "24 oz".to_string()),
            ],
        },
        ProductTypeConfig {
            name: "Backpack".to_string(),
            description: "A durable backpack for daily use or travel.".to_string(),
            weight: 0.2,
            common_attributes: vec![
                ("material".to_string(), "nylon".to_string()),
                ("compartments".to_string(), "3".to_string()),
            ],
        },
        ProductTypeConfig {
            name: "Headphones".to_string(),
            description: "High-quality headphones for music and calls.".to_string(),
            weight: 0.2,
            common_attributes: vec![
                ("type".to_string(), "wireless".to_string()),
                ("battery_life".to_string(), "20 hours".to_string()),
            ],
        },
        ProductTypeConfig {
            name: "Fitness Tracker".to_string(),
            description: "A wearable device to track your fitness activities.".to_string(),
            weight: 0.15,
            common_attributes: vec![
                ("water_resistant".to_string(), "true".to_string()),
                ("display".to_string(), "color".to_string()),
            ],
        },
        ProductTypeConfig {
            name: "Coffee Maker".to_string(),
            description: "A programmable coffee maker for your kitchen.".to_string(),
            weight: 0.15,
            common_attributes: vec![
                ("capacity".to_string(), "12 cups".to_string()),
                ("programmable".to_string(), "true".to_string()),
            ],
        },
    ]
}