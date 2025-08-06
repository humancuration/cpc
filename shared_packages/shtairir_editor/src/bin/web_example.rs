use shtairir_editor::{VisualEditor, Graph};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let graph = use_state(|| Graph::new());
    
    let on_graph_change = {
        let graph = graph.clone();
        Callback::from(move |new_graph| {
            graph.set(new_graph);
        })
    };
    
    html! {
        <div class="app">
            <h1>{"Shtairir Visual Editor"}</h1>
            <VisualEditor 
                graph={(*graph).clone()} 
                on_graph_change={on_graph_change} 
            />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}