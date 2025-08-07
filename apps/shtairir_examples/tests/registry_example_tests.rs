use std::path::PathBuf;

use shtairir_registry::model::Registry;

fn load_examples_registry() -> anyhow::Result<Registry> {
    let roots = vec![PathBuf::from("apps/shtairir_examples")];
    Registry::load(&roots)
}

#[test]
fn module_and_block_identifiers_are_valid() {
    let reg = load_examples_registry().expect("registry should load");
    // If load succeeded, identifier validation for module and blocks passed.
    // Additionally, ensure our module shows up and has expected blocks.
    let modules = reg.list_modules();
    assert!(modules.contains(&"examples.shtairir".to_string()));
    let blocks = reg.list_blocks("examples.shtairir");
    assert!(blocks.contains(&"math.add".to_string()));
    assert!(blocks.contains(&"string.concat".to_string()));
    assert!(blocks.contains(&"util.join_kv_pairs".to_string()));
    assert!(blocks.contains(&"math.sum_list".to_string()));
}

#[test]
fn v0_2_block_fields_are_present() {
    let reg = load_examples_registry().expect("registry should load");
    let module = "examples.shtairir";
    
    for block_name in ["math.add", "string.concat", "math.sum_list", "util.join_kv_pairs"] {
        let handle = reg.find_block(module, block_name, Some("^0.2"))
            .expect(&format!("Block {} should resolve with ^0.2", block_name));
        
        let spec = &handle.spec;
        
        // Check v0.2 fields
        assert!(!spec.id.is_empty(), "Block {} should have an ID", block_name);
        assert!(!spec.namespace.is_empty(), "Block {} should have a namespace", block_name);
        assert_eq!(spec.version, "0.2.0", "Block {} should have version 0.2.0", block_name);
        assert!(!spec.authors.is_empty(), "Block {} should have authors", block_name);
        assert!(!spec.license.is_empty(), "Block {} should have a license", block_name);
        assert!(!spec.tags.is_empty(), "Block {} should have tags", block_name);
        assert_eq!(spec.purity, shtairir_registry::model::Purity::Pure, 
                   "Block {} should be pure", block_name);
        assert!(spec.engine.capability_flags.contains(&"serde".to_string()), 
                "Block {} should have serde capability", block_name);
        assert!(spec.integrity.is_some(), "Block {} should have integrity section", block_name);
    }
}

#[test]
fn v0_2_graph_is_loaded() {
    let reg = load_examples_registry().expect("registry should load");
    let module = "examples.shtairir";
    
    // Check that graphs are loaded
    let graphs = reg.module_graph_names(module, "0.2.0");
    assert!(graphs.contains(&"stream_sum".to_string()));
    
    // Try to get the graph handle
    let graph_handle = reg.graphs.get("examples.shtairir@0.2.0:stream_sum")
        .expect("Graph should be loaded");
    
    let spec = &graph_handle.spec;
    assert_eq!(spec.schema_version, "0.2");
    assert_eq!(spec.namespace, "examples.shtairir");
    assert_eq!(spec.name, "stream_sum");
    assert_eq!(spec.version, "0.2.0");
    assert!(spec.integrity.is_some());
}

#[test]
fn composite_list_type_and_default() {
    let reg = load_examples_registry().expect("registry should load");
    let m = "examples.shtairir";
    // Ensure block exists and resolves with caret
    let sum = reg
        .find_block(m, "math.sum_list", Some("^0.2"))
        .expect("math.sum_list resolved");
    // Validate declared type spelling and that default [] was accepted for list<i64>
    let val_input = sum
        .spec
        .inputs
        .iter()
        .find(|p| p.name == "values")
        .expect("values input");
    assert_eq!(val_input.ty, "list<i64>");
    // default must exist and be an empty array
    let def = val_input
        .default
        .as_ref()
        .expect("default present for values");
    assert!(def.is_array(), "default should be JSON array");
    assert_eq!(def.as_array().unwrap().len(), 0, "default should be []");
}

