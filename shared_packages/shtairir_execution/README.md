# Shtairir Execution Scheduler

Execution scheduler for Shtairir graphs with deterministic scheduling and concurrent execution support.

## Overview

The `shtairir_execution` crate provides a deterministic execution scheduler for Shtairir graphs. It implements topological sorting using Kahn's algorithm to ensure deterministic execution order while enabling concurrent execution of independent nodes.

## Features

- **Deterministic Scheduling**: Uses topological sorting to ensure consistent execution order
- **Concurrent Execution**: Executes independent nodes in parallel using Rayon
- **Multiple Node Types**: Supports execution of blocks, subgraphs, and macros
- **Registry Integration**: Works with the Shtairir registry for block/graph lookup
- **Dependency Analysis**: Builds dependency graphs and identifies execution levels

## Modules

- `scheduler`: Main scheduler implementation with topological sorting
- `executor`: Node executors for different node kinds (block, subgraph, macro)
- `graph`: Graph utilities for dependency analysis and topological sorting
- `concurrency`: Concurrency control for parallel execution
- `registry`: Registry adapter for block/graph lookup

## Usage

```rust
use shtairir_execution::Scheduler;
use shtairir_execution::registry::RegistryAdapter;
use shtairir_registry::model::Registry;

// Create a registry and populate it with blocks/graphs
let registry = Registry::new();
let registry_adapter = RegistryAdapter::new(registry);

// Create a scheduler
let scheduler = Scheduler::new(registry_adapter);

// Schedule and execute a graph
// let result = scheduler.schedule(&graph_spec).await;
```

## Implementation Details

### Topological Sorting

The scheduler uses Kahn's algorithm for topological sorting:

1. Calculate in-degrees for all nodes
2. Find nodes with in-degree 0 (no dependencies)
3. Process nodes in topological order
4. Reduce in-degrees of dependent nodes
5. Continue until all nodes are processed

### Concurrent Execution

Independent nodes (nodes with no dependencies on each other) are executed concurrently:

1. Group nodes by dependency level
2. Execute all nodes within a level in parallel
3. Wait for level completion before proceeding to the next level
4. Maintain deterministic order through level-based execution

### Node Execution

Different node types are handled by specialized executors:

- **BlockExecutor**: Executes individual blocks by looking them up in the registry
- **SubgraphExecutor**: Recursively schedules and executes subgraphs
- **MacroExecutor**: Expands and executes macros

## Testing

The crate includes comprehensive tests for:

- Topological sorting algorithms
- Dependency graph construction
- Concurrent execution capabilities
- Registry adapter functionality
- Error handling

Run tests with:

```bash
cargo test
```

## Dependencies

- `shtairir_registry`: For registry and model definitions
- `rayon`: For parallel execution of independent nodes
- `tokio`: For async execution support
- `serde`: For serialization/deserialization
- `anyhow`: For error handling

## License

This crate is part of the CPC software ecosystem and is licensed under the CPC license.