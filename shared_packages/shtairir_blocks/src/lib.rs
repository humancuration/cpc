//! Shtairir Standard Library Blocks
//!
//! This crate provides the standard library building blocks for the Shtairir visual programming ecosystem.
//! These blocks form the foundation that users can compose into more complex workflows.

pub mod math;
pub mod collection;
pub mod string;
pub mod conversion;

// Re-export all block types for convenience
pub use math::{
    AddBlock, SubtractBlock, MultiplyBlock, DivideBlock,
    SqrtBlock, VectorAddBlock, MeanBlock, FixedMultiplyBlock
};
pub use collection::{
    MapBlock, FilterBlock, ReduceBlock,
    RandomSampleBlock, StatsSummaryBlock
};
pub use string::{
    ConcatBlock, SplitBlock, TrimBlock, FormatBlock
};
pub use conversion::{
    ToStringBlock, ToNumberBlock, ParseJsonBlock
};

// Re-export the main block traits and types for convenience
pub use shtairir::block::{Block, BlockInputs, BlockOutputs, BlockParams, ExecutionContext};
pub use shtairir_core::error::{ShtairirError, ShtairirResult};
pub use shtairir_registry::value::Value;