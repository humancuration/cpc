use tokio::process::Command;
use std::process::Output;
use async_trait::async_trait;

pub enum TargetFormat {
    Av1,   // AV1 video + Opus audio
    Opus,  // Audio only
}

#[async_trait]
pub trait FfmpegExecutor: Send + Sync {
    async fn transcode(
        &self,
        input_path: &str,
        output_path: &str,
        target: TargetFormat
    ) -> Result<(), FfmpegError>;
}

pub struct FfmpegExecutorImpl;

#[async_trait]
impl FfmpegExecutor for FfmpegExecutorImpl {
    async fn transcode(
        &self,
        input_path: &str,
        output_path: &str,
        target: TargetFormat
    ) -> Result<(), FfmpegError> {
        let args = match target {
            TargetFormat::Av1 => {
                vec![
                    "-i", input_path,
                    "-c:v", "libaom-av1",
                    "-crf", "30",
                    "-b:v", "0",
                    "-c:a", "libopus",
                    "-b:a", "96k",
                    output_path
                ]
            },
            TargetFormat::Opus => {
                vec![
                    "-i", input_path,
                    "-c:a", "libopus",
                    "-b:a", "96k",
                    "-vn", // No video
                    output_path
                ]
            }
        };

        // Execute ffmpeg command
        let output = Command::new("ffmpeg")
            .args(&args)
            .output()
            .await
            .map_err(FfmpegError::ProcessFailed)?;

        if !output.status.success() {
            return Err(FfmpegError::TranscodingFailed(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FfmpegError {
    #[error("Process failed: {0}")]
    ProcessFailed(std::io::Error),
    #[error("Transcoding failed: {0}")]
    TranscodingFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_format_enum() {
        let av1 = TargetFormat::Av1;
        let opus = TargetFormat::Opus;
        
        match av1 {
            TargetFormat::Av1 => assert!(true),
            _ => assert!(false, "Expected Av1 variant"),
        }
        
        match opus {
            TargetFormat::Opus => assert!(true),
            _ => assert!(false, "Expected Opus variant"),
        }
    }
}