use super::*;
use anyhow::anyhow;
use async_trait::async_trait;
use mockall::automock;
use mockall::predicate::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use cpc_protos::metrics::metrics_service_client::MetricsServiceClient;
use cpc_protos::metrics::TestResult;

// Mock for FFmpeg
#[automock]
#[async_trait]
trait FFmpegTrait {
    async fn run(&self, args: &[&str]) -> Result<(), String>;
    fn write_file(&self, name: &str, data: Vec<u8>);
    fn read_file(&self, name: &str) -> Result<Vec<u8>, String>;
}

#[async_trait]
impl FFmpegTrait for ffmpeg_wasm::FFmpeg {
    async fn run(&self, args: &[&str]) -> Result<(), String> {
        ffmpeg_wasm::FFmpeg::run(self, args).await
    }

    fn write_file(&self, name: &str, data: Vec<u8>) {
        ffmpeg_wasm::FFmpeg::write_file(self, name, data)
    }

    fn read_file(&self, name: &str) -> Result<Vec<u8>, String> {
        ffmpeg_wasm::FFmpeg::read_file(self, name)
    }
}

// Test helper to generate sample barcode images
fn generate_barcode_image(barcode: &str, format: Format) -> Vec<u8> {
    // In a real test, this would generate actual barcode images
    // For mocking purposes, we'll return a simple vector
    format!("barcode:{}:{}", format, barcode).into_bytes()
}

// Test helper to generate sample video data
fn generate_sample_video(barcode: &str, format: Format) -> Vec<u8> {
    // In a real test, this would generate actual AV1/Opus/WebM video
    // For mocking purposes, we'll return a simple vector
    format!("video:{}-{}", barcode, format).into_bytes()
}

// Helper to log test results
async fn log_test_result(test_name: &str, passed: bool, message: &str) {
    let endpoint = std::env::var("METRICS_SERVICE_ENDPOINT")
        .unwrap_or_else(|_| "http://[::1]:50051".to_string());

    if let Ok(mut client) = MetricsServiceClient::connect(endpoint).await {
        let request = tonic::Request::new(TestResult {
            test_name: test_name.to_string(),
            passed,
            message: message.to_string(),
        });
        let _ = client.log_test_result(request).await;
    } else {
        eprintln!("[WARN] Failed to connect to metrics service");
    }
}

#[tokio::test]
async fn test_scan_image_success() {
    let mut mock_ffmpeg = MockFFmpegTrait::new();
    let sample_barcode = "123456789012";
    let sample_format = Format::EAN13;
    let sample_image = generate_barcode_image(sample_barcode, sample_format);
    let sample_video = generate_sample_video(sample_barcode, sample_format);

    // Mock FFmpeg interactions
    mock_ffmpeg.expect_run()
        .with(eq(vec!["-i", "input", "-c:v", "av1", "-c:a", "opus", "-f", "webm", "-pix_fmt", "gray", "output"]))
        .times(1)
        .returning(|_| Ok(()));
    
    mock_ffmpeg.expect_write_file()
        .with(eq("input"), eq(sample_image.clone()))
        .times(1)
        .returning(|_, _| ());
    
    mock_ffmpeg.expect_read_file()
        .with(eq("output"))
        .times(1)
        .returning(move |_| Ok(sample_video.clone()));

    let scanner = BarcodeScanner {
        ffmpeg: Arc::new(Mutex::new(mock_ffmpeg)),
    };

    let result = scanner.scan_image(&sample_image).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), sample_barcode);
    
    // Log test result
    log_test_result("test_scan_image_success", true, "Successfully scanned barcode").await;
}

#[tokio::test]
async fn test_all_supported_formats() {
    let formats = vec![
        (Format::EAN13, "1234567890128"),
        (Format::EAN8, "12345670"),
        (Format::UPCA, "012345678905"),
        (Format::UPCE, "01234565"),
        (Format::CODE39, "ABC123"),
        (Format::CODE128, "ABC-123_456"),
        (Format::QR, "https://example.com"),
        (Format::DATAMATRIX, "DMATRIX123"),
        (Format::PDF417, "PDF417DATA"),
    ];

    for (format, barcode) in formats {
        let mut mock_ffmpeg = MockFFmpegTrait::new();
        let sample_image = generate_barcode_image(barcode, format);
        let sample_video = generate_sample_video(barcode, format);

        mock_ffmpeg.expect_run().returning(|_| Ok(()));
        mock_ffmpeg.expect_write_file().returning(|_, _| ());
        mock_ffmpeg.expect_read_file().returning(move |_| Ok(sample_video.clone()));

        let scanner = BarcodeScanner {
            ffmpeg: Arc::new(Mutex::new(mock_ffmpeg)),
        };

        let result = scanner.scan_image(&sample_image).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), barcode);
        
        // Log test result
        log_test_result(
            &format!("test_format_{:?}", format), 
            true, 
            &format!("Successfully scanned {:?} barcode", format)
        ).await;
    }
}

