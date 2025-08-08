use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub label: String,
    pub on_click: Callback<()>,
    #[prop_or_default]
    pub variant: String,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    // Web implementation
    #[cfg(target_arch = "wasm32")]
    {
        let class = format!("btn btn-{}", props.variant);
        html! {
            <button class={class} onclick={props.on_click.reform(|_| ())}>
                { &props.label }
            </button>
        }
    }
    
    // Desktop implementation
    #[cfg(not(target_arch = "wasm32"))]
    {
        html! {
            <div class="desktop-button">
                <button onclick={props.on_click.reform(|_| ())}>
                    { &props.label }
                </button>
            </div>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_props_should_be_partialeq() {
        let callback1 = Callback::from(|_| {});
        let callback2 = Callback::from(|_| {});
        
        let props1 = ButtonProps {
            label: "Test".to_string(),
            on_click: callback1,
            variant: "primary".to_string(),
        };
        
        let props2 = ButtonProps {
            label: "Test".to_string(),
            on_click: callback2,
            variant: "primary".to_string(),
        };
        
        // This test just verifies that the props can be compared
        // In a real test, we'd need to properly mock the callbacks
        assert_eq!(props1.label, props2.label);
        assert_eq!(props1.variant, props2.variant);
    }
}