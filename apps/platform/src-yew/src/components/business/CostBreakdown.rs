use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CostBreakdownProps {
    pub material_cost: f64,
    pub labor_cost: f64,
    pub price: f64,
}

#[function_component(CostBreakdown)]
pub fn cost_breakdown(props: &CostBreakdownProps) -> Html {
    let total_cost = props.material_cost + props.labor_cost;
    let profit = props.price - total_cost;
    let profit_margin = (profit / props.price) * 100.0;

    html! {
        <div class="cost-breakdown">
            <h4>{"Cost Breakdown"}</h4>
            <table>
                <tr>
                    <td>{"Material Cost:"}</td>
                    <td>{format!("${:.2}", props.material_cost)}</td>
                </tr>
                <tr>
                    <td>{"Labor Cost:"}</td>
                    <td>{format!("${:.2}", props.labor_cost)}</td>
                </tr>
                <tr>
                    <td>{"Total Cost:"}</td>
                    <td>{format!("${:.2}", total_cost)}</td>
                </tr>
                <tr>
                    <td>{"Selling Price:"}</td>
                    <td>{format!("${:.2}", props.price)}</td>
                </tr>
                <tr>
                    <td>{"Profit:"}</td>
                    <td class={if profit > 0.0 {"profit-positive"} else {"profit-negative"}}>
                        {format!("${:.2}", profit)}
                    </td>
                </tr>
                <tr>
                    <td>{"Profit Margin:"}</td>
                    <td class={if profit_margin > 0.0 {"profit-positive"} else {"profit-negative"}}>
                        {format!("{:.1}%", profit_margin)}
                    </td>
                </tr>
            </table>
        </div>
    }
}