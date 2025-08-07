# Shtairir Standard Library Blocks

This package provides the standard library building blocks for the Shtairir visual programming ecosystem. These blocks form the foundation that users can compose into more complex workflows.

## Overview

The Shtairir standard library includes blocks for:

- **Math Operations**: Basic arithmetic, advanced mathematical functions, vector operations, and fixed-point arithmetic
- **Collection Processing**: Map, filter, reduce operations, statistical analysis, and random sampling
- **String Manipulation**: Concatenation, splitting, trimming, and formatting
- **Type Conversion**: Serialization, parsing, and JSON handling

## Features

### Math Operations

- Basic arithmetic: `add`, `subtract`, `multiply`, `divide`
- Advanced functions: `sqrt` (using libm)
- Vector operations: `vector_add` (using nalgebra)
- Statistical functions: `mean` (using statrs)
- Fixed-point arithmetic: `fixed_multiply` (using fixed)

### Collection Processing

- Functional operations: `map`, `filter`, `reduce`
- Statistical analysis: `stats_summary` (using statrs)
- Random sampling: `random_sample` (using rand_distr)

### String Manipulation

- Concatenation with separators
- Splitting by delimiters
- Whitespace trimming
- Template-based formatting

### Type Conversion

- Value to string serialization
- String to number parsing
- JSON parsing and generation

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
shtairir_blocks = { path = "shared_packages/shtairir_blocks" }
```

Then use the blocks in your Shtairir programs:

```rust
use shtairir_blocks::{AddBlock, ConcatBlock, ParseJsonBlock};
```

## Dependencies

This package leverages several high-quality Rust crates:

- **libm**: Mathematical functions
- **nalgebra**: Linear algebra and vector operations
- **statrs**: Statistical computation
- **fixed**: Fixed-point arithmetic
- **glam**: Graphics math library
- **argmin**: Mathematical optimization
- **rand_distr**: Random number distributions

## License

This package is part of the CPC software ecosystem and is licensed under the CPC license.

## Contributing

Contributions are welcome! Please see the main CPC repository for contribution guidelines.