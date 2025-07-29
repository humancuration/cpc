use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::home::HomePage;
use crate::pages::product::scan::ProductScanPage;
use crate::pages::product::details::ProductDetails;
use crate::pages::invoicing::InvoicingPage;
use crate::pages::discovery::DiscoveryPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/invoices/new")]
    NewInvoice,
    #[at("/product/scan")]
    ProductScan,
    #[at("/product/:id")]
    ProductDetails { id: String },
    #[at("/discovery")]
    Discovery,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::NewInvoice => html! { <InvoicingPage /> },
        Route::ProductScan => html! { <ProductScanPage /> },
        Route::ProductDetails { id } => html! { <ProductDetails id={id} /> },
        Route::Discovery => html! { <DiscoveryPage /> },
        Route::NotFound => html! { <h1>{"404 Not Found"}</h1> },
    }
}