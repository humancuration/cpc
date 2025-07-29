use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use uuid::Uuid;

use crate::api::cooperative::{cooperatives, list_cooperatives};

#[derive(Properties, PartialEq)]
pub struct CooperativeSelectorProps {
    pub on_select: Callback<Option<Uuid>>,
}

#[function_component(CooperativeSelector)]
pub fn cooperative_selector(props: &CooperativeSelectorProps) -> Html {
    let cooperatives_state = use_state(|| None::<Vec<cooperatives::CooperativesCooperatives>>);
    let error_state = use_state(|| None::<String>);

    {
        let cooperatives_state = cooperatives_state.clone();
        let error_state = error_state.clone();
        use_effect_with((), move |_| {
            let cooperatives_state = cooperatives_state.clone();
            let error_state = error_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match list_cooperatives().await {
                    Ok(mut fetched_cooperatives) => {
                        fetched_cooperatives.sort_by(|a, b| a.name.cmp(&b.name));
                        cooperatives_state.set(Some(fetched_cooperatives));
                    }
                    Err(e) => {
                        error_state.set(Some(e.to_string()));
                    }
                }
            });
            || ()
        });
    }

    let onchange = {
        let on_select = props.on_select.clone();
        Callback::from(move |e: Event| {
            let target = e.target().and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(select) = target {
                let value = select.value();
                if value.is_empty() {
                    on_select.emit(None);
                } else {
                    match Uuid::parse_str(&value) {
                        Ok(id) => on_select.emit(Some(id)),
                        Err(_) => on_select.emit(None),
                    }
                }
            }
        })
    };

    html! {
        <div class="field">
            <label class="label">{ "Cooperative" }</label>
            <div class="control">
                <div class="select">
                    <select onchange={onchange}>
                        <option value="" selected=true disabled=true>{ "Select a Cooperative..." }</option>
                        {
                            if let Some(cooperatives) = &*cooperatives_state {
                                cooperatives.iter().map(|cooperative| {
                                    html! {
                                        <option value={cooperative.id.clone()}>{ cooperative.name.clone() }</option>
                                    }
                                }).collect::<Html>()
                            } else {
                                html! {}
                            }
                        }
                    </select>
                </div>
            </div>
            if let Some(error) = &*error_state {
                <p class="has-text-danger">{ error }</p>
            }
        </div>
    }
}