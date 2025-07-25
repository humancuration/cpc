use yew::prelude::*;
use cpc_core::accounting::Money;
use crate::types::TrendData;
use crate::components::common::trend_indicator::TrendIndicator;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MetricCardProps {
    pub title: String,
    pub value: Money,
    pub trend: Option<TrendData>,
    pub icon: Option<String>,
    pub format: Format,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Format {
    Currency,
    Percentage,
    Number,
}

#[function_component(MetricCard)]
pub fn metric_card(props: &MetricCardProps) -> Html {
    let formatted_value = match props.format {
        Format::Currency => format_currency(&props.value),
        Format::Percentage => format_percentage(props.value.amount),
        Format::Number => format_number(props.value.amount),
    };

    let trend_indicator = props.trend.as_ref().map(|trend| {
        html! {
            <TrendIndicator 
                direction={trend.direction.clone()} 
                percentage={trend.percentage} 
            />
        }
    });

    html! {
        <div class="metric-card">
            <div class="metric-header">
                <h3 class="metric-title">{&props.title}</h3>
                {trend_indicator}
            </div>
            <div class="metric-value">{formatted_value}</div>
            if let Some(icon) = &props.icon {
                <div class="metric-icon">{icon}</div>
            }
        </div>
    }
}

fn format_currency(money: &Money) -> String {
    format!("${:.2}", money.amount)
}

fn format_percentage(value: f64) -> String {
    format!("{:.1}%", value)
}

fn format_number(value: f64) -> String {
    format!("{:.0}", value)
}