use crate::media::types::*;
use crate::error::Error;
use anyhow::Result;
use chrono::Utc;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

/// Media processor using ffmpeg.wasm for royalty-free codec processing
pub struct MediaProcessor {
    jobs: Arc<Mutex<Vec<MediaProcessingJob>>>,
}

impl MediaProcessor {
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Process media file with the given configuration
    pub async fn process_media(
        &self,
        input_path: PathBuf,
        output_path: PathBuf,
        config: MediaProcessingConfig,
    ) -> Result<ProcessingResult> {
        let job_id = Uuid::new_v4();
        let job = MediaProcessingJob {
            id: job_id,
            input_path: input_path.clone(),
            output_path: output_path.clone(),
            config: config.clone(),
            status: ProcessingStatus::Pending,
            progress: 0.0,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            error_message: None,
        };

        // Add job to queue
        {
            let mut jobs = self.jobs.lock().await;
            jobs.push(job);
        }

        // Start processing
        self.execute_processing_job(job_id).await
    }

    /// Execute a processing job
    async fn execute_processing_job(&self, job_id: Uuid) -> Result<ProcessingResult> {
        let start_time = std::time::Instant::now();
        
        // Update job status to processing
        self.update_job_status(job_id, ProcessingStatus::Processing).await?;

        let job = self.get_job(job_id).await?;
        
        // Extract metadata from input file
        let metadata = self.extract_metadata(&job.input_path).await?;
        
        // Process based on media type
        let result = match metadata.media_type {
            MediaType::Video => self.process_video(&job).await,
            MediaType::Audio => self.process_audio(&job).await,
            MediaType::Image => self.process_image(&job).await,
        };

        let processing_time = start_time.elapsed().as_secs_f64();

        match result {
            Ok(output_path) => {
                self.update_job_status(job_id, ProcessingStatus::Completed).await?;
                Ok(ProcessingResult {
                    job_id,
                    success: true,
                    output_path: Some(output_path),
                    thumbnail_path: None, // Will be set by thumbnail generator
                    metadata: Some(metadata),
                    processing_time,
                    error_message: None,
                })
            }
            Err(e) => {
                let error_msg = e.to_string();
                self.update_job_status(job_id, ProcessingStatus::Failed(error_msg.clone())).await?;
                Ok(ProcessingResult {
                    job_id,
                    success: false,
                    output_path: None,
                    thumbnail_path: None,
                    metadata: Some(metadata),
                    processing_time,
                    error_message: Some(error_msg),
                })
            }
        }
    }

    /// Process video file using AV1 codec
    async fn process_video(&self, job: &MediaProcessingJob) -> Result<PathBuf> {
        // TODO: Integrate with ffmpeg.wasm for AV1 encoding
        // For now, this is a placeholder implementation
        
        let ffmpeg_args = self.build_video_ffmpeg_args(&job.config)?;
        
        // Simulate ffmpeg.wasm processing
        log::info!("Processing video with AV1 codec: {:?} -> {:?}", 
                  job.input_path, job.output_path);
        log::info!("FFmpeg args: {:?}", ffmpeg_args);
        
        // In a real implementation, this would call ffmpeg.wasm:
        // let result = ffmpeg_wasm::process(&job.input_path, &job.output_path, &ffmpeg_args).await?;
        
        // For now, just copy the file to simulate processing
        tokio::fs::copy(&job.input_path, &job.output_path).await?;
        
        Ok(job.output_path.clone())
    }

    /// Process audio file using Opus codec
    async fn process_audio(&self, job: &MediaProcessingJob) -> Result<PathBuf> {
        // TODO: Integrate with ffmpeg.wasm for Opus encoding
        
        let ffmpeg_args = self.build_audio_ffmpeg_args(&job.config)?;
        
        log::info!("Processing audio with Opus codec: {:?} -> {:?}", 
                  job.input_path, job.output_path);
        log::info!("FFmpeg args: {:?}", ffmpeg_args);
        
        // In a real implementation, this would call ffmpeg.wasm:
        // let result = ffmpeg_wasm::process(&job.input_path, &job.output_path, &ffmpeg_args).await?;
        
        // For now, just copy the file to simulate processing
        tokio::fs::copy(&job.input_path, &job.output_path).await?;
        
        Ok(job.output_path.clone())
    }

    /// Process image file
    async fn process_image(&self, job: &MediaProcessingJob) -> Result<PathBuf> {
        // For images, we mainly do optimization and format conversion
        
        log::info!("Processing image: {:?} -> {:?}", 
                  job.input_path, job.output_path);
        
        // Use the image crate for basic image processing
        let img = image::open(&job.input_path)?;
        
        // Apply quality settings if specified
        if let Some(quality) = job.config.quality {
            // Resize if resolution is specified
            let processed_img = if let Some((width, height)) = job.config.resolution {
                img.resize(width, height, image::imageops::FilterType::Lanczos3)
            } else {
                img
            };
            
            // Save with quality settings
            match job.config.container_format {
                ContainerFormat::JPEG => {
                    let mut output = std::fs::File::create(&job.output_path)?;
                    processed_img.write_to(&mut output, image::ImageOutputFormat::Jpeg(quality))?;
                }
                ContainerFormat::PNG => {
                    processed_img.save(&job.output_path)?;
                }
                _ => {
                    return Err(anyhow::anyhow!("Unsupported image format"));
                }
            }
        } else {
            img.save(&job.output_path)?;
        }
        
        Ok(job.output_path.clone())
    }

