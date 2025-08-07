//! Example usage of Shtairir standard library blocks

use shtairir_blocks::{math, collection, string, conversion};
use shtairir::block::{BlockInputs, BlockOutputs, BlockParams, ExecutionContext};
use shtairir_registry::model::{BlockSpec, Registry};
use shtairir_registry::value::Value;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Shtairir Standard Library Blocks Demo");
    
    // Create a registry (in a real application, this would be loaded from MODULE.toml files)
    let registry = Arc::new(Registry::new());
    
    // Create execution context
    let context = ExecutionContext::new("demo-execution".to_string(), registry);
    
    // Demonstrate math operations
    demonstrate_math_operations(&context).await?;
    
    // Demonstrate advanced math operations
    demonstrate_advanced_math_operations(&context).await?;
    
    // Demonstrate string operations
    demonstrate_string_operations(&context).await?;
    
    // Demonstrate conversion operations
    demonstrate_conversion_operations(&context).await?;
    
    // Demonstrate collection operations
    demonstrate_collection_operations(&context).await?;
    
    println!("All demonstrations completed successfully!");
    Ok(())
}

async fn demonstrate_math_operations(context: &ExecutionContext) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Basic Math Operations ---");
    
    // Create blocks
    let add_block = math::AddBlock::new(BlockSpec::default());
    let subtract_block = math::SubtractBlock::new(BlockSpec::default());
    let multiply_block = math::MultiplyBlock::new(BlockSpec::default());
    let divide_block = math::DivideBlock::new(BlockSpec::default());
    
    // Test addition
    let inputs = BlockInputs::new()
        .with_input("a".to_string(), Value::i64(5))
        .with_input("b".to_string(), Value::i64(3));
    
    let outputs = add_block.execute(&inputs, context).await?;
    println!("5 + 3 = {:?}", outputs.get("result"));
    
    // Test subtraction
    let inputs = BlockInputs::new()
        .with_input("a".to_string(), Value::i64(5))
        .with_input("b".to_string(), Value::i64(3));
    
    let outputs = subtract_block.execute(&inputs, context).await?;
    println!("5 - 3 = {:?}", outputs.get("result"));
    
    // Test multiplication
    let inputs = BlockInputs::new()
        .with_input("a".to_string(), Value::i64(5))
        .with_input("b".to_string(), Value::i64(3));
    
    let outputs = multiply_block.execute(&inputs, context).await?;
    println!("5 * 3 = {:?}", outputs.get("result"));
    
    // Test division
    let inputs = BlockInputs::new()
        .with_input("a".to_string(), Value::i64(6))
        .with_input("b".to_string(), Value::i64(3));
    
    let outputs = divide_block.execute(&inputs, context).await?;
    println!("6 / 3 = {:?}", outputs.get("result"));
    
    Ok(())
}

async fn demonstrate_advanced_math_operations(context: &ExecutionContext) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Advanced Math Operations ---");
    
    // Create blocks
    let sqrt_block = math::SqrtBlock::new(BlockSpec::default());
    let vector_add_block = math::VectorAddBlock::new(BlockSpec::default());
    let mean_block = math::MeanBlock::new(BlockSpec::default());
    let fixed_multiply_block = math::FixedMultiplyBlock::new(BlockSpec::default());
    
    // Test square root
    let inputs = BlockInputs::new()
        .with_input("value".to_string(), Value::f64(16.0));
    
    let outputs = sqrt_block.execute(&inputs, context).await?;
    println!("sqrt(16.0) = {:?}", outputs.get("result"));
    
    // Test vector addition
    let vec_a = vec![Value::f64(1.0), Value::f64(2.0), Value::f64(3.0)];
    let vec_b = vec![Value::f64(4.0), Value::f64(5.0), Value::f64(6.0)];
    let inputs = BlockInputs::new()
        .with_input("a".to_string(), Value::list(vec_a))
        .with_input("b".to_string(), Value::list(vec_b));
    
    let outputs = vector_add_block.execute(&inputs, context).await?;
    println!("Vector addition: {:?}", outputs.get("result"));
    
    // Test mean
    let values = vec![Value::f64(1.0), Value::f64(2.0), Value::f64(3.0), Value::f64(4.0), Value::f64(5.0)];
    let inputs = BlockInputs::new()
        .with_input("values".to_string(), Value::list(values));
    
    let outputs = mean_block.execute(&inputs, context).await?;
    println!("Mean of [1,2,3,4,5] = {:?}", outputs.get("result"));
    
    // Test fixed-point multiplication
    let inputs = BlockInputs::new()
        .with_input("a".to_string(), Value::string("1.5"))
        .with_input("b".to_string(), Value::string("2.0"));
    
    let outputs = fixed_multiply_block.execute(&inputs, context).await?;
    println!("Fixed multiply 1.5 * 2.0 = {:?}", outputs.get("result"));
    
    Ok(())
}

