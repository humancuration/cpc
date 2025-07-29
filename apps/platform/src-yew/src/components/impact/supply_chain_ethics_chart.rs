use yew::prelude::*;
use crate::types::impact::SupplyChainData;
use crate::components::common::chart::{Chart, ChartType};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct SupplyChainEthicsChartProps {
    pub data: SupplyChainData,
}

#[function_component(SupplyChainEthicsChart)]
pub fn supply_chain_ethics_chart(props: &SupplyChainEthicsChartProps) -> Html {
    let chart_data = vec![
        ("Ethics Score".to_string(), props.data.ethics_score),
        ("Local Suppliers".to_string(), props.data.local_suppliers_percentage),
    ];

    let score_color = if props.data.ethics_score >= 80.0 {
        "text-green"
    } else if props.data.ethics_score >= 60.0 {
        "text-yellow"
    } else {
        "text-red"
    };

    html! {
        <div class="metric-card impact-card">
            <div class="metric-header">
                <h3 class="metric-title">{"Supply Chain Ethics"}</h3>
            </div>
            
            <div class="supply-chain-summary">
                <div class="score-display">
                    <span class={classes!("score-value", score_color)}>
                        {format!("{:.0}", props.data.ethics_score)}
                    </span>
                    <span class="score-label">{"Overall Score"}</span>
                </div>
                
                <div class="supply-stats">
                    <div class="stat-item">
                        <span class="stat-label">{"Total Suppliers:"}</span>
                        <span class="stat-value">{props.data.supplier_count}</span>
                    </div>
                    <div class="stat-item">
                        <span class="stat-label">{"Local Suppliers:"}</span>
                        <span class="stat-value">
                            {format!("{:.1}%", props.data.local_suppliers_percentage)}
                        </span>
                    </div>
                </div>
            </div>

            <Chart 
                chart_type={ChartType::Bar}
                data={chart_data}
                title={"Supply Chain Metrics"}
                width={300}
                height={200}
            />
        </div>
    }
}