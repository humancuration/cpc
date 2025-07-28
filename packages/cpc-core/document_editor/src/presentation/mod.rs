pub mod editor;
pub mod toolbar;
pub mod preview;

use yew::prelude::*;
use crate::presentation::editor::DocumentEditor;
use crate::presentation::toolbar::Toolbar;
use crate::presentation::preview::Preview;

#[derive(Properties, PartialEq)]
pub struct DocumentEditorAppProps {
    pub document_id: String,
}

#[function_component(DocumentEditorApp)]
pub fn document_editor_app(props: &DocumentEditorAppProps) -> Html {
    let document_content = use_state(|| String::from("<p>Start typing your document here...</p>"));
    
    let on_bold = Callback::from(|_| {
        // Handle bold formatting
    });
    
    let on_italic = Callback::from(|_| {
        // Handle italic formatting
    });
    
    let on_underline = Callback::from(|_| {
        // Handle underline formatting
    });
    
    let on_insert_image = Callback::from(|_| {
        // Handle image insertion
    });
    
    html! {
        <div class="document-editor-app">
            <Toolbar
                on_bold={on_bold}
                on_italic={on_italic}
                on_underline={on_underline}
                on_insert_image={on_insert_image}
            />
            <div class="editor-container">
                <DocumentEditor document_id={props.document_id.clone()} />
                <Preview document_content={(*document_content).clone()} />
            </div>
        </div>
    }
}