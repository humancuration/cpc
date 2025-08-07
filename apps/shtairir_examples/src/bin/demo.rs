use std::path::PathBuf;
use anyhow::Result;
use shtairir_registry::model::BlockHandle;
use shtairir_registry::model::Registry;

fn main() -> Result<()> {
    // Load registry by scanning the examples module root.
    // This is in-memory only; no persistence or caching.
    let roots = vec![PathBuf::from("apps/shtairir_examples")];
    let reg = Registry::load(&roots)?;

    // List modules discovered
    let modules = reg.list_modules();
    println!("Discovered modules: {:?}", modules);

    // For this example, we expect "examples.shtairir"
    let module = "examples.shtairir";
    let blocks = reg.list_blocks(module);
    println!("Blocks in {}: {:?}", module, blocks);
// Try resolving with a caret requirement "^1.0"
for blk in ["math.add", "string.concat", "math.sum_list", "util.join_kv_pairs"] {
   match reg.find_block(module, blk, Some("^1.0")) {
       Some(BlockHandle { module, version, spec }) => {
           println!("Resolved: {}@{} -> {}", module, version, spec.name);
       }
       None => {
           println!("Could not resolve block '{}' with ^1.0 in module '{}'", blk, module);
       }
   }
}
    }

    Ok(())
}