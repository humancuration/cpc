use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ToolbarProps {
    pub on_bold: Callback<()>,
    pub on_italic: Callback<()>,
    pub on_underline: Callback<()>,
    pub on_insert_image: Callback<()>,
}

#[function_component(Toolbar)]
pub fn toolbar(props: &ToolbarProps) -> Html {
    let on_bold = props.on_bold.clone();
    let on_italic = props.on_italic.clone();
    let on_underline = props.on_underline.clone();
    let on_insert_image = props.on_insert_image.clone();
    
    html! {
        <div class="toolbar">
            <button onclick={move |_| on_bold.emit(())}>
                <strong>{"B"}</strong>
            </button>
            <button onclick={move |_| on_italic.emit(())}>
                <em>{"I"}</em>
            </button>
            <button onclick={move |_| on_underline.emit(())}>
                <u>{"U"}</u>
            </button>
            <button onclick={move |_| on_insert_image.emit(())}>
                {"Insert Image"}
            </button>
        </div>
    }
}