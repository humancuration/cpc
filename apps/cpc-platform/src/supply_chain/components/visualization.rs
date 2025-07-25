use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SupplyChainVisualizationProps {
    pub product_id: String,
}

#[function_component(SupplyChainVisualization)]
pub fn supply_chain_visualization(props: &SupplyChainVisualizationProps) -> Html {
    html! {
        <div>
            <h2 class="text-xl">{ "Supply Chain Graph" }</h2>
            <p>{ "Visualization will be implemented here for product: "}{ &props.product_id }</p>
            // This will be a graphical representation
        </div>
    }
}