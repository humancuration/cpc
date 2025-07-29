use yew::prelude::*;
use crate::types::Product;

#[derive(Properties, PartialEq, Clone)]
pub struct ProductHeaderProps {
    pub product: Product,
}

#[function_component(ProductHeader)]
pub fn product_header(props: &ProductHeaderProps) -> Html {
    html! {
        <div class="product-header">
            <h1>{&props.product.name}</h1>
            <h2>{&props.product.brand}</h2>
        </div>
    }
}