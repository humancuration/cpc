use yew::prelude::*;
use crate::types::TrendDirection;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TrendIndicatorProps {
    pub direction: TrendDirection,
    pub percentage: f64,
}

#[function_component(TrendIndicator)]
pub fn trend_indicator(props: &TrendIndicatorProps) -> Html {
    let (icon_class, text_class) = match props.direction {
        TrendDirection::Up => ("trend-up", "text-green"),
        TrendDirection::Down => ("trend-down", "text-red"),
        TrendDirection::Neutral => ("trend-neutral", "text-gray"),
    };

    let icon = match props.direction {
        TrendDirection::Up => "↑",
        TrendDirection::Down => "↓",
        TrendDirection::Neutral => "→",
    };

    html! {
        <div class={classes!("trend-indicator", text_class)}>
            <span class={classes!("trend-icon", icon_class)}>{icon}</span>
            <span class="trend-percentage">{format!("{:.1}%", props.percentage)}</span>
        </div>
    }
}