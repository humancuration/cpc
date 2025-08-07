# Shtairir v0.2 - Implementation Roadmap

## Overview

This document provides a detailed, prioritized implementation roadmap for the Shtairir v0.2 improvements outlined in the architectural plan. The roadmap is organized into phases, with specific tasks, estimated timelines, and dependencies clearly identified.

## Phase 1: Core Infrastructure (Priority: High, Duration: 7 weeks)

### 1.1 Complete Registry Implementation (2 weeks)

#### Task 1.1.1: Add Missing Methods to Registry Struct
- **Description**: Implement `insert_graph` and `module_graph_names` methods in the Registry struct
- **Files to Modify**: `shared_packages/shtairir_registry/src/model.rs`
- **Dependencies**: None
- **Acceptance Criteria**:
  - Graphs can be properly registered in the registry
  - Graph names can be retrieved for a module
  - All existing functionality continues to work
  - Comprehensive tests are added

#### Task 1.1.2: Improve Graph Indexing and Lookup
- **Description**: Enhance the registry's graph storage and retrieval mechanisms
- **Files to Modify**: `shared_packages/shtairir_registry/src/model.rs`, `shared_packages/shtairir_registry/src/lib.rs`
- **Dependencies**: Task 1.1.1
- **Acceptance Criteria**:
  - Graph lookup is efficient even with large registries
  - Graph version resolution works correctly
  - Graph dependencies are properly tracked

#### Task 1.1.3: Add Comprehensive Registry Tests
- **Description**: Create thorough tests for all registry functionality
- **Files to Modify**: `shared_packages/shtairir_registry/src/lib.rs` (add tests module)
- **Dependencies**: Tasks 1.1.1, 1.1.2
- **Acceptance Criteria**:
  - All registry methods have test coverage
  - Edge cases are properly tested
  - Performance tests validate efficiency

### 1.2 Enhance Type System (3 weeks)

#### Task 1.2.1: Add Full Support for ADTs
- **Description**: Extend the type validator to properly handle struct and enum types
- **Files to Modify**: `shared_packages/shtairir_registry/src/validator.rs`
- **Dependencies**: None
- **Acceptance Criteria**:
  - Struct types are properly validated
  - Enum types are properly validated
  - Type compatibility rules for ADTs are implemented
  - Pattern matching and destructuring are supported

#### Task 1.2.2: Implement Proper Type Compatibility Checking
- **Description**: Create a comprehensive type compatibility system
- **Files to Modify**: `shared_packages/shtairir_registry/src/validator.rs` (add new module)
- **Dependencies**: Task 1.2.1
- **Acceptance Criteria**:
  - Rich type compatibility checking is implemented
  - Subtyping relationships are supported where appropriate
  - Generic type instantiation works correctly
  - Type inference is improved

#### Task 1.2.3: Add Generic Bounds Checking
- **Description**: Implement trait-like constraint system for generics
- **Files to Modify**: `shared_packages/shtairir_registry/src/validator.rs`
- **Dependencies**: Task 1.2.2
- **Acceptance Criteria**:
  - Generic bounds are validated for blocks and graphs
  - Constraint propagation works correctly
  - Standard library traits are supported
  - Clear error messages for bound violations

### 1.3 Improve Validator (2 weeks)

#### Task 1.3.1: Complete Edge Type Compatibility Validation
- **Description**: Implement proper validation of edge connections
- **Files to Modify**: `shared_packages/shtairir_registry/src/validator.rs`
- **Dependencies**: Tasks 1.2.2, 1.2.3
- **Acceptance Criteria**:
  - Type compatibility between connected ports is validated
  - Implicit conversions are properly handled
  - Adapter requirements are identified
  - Clear error messages for type mismatches

#### Task 1.3.2: Add Stream Merge Policy Validation
- **Description**: Implement validation for stream merge configurations
- **Files to Modify**: `shared_packages/shtairir_registry/src/validator.rs`
- **Dependencies**: Task 1.3.1
- **Acceptance Criteria**:
  - Stream merge policies are validated
  - Multiple producer scenarios are handled
  - Required merge policies are identified
  - Policy conflicts are detected

