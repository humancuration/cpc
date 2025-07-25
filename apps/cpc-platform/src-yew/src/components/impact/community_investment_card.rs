use yew::prelude::*;
use crate::types::impact::CommunityInvestmentData;
use crate::types::TrendDirection;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CommunityInvestmentCardProps {
    pub data: CommunityInvestmentData,
}

#[function_component(CommunityInvestmentCard)]
pub fn community_investment_card(props: &CommunityInvestmentCardProps) -> Html {
    let trend_direction = if props.data.trend > 0.0 {
        TrendDirection::Up
    } else if props.data.trend < 0.0 {
        TrendDirection::Down
    } else {
        TrendDirection::Neutral
    };

    let trend_icon = match trend_direction {
        TrendDirection::Up => "↑",
        TrendDirection::Down => "↓",
        TrendDirection::Neutral => "→",
    };

    html! {
        <div class="metric-card impact-card">
            <div class="metric-header">
                <h3 class="metric-title">{"Community Investment"}</h3>
                <div class={classes!("trend-indicator", match trend_direction {
                    TrendDirection::Up => "text-green",
                    TrendDirection::Down => "text-red",
                    TrendDirection::Neutral => "text-gray",
                })}>
                    <span class="trend-icon">{trend_icon}</span>
                    <span class="trend-percentage">
                        {format!("{:.1}%", props.data.trend.abs())}
                    </span>
                </div>
            </div>
            <div class="metric-value">
                {format!("${:.0}K", props.data.total_amount / 1000.0)}
            </div>
            <div class="investment-details">
                <div class="investment-item">
                    <span class="investment-label">{"Beneficiaries:"}</span>
                    <span class="investment-value">{props.data.beneficiaries}</span>
                </div>
                <div class="investment-per-person">
                    {format!("${:.2} per person", 
                        if props.data.beneficiaries > 0 {
                            props.data.total_amount / props.data.beneficiaries as f64
                        } else {
                            0.0
                        }
                    )}
                </div>
            </div>
        </div>
    }
}