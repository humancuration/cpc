//! Action bar component with generate, export, and reset buttons

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ActionBarProps {
    pub on_generate: Callback<()>,
    pub on_reset: Callback<()>,
    pub on_export: Callback<()>,
    pub generating: bool,
    pub has_data: bool,
}

#[function_component(ActionBar)]
pub fn action_bar(props: &ActionBarProps) -> Html {
    let on_generate = props.on_generate.clone();
    let on_reset = props.on_reset.clone();
    let on_export = props.on_export.clone();
    let generating = props.generating;
    let has_data = props.has_data;

    let on_generate_click = Callback::from(move |_| on_generate.emit(()));
    let on_reset_click = Callback::from(move |_| on_reset.emit(()));
    let on_export_click = Callback::from(move |_| on_export.emit(()));

    html! {
        <div class="action-bar">
            <button
                onclick={on_generate_click}
                disabled={generating}
                class="generate-btn"
            >
                {if generating { "Generating..." } else { "Generate Data" }}
            </button>
            
            <button
                onclick={on_export_click}
                disabled={!has_data}
                class="export-btn"
            >
                {"Export Data"}
            </button>
            
            <button onclick={on_reset_click} class="reset-btn">
                {"Reset to Default"}
            </button>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_action_bar_render() {
        let on_generate = Callback::from(|_| ());
        let on_reset = Callback::from(|_| ());
        let on_export = Callback::from(|_| ());
        
        let props = ActionBarProps {
            on_generate,
            on_reset,
            on_export,
            generating: false,
            has_data: true,
        };
        
        let html = yew::Renderer::<ActionBar>::with_props(props).render();
        assert!(html.contains("Generate Data"));
        assert!(html.contains("Export Data"));
        assert!(html.contains("Reset to Default"));
    }
}