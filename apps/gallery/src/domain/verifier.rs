// Gallery domain - Media verification
// This file defines the MediaVerifier trait and implementations

use async_trait::async_trait;
use crate::domain::media::{Media, MediaType};

/// Error types for verification operations
#[derive(Debug, thiserror::Error)]
pub enum VerificationError {
    #[error("Invalid codec: expected {0}")]
    InvalidCodec(String),
    
    #[error("Zero duration")]
    ZeroDuration,
    
    #[error("Low quality output")]
    LowQuality,
    
    #[error("Container mismatch")]
    ContainerMismatch,
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Trait for media verification
#[async_trait]
pub trait MediaVerifier: Send + Sync {
    async fn verify(&self, path: &str, media_type: MediaType) -> Result<(), VerificationError>;
}

/// FFprobe-based verifier implementation
pub struct FfprobeVerifier;

#[async_trait]
impl MediaVerifier for FfprobeVerifier {
    async fn verify(&self, path: &str, media_type: MediaType) -> Result<(), VerificationError> {
        // In a real implementation, this would use ffprobe to check the actual codec of the file
        // For now, we'll implement a stub that just checks file extension
        match media_type {
            MediaType::Video => {
                if !path.ends_with(".webm") {
                    return Err(VerificationError::ContainerMismatch);
                }
                // In a real implementation, we would check for AV1 codec
            },
            MediaType::Audio => {
                if !path.ends_with(".webm") {
                    return Err(VerificationError::ContainerMismatch);
                }
                // In a real implementation, we would check for Opus codec
            },
            _ => return Ok(())
        }
        
        // Check file exists and has content
        let metadata = tokio::fs::metadata(path).await?;
        if metadata.len() == 0 {
            return Err(VerificationError::ZeroDuration);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::Write;

    #[tokio::test]
    async fn test_ffprobe_verifier_video() {
        let verifier = FfprobeVerifier;
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.webm");
        let file_path_str = file_path.to_str().unwrap();
        
        // Create a dummy file for testing
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"dummy video content").unwrap();
        
        // Test verification
        let result = verifier.verify(file_path_str, MediaType::Video).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_ffprobe_verifier_audio() {
        let verifier = FfprobeVerifier;
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.webm");
        let file_path_str = file_path.to_str().unwrap();
        
        // Create a dummy file for testing
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"dummy audio content").unwrap();
        
        // Test verification
        let result = verifier.verify(file_path_str, MediaType::Audio).await;
        assert!(result.is_ok());
    }
}