#### Task 1.3.3: Implement Proper Cycle Detection
- **Description**: Add cycle detection with stateful-breaker node support
- **Files to Modify**: `shared_packages/shtairir_registry/src/validator.rs`
- **Dependencies**: Task 1.3.2
- **Acceptance Criteria**:
  - Cycles are properly detected
  - Stateful-breaker nodes are identified
  - Valid feedback loops are allowed
  - Clear error messages for invalid cycles

## Phase 2: Execution Model (Priority: High, Duration: 8 weeks)

### 2.1 Design Execution Scheduler (3 weeks)

#### Task 2.1.1: Create Execution Scheduler Structure
- **Description**: Design and implement the basic scheduler structure
- **Files to Create**: `shared_packages/shtairir_execution/src/lib.rs`, `shared_packages/shtairir_execution/src/scheduler.rs`
- **Dependencies**: Phase 1 tasks
- **Acceptance Criteria**:
  - Scheduler structure is defined
  - Basic scheduling algorithm is implemented
  - Integration with registry is established
  - Unit tests validate core functionality

#### Task 2.1.2: Implement Deterministic Scheduling Algorithm
- **Description**: Create a deterministic scheduling algorithm based on topological sorting
- **Files to Modify**: `shared_packages/shtairir_execution/src/scheduler.rs`
- **Dependencies**: Task 2.1.1
- **Acceptance Criteria**:
  - Topological sorting is implemented
  - Deterministic execution order is ensured
  - Node prioritization is supported
  - Performance is acceptable for large graphs

#### Task 2.1.3: Add Support for Concurrent Execution
- **Description**: Enable safe concurrent execution of independent nodes
- **Files to Modify**: `shared_packages/shtairir_execution/src/scheduler.rs`
- **Dependencies**: Task 2.1.2
- **Acceptance Criteria**:
  - Independent nodes can execute concurrently
  - Determinism is preserved
  - Resource limits can be configured
  - Performance improvements are measurable

### 2.2 Add Async Handling (3 weeks)

#### Task 2.2.1: Design Async Execution Model
- **Description**: Create the foundation for async block execution
- **Files to Create**: `shared_packages/shtairir_execution/src/async_executor.rs`
- **Dependencies**: Tasks 2.1.1-2.1.3
- **Acceptance Criteria**:
  - Async execution model is defined
  - Integration with scheduler is established
  - Basic async block execution works
  - Proper error handling is implemented

#### Task 2.2.2: Implement Async/Await Patterns
- **Description**: Add support for async/await patterns in blocks
- **Files to Modify**: `shared_packages/shtairir_execution/src/async_executor.rs`
- **Dependencies**: Task 2.2.1
- **Acceptance Criteria**:
  - Async/await patterns are supported
  - Async streams and events work correctly
  - Cancellation is properly handled
  - Integration with type system is complete

#### Task 2.2.3: Ensure Proper Error Handling
- **Description**: Implement comprehensive error handling for async operations
- **Files to Modify**: `shared_packages/shtairir_execution/src/async_executor.rs`
- **Dependencies**: Task 2.2.2
- **Acceptance Criteria**:
  - Async errors are properly propagated
  - Error recovery mechanisms are in place
  - Timeout handling is implemented
  - Logging and diagnostics are comprehensive

### 2.3 Implement Backpressure Handling (2 weeks)

#### Task 2.3.1: Extend Backpressure Strategies
- **Description**: Add more sophisticated backpressure handling options
- **Files to Modify**: `shared_packages/shtairir_registry/src/model.rs`, `shared_packages/shtairir_execution/src/backpressure.rs`
- **Dependencies**: Tasks 2.1.1-2.1.3
- **Acceptance Criteria**:
  - Extended backpressure enum is defined
  - New backpressure strategies are implemented
  - Integration with execution model is complete
  - Basic tests validate functionality

#### Task 2.3.2: Implement Backpressure Propagation
- **Description**: Create mechanisms for backpressure to propagate through the graph
- **Files to Modify**: `shared_packages/shtairir_execution/src/backpressure.rs`
- **Dependencies**: Task 2.3.1
- **Acceptance Criteria**:
  - Backpressure propagates correctly
  - Rate mismatches are handled gracefully
  - Buffer management is efficient
  - System remains stable under load

