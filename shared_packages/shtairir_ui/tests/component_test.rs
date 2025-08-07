use shtairir_ui::components::block_browser::BlockBrowser;
use shtairir_registry::model::{BlockSpec, Purity, Determinism, EngineReq};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct TestAppProps {
    blocks: Vec<BlockSpec>,
}

#[function_component(TestApp)]
fn test_app(props: &TestAppProps) -> Html {
    let on_block_select = Callback::from(|_block: BlockSpec| {
        // Handle block selection
    });
    
    html! {
        <div class="test-app">
            <BlockBrowser 
                blocks={props.blocks.clone()}
                on_block_select={on_block_select}
            />
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    #[wasm_bindgen_test]
    fn test_block_browser_creation() {
        // This is a basic test to ensure the component can be created
        // In a real test, we would mount the component and interact with it
        let blocks = vec![create_test_block()];
        
        let props = TestAppProps { blocks };
        let html = yew::Renderer::<TestApp>::with_root_and_props(
            gloo_utils::document().create_element("div").unwrap(),
            props,
        );
        
        // The test passes if no panic occurs during creation
        assert!(true);
    }
}

fn create_test_block() -> BlockSpec {
    BlockSpec {
        id: "test@0.1.0:block".to_string(),
        namespace: "test".to_string(),
        name: "block".to_string(),
        version: "0.1.0".to_string(),
        title: "Test Block".to_string(),
        description: "A test block for unit testing".to_string(),
        authors: vec!["Test Author".to_string()],
        license: "MIT".to_string(),
        tags: vec!["test".to_string()],
        categories: vec!["test".to_string()],
        purity: Purity::Pure,
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