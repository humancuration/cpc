use yew::prelude::*;
use yew_router::prelude::*;
use crate::impact::components::impact_page::ImpactPage;

#[derive(Clone, Routable, PartialEq)]
pub enum ImpactRoute {
    #[at("/impact")]
    Report,
}

pub fn switch_impact(routes: ImpactRoute) -> Html {
    match routes {
        ImpactRoute::Report => html! { <ImpactPage /> },
    }
}