#### Task 2.3.3: Add Monitoring and Diagnostics
- **Description**: Implement tools for monitoring backpressure and diagnosing issues
- **Files to Create**: `shared_packages/shtairir_execution/src/monitoring.rs`
- **Dependencies**: Task 2.3.2
- **Acceptance Criteria**:
  - Backpressure events are logged
  - Monitoring metrics are collected
  - Diagnostic tools are available
  - Performance impact is minimal

## Phase 3: Visual Scripting (Priority: Medium, Duration: 7 weeks)

### 3.1 Create Adapter Nodes (3 weeks)

#### Task 3.1.1: Implement Map and Filter Adapters
- **Description**: Create block specifications and implementations for map and filter adapters
- **Files to Create**: `apps/shtairir_stdlib/blocks/adapters/map.toml`, `apps/shtairir_stdlib/blocks/adapters/filter.toml`
- **Files to Modify**: `apps/shtairir_stdlib/MODULE.toml`
- **Dependencies**: Phase 1 and 2 tasks
- **Acceptance Criteria**:
  - Map adapter works with all supported types
  - Filter adapter supports complex predicates
  - Parameter validation is comprehensive
  - Examples demonstrate usage patterns

#### Task 3.1.2: Implement Buffer and Window Adapters
- **Description**: Create block specifications and implementations for buffer and window adapters
- **Files to Create**: `apps/shtairir_stdlib/blocks/adapters/buffer.toml`, `apps/shtairir_stdlib/blocks/adapters/window.toml`
- **Files to Modify**: `apps/shtairir_stdlib/MODULE.toml`
- **Dependencies**: Task 3.1.1
- **Acceptance Criteria**:
  - Buffer adapter supports various strategies
  - Window adapter handles time and count windows
  - Integration with stream processing is seamless
  - Performance is optimized for common cases

#### Task 3.1.3: Implement Debounce, Merge, and Zip Adapters
- **Description**: Create block specifications and implementations for remaining adapters
- **Files to Create**: `apps/shtairir_stdlib/blocks/adapters/debounce.toml`, `apps/shtairir_stdlib/blocks/adapters/merge.toml`, `apps/shtairir_stdlib/blocks/adapters/zip.toml`
- **Files to Modify**: `apps/shtairir_stdlib/MODULE.toml`
- **Dependencies**: Task 3.1.2
- **Acceptance Criteria**:
  - Debounce adapter handles various timing scenarios
  - Merge adapter supports different merge strategies
  - Zip adapter works with multiple stream sources
  - All adapters have comprehensive documentation

### 3.2 Enhance Stream Processing (2 weeks)

#### Task 3.2.1: Implement Stream Merge Policies
- **Description**: Add runtime support for different stream merge strategies
- **Files to Create**: `shared_packages/shtairir_execution/src/stream_merge.rs`
- **Files to Modify**: `shared_packages/shtairir_registry/src/model.rs`
- **Dependencies**: Tasks 3.1.1-3.1.3
- **Acceptance Criteria**:
  - Ordered merge strategy works correctly
  - Timestamp-based merge is deterministic
  - Interleaved merge handles multiple sources
  - Performance is optimized for each strategy

#### Task 3.2.2: Add Support for Composite Projections
- **Description**: Implement syntax and semantics for projecting fields from composite ports
- **Files to Modify**: `shared_packages/shtairir_registry/src/model.rs`, `shared_packages/shtairir_execution/src/composite.rs`
- **Dependencies**: Tasks 1.2.1, 3.2.1
- **Acceptance Criteria**:
  - Struct field projection works correctly
  - Enum variant projection is supported
  - Projection paths are validated
  - Type safety is maintained

#### Task 3.2.3: Ensure Deterministic Behavior
- **Description**: Verify and ensure deterministic behavior for all stream operations
- **Files to Modify**: `shared_packages/shtairir_execution/src/stream_merge.rs`, `shared_packages/shtairir_execution/src/composite.rs`
- **Dependencies**: Task 3.2.2
- **Acceptance Criteria**:
  - All stream operations are deterministic
  - Reproducibility tests pass
  - Non-deterministic operations are properly flagged
  - Determinism guarantees are documented

### 3.3 Improve Visual Editor Support (2 weeks)

#### Task 3.3.1: Add Better Error Visualization
- **Description**: Enhance the visual editor to display validation errors more effectively
- **Files to Create**: `apps/shtairir_editor/src/error_visualization.rs`
- **Dependencies**: Phase 1 and 2 tasks
- **Acceptance Criteria**:
  - Validation errors are clearly displayed
  - Error locations are highlighted
  - Quick fixes are suggested where possible
  - Error severity is indicated visually

