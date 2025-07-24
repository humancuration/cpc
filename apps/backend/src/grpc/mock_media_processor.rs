use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::grpc::media_processor_adapter::{MediaProcessorService, ProcessMediaRequest, ProcessMediaResponse};

#[derive(Debug)]
pub struct MockMediaProcessor {
    delay: Duration,
    should_fail: bool,
    progress_sender: Option<mpsc::Sender<ProcessMediaResponse>>,
}

impl MockMediaProcessor {
    pub fn new(delay: Duration, should_fail: bool) -> Self {
        Self {
            delay,
            should_fail,
            progress_sender: None,
        }
    }
    
    pub fn with_progress_sender(mut self, sender: mpsc::Sender<ProcessMediaResponse>) -> Self {
        self.progress_sender = Some(sender);
        self
    }
}

#[tonic::async_trait]
impl MediaProcessorService for MockMediaProcessor {
    async fn process_media(
        &self,
        request: Request<ProcessMediaRequest>,
    ) -> Result<Response<ProcessMediaResponse>, Status> {
        let req = request.into_inner();
        
        if self.should_fail {
            return Err(Status::internal("Mock processing failed"));
        }
        
        let media_id = Uuid::from_bytes(req.media_id.try_into().unwrap());
        
        // Simulate processing with progress updates
        if let Some(sender) = &self.progress_sender {
            // Send initial progress
            let _ = sender.send(ProcessMediaResponse {
                media_id: req.media_id.clone(),
                status: "PROCESSING".to_string(),
                progress: 0,
                processed_url: None,
                error: None,
            }).await;
            
            // Simulate progress
            for i in 1..=10 {
                sleep(self.delay / 10).await;
                let _ = sender.send(ProcessMediaResponse {
                    media_id: req.media_id.clone(),
                    status: "PROCESSING".to_string(),
                    progress: i * 10,
                    processed_url: None,
                    error: None,
                }).await;
            }
        } else {
            sleep(self.delay).await;
        }
        
        let processed_url = format!("https://cdn.example.com/processed/{}", media_id);
        
        Ok(Response::new(ProcessMediaResponse {
            media_id: req.media_id,
            status: "PROCESSED".to_string(),
            progress: 100,
            processed_url: Some(processed_url),
            error: None,
        }))
    }
    
    async fn cancel_processing(
        &self,
        request: Request<ProcessMediaRequest>,
    ) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_mock_processor_success() {
        let processor = MockMediaProcessor::new(Duration::from_millis(100), false);
        let service = Arc::new(processor);
        
        let request = ProcessMediaRequest {
            media_id: Uuid::new_v4().as_bytes().to_vec(),
            media_type: "image/jpeg".to_string(),
            original_url: "https://example.com/original.jpg".to_string(),
        };
        
        let response = service.process_media(Request::new(request)).await;
        assert!(response.is_ok());
        
        let resp = response.unwrap().into_inner();
        assert_eq!(resp.status, "PROCESSED");
        assert_eq!(resp.progress, 100);
        assert!(resp.processed_url.is_some());
    }
    
    #[tokio::test]
    async fn test_mock_processor_failure() {
        let processor = MockMediaProcessor::new(Duration::from_millis(100), true);
        let service = Arc::new(processor);
        
        let request = ProcessMediaRequest {
            media_id: Uuid::new_v4().as_bytes().to_vec(),
            media_type: "image/jpeg".to_string(),
            original_url: "https://example.com/original.jpg".to_string(),
        };
        
        let response = service.process_media(Request::new(request)).await;
        assert!(response.is_err());
    }
}