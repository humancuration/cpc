use yew::prelude::*;
use yew_router::prelude::*;
use crate::forum::community_browser::ui::CommunityBrowser;
use cpc_core::invoicing::components::invoice_list::InvoiceList;
use cpc_core::invoicing::components::invoice_detail::InvoiceDetail;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/c/:slug")]
    Community { slug: String },
    #[at("/invoices")]
    Invoices,
    #[at("/invoices/:id")]
    Invoice { id: String },
}

// temp component
#[function_component(CommunityHome)]
fn community_home(props: &CommunityHomeProps) -> Html {
    html! {
        <div>
            <h1>{ format!("Welcome to {}", &props.slug) }</h1>
            <p>{ "Community landing page placeholder." }</p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CommunityHomeProps {
    pub slug: String,
}

pub fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <CommunityBrowser /> },
        AppRoute::Community { slug } => html! { <CommunityHome slug={slug} /> },
        AppRoute::Invoices => html! { <InvoiceList /> },
        AppRoute::Invoice { id } => html! { <InvoiceDetail invoice_id={id} /> },
    }
}