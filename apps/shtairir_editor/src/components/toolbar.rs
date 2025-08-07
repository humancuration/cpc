use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ToolbarProps {
    pub on_export_toml: Callback<()>,
    pub on_import_toml: Callback<()>,
}

#[function_component(Toolbar)]
pub fn toolbar(props: &ToolbarProps) -> Html {
    html! {
        <div class="editor-toolbar">
            <div class="toolbar-group">
                <h3>{"File"}</h3>
                <button onclick={props.on_export_toml.clone()} class="toolbar-btn">
                    {"Export TOML"}
                </button>
                <button onclick={props.on_import_toml.clone()} class="toolbar-btn">
                    {"Import TOML"}
                </button>
            </div>
            
            <div class="toolbar-group">
                <h3>{"Edit"}</h3>
                <button class="toolbar-btn">
                    {"Undo"}
                </button>
                <button class="toolbar-btn">
                    {"Redo"}
                </button>
            </div>
            
            <div class="toolbar-group">
                <h3>{"View"}</h3>
                <button class="toolbar-btn">
                    {"Auto-layout"}
                </button>
                <button class="toolbar-btn">
                    {"Performance Metrics"}
                </button>
            </div>
            
            <div class="toolbar-group">
                <h3>{"Help"}</h3>
                <button class="toolbar-btn">
                    {"Documentation"}
                </button>
                <button class="toolbar-btn">
                    {"Tutorials"}
                </button>
            </div>
        </div>
    }
}