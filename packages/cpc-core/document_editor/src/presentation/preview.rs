use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PreviewProps {
    pub document_content: String,
}

#[function_component(Preview)]
pub fn preview(props: &PreviewProps) -> Html {
    html! {
        <div class="document-preview">
            <h2>{"Document Preview"}</h2>
            <div class="preview-content">
                { Html::from_html_unchecked(props.document_content.clone().into()) }
            </div>
        </div>
    }
}