#### Task 3.3.2: Improve Type Inference Display
- **Description**: Enhance the visual editor to show inferred types
- **Files to Create**: `apps/shtairir_editor/src/type_inference_display.rs`
- **Dependencies**: Tasks 1.2.2, 1.2.3, 3.3.1
- **Acceptance Criteria**:
  - Inferred types are displayed for connections
  - Type information is updated dynamically
  - Generic type instantiations are shown
  - Type conflicts are highlighted

#### Task 3.3.3: Enhance Debugging Capabilities
- **Description**: Add debugging features to the visual editor
- **Files to Create**: `apps/shtairir_editor/src/debugging.rs`
- **Dependencies**: Tasks 2.1.1-2.1.3, 3.3.2
- **Acceptance Criteria**:
  - Step-by-step execution is supported
  - Intermediate values can be inspected
  - Execution traces can be viewed
  - Performance profiling is available

## Phase 4: Examples and Interoperability (Priority: Medium, Duration: 8 weeks)

### 4.1 Create Example Blocks and Graphs (3 weeks)

#### Task 4.1.1: Add Complex Generic Examples
- **Description**: Create examples demonstrating advanced generic patterns
- **Files to Create**: `apps/shtairir_examples/blocks/generics/`, `apps/shtairir_examples/graphs/generics/`
- **Files to Modify**: `apps/shtairir_examples/MODULE.toml`
- **Dependencies**: Tasks 1.2.2, 1.2.3
- **Acceptance Criteria**:
  - Examples show advanced generic patterns
  - Constraint usage is demonstrated
  - Multiple generic parameters are used
  - Generic graph composition is shown

#### Task 4.1.2: Create Effectful Operation Examples
- **Description**: Develop examples showing effectful operations and async patterns
- **Files to Create**: `apps/shtairir_examples/blocks/effects/`, `apps/shtairir_examples/graphs/effects/`
- **Files to Modify**: `apps/shtairir_examples/MODULE.toml`
- **Dependencies**: Tasks 2.2.1-2.2.3
- **Acceptance Criteria**:
  - Examples show common effect patterns
  - Async/await patterns are demonstrated
  - Effect boundary handling is shown
  - External system integration examples

#### Task 4.1.3: Develop Stream Processing Patterns
- **Description**: Create examples of common stream processing patterns
- **Files to Create**: `apps/shtairir_examples/graphs/streams/`
- **Files to Modify**: `apps/shtairir_examples/MODULE.toml`
- **Dependencies**: Tasks 3.1.1-3.1.3, 3.2.1-3.2.3
- **Acceptance Criteria**:
  - Real-time processing scenarios are covered
  - Windowing and aggregation patterns are shown
  - Complex event processing is demonstrated
  - Performance benchmarks are included

### 4.2 Design FFI Interface (3 weeks)

#### Task 4.2.1: Define Rust Traits for Blocks
- **Description**: Create the FFI interface with clear Rust traits for block implementation
- **Files to Create**: `shared_packages/shtairir-ffi/src/lib.rs`, `shared_packages/shtairir-ffi/src/traits.rs`
- **Dependencies**: Phase 1 and 2 tasks
- **Acceptance Criteria**:
  - Block trait is defined with clear interface
  - StreamBlock and AsyncBlock traits are defined
  - Type mappings between Shtairir and Rust are clear
  - Documentation is comprehensive

#### Task 4.2.2: Add Code Generation Support
- **Description**: Implement procedural macros for easier block implementation
- **Files to Create**: `shared_packages/shtairir-ffi/src/macros.rs`, `shared_packages/shtairir-ffi-derive/src/lib.rs`
- **Dependencies**: Task 4.2.1
- **Acceptance Criteria**:
  - Procedural macros simplify block implementation
  - Boilerplate code is automatically generated
  - Type safety is maintained
  - Examples demonstrate usage

#### Task 4.2.3: Include Comprehensive Documentation
- **Description**: Create detailed documentation for the FFI interface
- **Files to Create**: `docs/shtairir/ffi-guide.md`, `docs/shtairir/ffi-examples.md`
- **Dependencies**: Tasks 4.2.1, 4.2.2
- **Acceptance Criteria**:
  - FFI guide explains all concepts
  - Examples show common usage patterns
  - Best practices are documented
  - Performance considerations are discussed

