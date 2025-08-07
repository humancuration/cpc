use yew::prelude::*;
use crate::components::{BlockLibrary, Canvas, PropertiesPanel, Toolbar};
use crate::models::{Graph, Node, Connection};
use crate::registry::RegistryManager;
use crate::serializer::Serializer;
use crate::validator::Validator;

#[function_component(App)]
pub fn app() -> Html {
    let graph = use_state(|| Graph::new());
    let selected_node = use_state(|| Option::<Node>::None);
    let registry = use_state(|| RegistryManager::new());
    let show_properties = use_state(|| false);

    let on_node_select = {
        let selected_node = selected_node.clone();
        let show_properties = show_properties.clone();
        Callback::from(move |node: Node| {
            selected_node.set(Some(node));
            show_properties.set(true);
        })
    };

    let on_node_update = {
        let graph = graph.clone();
        let selected_node = selected_node.clone();
        Callback::from(move |updated_node: Node| {
            let mut new_graph = (*graph).clone();
            new_graph.add_node(updated_node.clone());
            graph.set(new_graph);
            selected_node.set(Some(updated_node));
        })
    };

    let on_graph_update = {
        let graph = graph.clone();
        Callback::from(move |new_graph: Graph| {
            graph.set(new_graph);
        })
    };

    let on_add_node = {
        let graph = graph.clone();
        let registry = registry.clone();
        Callback::from(move |block_spec: String| {
            let mut new_graph = (*graph).clone();
            // TODO: Implement node creation from block spec
            graph.set(new_graph);
        })
    };

    let on_export_toml = {
        let graph = graph.clone();
        Callback::from(move |_| {
            // TODO: Implement TOML export
            web_sys::console::log_1(&"Exporting to TOML".into());
        })
    };

    let on_import_toml = Callback::from(move |_| {
        // TODO: Implement TOML import
        web_sys::console::log_1(&"Importing from TOML".into());
    });

    html! {
        <div class="shtairir-editor">
            <style>
                {include_str!("../assets/style.css")}
            </style>
            
            <header class="editor-header">
                <h1>{"Shtairir Visual Editor"}</h1>
                <p>{"Visual programming for the CPC platform"}</p>
            </header>

            <Toolbar 
                on_export_toml={on_export_toml}
                on_import_toml={on_import_toml}
            />

            <div class="editor-main">
                <BlockLibrary 
                    registry={(*registry).clone()}
                    on_add_node={on_add_node}
                />
                
                <Canvas 
                    graph={(*graph).clone()}
                    on_graph_update={on_graph_update}
                    on_node_select={on_node_select}
                />
                
                if *show_properties {
                    <PropertiesPanel 
                        node={selected_node.as_ref().cloned()}
                        on_node_update={on_node_update}
                        on_close={Callback::from(move |_| show_properties.set(false))}
                    />
                }
            </div>

            <footer class="editor-footer">
                <p>{"Shtairir Visual Editor - CPC Platform"}</p>
            </footer>
        </div>
    }
}