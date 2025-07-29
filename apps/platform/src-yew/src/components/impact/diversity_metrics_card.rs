use yew::prelude::*;
use crate::types::impact::DiversityMetricsData;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct DiversityMetricsCardProps {
    pub data: DiversityMetricsData,
}

#[function_component(DiversityMetricsCard)]
pub fn diversity_metrics_card(props: &DiversityMetricsCardProps) -> Html {
    let gender_percentage = props.data.gender_diversity * 100.0;
    let ethnic_percentage = props.data.ethnic_diversity * 100.0;
    let pay_equity_percentage = props.data.pay_equity * 100.0;

    html! {
        <div class="metric-card impact-card">
            <div class="metric-header">
                <h3 class="metric-title">{"Diversity & Inclusion"}</h3>
            </div>
            <div class="diversity-metrics">
                <div class="diversity-metric">
                    <div class="metric-label">{"Gender Balance"}</div>
                    <div class="progress-bar">
                        <div 
                            class="progress-fill" 
                            style={format!("width: {:.0}%", gender_percentage)}
                        />
                    </div>
                    <div class="metric-value">{format!("{:.1}%", gender_percentage)}</div>
                </div>
                
                <div class="diversity-metric">
                    <div class="metric-label">{"Ethnic Diversity"}</div>
                    <div class="progress-bar">
                        <div 
                            class="progress-fill" 
                            style={format!("width: {:.0}%", ethnic_percentage)}
                        />
                    </div>
                    <div class="metric-value">{format!("{:.1}%", ethnic_percentage)}</div>
                </div>
                
                <div class="diversity-metric">
                    <div class="metric-label">{"Pay Equity"}</div>
                    <div class="progress-bar">
                        <div 
                            class="progress-fill" 
                            style={format!("width: {:.0}%", pay_equity_percentage)}
                        />
                    </div>
                    <div class="metric-value">{format!("{:.1}%", pay_equity_percentage)}</div>
                </div>
            </div>
        </div>
    }
}