use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DocumentEditorProps {
    pub document_id: String,
}

#[function_component(DocumentEditor)]
pub fn document_editor(props: &DocumentEditorProps) -> Html {
    let document_content = use_state(|| String::from("Start typing your document here..."));
    
    let oninput = {
        let document_content = document_content.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            document_content.set(input.value());
        })
    };
    
    html! {
        <div class="document-editor">
            <h1>{"Document Editor"}</h1>
            <textarea
                class="editor-textarea"
                value={(*document_content).clone()}
                oninput={oninput}
                rows="20"
                cols="80"
            />
            <div class="editor-actions">
                <button>{"Save"}</button>
                <button>{"Export"}</button>
            </div>
        </div>
    }
}