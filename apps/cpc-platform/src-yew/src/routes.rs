use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::home::HomePage;
use crate::pages::product::scan::ProductScanPage;
use crate::pages::product::details::ProductDetails;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/product/scan")]
    ProductScan,
    #[at("/product/:id")]
    ProductDetails { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::ProductScan => html! { <ProductScanPage /> },
        Route::ProductDetails { id } => html! { <ProductDetails id={id} /> },
        Route::NotFound => html! { <h1>{"404 Not Found"}</h1> },
    }
}