//! Example usage of the shtairir_ui components

use shtairir_ui::components::BlockBrowser;
use shtairir_registry::model::{BlockSpec, Purity, Determinism, EngineReq};
use yew::prelude::*;

#[function_component(ExampleApp)]
fn example_app() -> Html {
    // Create some sample blocks for demonstration
    let blocks = vec![
        create_sample_block("math", "add", "Add Numbers", "Add two numbers together", vec!["math", "arithmetic"], Purity::Pure),
        create_sample_block("text", "concat", "Concatenate Strings", "Join two strings together", vec!["text", "string"], Purity::Pure),
        create_sample_block("io", "read_file", "Read File", "Read data from a file", vec!["io", "file"], Purity::Effect),
    ];
    
    let on_block_select = Callback::from(|block: BlockSpec| {
        web_sys::console::log_1(&format!("Selected block: {}", block.name).into());
    });
    
    html! {
        <div class="example-app">
            <h1>{"Shtairir UI Components Example"}</h1>
            <div class="example-content">
                <BlockBrowser 
                    blocks={blocks}
                    on_block_select={on_block_select}
                />
            </div>
        </div>
    }
}

fn create_sample_block(
    namespace: &str,
    name: &str,
    title: &str,
    description: &str,
    categories: Vec<&str>,
    purity: Purity,
) -> BlockSpec {
    BlockSpec {
        id: format!("{}@0.1.0:{}", namespace, name),
        namespace: namespace.to_string(),
        name: name.to_string(),
        version: "0.1.0".to_string(),
        title: title.to_string(),
        description: description.to_string(),
        authors: vec!["Example Author".to_string()],
        license: "MIT".to_string(),
        tags: categories.iter().map(|s| s.to_string()).collect(),
        categories: categories.iter().map(|s| s.to_string()).collect(),
        purity,
        effects: vec![],
        determinism: Determinism::Deterministic,
        generics: vec![],
        inputs: vec![],
        outputs: vec![],
        params: vec![],
        examples: vec![],
        tests: vec![],
        engine: EngineReq {
            version_req: "^0.2".to_string(),
            capability_flags: vec![],
        },
        integrity: None,
        metadata: None,
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<ExampleApp>::new().render();
}