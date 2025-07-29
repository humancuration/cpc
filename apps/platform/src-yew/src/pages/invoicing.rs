use yew::prelude::*;
use crate::components::invoicing::InvoiceForm;

#[function_component(InvoicingPage)]
pub fn invoicing_page() -> Html {
    html! {
        <InvoiceForm />
    }
}