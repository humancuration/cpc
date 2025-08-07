use shtairir_blocks::{AddBlock, SqrtBlock, ConcatBlock, ToStringBlock, MapBlock, StatsSummaryBlock};
use shtairir::block::{BlockInputs, BlockOutputs, BlockParams, ExecutionContext};
use shtairir_registry::model::{BlockSpec, Registry};
use shtairir_registry::value::Value;
use std::sync::Arc;

#[tokio::test]
async fn test_basic_blocks() {
    // Create a registry
    let registry = Arc::new(Registry::new());
    
    // Create execution context
    let context = ExecutionContext::new("test-execution".to_string(), registry);
    
    // Test math add block
    let add_block = AddBlock::new(BlockSpec::default());
    let inputs = BlockInputs::new()
        .with_input("a".to_string(), Value::i64(5))
        .with_input("b".to_string(), Value::i64(3));
    let outputs = add_block.execute(&inputs, &context).await.unwrap();
    assert_eq!(outputs.get("result"), Some(&Value::i64(8)));
    
    // Test math sqrt block
    let sqrt_block = SqrtBlock::new(BlockSpec::default());
    let inputs = BlockInputs::new()
        .with_input("value".to_string(), Value::f64(16.0));
    let outputs = sqrt_block.execute(&inputs, &context).await.unwrap();
    assert_eq!(outputs.get("result"), Some(&Value::f64(4.0)));
    
    // Test string concat block
    let concat_block = ConcatBlock::new(BlockSpec::default());
    let strings = vec![Value::string("hello"), Value::string("world")];
    let inputs = BlockInputs::new()
        .with_input("strings".to_string(), Value::list(strings))
        .with_input("separator".to_string(), Value::string(" "));
    let outputs = concat_block.execute(&inputs, &context).await.unwrap();
    assert_eq!(outputs.get("result"), Some(&Value::string("hello world")));
    
    // Test conversion to_string block
    let to_string_block = ToStringBlock::new(BlockSpec::default());
    let inputs = BlockInputs::new()
        .with_input("value".to_string(), Value::i64(42));
    let outputs = to_string_block.execute(&inputs, &context).await.unwrap();
    assert_eq!(outputs.get("result"), Some(&Value::string("42")));
    
    println!("All basic tests passed!");
}