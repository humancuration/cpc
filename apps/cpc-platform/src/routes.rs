use yew_router::prelude::*;
use crate::supply_chain::routing::SupplyChainRoute;
use crate::supply_chain::components::{
    list::SupplyChainList,
    detail::SupplyChainDetail,
    edit::SupplyChainEdit,
};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[nest("/")]
    SupplyChain(SupplyChainRoute),
}

pub fn switch(routes: AppRoute) -> yew::Html {
    match routes {
        AppRoute::Home => yew::html! { <h1>{ "Home" }</h1> },
        AppRoute::SupplyChain(route) => match route {
            SupplyChainRoute::List => yew::html! { <SupplyChainList /> },
            SupplyChainRoute::Detail { id } => yew::html! { <SupplyChainDetail id={id} /> },
            SupplyChainRoute::Edit { id } => yew::html! { <SupplyChainEdit id={id} /> },
        },
    }
}