#[test]
fn type_and_default_compatibility() {
    let reg = load_examples_registry().expect("registry should load");
    // string.concat has param separator: option<string> with default null
    // math.add has param bias: i64 default 0; input b has default 0
    let m = "examples.shtairir";

    let add = reg
        .find_block(m, "math.add", Some("^0.2"))
        .expect("math.add resolved");
    // Validate types we declared exist and defaults were accepted by validator (implied by successful load).
    // Spot-check by reading back the spec fields of interest:
    let bias_param = add
        .spec
        .params
        .iter()
        .find(|p| p.name == "bias")
        .expect("bias param");
    assert_eq!(bias_param.ty, "i64");
    assert!(bias_param.default.is_some());

    let b_input = add
        .spec
        .inputs
        .iter()
        .find(|p| p.name == "b")
        .expect("b input");
    assert_eq!(b_input.ty, "i64");
    assert!(b_input.default.is_some());

    let concat = reg
        .find_block(m, "string.concat", Some("^0.2"))
        .expect("string.concat resolved");
    let sep_param = concat
        .spec
        .params
        .iter()
        .find(|p| p.name == "separator")
        .expect("separator param");
    assert_eq!(sep_param.ty, "option<string>");
    assert!(sep_param.default.is_some()); // default null present
    assert!(sep_param.default.as_ref().unwrap().is_null());
}

#[test]
fn map_and_option_defaults_compatibility() {
    let reg = load_examples_registry().expect("registry should load");
    let m = "examples.shtairir";
    let blk = reg
        .find_block(m, "util.join_kv_pairs", Some("^0.2"))
        .expect("util.join_kv_pairs resolved");

    // inputs should include pairs: map<string,string> with default {}
    let pairs = blk
        .spec
        .inputs
        .iter()
        .find(|p| p.name == "pairs")
        .expect("pairs input present");
    assert_eq!(pairs.ty, "map<string,string>");
    let def_pairs = pairs.default.as_ref().expect("default {} present for pairs");
    assert!(def_pairs.is_object(), "pairs default should be JSON {} object");
    assert_eq!(def_pairs.as_object().unwrap().len(), 0, "pairs default should be {}");

    // inputs should include separator: option<string> with default null
    let sep = blk
        .spec
        .inputs
        .iter()
        .find(|p| p.name == "separator")
        .expect("separator input present");
    assert_eq!(sep.ty, "option<string>");
    let def_sep = sep.default.as_ref().expect("default null present for separator");
    assert!(def_sep.is_null(), "separator default should be null");
}

#[test]
fn determinism_without_effects() {
    let reg = load_examples_registry().expect("registry should load");
    let m = "examples.shtairir";
    for blk in ["math.add", "string.concat", "math.sum_list", "util.join_kv_pairs"] {
        let bh = reg
            .find_block(m, blk, Some("^0.2"))
            .expect("block resolved");
        // Deterministic blocks should have zero effects per validator rules.
        assert!(
            bh.spec.effects.is_empty(),
            "deterministic block should have no effects"
        );
    }
}

#[test]
fn version_resolution_with_tilde() {
    let reg = load_examples_registry().expect("registry should load");
    let m = "examples.shtairir";
    // Module is 0.2.0 currently; ~0.2 should resolve to 0.2.0 per SemVer patch-only selection
    assert!(reg.find_block(m, "math.add", Some("~0.2")).is_some());
    assert!(reg.find_block(m, "string.concat", Some("~0.2")).is_some());
    assert!(reg.find_block(m, "math.sum_list", Some("~0.2")).is_some());
    assert!(reg.find_block(m, "util.join_kv_pairs", Some("~0.2")).is_some());
}

#[test]
fn version_resolution_with_caret() {
    let reg = load_examples_registry().expect("registry should load");
    let m = "examples.shtairir";
    assert!(reg.find_block(m, "math.add", Some("^0.2")).is_some());
    assert!(reg.find_block(m, "string.concat", Some("^0.2")).is_some());
    assert!(reg.find_block(m, "math.sum_list", Some("^0.2")).is_some());
    assert!(reg.find_block(m, "util.join_kv_pairs", Some("^0.2")).is_some());
    // Also ensure that asking for None returns something (highest available)
    assert!(reg.find_block(m, "math.add", None).is_some());
}

