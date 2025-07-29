use chrono::NaiveDate;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::*;
use uuid::Uuid;
use web_sys::HtmlInputElement;

use crate::api::cooperative::{
    create_cooperative, get_cooperative, update_cooperative, CreateCooperativeInput,
    UpdateCooperativeInput,
};
use crate::cooperative::routing::CooperativeRoute;

#[derive(Properties, PartialEq, Clone)]
pub struct CooperativeFormProps {
    #[prop_or_default]
    pub cooperative_id: Option<String>,
}

#[function_component(CooperativeFormComponent)]
pub fn cooperative_form_component(props: &CooperativeFormProps) -> Html {
    let name_state = use_state(String::new);
    let description_state = use_state(String::new);
    let founded_date_state = use_state(String::new);
    let website_state = use_state(String::new);
    let error_state = use_state(|| None::<String>);
    let navigator = use_navigator().unwrap();

    let props_clone = props.clone();

    // Fetch existing cooperative data if in edit mode
    let get_cooperative_handle = {
        let cooperative_id_for_fetch = props.cooperative_id.clone();
        use_async(async move {
            if let Some(id_str) = cooperative_id_for_fetch {
                match Uuid::parse_str(&id_str) {
                    Ok(id) => get_cooperative(id).await.map_err(|e| e.to_string()),
                    Err(_) => Err("Invalid cooperative ID".to_string()),
                }
            } else {
                // This is not an error, just means we are in "create" mode.
                // We return Ok(Ok(None)) to signify success with no data.
                Ok(Ok(None))
            }
        })
    };

    // Populate form fields when data is fetched
    {
        let name_state = name_state.clone();
        let description_state = description_state.clone();
        let founded_date_state = founded_date_state.clone();
        let website_state = website_state.clone();
        let get_cooperative_handle = get_cooperative_handle.clone();

        use_effect_with(
            get_cooperative_handle.data,
            move |data| {
                if let Some(Ok(Some(cooperative))) = data {
                    name_state.set(cooperative.name.clone());
                    description_state.set(cooperative.description.clone().unwrap_or_default());
                    if let Some(date_str) = &cooperative.founded_date {
                        if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                             founded_date_state.set(date.format("%Y-%m-%d").to_string());
                        }
                    }
                    website_state.set(cooperative.website.clone().unwrap_or_default());
                }
            },
        );
    }

    // Run the fetch hook when the component mounts if in edit mode
    use_effect_with((), move |_| {
        if props_clone.cooperative_id.is_some() {
            get_cooperative_handle.run();
        }
        || ()
    });

    // Handle form submission for both create and update
    let onsubmit = {
        let name_state = name_state.clone();
        let description_state = description_state.clone();
        let founded_date_state = founded_date_state.clone();
        let website_state = website_state.clone();
        let error_state = error_state.clone();
        let navigator = navigator.clone();
        let cooperative_id = props.cooperative_id.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            error_state.set(None);

            let name = (*name_state).clone();
            let description = (*description_state).clone();
            let founded_date_str = (*founded_date_state).clone();
            let website = (*website_state).clone();

            if name.is_empty() {
                error_state.set(Some("Name is required".to_string()));
                return;
            }

            let founded_date = match NaiveDate::parse_from_str(&founded_date_str, "%Y-%m-%d") {
                Ok(date) => date,
                Err(_) => {
                    error_state.set(Some("Invalid date format. Use YYYY-MM-DD.".to_string()));
                    return;
                }
            };
            
            let navigator = navigator.clone();
            let error_state = error_state.clone();
            let cooperative_id = cooperative_id.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Some(id_str) = cooperative_id {
                    // Update existing cooperative
                     match Uuid::parse_str(&id_str) {
                        Ok(id) => {
                            let input = UpdateCooperativeInput {
                                id,
                                name: Some(name),
                                description: Some(description),
                                website: Some(website),
                            };
                            match update_cooperative(input).await {
                                Ok(_) => navigator.push(&CooperativeRoute::List),
                                Err(e) => error_state.set(Some(e.to_string())),
                            }
                        }
                        Err(_) => error_state.set(Some("Invalid cooperative ID".to_string())),
                    }
                } else {
                    // Create new cooperative
                    let input = CreateCooperativeInput {
                        name,
                        description: Some(description),
                        founded_date,
                        website: Some(website),
                    };
                    match create_cooperative(input).await {
                        Ok(response) => {
                            let new_id = response.create_cooperative.id;
                            navigator.push(&CooperativeRoute::Edit { id: new_id });
                        }
                        Err(e) => error_state.set(Some(e.to_string())),
                    }
                }
            });
        })
    };
    
    // Change handlers for form inputs
    let on_name_change = {
        let name_state = name_state.clone();
        Callback::from(move |e: Event| {
            let target = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = target {
                name_state.set(input.value());
            }
        })
    };
    let on_description_change = {
        let description_state = description_state.clone();
        Callback::from(move |e: Event| {
            let target = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = target {
                description_state.set(input.value());
            }
        })
    };
    let on_founded_date_change = {
        let founded_date_state = founded_date_state.clone();
        Callback::from(move |e: Event| {
            let target = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = target {
                founded_date_state.set(input.value());
            }
        })
    };
    let on_website_change = {
        let website_state = website_state.clone();
        Callback::from(move |e: Event| {
            let target = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = target {
                website_state.set(input.value());
            }
        })
    };

    html! {
        <div class="container">
            <h1 class="title">
                { if props.cooperative_id.is_some() { "Edit Cooperative" } else { "Create Cooperative" } }
            </h1>

            if get_cooperative_handle.loading {
                <p>{ "Loading..." }</p>
            } else {
                <form onsubmit={onsubmit}>
                    <div class="field">
                        <label class="label">{ "Name" }</label>
                        <div class="control">
                            <input class="input" type="text" value={(*name_state).clone()} onchange={on_name_change} required=true />
                        </div>
                    </div>

                    <div class="field">
                        <label class="label">{ "Description" }</label>
                        <div class="control">
                            <textarea class="textarea" value={(*description_state).clone()} onchange={on_description_change}></textarea>
                        </div>
                    </div>

                    <div class="field">
                        <label class="label">{ "Founded Date" }</label>
                        <div class="control">
                            <input class="input" type="date" value={(*founded_date_state).clone()} onchange={on_founded_date_change} required=true />
                        </div>
                    </div>
                    
                    <div class="field">
                        <label class="label">{ "Website" }</label>
                        <div class="control">
                            <input class="input" type="url" value={(*website_state).clone()} onchange={on_website_change} />
                        </div>
                    </div>

                    if let Some(error) = &*error_state {
                        <p class="has-text-danger">{ error }</p>
                    }
                    if let Some(error) = get_cooperative_handle.error.as_ref() {
                        <p class="has-text-danger">{ error }</p>
                    }
                    
                    <div class="field is-grouped">
                        <div class="control">
                            <button class="button is-primary" type="submit">
                                { "Save" }
                            </button>
                        </div>
                        <div class="control">
                            <Link<CooperativeRoute> to={CooperativeRoute::List} classes="button is-light">
                                { "Cancel" }
                            </Link<CooperativeRoute>>
                        </div>
                    </div>
                </form>
            }
        </div>
    }
}