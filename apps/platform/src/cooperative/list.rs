use yew::prelude::*;
use yew_router::prelude::*;
use crate::api::cooperative::{list_cooperatives, cooperatives};
use crate::cooperative::routing::CooperativeRoute;

#[function_component(CooperativeListComponent)]
pub fn cooperative_list_component() -> Html {
    let cooperatives = use_state(|| None);
    let error = use_state(|| None);

    {
        let cooperatives = cooperatives.clone();
        let error = error.clone();
        use_effect_with((), move |_| {
            let cooperatives = cooperatives.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match list_cooperatives().await {
                    Ok(mut fetched_cooperatives) => {
                        fetched_cooperatives.sort_by(|a, b| a.name.cmp(&b.name));
                        cooperatives.set(Some(fetched_cooperatives));
                    }
                    Err(e) => {
                        error.set(Some(e.to_string()));
                    }
                }
            });
            || ()
        });
    }

    html! {
        <div class="cooperative-list-container">
            <h1>{ "Cooperatives" }</h1>
            <Link<CooperativeRoute> to={CooperativeRoute::Create}>
                <button>{ "Create New Cooperative" }</button>
            </Link<CooperativeRoute>>
            {
                if let Some(error) = &*error {
                    html! { <p class="error-message">{ format!("Error: {}", error) }</p> }
                } else if let Some(cooperatives) = &*cooperatives {
                    html! {
                        <ul class="cooperative-list">
                            { for cooperatives.iter().map(|cooperative| html! {
                                <li key={cooperative.id.clone()} class="cooperative-item">
                                    {
                                        if let Ok(id) = cooperative.id.parse::<uuid::Uuid>() {
                                            html! {
                                                <Link<CooperativeRoute> to={CooperativeRoute::Detail { id }}>
                                                    <h2>{ &cooperative.name }</h2>
                                                </Link<CooperativeRoute>>
                                            }
                                        } else {
                                            html! { <h2>{ &cooperative.name }</h2> }
                                        }
                                    }
                                    <p>{ cooperative.description.as_deref().unwrap_or("No description available") }</p>
                                </li>
                            })}
                        </ul>
                    }
                } else {
                    html! { <p>{ "Loading cooperatives..." }</p> }
                }
            }
        </div>
    }
}