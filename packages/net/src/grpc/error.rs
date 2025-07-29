use thiserror::Error;

#[derive(Debug, Error)]
pub enum GrpcError {
    #[error("Connection error: {0}")]
    ConnectionError(#[from] tonic::transport::Error),
    
    #[error("gRPC status: {0}")]
    Status(#[from] tonic::Status),
    
    #[error("Retry limit exceeded")]
    RetryExceeded,
    
    #[error("Fatal error: {0}")]
    Fatal(String),
}

impl GrpcError {
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::ConnectionError(_) => true,
            Self::Status(status) => matches!(
                status.code(),
                tonic::Code::Unavailable | tonic::Code::DeadlineExceeded
            ),
            _ => false,
        }
    }
}