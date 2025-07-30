//! Error types for the Sheets application visualization functionality

/// Error types for visualization operations
#[derive(Debug, Clone)]
pub enum VizError {
    /// Accessibility service failure with descriptive message
    AccessibilityFailure(String),
    
    /// Cache version conflict between expected and actual versions
    CacheVersionConflict { 
        expected: String, 
        actual: String 
    },
    
    /// Fallback renderer was triggered due to primary rendering failure
    RenderFallbackTriggered,
    
    /// Error during data transformation from sheet to visualization format
    DataTransformationError,
}

impl std::fmt::Display for VizError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VizError::AccessibilityFailure(msg) => write!(f, "Accessibility failure: {}", msg),
            VizError::CacheVersionConflict { expected, actual } => {
                write!(f, "Cache version conflict: expected {}, actual {}", expected, actual)
            },
            VizError::RenderFallbackTriggered => write!(f, "Render fallback triggered"),
            VizError::DataTransformationError => write!(f, "Data transformation error"),
        }
    }
}

impl std::error::Error for VizError {}