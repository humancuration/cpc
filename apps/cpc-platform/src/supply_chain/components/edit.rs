use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SupplyChainEditProps {
    pub id: String,
}

#[function_component(SupplyChainEdit)]
pub fn supply_chain_edit(props: &SupplyChainEditProps) -> Html {
    html! {
        <div>
            <h1 class="text-2xl">{ format!("Edit Supply Chain for Product {}", &props.id) }</h1>
            // Form will go here
        </div>
    }
}