### 4.3 Create WASM Target (2 weeks)

#### Task 4.3.1: Define WASM Compilation Target
- **Description**: Specify how blocks can be compiled to WASM
- **Files to Create**: `shared_packages/shtairir-wasm/src/lib.rs`, `shared_packages/shtairir-wasm/src/abi.rs`
- **Dependencies**: Tasks 4.2.1-4.2.3
- **Acceptance Criteria**:
  - WASM ABI is clearly defined
  - Compilation process is documented
  - Type mappings for WASM are specified
  - Basic examples compile successfully

#### Task 4.3.2: Add Runtime Integration
- **Description**: Implement the runtime side for executing WASM blocks
- **Files to Create**: `shared_packages/shtairir-wasm/src/runtime.rs`
- **Dependencies**: Task 4.3.1
- **Acceptance Criteria**:
  - WASM blocks can be loaded and executed
  - Integration with scheduler is complete
  - Performance is acceptable
  - Memory management is safe

#### Task 4.3.3: Ensure Proper Sandboxing
- **Description**: Implement security measures for WASM block execution
- **Files to Modify**: `shared_packages/shtairir-wasm/src/runtime.rs`
- **Dependencies**: Task 4.3.2
- **Acceptance Criteria**:
  - WASM blocks are properly sandboxed
  - Resource limits can be configured
  - Security vulnerabilities are mitigated
  - Sandboxing overhead is minimal

## Phase 5: Serialization and Tooling (Priority: Low, Duration: 6 weeks)

### 5.1 Implement Serialization (2 weeks)

#### Task 5.1.1: Add Support for All Types
- **Description**: Implement comprehensive serialization for all supported types
- **Files to Create**: `shared_packages/shtairir-serialization/src/lib.rs`, `shared_packages/shtairir-serialization/src/types.rs`
- **Dependencies**: Task 1.2.1
- **Acceptance Criteria**:
  - All scalar types can be serialized
  - All composite types can be serialized
  - Custom serialization for complex types
  - Round-trip serialization is lossless

#### Task 5.1.2: Ensure Deterministic Behavior
- **Description**: Make serialization deterministic for reproducible execution
- **Files to Modify**: `shared_packages/shtairir-serialization/src/types.rs`
- **Dependencies**: Task 5.1.1
- **Acceptance Criteria**:
  - Serialization is deterministic
  - Hash verification works
  - Versioning is supported
  - Performance is optimized

#### Task 5.1.3: Support Multiple Formats
- **Description**: Add support for multiple serialization formats
- **Files to Create**: `shared_packages/shtairir-serialization/src/formats.rs`
- **Dependencies**: Task 5.1.2
- **Acceptance Criteria**:
  - JSON format is supported
  - CBOR format is supported
  - Binary format is optimized
  - Format conversion is possible

### 5.2 Improve Tooling (2 weeks)

#### Task 5.2.1: Add CLI Improvements
- **Description**: Enhance the command-line interface for better developer experience
- **Files to Create**: `apps/shtairir_cli/src/commands.rs`
- **Files to Modify**: `apps/shtairir_cli/src/main.rs`
- **Dependencies**: Phase 1-4 tasks
- **Acceptance Criteria**:
  - Registry management commands are added
  - Graph validation and execution commands
  - Performance profiling commands
  - Help documentation is comprehensive

#### Task 5.2.2: Enhance Error Reporting
- **Description**: Improve error messages and diagnostic information
- **Files to Modify**: `shared_packages/shtairir_registry/src/validator.rs`, `shared_packages/shtairir_execution/src/scheduler.rs`
- **Dependencies**: Phase 1-4 tasks
- **Acceptance Criteria**:
  - Error messages are clear and actionable
  - Diagnostic information is comprehensive
  - Error codes are consistent
  - Quick fixes are suggested where possible

#### Task 5.2.3: Include Performance Profiling
- **Description**: Add tools for profiling and optimizing graph performance
- **Files to Create**: `shared_packages/shtairir_profiling/src/lib.rs`
- **Files to Modify**: `shared_packages/shtairir_execution/src/scheduler.rs`
- **Dependencies**: Tasks 5.2.1, 5.2.2
- **Acceptance Criteria**:
  - Execution time profiling is available
  - Memory usage profiling is supported
  - Bottleneck identification is possible
  - Performance comparisons can be made

