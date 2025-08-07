//! Edge policies for data flow control in Shtairir
//! 
//! This module defines the edge policies that control how data flows
//! between nodes in the visual programming system.

/// Policy for edge behavior
#[derive(Debug, Clone)]
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

impl Default for EdgePolicy {
    fn default() -> Self {
        Self {
            adapter: EdgeAdapter::None,
            backpressure: BackpressureStrategy::Block,
            ordering: OrderingStrategy::Source,
            buffering: BufferingStrategy::default(),
            error_handling: ErrorHandlingStrategy::FailFast,
        }
    }
}

/// Edge adapter
#[derive(Debug, Clone)]
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

/// Map adapter for data transformation
#[derive(Debug, Clone)]
pub struct MapAdapter {
    pub transform_function: String, // TODO: Define proper function type
}

/// Filter adapter for data filtering
#[derive(Debug, Clone)]
pub struct FilterAdapter {
    pub predicate: String, // TODO: Define proper predicate type
}

/// Buffer adapter for data buffering
#[derive(Debug, Clone)]
pub struct BufferAdapter {
    pub capacity: usize,
}

/// Window adapter for window operations
#[derive(Debug, Clone)]
pub struct WindowAdapter {
    pub size: usize,
    pub slide: Option<usize>,
}

/// Debounce adapter for event debouncing
#[derive(Debug, Clone)]
pub struct DebounceAdapter {
    pub delay_ms: u64,
}

/// Merge adapter for merging streams
#[derive(Debug, Clone)]
pub struct MergeAdapter {
    pub strategy: String, // e.g., "round_robin", "priority", "zip"
}

/// Zip adapter for combining streams
#[derive(Debug, Clone)]
pub struct ZipAdapter {
    // Zip adapter doesn't need additional parameters
}

/// Boundary adapter for boundary detection
#[derive(Debug, Clone)]
pub struct BoundaryAdapter {
    // Boundary adapter doesn't need additional parameters
}

/// Backpressure strategy
#[derive(Debug, Clone)]
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

/// Ordering strategy
#[derive(Debug, Clone)]
pub enum OrderingStrategy {
    /// Preserve source ordering
    Source,
    /// Order by timestamp
    Timestamp,
    /// Order by stable key
    StableKey,
}

/// Buffering strategy
#[derive(Debug, Clone)]
pub struct BufferingStrategy {
    /// Buffer size
    pub size: usize,
    
    /// Whether to use circular buffer
    pub circular: bool,
    
    /// Flush strategy
    pub flush: FlushStrategy,
}

impl Default for BufferingStrategy {
    fn default() -> Self {
        Self {
            size: 100,
            circular: false,
            flush: FlushStrategy::Manual,
        }
    }
}

/// Flush strategy for buffers
#[derive(Debug, Clone)]
pub enum FlushStrategy {
    /// Manual flush only
    Manual,
    /// Flush when buffer is full
    Full,
    /// Flush after a time interval
    Interval(u64), // milliseconds
    /// Flush based on a condition
    Conditional(String), // TODO: Define proper condition type
}

/// Error handling strategy
#[derive(Debug, Clone)]
pub enum ErrorHandlingStrategy {
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
        // TODO: Define fallback block reference
    },
    
    /// Custom error handler
    Custom {
        // TODO: Define custom error handler reference
    },
}

/// Backoff strategy for retries
#[derive(Debug, Clone)]
pub enum BackoffStrategy {
    /// Fixed delay between retries
    Fixed(u64), // milliseconds
    
    /// Exponential backoff
    Exponential {
        initial_delay_ms: u64,
        max_delay_ms: u64,
        multiplier: f64,
    },
    
    /// Fibonacci backoff
    Fibonacci {
        initial_delay_ms: u64,
        max_delay_ms: u64,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_edge_policy_default() {
        let policy = EdgePolicy::default();
        
        match policy.backpressure {
            BackpressureStrategy::Block => {}, // Test passes
            _ => panic!("Expected Block backpressure strategy"),
        }
        
        match policy.ordering {
            OrderingStrategy::Source => {}, // Test passes
            _ => panic!("Expected Source ordering strategy"),
        }
        
        match policy.error_handling {
            ErrorHandlingStrategy::FailFast => {}, // Test passes
            _ => panic!("Expected FailFast error handling strategy"),
        }
    }
    
    #[test]
    fn test_buffering_strategy_default() {
        let buffering = BufferingStrategy::default();
        
        assert_eq!(buffering.size, 100);
        assert_eq!(buffering.circular, false);
        
        match buffering.flush {
            FlushStrategy::Manual => {}, // Test passes
            _ => panic!("Expected Manual flush strategy"),
        }
    }
    
    #[test]
    fn test_backoff_strategy() {
        let fixed = BackoffStrategy::Fixed(1000);
        match fixed {
            BackoffStrategy::Fixed(delay) => assert_eq!(delay, 1000),
            _ => panic!("Expected Fixed backoff strategy"),
        }
        
        let exponential = BackoffStrategy::Exponential {
            initial_delay_ms: 100,
            max_delay_ms: 5000,
            multiplier: 2.0,
        };
        match exponential {
            BackoffStrategy::Exponential { initial_delay_ms, max_delay_ms, multiplier } => {
                assert_eq!(initial_delay_ms, 100);
                assert_eq!(max_delay_ms, 5000);
                assert_eq!(multiplier, 2.0);
            },
            _ => panic!("Expected Exponential backoff strategy"),
        }
    }
}