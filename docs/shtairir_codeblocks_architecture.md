# Shtairir Codeblocks Architecture

## Overview

This document outlines the architectural design for the building blocks that will comprise the Shtairir visual programming and scripting system. The architecture is designed to be robust, extensible, and performant, supporting both deterministic and non-deterministic blocks with clear separation of pure functions from effectful operations.

## Current Implementation Analysis

The current Shtairir system consists of several key packages:

1. **shtairir** - Main parser and engine with AST and grammar
2. **shtairir_core** - Core infrastructure with abstractions, error handling, event bus
3. **shtairir_execution** - Execution scheduler with concurrent execution support
4. **shtairir_registry** - Module/block registry with loading, validation, and integrity checking
5. **shtairir_examples** - Example modules demonstrating usage

### Key Components Already Implemented

- **Type System**: Rich type system with scalars, composite types, and generic parameters
- **Module System**: MODULE.toml manifests, block specifications, and graph specifications
- **Execution Model**: Deterministic scheduling with concurrent execution support
- **Registry System**: Module loading, block/graph registration, and version resolution

## 1. Core Building Block Architecture

### 1.1 Block Interface/Traits

The fundamental block interface will be defined through a set of traits that provide the core functionality for all blocks:

```rust
// Core trait that all blocks must implement
#[async_trait]
pub trait Block: Send + Sync {
    /// Get the block specification
    fn spec(&self) -> &BlockSpec;
    
    /// Execute the block with given inputs
    async fn execute(&self, inputs: &BlockInputs, context: &ExecutionContext) -> Result<BlockOutputs, ExecutionError>;
    
    /// Validate the block configuration
    fn validate(&self, params: &BlockParams) -> Result<(), ValidationError>;
    
    /// Get the block's purity (pure/effect)
    fn purity(&self) -> Purity;
    
    /// Get the block's determinism (deterministic/non-deterministic)
    fn determinism(&self) -> Determinism;
}

// Trait for blocks that can be compiled
pub trait CompilableBlock: Block {
    /// Compile the block to executable form
    fn compile(&self, compilation_context: &CompilationContext) -> Result<CompiledBlock, CompilationError>;
    
    /// Get the compiled form if available
    fn get_compiled(&self) -> Option<&CompiledBlock>;
}

// Trait for blocks that can be introspected
pub trait IntrospectableBlock: Block {
    /// Get the block's schema
    fn schema(&self) -> &BlockSchema;
    
    /// Get the block's metadata
    fn metadata(&self) -> &BlockMetadata;
    
    /// Get the block's documentation
    fn documentation(&self) -> &BlockDocumentation;
}
```

### 1.2 Block Composition

Blocks can be composed in several ways:

1. **Sequential Composition**: Output of one block feeds into input of another
2. **Parallel Composition**: Multiple blocks execute concurrently
3. **Conditional Composition**: Blocks execute based on conditions
4. **Iterative Composition**: Blocks execute in loops

The composition system will be built around these concepts:

```rust
// Block composition types
pub enum BlockComposition {
    /// Sequential composition of blocks
    Sequential {
        blocks: Vec<Box<dyn Block>>,
        connections: Vec<Connection>,
    },
    /// Parallel composition of blocks
    Parallel {
        blocks: Vec<Box<dyn Block>>,
        synchronization: SynchronizationStrategy,
    },
    /// Conditional composition
    Conditional {
        condition: Box<dyn Block>,
        true_branch: Box<dyn Block>,
        false_branch: Option<Box<dyn Block>>,
    },
    /// Iterative composition
    Iterative {
        body: Box<dyn Block>,
        condition: Option<Box<dyn Block>>,
        collection: Option<Box<dyn Block>>,
    },
}

// Connection between blocks
pub struct Connection {
    pub from: OutputPort,
    pub to: InputPort,
    pub adapter: Option<EdgeAdapter>,
}
```

### 1.3 Execution Context and Adapter System

The execution context provides the environment in which blocks execute:

```rust
// Execution context for block execution
pub struct ExecutionContext {
    /// Unique identifier for this execution
    pub execution_id: ExecutionId,
    
    /// Registry for looking up blocks and graphs
    pub registry: Arc<Registry>,
    
    /// Event system for cross-block communication
    pub event_system: Arc<dyn EventSystem>,
    
    /// Configuration manager
    pub config: Arc<dyn ConfigManager>,
    
    /// Type system
    pub type_system: Arc<TypeSystem>,
    
    /// Memory manager
    pub memory_manager: Arc<MemoryManager>,
    
    /// Caching system
    pub cache: Arc<dyn CacheSystem>,
    
    /// Security context
    pub security_context: SecurityContext,
    
    /// Execution metadata
    pub metadata: HashMap<String, Value>,
}

// Adapter system for connecting blocks
pub trait BlockAdapter: Send + Sync {
    /// Adapt output from one block to input of another
    fn adapt(&self, output: &Value, target_type: &Type) -> Result<Value, AdapterError>;
    
    /// Get the adapter's specification
    fn spec(&self) -> &AdapterSpec;
    
    /// Validate the adapter configuration
    fn validate(&self, config: &AdapterConfig) -> Result<(), AdapterError>;
}

// Built-in adapters
pub enum BuiltInAdapter {
    /// Type conversion adapter
    TypeConversion(TypeConversionAdapter),
    /// Data transformation adapter
    Transform(TransformAdapter),
    /// Filtering adapter
    Filter(FilterAdapter),
    /// Aggregation adapter
    Aggregate(AggregateAdapter),
}
```

## 2. Visual Programming Components

### 2.1 Node and Edge Definitions

The visual programming system will be based on nodes and edges:

```rust
// Node in a visual graph
pub struct VisualNode {
    /// Unique identifier for the node
    pub id: NodeId,
    
    /// Position in the visual editor
    pub position: Position,
    
    /// Block that this node represents
    pub block: Box<dyn Block>,
    
    /// Input ports
    pub inputs: Vec<InputPort>,
    
    /// Output ports
    pub outputs: Vec<OutputPort>,
    
    /// Visual properties
    pub visual_properties: VisualProperties,
    
    /// User data
    pub user_data: HashMap<String, Value>,
}

// Edge connecting nodes
pub struct VisualEdge {
    /// Unique identifier for the edge
    pub id: EdgeId,
    
    /// Source node and port
    pub source: EdgeEndpoint,
    
    /// Target node and port
    pub target: EdgeEndpoint,
    
    /// Edge policy for data flow control
    pub policy: EdgePolicy,
    
    /// Visual properties
    pub visual_properties: VisualProperties,
}

// Endpoint for edges
pub struct EdgeEndpoint {
    pub node_id: NodeId,
    pub port_id: PortId,
}
```

### 2.2 Port Specifications with Type Checking

Ports define the interface for nodes:

```rust
// Input port for a node
pub struct InputPort {
    /// Unique identifier for the port
    pub id: PortId,
    
    /// Port name
    pub name: String,
    
    /// Port type
    pub ty: Type,
    
    /// Port kind (value, stream, event)
    pub kind: PortKind,
    
    /// Default value
    pub default: Option<Value>,
    
    /// Whether the port is required
    pub required: bool,
    
    /// Port description
    pub description: Option<String>,
    
    /// Validation rules
    pub validation: Vec<ValidationRule>,
}

// Output port for a node
pub struct OutputPort {
    /// Unique identifier for the port
    pub id: PortId,
    
    /// Port name
    pub name: String,
    
    /// Port type
    pub ty: Type,
    
    /// Port kind (value, stream, event)
    pub kind: PortKind,
    
    /// Port description
    pub description: Option<String>,
}

// Port kind
pub enum PortKind {
    /// Value port (synchronous data)
    Value,
    /// Stream port (asynchronous data stream)
    Stream,
    /// Event port (event notifications)
    Event,
    /// Composite port (structured data)
    Composite,
}
```

### 2.3 Edge Policies for Data Flow Control

Edge policies control how data flows between nodes:

