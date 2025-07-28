//! Visual consent status indicators.

use yew::prelude::*;
use crate::domain::consent::{Domain, DataSharingLevel};

/// Properties for the consent indicator
#[derive(Properties, PartialEq)]
pub struct ConsentIndicatorProps {
    /// The domain for which to show consent status
    pub domain: Domain,
    /// Current consent level
    pub level: DataSharingLevel,
    /// Optional size modifier
    #[prop_or_default]
    pub small: bool,
}

/// Get color class based on consent level
fn level_color_class(level: &DataSharingLevel) -> &'static str {
    match level {
        DataSharingLevel::None => "consent-none",
        DataSharingLevel::Minimal => "consent-minimal",
        DataSharingLevel::Standard => "consent-standard",
        DataSharingLevel::Full => "consent-full",
    }
}

/// Get aria label based on consent level
fn aria_label(domain: &Domain, level: &DataSharingLevel) -> String {
    let domain_name = match domain {
        Domain::FinancialData => "Financial Data",
        Domain::HealthData => "Health Data",
        Domain::CalendarData => "Calendar Data",
        Domain::CrmData => "CRM Data",
        Domain::ScmData => "SCM Data",
        Domain::DocumentData => "Document Data",
        Domain::WebsiteData => "Website Data",
        Domain::RecruitmentData => "Recruitment Data",
        Domain::DataLakehouse => "Data Lakehouse",
        Domain::ForecastingData => "Forecasting Data",
    };
    
    let level_description = match level {
        DataSharingLevel::None => "no data sharing",
        DataSharingLevel::Minimal => "minimal data sharing",
        DataSharingLevel::Standard => "standard data sharing",
        DataSharingLevel::Full => "full data sharing",
    };
    
    format!("{}: {}", domain_name, level_description)
}

/// Visual consent status indicator component
#[function_component(ConsentIndicator)]
pub fn consent_indicator(props: &ConsentIndicatorProps) -> Html {
    let size_class = if props.small { "small" } else { "" };
    let color_class = level_color_class(&props.level);
    let aria_label = aria_label(&props.domain, &props.level);
    
    html! {
        <span
            class={classes!("consent-indicator", size_class, color_class)}
            aria-label={aria_label}
            role="img"
            title={aria_label.clone()}
        >
            match &props.level {
                DataSharingLevel::None => {"🚫"},
                DataSharingLevel::Minimal => {"🟡"},
                DataSharingLevel::Standard => {"🟢"},
                DataSharingLevel::Full => {"🔵"},
            }
        </span>
    }
}