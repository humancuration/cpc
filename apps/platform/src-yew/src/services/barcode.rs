use quirc_wasm::{decode, QuircResult};
use web_sys::{HtmlCanvasElement, HtmlVideoElement};
use wasm_bindgen::prelude::*;
use tracing::error;
use crate::services::api::ApiService;
use crate::types::error::ApiError;
use crate::types::product::{ProductResponse, BarcodeError, BarcodeErrorCode};

pub struct BarcodeService;

impl BarcodeService {
    pub fn scan_from_video(video: &HtmlVideoElement, canvas: &HtmlCanvasElement) -> Option<String> {
        let context = canvas
            .get_context("2d")
            .ok()
            .flatten()
            .and_then(|ctx| ctx.dyn_into::<web_sys::CanvasRenderingContext2d>().ok());
        
        if let Some(ctx) = context {
            // Set canvas dimensions to match video
            canvas.set_width(video.video_width());
            canvas.set_height(video.video_height());
            
            // Draw video frame to canvas
            ctx.draw_image_with_html_video_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                video, 
                0.0, 
                0.0,
                video.video_width() as f64,
                video.video_height() as f64,
                0.0,
                0.0,
                canvas.width() as f64,
                canvas.height() as f64,
            )
            .expect("Should draw video to canvas");
            
            // Get image data from canvas
            let image_data = ctx
                .get_image_data(0.0, 0.0, canvas.width() as f64, canvas.height() as f64)
                .expect("Should get image data");
            
            // Decode barcodes
            let mut decoder = decode::Decoder::new();
            let results = decoder.identify(
                image_data.width() as usize,
                image_data.height() as usize,
                &image_data.data().to_vec(),
            );
            
            // Process results
            for result in results {
                match result {
                    QuircResult::Code(code) => {
                        let decoded = code.decode();
                        if let Ok(data) = decoded {
                            return Some(String::from_utf8_lossy(&data.payload).to_string());
                        }
                    }
                    QuircResult::Error(e) => {
                        error!("Quirc decoding error: {:?}", e);
                    }
                }
            }
        }
        
        None
    }
}

impl BarcodeService {
    pub async fn scan_barcode(barcode: &str) -> Result<ProductResponse, BarcodeError> {
        // Validate barcode format
        if !is_valid_barcode(barcode) {
            return Err(BarcodeError {
                code: BarcodeErrorCode::InvalidBarcodeFormat,
                message: "Invalid barcode format".to_string(),
            });
        }

        // Set scan timeout (10 seconds)
        let timeout = gloo_timers::future::TimeoutFuture::new(10_000);
        let fetch_future = ApiService::fetch_product_by_barcode(barcode);

        match wasm_bindgen_futures::spawn_local(async {
            wasm_bindgen_futures::JsFuture::from(
                js_sys::Promise::race(&js_sys::Array::of2(
                    &timeout.into(),
                    &fetch_future.into(),
                ))
            )
            .await
        })
        .await
        {
            Ok(_) => fetch_future.await.map_err(|e| match e {
                ApiError::EmptyResponse => BarcodeError {
                    code: BarcodeErrorCode::NotFound,
                    message: "Product not found".to_string(),
                },
                _ => BarcodeError {
                    code: BarcodeErrorCode::NetworkError,
                    message: e.to_string(),
                },
            }),
            Err(_) => Err(BarcodeError {
                code: BarcodeErrorCode::ScanTimeout,
                message: "Barcode scan timed out".to_string(),
            }),
        }
    }
}

// Helper function to validate barcode format
fn is_valid_barcode(barcode: &str) -> bool {
    // Simple validation: at least 8 digits, no special characters
    barcode.len() >= 8 && barcode.chars().all(|c| c.is_ascii_digit())
}
}