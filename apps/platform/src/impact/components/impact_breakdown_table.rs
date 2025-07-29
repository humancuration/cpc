use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
use crate::api::impact::get_impact_report;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ImpactBreakdownTableProps {
    pub breakdown: Vec<get_impact_report::GetImpactReportBreakdown>,
    pub threshold: f64,
}

#[function_component(ImpactBreakdownTable)]
pub fn impact_breakdown_table(props: &ImpactBreakdownTableProps) -> Html {
    // Calculate total impact score for percentage calculation
    let total: f64 = props.breakdown.iter().map(|item| item.impact_score).sum();
    
    html! {
        <div>
            <h2>{ "Impact Breakdown" }</h2>
            <table>
                <thead>
                    <tr>
                        <th>{ "Category" }</th>
                        <th>{ "Item Name" }</th>
                        <th>{ "Contribution" }</th>
                        <th>{ "Impact Score" }</th>
                        <th>{ "Percentage" }</th>
                    </tr>
                </thead>
                <tbody>
                    { for props.breakdown.iter().map(|item| {
                        let percentage = if total > 0.0 {
                            (item.impact_score / total) * 100.0
                        } else {
                            0.0
                        };
                        let is_above_threshold = percentage > props.threshold * 100.0;
                        
                        html! {
                            <tr class={if is_above_threshold { "above-threshold" } else { "" }}>
                                <td>{ &item.category }</td>
                                <td>{ &item.item_name }</td>
                                <td>{ &item.contribution }</td>
                                <td>{ item.impact_score }</td>
                                <td>{ format!("{:.2}%", percentage) }</td>
                            </tr>
                        }
                    })}
                </tbody>
            </table>
        </div>
    }
}