use anyhow::Result;
use std::path::PathBuf;
use shtairir_registry::model::Registry;
use shtairir_registry::loader::ModuleLoader;

#[test]
fn test_load_example_modules() -> Result<()> {
    // Create a registry and load modules from the examples directory
    let roots = vec![
        PathBuf::from("apps/shtairir_examples"),
    ];
    
    let reg = Registry::load(&roots)?;
    
    // Check that modules were loaded
    let modules = reg.list_modules();
    assert!(!modules.is_empty(), "No modules were loaded");
    
    // Check that the shtairir_examples module was loaded
    assert!(modules.contains(&"shtairir_examples".to_string()), 
        "shtairir_examples module not found in loaded modules: {:?}", modules);
    
    // Check that blocks were loaded
    let blocks = reg.list_blocks("shtairir_examples");
    assert!(!blocks.is_empty(), "No blocks were loaded from shtairir_examples module");
    
    // Check for specific expected blocks
    let expected_blocks = vec![
        "math.add",
        "string.concat", 
        "math.sum_list",
        "util.join_kv_pairs"
    ];
    
    for block_name in expected_blocks {
        assert!(blocks.contains(&block_name.to_string()), 
            "Block '{}' not found in loaded blocks: {:?}", block_name, blocks);
    }
    
    // Test version resolution
    if let Some(block_handle) = reg.find_block("shtairir_examples", "math.add", Some("^0.1")) {
        assert_eq!(block_handle.module, "shtairir_examples");
        assert_eq!(block_handle.spec.name, "math.add");
        assert_eq!(block_handle.spec.inputs.len(), 2);
        assert_eq!(block_handle.spec.outputs.len(), 1);
    } else {
        panic!("Failed to resolve math.add block with version requirement ^0.1");
    }
    
    Ok(())
}

#[test]
fn test_block_validation() -> Result<()> {
    let roots = vec![
        PathBuf::from("apps/shtairir_examples"),
    ];
    
    let reg = Registry::load(&roots)?;
    
    // Test that all loaded blocks are valid
    for (key, block_handle) in &reg.blocks {
        // Basic validation
        assert!(!block_handle.module.is_empty(), "Module name is empty for block {}", key);
        assert!(!block_handle.version.is_empty(), "Version is empty for block {}", key);
        assert!(!block_handle.spec.name.is_empty(), "Block name is empty for block {}", key);
        assert!(!block_handle.spec.inputs.is_empty(), "Block {} has no inputs", key);
        assert!(!block_handle.spec.outputs.is_empty(), "Block {} has no outputs", key);
        
        // Type validation (all types should be valid)
        for input in &block_handle.spec.inputs {
            // This shouldn't panic if the type is valid
            let _ = shtairir_registry::types::Type::parse(&input.ty)
                .unwrap_or_else(|_| panic!("Invalid type '{}' for input port '{}' in block {}", input.ty, input.name, key));
        }
        
        for output in &block_handle.spec.outputs {
            // This shouldn't panic if the type is valid
            let _ = shtairir_registry::types::Type::parse(&output.ty)
                .unwrap_or_else(|_| panic!("Invalid type '{}' for output port '{}' in block {}", output.ty, output.name, key));
        }
        
        for param in &block_handle.spec.params {
            // This shouldn't panic if the type is valid
            let _ = shtairir_registry::types::Type::parse(&param.ty)
                .unwrap_or_else(|_| panic!("Invalid type '{}' for param '{}' in block {}", param.ty, param.name, key));
        }
    }
    
    Ok(())
}

#[test]
fn test_graph_loading() -> Result<()> {
    let roots = vec![
        PathBuf::from("apps/shtairir_examples"),
    ];
    
    let reg = Registry::load(&roots)?;
    
    // Check if graphs were loaded
    let graphs = reg.list_graphs();
    println!("Loaded graphs: {:?}", graphs);
    
    // If there are graphs, test their validation
    for (key, graph_handle) in &reg.graphs {
        // Basic validation
        assert!(!graph_handle.module.is_empty(), "Module name is empty for graph {}", key);
        assert!(!graph_handle.version.is_empty(), "Version is empty for graph {}", key);
        assert!(!graph_handle.spec.name.is_empty(), "Graph name is empty for graph {}", key);
        assert_eq!(graph_handle.spec.schema_version, "0.2", "Graph {} has wrong schema version", key);
        
        // Node validation
        for node in &graph_handle.spec.nodes {
            assert!(!node.id.is_empty(), "Node ID is empty in graph {}", key);
            
            // Port validation
            for input in &node.inputs {
                // This shouldn't panic if the type is valid
                let _ = shtairir_registry::types::Type::parse(&input.ty)
                    .unwrap_or_else(|_| panic!("Invalid type '{}' for input port '{}' in node {} of graph {}", input.ty, input.name, node.id, key));
            }
            
            for output in &node.outputs {
                // This shouldn't panic if the type is valid
                let _ = shtairir_registry::types::Type::parse(&output.ty)
                    .unwrap_or_else(|_| panic!("Invalid type '{}' for output port '{}' in node {} of graph {}", output.ty, output.name, node.id, key));
            }
        }
        
        // Edge validation
        for edge in &graph_handle.spec.edges {
            assert!(!edge.id.is_empty(), "Edge ID is empty in graph {}", key);
            assert!(!edge.from.node.is_empty(), "Edge from node is empty in graph {}", key);
            assert!(!edge.from.port.is_empty(), "Edge from port is empty in graph {}", key);
            assert!(!edge.to.node.is_empty(), "Edge to node is empty in graph {}", key);
            assert!(!edge.to.port.is_empty(), "Edge to port is empty in graph {}", key);
        }
    }
    
    Ok(())
}

#[test]
fn test_module_loader_directly() -> Result<()> {
    let loader = ModuleLoader::new();
    let roots = vec![
        PathBuf::from("apps/shtairir_examples"),
    ];
    
    let modules = loader.load_modules(&roots)?;
    
    // Check that modules were loaded
    assert!(!modules.is_empty(), "No modules were loaded by ModuleLoader");
    
    // Check that the shtairir_examples module was loaded
    let example_module = modules.iter()
        .find(|m| m.name == "shtairir_examples")
        .expect("shtairir_examples module not found");
    
    // Check module properties
    assert_eq!(example_module.version, "0.1.0");
    assert_eq!(example_module.title, "Shtairir Examples");
    assert!(example_module.description.contains("Example blocks"));
    
    // Check that blocks were loaded
    assert!(!example_module.blocks.is_empty(), "No blocks were loaded in shtairir_examples module");
    
    // Check for specific expected blocks
    let expected_blocks = vec![
        "math.add",
        "string.concat", 
        "math.sum_list",
        "util.join_kv_pairs"
    ];
    
    for block_name in expected_blocks {
        assert!(example_module.blocks.iter().any(|b| b.name == block_name), 
            "Block '{}' not found in loaded blocks", block_name);
    }
    
    Ok(())
}