### 5.3 Documentation and Testing (2 weeks)

#### Task 5.3.1: Complete Documentation
- **Description**: Finalize all documentation for the v0.2 release
- **Files to Create**: `docs/shtairir/v0.2-release-notes.md`, `docs/shtairir/v0.2-migration-guide.md`
- **Files to Modify**: All existing documentation files
- **Dependencies**: All previous tasks
- **Acceptance Criteria**:
  - All features are documented
  - API reference is complete
  - Tutorials cover common use cases
  - Migration guide is comprehensive

#### Task 5.3.2: Add Comprehensive Tests
- **Description**: Ensure all functionality has adequate test coverage
- **Files to Modify**: All source files to add tests
- **Dependencies**: All previous tasks
- **Acceptance Criteria**:
  - Unit test coverage is >90%
  - Integration tests cover all major features
  - Performance tests validate efficiency
  - Documentation examples are tested

#### Task 5.3.3: Include Performance Benchmarks
- **Description**: Create benchmarks to validate and track performance
- **Files to Create**: `benches/shtairir_benchmark.rs`
- **Dependencies**: Task 5.3.2
- **Acceptance Criteria**:
  - Benchmarks cover core operations
  - Performance regression tests are in place
  - Benchmark results are documented
  - Optimization opportunities are identified

## Resource Requirements

### Team Composition
- **Core Infrastructure Phase**: 2-3 developers with strong Rust and systems programming experience
- **Execution Model Phase**: 2-3 developers with experience in async programming and scheduling algorithms
- **Visual Scripting Phase**: 2 developers with experience in UI/UX and visual programming
- **Examples and Interoperability Phase**: 2 developers with experience in FFI, WASM, and example creation
- **Serialization and Tooling Phase**: 1-2 developers with experience in serialization formats and CLI tooling

### Infrastructure Requirements
- CI/CD pipeline for automated testing and building
- Documentation generation and hosting
- Performance benchmarking infrastructure
- Package registry for publishing components

## Risk Assessment

### Technical Risks
1. **Complexity of Deterministic Async Execution**: Ensuring determinism while supporting async operations is challenging. Mitigated by thorough testing and clear design principles.
2. **Performance of Type System**: Rich type checking could impact performance. Mitigated by optimizing critical paths and using caching.
3. **WASM Integration**: Ensuring seamless WASM integration while maintaining security. Mitigated by careful API design and sandboxing.

### Schedule Risks
1. **Dependencies Between Phases**: Each phase depends on the previous one. Mitigated by clear interface definitions and parallel work where possible.
2. **Unforeseen Technical Challenges**: Complex systems often reveal unexpected issues. Mitigated by iterative development and regular reviews.
3. **Resource Availability**: Team availability may change. Mitigated by clear documentation and knowledge sharing.

## Success Metrics

### Technical Metrics
- Registry performance: <100ms to load and query 1000 modules
- Validation performance: <500ms to validate a complex graph with 100 nodes
- Type inference accuracy: >99% of cases handled without explicit annotations
- Determinism guarantees: 100% reproducible execution for pure graphs

### User Experience Metrics
- Developer onboarding time: <1 day for basic block creation
- Error message clarity: >90% of errors resolved without external help
- Example coverage: >80% of common patterns demonstrated in examples
- Documentation completeness: >95% of APIs documented with examples

### Ecosystem Metrics
- Number of reusable blocks: >100 in standard library
- Graph complexity supported: >1000 nodes without performance degradation
- Cross-platform compatibility: Support for Windows, Linux, macOS, and Web
- Integration capabilities: Seamless integration with at least 3 external systems

## Conclusion

This implementation roadmap provides a clear, prioritized path for delivering the Shtairir v0.2 improvements. By following this roadmap, we can systematically address the identified areas while managing risks and ensuring high-quality results.

The phased approach allows us to deliver value incrementally while building toward our long-term vision. Each phase builds on the previous one, creating a solid foundation for the Shtairir scripting language and ecosystem.

With this roadmap, we can create a powerful, flexible, and reliable platform for developing modular, interoperable applications across our ecosystem.