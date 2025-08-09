//! Reusable responsive container component
//!
//! This module provides a responsive container that handles
//! different screen sizes for web and desktop applications.

use crate::components::base::{BaseComponent, CommonProps};
use crate::hooks::{use_breakpoint, Breakpoint};
use yew::prelude::*;
use stylist::{style, yew::styled_component};

/// Properties for the Container component
#[derive(Properties, PartialEq, Clone)]
pub struct ContainerProps {
    /// Common properties
    #[prop_or_default]
    pub common: CommonProps,
    
    /// Content of the container
    #[prop_or_default]
    pub children: Children,
    
    /// Maximum width of the container
    #[prop_or_default]
    pub max_width: Option<ContainerMaxWidth>,
    
    /// Whether the container is fluid (takes full width)
    #[prop_or_default]
    pub fluid: bool,
    
    /// Padding for the container
    #[prop_or_default]
    pub padding: Option<ContainerPadding>,
}

/// Container maximum width options
#[derive(PartialEq, Clone, Debug)]
pub enum ContainerMaxWidth {
    /// Small maximum width (540px)
    Small,
    /// Medium maximum width (720px)
    Medium,
    /// Large maximum width (960px)
    Large,
    /// Extra large maximum width (1140px)
    XLarge,
    /// Extra extra large maximum width (1320px)
    XXLarge,
}

/// Container padding options
#[derive(PartialEq, Clone, Debug)]
pub enum ContainerPadding {
    /// No padding
    None,
    /// Small padding
    Small,
    /// Medium padding (default)
    Medium,
    /// Large padding
    Large,
}

impl Default for ContainerPadding {
    fn default() -> Self {
        Self::Medium
    }
}

/// A responsive container component
#[styled_component(Container)]
pub struct Container {
    props: ContainerProps,
}

impl BaseComponent for Container {
    type Properties = ContainerProps;
    
    fn create(props: &Self::Properties) -> Self {
        Self { props: props.clone() }
    }
    
    fn update_props(&mut self, props: Self::Properties) {
        self.props = props;
    }
    
    fn view(&self) -> Html {
        let breakpoint = use_breakpoint();
        let padding_value = match self.props.padding.as_ref().unwrap_or(&ContainerPadding::Medium) {
            ContainerPadding::None => "0",
            ContainerPadding::Small => "var(--cpc-spacing-sm)",
            ContainerPadding::Medium => "var(--cpc-spacing-md)",
            ContainerPadding::Large => "var(--cpc-spacing-lg)",
        };
        
        let max_width = if self.props.fluid {
            "100%".to_string()
        } else {
            match &self.props.max_width {
                Some(ContainerMaxWidth::Small) => "var(--cpc-breakpoint-sm)".to_string(),
                Some(ContainerMaxWidth::Medium) => "var(--cpc-breakpoint-md)".to_string(),
                Some(ContainerMaxWidth::Large) => "var(--cpc-breakpoint-lg)".to_string(),
                Some(ContainerMaxWidth::XLarge) => "var(--cpc-breakpoint-xl)".to_string(),
                Some(ContainerMaxWidth::XXLarge) => "var(--cpc-breakpoint-xxl)".to_string(),
                None => match breakpoint {
                    Breakpoint::XS | Breakpoint::SM => "100%".to_string(),
                    Breakpoint::MD => "var(--cpc-breakpoint-md)".to_string(),
                    Breakpoint::LG => "var(--cpc-breakpoint-lg)".to_string(),
                    Breakpoint::XL => "var(--cpc-breakpoint-xl)".to_string(),
                    Breakpoint::XXL => "var(--cpc-breakpoint-xxl)".to_string(),
                },
            }
        };
        
        let container_style = style!(
            r#"
            width: 100%;
            padding-right: {};
            padding-left: {};
            margin-right: auto;
            margin-left: auto;
            
            @media (min-width: 576px) {{
                max-width: {};
            }}
            
            @media (min-width: 768px) {{
                max-width: {};
            }}
            
            @media (min-width: 992px) {{
                max-width: {};
            }}
            
            @media (min-width: 1200px) {{
                max-width: {};
            }}
            
            @media (min-width: 1400px) {{
                max-width: {};
            }}
        "#,
            padding_value,
            padding_value,
            max_width,
            max_width,
            max_width,
            max_width,
            max_width
        );
        
        let classes = classes!(
            container_style.get_class_name(),
            self.props.common.class.clone()
        );
        
        html! {
            <div
                id={self.props.common.id.clone()}
                class={classes}
                style={self.props.common.style.clone()}
            >
                { for self.props.children.iter() }
            </div>
        }
    }
}

impl Container {
    /// Create a new container component
    pub fn new(props: ContainerProps) -> Self {
        Self::create(&props)
    }
}