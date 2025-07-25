use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SustainabilityBadgeProps {
    pub carbon_footprint: f64,
    pub packaging_type: String,
}

#[function_component(SustainabilityBadge)]
pub fn sustainability_badge(props: &SustainabilityBadgeProps) -> Html {
    let sustainability_class = if props.carbon_footprint < 100.0 {
        "sustainability-good"
    } else if props.carbon_footprint < 500.0 {
        "sustainability-medium"
    } else {
        "sustainability-poor"
    };

    html! {
        <div class={classes!("sustainability-badge", sustainability_class)}>
            <div class="carbon-footprint">
                <strong>{"CO₂: "}</strong>
                {format!("{:.1}g", props.carbon_footprint)}
            </div>
            <div class="packaging">
                <strong>{"Packaging: "}</strong>
                {&props.packaging_type}
            </div>
        </div>
    }
}