```rust
// Policy for edge behavior
pub struct EdgePolicy {
    /// Adapter for data transformation
    pub adapter: EdgeAdapter,
    
    /// Backpressure strategy
    pub backpressure: BackpressureStrategy,
    
    /// Ordering strategy
    pub ordering: OrderingStrategy,
    
    /// Buffering strategy
    pub buffering: BufferingStrategy,
    
    /// Error handling strategy
    pub error_handling: ErrorHandlingStrategy,
}

// Edge adapter
pub enum EdgeAdapter {
    /// No adapter (direct connection)
    None,
    /// Map adapter (transform data)
    Map(MapAdapter),
    /// Filter adapter (filter data)
    Filter(FilterAdapter),
    /// Buffer adapter (buffer data)
    Buffer(BufferAdapter),
    /// Window adapter (window operations)
    Window(WindowAdapter),
    /// Debounce adapter (debounce events)
    Debounce(DebounceAdapter),
    /// Merge adapter (merge multiple streams)
    Merge(MergeAdapter),
    /// Zip adapter (combine streams)
    Zip(ZipAdapter),
    /// Boundary adapter (boundary detection)
    Boundary(BoundaryAdapter),
}

// Backpressure strategy
pub enum BackpressureStrategy {
    /// Block when downstream is full
    Block,
    /// Drop oldest data when downstream is full
    DropOldest,
    /// Drop newest data when downstream is full
    DropNewest,
    /// Expand buffer when full
    Expand,
}

// Ordering strategy
pub enum OrderingStrategy {
    /// Preserve source ordering
    Source,
    /// Order by timestamp
    Timestamp,
    /// Order by stable key
    StableKey,
}
```

## 3. Runtime System Design

### 3.1 Execution Scheduler Architecture

The execution scheduler is responsible for orchestrating block execution:

```rust
// Main execution scheduler
pub struct ExecutionScheduler {
    /// Registry for looking up blocks and graphs
    registry: Arc<Registry>,
    
    /// Task scheduler for parallel execution
    task_scheduler: Arc<dyn TaskScheduler>,
    
    /// Memory manager for block instances
    memory_manager: Arc<MemoryManager>,
    
    /// Event system for cross-block communication
    event_system: Arc<dyn EventSystem>,
    
    /// Performance monitor
    performance_monitor: Arc<PerformanceMonitor>,
    
    /// Configuration
    config: SchedulerConfig,
}

// Task scheduler for parallel execution
#[async_trait]
pub trait TaskScheduler: Send + Sync {
    /// Schedule a task for execution
    async fn schedule(&self, task: Task) -> Result<TaskId, SchedulerError>;
    
    /// Cancel a scheduled task
    async fn cancel(&self, task_id: TaskId) -> Result<bool, SchedulerError>;
    
    /// Get task status
    async fn get_status(&self, task_id: TaskId) -> Result<TaskStatus, SchedulerError>;
    
    /// Wait for task completion
    async fn wait_for_completion(&self, task_id: TaskId) -> Result<TaskResult, SchedulerError>;
}

// Task representation
pub struct Task {
    /// Unique identifier for the task
    pub id: TaskId,
    
    /// Block to execute
    pub block: Arc<dyn Block>,
    
    /// Inputs for the task
    pub inputs: BlockInputs,
    
    /// Execution context
    pub context: ExecutionContext,
    
    /// Dependencies
    pub dependencies: Vec<TaskId>,
    
    /// Priority
    pub priority: TaskPriority,
    
    /// Resource requirements
    pub resource_requirements: ResourceRequirements,
}
```

### 3.2 Memory Management for Block Instances

The memory manager handles allocation and deallocation of block instances:

```rust
// Memory manager for block instances
pub struct MemoryManager {
    /// Memory allocator
    allocator: Box<dyn MemoryAllocator>,
    
    /// Garbage collector
    garbage_collector: Box<dyn GarbageCollector>,
    
    /// Memory pool
    memory_pool: Arc<MemoryPool>,
    
    /// Memory limits
    limits: MemoryLimits,
    
    /// Memory usage statistics
    stats: MemoryStats,
}

// Memory allocator trait
pub trait MemoryAllocator: Send + Sync {
    /// Allocate memory for a block instance
    fn allocate(&self, size: usize) -> Result<MemoryBlock, AllocationError>;
    
    /// Free memory for a block instance
    fn free(&self, block: MemoryBlock) -> Result<(), AllocationError>;
    
    /// Resize an allocated memory block
    fn resize(&self, block: MemoryBlock, new_size: usize) -> Result<MemoryBlock, AllocationError>;
    
    /// Get memory usage statistics
    fn get_stats(&self) -> MemoryStats;
}

// Memory block representation
pub struct MemoryBlock {
    /// Unique identifier for the block
    pub id: MemoryBlockId,
    
    /// Pointer to the memory
    pub ptr: *mut u8,
    
    /// Size of the block
    pub size: usize,
    
    /// Alignment of the block
    pub alignment: usize,
    
    /// Memory flags
    pub flags: MemoryFlags,
}
```