#[tokio::test]
async fn test_empty_image_data() {
    let mock_ffmpeg = MockFFmpegTrait::new();
    let scanner = BarcodeScanner {
        ffmpeg: Arc::new(Mutex::new(mock_ffmpeg)),
    };

    let result = scanner.scan_image(&[]).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        BarcodeError::Unreadable(msg) => assert_eq!(msg, "Empty image data"),
        _ => panic!("Unexpected error type"),
    }
    
    // Log test result
    log_test_result("test_empty_image_data", true, "Correctly handled empty image data").await;
}

#[tokio::test]
async fn test_ffmpeg_error() {
    let mut mock_ffmpeg = MockFFmpegTrait::new();
    let sample_image = generate_barcode_image("123", Format::QR);

    mock_ffmpeg.expect_run()
        .times(1)
        .returning(|_| Err("FFmpeg failed".to_string()));

    let scanner = BarcodeScanner {
        ffmpeg: Arc::new(Mutex::new(mock_ffmpeg)),
    };

    let result = scanner.scan_image(&sample_image).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        BarcodeError::FfmpegInit(msg) => assert_eq!(msg, "FFmpeg failed"),
        _ => panic!("Unexpected error type"),
    }
    
    // Log test result
    log_test_result("test_ffmpeg_error", true, "Correctly handled FFmpeg error").await;
}

#[tokio::test]
async fn test_no_barcode_detected() {
    let mut mock_ffmpeg = MockFFmpegTrait::new();
    let sample_image = vec![0, 0, 0]; // Empty/invalid image data
    let sample_video = vec![0, 0, 0]; // Empty/invalid video data

    mock_ffmpeg.expect_run().returning(|_| Ok(()));
    mock_ffmpeg.expect_write_file().returning(|_, _| ());
    mock_ffmpeg.expect_read_file().returning(move |_| Ok(sample_video.clone()));

    let scanner = BarcodeScanner {
        ffmpeg: Arc::new(Mutex::new(mock_ffmpeg)),
    };

    let result = scanner.scan_image(&sample_image).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        BarcodeError::Unreadable(msg) => assert_eq!(msg, "No barcode detected"),
        _ => panic!("Unexpected error type"),
    }
    
    // Log test result
    log_test_result("test_no_barcode_detected", true, "Correctly handled no barcode case").await;
}

#[tokio::test]
async fn test_metrics_service_unavailable() {
    // Set invalid endpoint
    std::env::set_var("METRICS_SERVICE_ENDPOINT", "http://invalid:9999");
    
    // Should not panic, just print warning
    log_test_result("test_metrics_service_unavailable", true, "Service unreachable").await;
    
    // Clear the env var to avoid affecting other tests
    std::env::remove_var("METRICS_SERVICE_ENDPOINT");
}

#[tokio::test]
async fn test_corrupted_video_data() {
    let mut mock_ffmpeg = MockFFmpegTrait::new();
    let sample_image = generate_barcode_image("123", Format::QR);
    let corrupted_video = vec![0, 1, 2, 3]; // Invalid video data

    mock_ffmpeg.expect_run().returning(|_| Ok(()));
    mock_ffmpeg.expect_write_file().returning(|_, _| ());
    mock_ffmpeg.expect_read_file().returning(move |_| Ok(corrupted_video.clone()));

    let scanner = BarcodeScanner {
        ffmpeg: Arc::new(Mutex::new(mock_ffmpeg)),
    };

    let result = scanner.scan_image(&sample_image).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        BarcodeError::ImageProcessing(_) => {}, // We expect any ImageProcessing error
        _ => panic!("Unexpected error type"),
    }
    
    // Log test result
    log_test_result("test_corrupted_video_data", true, "Correctly handled corrupted video data").await;
}