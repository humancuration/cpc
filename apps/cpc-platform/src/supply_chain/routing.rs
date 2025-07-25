use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum SupplyChainRoute {
    #[at("/supply-chain")]
    List,
    #[at("/supply-chain/:id")]
    Detail { id: String },
    #[at("/supply-chain/:id/edit")]
    Edit { id: String },
}