use yew::prelude::*;
use crate::types::impact::CarbonFootprintData;
use crate::types::TrendDirection;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CarbonFootprintCardProps {
    pub data: CarbonFootprintData,
}

#[function_component(CarbonFootprintCard)]
pub fn carbon_footprint_card(props: &CarbonFootprintCardProps) -> Html {
    let net_footprint = props.data.net_footprint;
    let trend_direction = if props.data.trend > 0.0 {
        TrendDirection::Up
    } else if props.data.trend < 0.0 {
        TrendDirection::Down
    } else {
        TrendDirection::Neutral
    };

    html! {
        <div class="metric-card impact-card">
            <div class="metric-header">
                <h3 class="metric-title">{"Carbon Footprint"}</h3>
                <div class={classes!("trend-indicator", match trend_direction {
                    TrendDirection::Up => "text-red",
                    TrendDirection::Down => "text-green",
                    TrendDirection::Neutral => "text-gray",
                })}>
                    <span class="trend-percentage">
                        {format!("{:.1}%", props.data.trend.abs())}
                    </span>
                </div>
            </div>
            <div class="metric-value">
                {format!("{:.1} tCO₂e", net_footprint)}
            </div>
            <div class="carbon-details">
                <div class="carbon-item">
                    <span class="carbon-label">{"Emissions:"}</span>
                    <span class="carbon-value">{format!("{:.1} tCO₂e", props.data.total_emissions)}</span>
                </div>
                <div class="carbon-item">
                    <span class="carbon-label">{"Sequestered:"}</span>
                    <span class="carbon-value">{format!("{:.1} tCO₂e", props.data.total_sequestered)}</span>
                </div>
            </div>
        </div>
    }
}