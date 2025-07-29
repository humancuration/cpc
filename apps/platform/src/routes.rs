use yew_router::prelude::*;
use crate::cooperative::routing::{switch_cooperative, CooperativeRoute};
use crate::impact::routing::{switch_impact, ImpactRoute};
use crate::project::routing::{switch_project, ProjectRoute};
use crate::supply_chain::routing::{switch_supply_chain, SupplyChainRoute};
use crate::components::home::Home;
use crate::components::discovery::DiscoveryPage;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/discover")]
    Discovery,
    #[nest("/")]
    SupplyChain(SupplyChainRoute),
    #[nest("/")]
    Cooperative(CooperativeRoute),
    #[nest("/")]
    Project(ProjectRoute),
    #[nest("/impact")]
    Impact(ImpactRoute),
}

pub fn switch(routes: AppRoute) -> yew::Html {
    match routes {
        AppRoute::Home => yew::html! { <Home /> },
        AppRoute::Discovery => yew::html! { <DiscoveryPage /> },
        AppRoute::SupplyChain(route) => switch_supply_chain(route),
        AppRoute::Cooperative(route) => switch_cooperative(route),
        AppRoute::Project(route) => switch_project(route),
        AppRoute::Impact(route) => switch_impact(route),
    }
}