use yew::prelude::*;
use yew_router::prelude::*;
use uuid::Uuid;

use crate::supply_chain::components::{
    list::SupplyChainList,
    detail::SupplyChainDetail,
    edit::SupplyChainEdit,
    create_stage::CreateStageComponent,
    stage_list::StageListComponent,
};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum SupplyChainRoute {
    #[at("/supply-chain")]
    List,
    #[at("/supply-chain/:id")]
    Detail { id: String },
    #[at("/supply-chain/:id/edit")]
    Edit { id: String },
    #[at("/supply-chain/:product_id/stages/create")]
    CreateStage { product_id: String },
    #[at("/supply-chain/products/:product_id/stages")]
    ProductStages { product_id: String },
    #[at("/supply-chain/stages/:stage_id/edit")]
    EditStage { stage_id: String },
    #[at("/supply-chain/stages/:stage_id")]
    StageDetail { stage_id: String },
}

pub fn switch_supply_chain(route: SupplyChainRoute) -> Html {
    match route {
        SupplyChainRoute::List => html! { <SupplyChainList /> },
        SupplyChainRoute::Detail { id } => html!{ <SupplyChainDetail product_id={id} /> },
        SupplyChainRoute::Edit { id } => html!{ <SupplyChainEdit product_id={id} /> },
        SupplyChainRoute::CreateStage { product_id } => html! { <CreateStageComponent product_id={product_id} /> },
        SupplyChainRoute::ProductStages { product_id } => html! { <StageListComponent product_id={product_id} /> },
        SupplyChainRoute::EditStage { stage_id } => html! { <EditStageComponent stage_id={stage_id} /> },
        SupplyChainRoute::StageDetail { stage_id } => html! { <StageDetailComponent stage_id={stage_id} /> },
    }
}