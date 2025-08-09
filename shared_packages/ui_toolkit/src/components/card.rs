//! Reusable card component with header and footer slots
//!
//! This module provides a flexible card component that can be
//! used throughout applications.

use crate::components::base::{BaseComponent, CommonProps};
use yew::prelude::*;
use stylist::{style, yew::styled_component};

/// Properties for the Card component
#[derive(Properties, PartialEq, Clone)]
pub struct CardProps {
    /// Common properties
    #[prop_or_default]
    pub common: CommonProps,
    
    /// Content to display in the header
    #[prop_or_default]
    pub header: Option<Html>,
    
    /// Main content of the card
    #[prop_or_default]
    pub children: Children,
    
    /// Content to display in the footer
    #[prop_or_default]
    pub footer: Option<Html>,
    
    /// Whether the card has a border
    #[prop_or_default]
    pub bordered: bool,
    
    /// Whether the card has a shadow
    #[prop_or_default]
    pub shadow: bool,
    
    /// Size of the card
    #[prop_or_default]
    pub size: CardSize,
}

/// Card size options
#[derive(PartialEq, Clone, Debug)]
pub enum CardSize {
    /// Small card with compact padding
    Small,
    /// Medium card (default)
    Medium,
    /// Large card with more padding
    Large,
}

impl Default for CardSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// A reusable card component with header and footer slots
#[styled_component(Card)]
pub struct Card {
    props: CardProps,
}

impl BaseComponent for Card {
    type Properties = CardProps;
    
    fn create(props: &Self::Properties) -> Self {
        Self { props: props.clone() }
    }
    
    fn update_props(&mut self, props: Self::Properties) {
        self.props = props;
    }
    
    fn view(&self) -> Html {
        let padding = match self.props.size {
            CardSize::Small => "var(--cpc-spacing-md)",
            CardSize::Medium => "var(--cpc-spacing-lg)",
            CardSize::Large => "var(--cpc-spacing-xl)",
        };
        
        let border_style = if self.props.bordered {
            "1px solid var(--cpc-gray-300)"
        } else {
            "none"
        };
        
        let shadow_style = if self.props.shadow {
            "var(--cpc-shadow-sm)"
        } else {
            "none"
        };
        
        let card_style = style!(
            r#"
            background: var(--cpc-surface);
            border: {};
            border-radius: var(--cpc-border-radius-md);
            box-shadow: {};
            transition: transform 0.2s, box-shadow 0.2s;
            
            &:hover {
                transform: translateY(-2px);
                box-shadow: var(--cpc-shadow-md);
            }
        "#,
            border_style,
            shadow_style
        );
        
        let content_style = style!(
            r#"
            padding: {};
            flex-grow: 1;
            display: flex;
            flex-direction: column;
        "#,
            padding
        );
        
        let header_style = style!(
            r#"
            padding: {} {};
            border-bottom: 1px solid var(--cpc-gray-200);
            margin: 0;
            font-size: var(--cpc-font-size-lg);
            font-weight: var(--cpc-font-weight-medium);
        "#,
            padding,
            padding
        );
        
        let footer_style = style!(
            r#"
            padding: {} {};
            border-top: 1px solid var(--cpc-gray-200);
            margin: 0;
        "#,
            padding,
            padding
        );
        
        let classes = classes!(
            card_style.get_class_name(),
            self.props.common.class.clone()
        );
        
        html! {
            <div
                id={self.props.common.id.clone()}
                class={classes}
                style={self.props.common.style.clone()}
            >
                if let Some(header) = &self.props.header {
                    <div class={header_style.get_class_name()}>
                        { header.clone() }
                    </div>
                }
                <div class={content_style.get_class_name()}>
                    { for self.props.children.iter() }
                </div>
                if let Some(footer) = &self.props.footer {
                    <div class={footer_style.get_class_name()}>
                        { footer.clone() }
                    </div>
                }
            </div>
        }
    }
}

impl Card {
    /// Create a new card component
    pub fn new(props: CardProps) -> Self {
        Self::create(&props)
    }
}