#[test]
fn uniqueness_and_output_presence() {
    // Successful load implies:
    // - No duplicate port/param names
    // - At least one output per block
    let reg = load_examples_registry().expect("registry should load");
    let m = "examples.shtairir";
    for blk in ["math.add", "string.concat", "math.sum_list", "util.join_kv_pairs"] {
        let bh = reg
            .find_block(m, blk, Some("^0.2"))
            .expect("block resolved");
        assert!(
            !bh.spec.outputs.is_empty(),
            "block must have at least one output"
        );
        // Check uniqueness locally by collecting names
        let mut in_names = std::collections::BTreeSet::new();
        for p in &bh.spec.inputs {
            assert!(in_names.insert(&p.name));
        }
        let mut out_names = std::collections::BTreeSet::new();
        for p in &bh.spec.outputs {
            assert!(out_names.insert(&p.name));
        }
        let mut param_names = std::collections::BTreeSet::new();
        for p in &bh.spec.params {
            assert!(param_names.insert(&p.name));
        }
    }
}

#[test]
fn negative_deterministic_effects_rejected() {
    // Create a tiny temporary module manifest that includes a single invalid block spec.
    // The invalid block is Deterministic but declares an effect, which must fail validation.
    use std::fs;
    use tempfile::tempdir;
    use shtairir_registry::model::Registry;

    let dir = tempdir().expect("temp dir");
    let module_dir = dir.path();

    // Write MODULE.toml
    let module_toml = r#"
name = "tmp.invalid_examples"
version = "0.2.0"
title = "Tmp Invalid Examples"
description = "Temp module to test validator negative case"
authors = ["test"]
categories = ["tests"]
min_shtairir_version = "0.2.0"

blocks = [
  "blocks/test_only.invalid.toml",
]
"#;
    std::fs::create_dir_all(module_dir.join("blocks")).unwrap();
    std::fs::write(module_dir.join("MODULE.toml"), module_toml).unwrap();

    // Write invalid block (v0.2 format)
    let invalid_block = r#"
id = "tmp.invalid/test@0.2.0"
namespace = "tmp.invalid"
name = "test"
version = "0.2.0"
title = "Sneaky IO"
description = "Pretends to be deterministic but has an effect"
authors = ["test"]
license = "test"
tags = ["test"]
purity = "pure"
effects = ["fs"]
determinism = "Deterministic"

inputs = [
  { name = "x", ty = "i64" }
]

outputs = [
  { name = "y", ty = "i64" }
]

[engine]
version_req = "^0.2"
capability_flags = ["serde"]
"#;
    std::fs::write(module_dir.join("blocks/test_only.invalid.toml"), invalid_block).unwrap();

    // Attempt to load registry from this temp root. It should fail validation.
    let roots = vec![module_dir.to_path_buf()];
    let reg = Registry::load(&roots);
    assert!(reg.is_err(), "Registry::load should fail when a deterministic block declares effects");
}

#[test]
fn graph_validation_passes() {
    let reg = load_examples_registry().expect("registry should load");
    let module = "examples.shtairir";
    
    // Get the graph handle
    let graph_handle = reg.graphs.get("examples.shtairir@0.2.0:stream_sum")
        .expect("Graph should be loaded");
    
    // The graph should be valid (no validation errors)
    // Since we're testing successful loading, validation has already passed
    assert_eq!(graph_handle.spec.effects, Vec::<String>::new(), 
               "Graph should have no effects");
}

#[test]
fn integrity_hashes_are_present() {
    let reg = load_examples_registry().expect("registry should load");
    let module = "examples.shtairir";
    
    // Check blocks have integrity hashes
    for block_name in ["math.add", "string.concat", "math.sum_list", "util.join_kv_pairs"] {
        let handle = reg.find_block(module, block_name, Some("^0.2"))
            .expect(&format!("Block {} should resolve with ^0.2", block_name));
        
        let integrity = handle.spec.integrity
            .as_ref()
            .expect(&format!("Block {} should have integrity section", block_name));
        
        assert!(integrity.content_hash.starts_with("sha256:"), 
                "Block {} should have sha256 hash", block_name);
    }
    
    // Check graph has integrity hash
    let graph_handle = reg.graphs.get("examples.shtairir@0.2.0:stream_sum")
        .expect("Graph should be loaded");
    
    let graph_integrity = graph_handle.spec.integrity
        .as_ref()
        .expect("Graph should have integrity section");
    
    assert!(graph_integrity.content_hash.starts_with("sha256:"), 
            "Graph should have sha256 hash");
}