### 3.3 Error Handling and Propagation Mechanisms

The error handling system provides robust error management:

```rust
// Execution error
pub enum ExecutionError {
    /// Block execution error
    BlockError {
        block_id: BlockId,
        error: BlockError,
    },
    
    /// Type error
    TypeError {
        expected_type: Type,
        actual_type: Type,
        context: String,
    },
    
    /// Resource error
    ResourceError {
        resource: String,
        error: ResourceError,
    },
    
    /// System error
    SystemError {
        component: String,
        error: SystemError,
    },
    
    /// User error
    UserError {
        message: String,
        details: Option<Value>,
    },
}

// Error propagation strategy
pub enum ErrorPropagationStrategy {
    /// Fail immediately
    FailFast,
    
    /// Continue with other blocks
    Continue,
    
    /// Retry with exponential backoff
    Retry {
        max_attempts: usize,
        backoff_strategy: BackoffStrategy,
    },
    
    /// Fallback to alternative block
    Fallback {
        fallback_block: Box<dyn Block>,
    },
    
    /// Custom error handler
    Custom {
        handler: Arc<dyn ErrorHandler>,
    },
}

// Error handler trait
#[async_trait]
pub trait ErrorHandler: Send + Sync {
    /// Handle an error
    async fn handle_error(&self, error: &ExecutionError, context: &ExecutionContext) -> ErrorHandlingResult;
    
    /// Get the handler specification
    fn spec(&self) -> &ErrorHandlerSpec;
}
```

## 4. Extensibility Framework

### 4.1 Plugin System for Custom Blocks

The plugin system allows for dynamic loading of custom blocks:

```rust
// Plugin manager for custom blocks
pub struct PluginManager {
    /// Loaded plugins
    plugins: HashMap<PluginId, Arc<dyn Plugin>>,
    
    /// Plugin loader
    loader: Box<dyn PluginLoader>,
    
    /// Plugin registry
    registry: Arc<Registry>,
    
    /// Configuration
    config: PluginManagerConfig,
}

// Plugin trait
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Get the plugin specification
    fn spec(&self) -> &PluginSpec;
    
    /// Initialize the plugin
    async fn initialize(&self, context: &PluginContext) -> Result<(), PluginError>;
    
    /// Get blocks provided by this plugin
    fn get_blocks(&self) -> Vec<Arc<dyn Block>>;
    
    /// Get adapters provided by this plugin
    fn get_adapters(&self) -> Vec<Arc<dyn BlockAdapter>>;
    
    /// Shutdown the plugin
    async fn shutdown(&self) -> Result<(), PluginError>;
}

// Plugin loader trait
#[async_trait]
pub trait PluginLoader: Send + Sync {
    /// Load a plugin from a file
    async fn load_from_file(&self, path: &Path) -> Result<Arc<dyn Plugin>, PluginError>;
    
    /// Load a plugin from a library
    async fn load_from_library(&self, library: &Library) -> Result<Arc<dyn Plugin>, PluginError>;
    
    /// Unload a plugin
    async fn unload(&self, plugin_id: PluginId) -> Result<(), PluginError>;
    
    /// Get loaded plugins
    fn get_loaded_plugins(&self) -> Vec<PluginId>;
}
```

### 4.2 Adapter Patterns for External System Integration

Adapters enable integration with external systems:

```rust
// External system adapter
pub struct ExternalSystemAdapter {
    /// Adapter specification
    spec: ExternalSystemAdapterSpec,
    
    /// Connection pool
    connection_pool: Arc<dyn ConnectionPool>,
    
    /// Message serializer/deserializer
    codec: Box<dyn MessageCodec>,
    
    /// Authentication provider
    auth: Arc<dyn AuthenticationProvider>,
    
    /// Configuration
    config: ExternalSystemAdapterConfig,
}

// External system block
pub struct ExternalSystemBlock {
    /// Block specification
    spec: BlockSpec,
    
    /// External system adapter
    adapter: Arc<ExternalSystemAdapter>,
    
    /// Operation to perform
    operation: String,
    
    /// Request template
    request_template: ValueTemplate,
    
    /// Response processor
    response_processor: Arc<dyn ResponseProcessor>,
}

// Response processor trait
#[async_trait]
pub trait ResponseProcessor: Send + Sync {
    /// Process a response from the external system
    async fn process_response(&self, response: &Value, context: &ExecutionContext) -> Result<Value, ProcessingError>;
    
    /// Get the processor specification
    fn spec(&self) -> &ResponseProcessorSpec;
}
```

### 4.3 Testing Framework for Block Validation

The testing framework provides comprehensive block validation:

```rust
// Block test runner
pub struct BlockTestRunner {
    /// Test registry
    test_registry: Arc<TestRegistry>,
    
    /// Test executor
    executor: Arc<dyn TestExecutor>,
    
    /// Test reporter
    reporter: Arc<dyn TestReporter>,
    
    /// Configuration
    config: TestRunnerConfig,
}

// Block test case
pub struct BlockTest {
    /// Test name
    pub name: String,
    
    /// Block to test
    pub block: Arc<dyn Block>,
    
    /// Test inputs
    pub inputs: BlockInputs,
    
    /// Expected outputs
    pub expected_outputs: Option<BlockOutputs>,
    
    /// Expected errors
    pub expected_errors: Option<Vec<ExecutionError>>,
    
    /// Test timeout
    pub timeout: Duration,
    
    /// Test metadata
    pub metadata: HashMap<String, Value>,
}

// Test executor trait
#[async_trait]
pub trait TestExecutor: Send + Sync {
    /// Execute a test
    async fn execute_test(&self, test: &BlockTest) -> TestResult;
    
    /// Execute multiple tests
    async fn execute_tests(&self, tests: Vec<BlockTest>) -> Vec<TestResult>;
    
    /// Get test execution statistics
    fn get_stats(&self) -> TestExecutionStats;
}

// Test result
pub struct TestResult {
    /// Test name
    pub name: String,
    
    /// Test status
    pub status: TestStatus,
    
    /// Execution time
    pub execution_time: Duration,
    
    /// Actual outputs
    pub actual_outputs: Option<BlockOutputs>,
    
    /// Actual errors
    pub actual_errors: Option<Vec<ExecutionError>>,
    
    /// Test artifacts
    pub artifacts: Vec<TestArtifact>,
}
```

## 5. Performance Optimization

### 5.1 Caching Strategies for Block Results

The caching system optimizes performance by caching block results:

```rust
// Cache system for block results
pub struct BlockResultCache {
    /// Cache backend
    backend: Box<dyn CacheBackend>,
    
    /// Cache policy
    policy: CachePolicy,
    
    /// Cache statistics
    stats: CacheStats,
    
    /// Cache validator
    validator: Arc<dyn CacheValidator>,
}

// Cache backend trait
#[async_trait]
pub trait CacheBackend: Send + Sync {
    /// Get a value from the cache
    async fn get(&self, key: &CacheKey) -> Result<Option<CacheValue>, CacheError>;
    
    /// Put a value in the cache
    async fn put(&self, key: CacheKey, value: CacheValue) -> Result<(), CacheError>;
    
    /// Remove a value from the cache
    async fn remove(&self, key: &CacheKey) -> Result<bool, CacheError>;
    
    /// Clear the cache
    async fn clear(&self) -> Result<(), CacheError>;
    
    /// Get cache statistics
    fn get_stats(&self) -> CacheStats;
}

// Cache key
pub struct CacheKey {
    /// Block identifier
    pub block_id: BlockId,
    
    /// Inputs hash
    pub inputs_hash: Hash,
    
    /// Parameters hash
    pub params_hash: Hash,
    
    /// Context hash
    pub context_hash: Hash,
}

// Cache policy
pub struct CachePolicy {
    /// Time-to-live for cache entries
    pub ttl: Duration,
    
    /// Maximum cache size
    pub max_size: usize,
    
    /// Eviction policy
    pub eviction_policy: EvictionPolicy,
    
    /// Compression settings
    pub compression: CompressionSettings,
}
```