    /// Build FFmpeg arguments for video processing
    fn build_video_ffmpeg_args(&self, config: &MediaProcessingConfig) -> Result<Vec<String>> {
        let mut args = vec!["-c:v".to_string(), "libaom-av1".to_string()]; // AV1 encoder
        
        if let Some(bitrate) = config.video_bitrate {
            args.extend_from_slice(&["-b:v".to_string(), format!("{}k", bitrate / 1000)]);
        }
        
        if let Some((width, height)) = config.resolution {
            args.extend_from_slice(&["-s".to_string(), format!("{}x{}", width, height)]);
        }
        
        if let Some(fps) = config.frame_rate {
            args.extend_from_slice(&["-r".to_string(), fps.to_string()]);
        }
        
        // Audio codec
        args.extend_from_slice(&["-c:a".to_string(), "libopus".to_string()]);
        
        if let Some(audio_bitrate) = config.audio_bitrate {
            args.extend_from_slice(&["-b:a".to_string(), format!("{}k", audio_bitrate / 1000)]);
        }
        
        // Container format
        match config.container_format {
            ContainerFormat::WebM => {
                args.extend_from_slice(&["-f".to_string(), "webm".to_string()]);
            }
            _ => {
                return Err(anyhow::anyhow!("Unsupported container format for video"));
            }
        }
        
        Ok(args)
    }

    /// Build FFmpeg arguments for audio processing
    fn build_audio_ffmpeg_args(&self, config: &MediaProcessingConfig) -> Result<Vec<String>> {
        let mut args = vec!["-c:a".to_string(), "libopus".to_string()]; // Opus encoder
        
        if let Some(bitrate) = config.audio_bitrate {
            args.extend_from_slice(&["-b:a".to_string(), format!("{}k", bitrate / 1000)]);
        }
        
        Ok(args)
    }

    /// Extract metadata from media file
    async fn extract_metadata(&self, path: &Path) -> Result<MediaMetadata> {
        // TODO: Use ffmpeg.wasm to extract metadata
        // For now, this is a basic implementation using file system info
        
        let metadata = tokio::fs::metadata(path).await?;
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        // Determine media type from extension
        let media_type = match path.extension().and_then(|e| e.to_str()) {
            Some("mp4") | Some("webm") | Some("avi") | Some("mov") => MediaType::Video,
            Some("mp3") | Some("wav") | Some("ogg") | Some("opus") => MediaType::Audio,
            Some("jpg") | Some("jpeg") | Some("png") | Some("gif") => MediaType::Image,
            _ => MediaType::Image, // Default to image
        };
        
        // Calculate checksum
        let content = tokio::fs::read(path).await?;
        let checksum = format!("{:x}", sha256::digest(&content));
        
        Ok(MediaMetadata {
            id: Uuid::new_v4(),
            file_name,
            file_size: metadata.len(),
            media_type,
            duration: None, // TODO: Extract from ffmpeg
            width: None,    // TODO: Extract from ffmpeg
            height: None,   // TODO: Extract from ffmpeg
            frame_rate: None, // TODO: Extract from ffmpeg
            bitrate: None,  // TODO: Extract from ffmpeg
            codec: None,    // TODO: Extract from ffmpeg
            created_at: Utc::now(),
            checksum,
        })
    }

    /// Get job by ID
    async fn get_job(&self, job_id: Uuid) -> Result<MediaProcessingJob> {
        let jobs = self.jobs.lock().await;
        jobs.iter()
            .find(|job| job.id == job_id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Job not found: {}", job_id))
    }

    /// Update job status
    async fn update_job_status(&self, job_id: Uuid, status: ProcessingStatus) -> Result<()> {
        let mut jobs = self.jobs.lock().await;
        if let Some(job) = jobs.iter_mut().find(|job| job.id == job_id) {
            job.status = status;
            match &job.status {
                ProcessingStatus::Processing => job.started_at = Some(Utc::now()),
                ProcessingStatus::Completed | ProcessingStatus::Failed(_) => {
                    job.completed_at = Some(Utc::now());
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Get all jobs
    pub async fn get_jobs(&self) -> Vec<MediaProcessingJob> {
        let jobs = self.jobs.lock().await;
        jobs.clone()
    }

    /// Get job status
    pub async fn get_job_status(&self, job_id: Uuid) -> Option<ProcessingStatus> {
        let jobs = self.jobs.lock().await;
        jobs.iter()
            .find(|job| job.id == job_id)
            .map(|job| job.status.clone())
    }
}

impl Default for MediaProcessor {
    fn default() -> Self {
        Self::new()
    }
}

// Simple SHA-256 implementation for checksums
mod sha256 {
    use std::fmt;

    pub fn digest(data: &[u8]) -> String {
        // This is a placeholder - in a real implementation, use a proper crypto library
        // For now, just return a simple hash based on content length and first few bytes
        let mut hash = data.len() as u64;
        for (i, &byte) in data.iter().take(32).enumerate() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64).wrapping_add(i as u64);
        }
        format!("{:016x}", hash)
    }
}