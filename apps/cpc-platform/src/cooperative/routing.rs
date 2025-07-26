use yew::prelude::*;
use yew_router::prelude::*;

use crate::cooperative::list::CooperativeListComponent;
use crate::cooperative::detail::CooperativeDetail;
use crate::cooperative::form::CooperativeFormComponent;
use uuid::Uuid;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum CooperativeRoute {
    #[at("/cooperatives")]
    List,
    #[at("/cooperatives/new")]
    Create,
    #[at("/cooperatives/:id")]
    Detail { id: Uuid },
    #[at("/cooperatives/:id/edit")]
    Edit { id: String },
}

pub fn switch_cooperative(route: CooperativeRoute) -> Html {
    match route {
        CooperativeRoute::List => html! { <CooperativeListComponent /> },
        CooperativeRoute::Create => html! { <CooperativeFormComponent /> },
        CooperativeRoute::Detail { id } => html!{ <CooperativeDetail id={id} /> },
        CooperativeRoute::Edit { id } => html!{ <CooperativeFormComponent cooperative_id={Some(id)} /> },
    }
}