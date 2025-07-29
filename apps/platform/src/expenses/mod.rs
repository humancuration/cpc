pub mod components;

use yew::prelude::*;
use yew_router::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::expenses::components::{
    create::CreateExpense,
    list::ExpenseList,
    detail::ExpenseDetail,
    edit::EditExpense,
};

#[derive(Clone, Routable, PartialEq)]
pub enum ExpenseRoute {
    #[at("/expenses")]
    List,
    #[at("/expenses/new")]
    Create,
    #[at("/expenses/:id")]
    Detail { id: String },
    #[at("/expenses/:id/edit")]
    Edit { id: String },
}

pub fn switch(route: ExpenseRoute) -> Html {
    match route {
        ExpenseRoute::List => html! { <ExpenseList /> },
        ExpenseRoute::Create => html! { <CreateExpense /> },
        ExpenseRoute::Detail { id } => html! { <ExpenseDetail id={id} /> },
        ExpenseRoute::Edit { id } => html! { <EditExpense id={id} /> },
    }
}