### 5.2 Parallel Execution Planning

The parallel execution planner optimizes task scheduling:

```rust
// Parallel execution planner
pub struct ParallelExecutionPlanner {
    /// Dependency analyzer
    dependency_analyzer: Arc<dyn DependencyAnalyzer>,
    
    /// Resource estimator
    resource_estimator: Arc<dyn ResourceEstimator>,
    
    /// Cost model
    cost_model: Arc<dyn CostModel>,
    
    /// Optimization strategy
    optimization_strategy: OptimizationStrategy,
}

// Execution plan
pub struct ExecutionPlan {
    /// Tasks in the plan
    pub tasks: Vec<Task>,
    
    /// Dependencies between tasks
    pub dependencies: Vec<TaskDependency>,
    
    /// Resource allocation
    pub resource_allocation: ResourceAllocation,
    
    /// Estimated execution time
    pub estimated_execution_time: Duration,
    
    /// Plan metadata
    pub metadata: HashMap<String, Value>,
}

// Dependency analyzer trait
pub trait DependencyAnalyzer: Send + Sync {
    /// Analyze dependencies between blocks
    fn analyze_dependencies(&self, blocks: &[Arc<dyn Block>]) -> Result<DependencyGraph, AnalysisError>;
    
    /// Detect cycles in the dependency graph
    fn detect_cycles(&self, graph: &DependencyGraph) -> Result<Vec<CycleInfo>, AnalysisError>;
    
    /// Optimize the dependency graph
    fn optimize_graph(&self, graph: DependencyGraph) -> Result<DependencyGraph, AnalysisError>;
}
```

### 5.3 Memory-Efficient Data Structures

Memory-efficient data structures optimize resource usage:

```rust
// Memory-efficient block input container
pub struct BlockInputs {
    /// Input values
    values: HashMap<String, Value>,
    
    /// Memory layout
    memory_layout: MemoryLayout,
    
    /// Memory allocator
    allocator: Arc<dyn MemoryAllocator>,
}

// Memory-efficient block output container
pub struct BlockOutputs {
    /// Output values
    values: HashMap<String, Value>,
    
    /// Memory layout
    memory_layout: MemoryLayout,
    
    /// Memory allocator
    allocator: Arc<dyn MemoryAllocator>,
}

// Memory layout for efficient storage
pub struct MemoryLayout {
    /// Layout specification
    spec: MemoryLayoutSpec,
    
    /// Memory alignment
    alignment: usize,
    
    /// Memory padding
    padding: usize,
    
    /// Memory optimization flags
    optimization_flags: MemoryOptimizationFlags,
}

// Memory layout specification
pub enum MemoryLayoutSpec {
    /// Compact layout (minimal memory usage)
    Compact,
    
    /// Cache-friendly layout (optimized for cache access)
    CacheFriendly,
    
    /// SIMD-friendly layout (optimized for SIMD operations)
    SimdFriendly,
    
    /// Custom layout
    Custom {
        layout: Box<dyn CustomMemoryLayout>,
    },
}
```

## Integration with Existing Codebase

The proposed architecture integrates seamlessly with the existing Shtairir codebase:

1. **Type System Integration**: The new block architecture uses the existing type system from `shtairir_registry::types`.
2. **Registry Integration**: The block system extends the existing registry system for block discovery and management.
3. **Execution Integration**: The execution scheduler builds on the existing graph execution infrastructure.
4. **Error Handling Integration**: The error handling system extends the existing error types from `shtairir_core::error`.
5. **Value System Integration**: The value system integrates with the existing `shtairir_registry::value` module.

## Implementation Roadmap

1. **Phase 1**: Core block traits and interfaces
2. **Phase 2**: Basic execution engine and scheduler
3. **Phase 3**: Visual programming components
4. **Phase 4**: Extensibility framework and plugin system
5. **Phase 5**: Performance optimization and caching

## Conclusion

The proposed Shtairir codeblocks architecture provides a robust, extensible, and performant foundation for visual programming and scriptable workflows. The architecture supports both deterministic and non-deterministic blocks, with clear separation of pure functions from effectful operations, and integrates seamlessly with the existing Shtairir codebase.