async fn demonstrate_string_operations(context: &ExecutionContext) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- String Operations ---");
    
    // Create blocks
    let concat_block = string::ConcatBlock::new(BlockSpec::default());
    let split_block = string::SplitBlock::new(BlockSpec::default());
    let trim_block = string::TrimBlock::new(BlockSpec::default());
    let format_block = string::FormatBlock::new(BlockSpec::default());
    
    // Test concatenation
    let strings = vec![Value::string("hello"), Value::string("world")];
    let inputs = BlockInputs::new()
        .with_input("strings".to_string(), Value::list(strings))
        .with_input("separator".to_string(), Value::string(" "));
    
    let outputs = concat_block.execute(&inputs, context).await?;
    println!("Concatenated: {:?}", outputs.get("result"));
    
    // Test splitting
    let inputs = BlockInputs::new()
        .with_input("text".to_string(), Value::string("hello,world,test"))
        .with_input("delimiter".to_string(), Value::string(","));
    
    let outputs = split_block.execute(&inputs, context).await?;
    println!("Split parts: {:?}", outputs.get("parts"));
    
    // Test trimming
    let inputs = BlockInputs::new()
        .with_input("text".to_string(), Value::string("  hello world  "));
    
    let outputs = trim_block.execute(&inputs, context).await?;
    println!("Trimmed: {:?}", outputs.get("result"));
    
    // Test formatting
    let mut values = std::collections::BTreeMap::new();
    values.insert("name".to_string(), Value::string("Alice"));
    values.insert("age".to_string(), Value::i64(30));
    
    let inputs = BlockInputs::new()
        .with_input("template".to_string(), Value::string("Hello {name}! You are {age} years old."))
        .with_input("values".to_string(), Value::object(values));
    
    let outputs = format_block.execute(&inputs, context).await?;
    println!("Formatted: {:?}", outputs.get("result"));
    
    Ok(())
}

async fn demonstrate_conversion_operations(context: &ExecutionContext) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Conversion Operations ---");
    
    // Create blocks
    let to_string_block = conversion::ToStringBlock::new(BlockSpec::default());
    let to_number_block = conversion::ToNumberBlock::new(BlockSpec::default());
    let parse_json_block = conversion::ParseJsonBlock::new(BlockSpec::default());
    
    // Test to_string
    let inputs = BlockInputs::new()
        .with_input("value".to_string(), Value::i64(42));
    
    let outputs = to_string_block.execute(&inputs, context).await?;
    println!("To string: {:?}", outputs.get("result"));
    
    // Test to_number
    let inputs = BlockInputs::new()
        .with_input("text".to_string(), Value::string("42"));
    
    let params = BlockParams::new()
        .with_param("target_type".to_string(), Value::string("i64"));
    
    let outputs = to_number_block.execute(&inputs, &params).await?;
    println!("To number: {:?}", outputs.get("result"));
    
    // Test parse_json
    let inputs = BlockInputs::new()
        .with_input("json_text".to_string(), Value::string("{\"name\":\"Alice\",\"age\":30}"));
    
    let outputs = parse_json_block.execute(&inputs, context).await?;
    println!("Parsed JSON: {:?}", outputs.get("result"));
    
    Ok(())
}

async fn demonstrate_collection_operations(context: &ExecutionContext) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Collection Operations ---");
    
    // Create blocks
    let map_block = collection::MapBlock::new(BlockSpec::default());
    let filter_block = collection::FilterBlock::new(BlockSpec::default());
    let reduce_block = collection::ReduceBlock::new(BlockSpec::default());
    let stats_summary_block = collection::StatsSummaryBlock::new(BlockSpec::default());
    
    // Test map (simplified)
    let list = vec![Value::i64(1), Value::i64(2), Value::i64(3)];
    let inputs = BlockInputs::new()
        .with_input("collection".to_string(), Value::list(list))
        .with_input("function".to_string(), Value::string("x -> x * 2"));
    
    let outputs = map_block.execute(&inputs, context).await?;
    println!("Map result: {:?}", outputs.get("result"));
    
    // Test filter (simplified)
    let list = vec![Value::i64(1), Value::i64(2), Value::i64(3), Value::i64(4)];
    let inputs = BlockInputs::new()
        .with_input("collection".to_string(), Value::list(list))
        .with_input("predicate".to_string(), Value::string("x -> x > 2"));
    
    let outputs = filter_block.execute(&inputs, context).await?;
    println!("Filter result: {:?}", outputs.get("result"));
    
    // Test reduce (simplified)
    let list = vec![Value::i64(1), Value::i64(2), Value::i64(3), Value::i64(4)];
    let inputs = BlockInputs::new()
        .with_input("collection".to_string(), Value::list(list))
        .with_input("initial".to_string(), Value::i64(0))
        .with_input("function".to_string(), Value::string("(acc, x) -> acc + x"));
    
    let outputs = reduce_block.execute(&inputs, context).await?;
    println!("Reduce result: {:?}", outputs.get("result"));
    
    // Test stats summary
    let values = vec![Value::f64(1.0), Value::f64(2.0), Value::f64(3.0), Value::f64(4.0), Value::f64(5.0)];
    let inputs = BlockInputs::new()
        .with_input("values".to_string(), Value::list(values));
    
    let outputs = stats_summary_block.execute(&inputs, context).await?;
    println!("Stats summary: {:?}", outputs.get("summary"));
    
    Ok(())
}