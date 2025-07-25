use anyhow::{Result, Context};
use crate::product::error::BarcodeError;
use ffmpeg_wasm::{FFmpeg, File, Codec};
use bardecoder;
use bardecoder::Format;

pub struct BarcodeScanner {
    ffmpeg: FFmpeg,
}

impl BarcodeScanner {
    pub fn new() -> Result<Self, BarcodeError> {
        let ffmpeg = FFmpeg::new().map_err(|e| BarcodeError::FfmpegInit(e.to_string()))?;
        Ok(BarcodeScanner {
            ffmpeg
        })
    }

    pub async fn scan_image(&self, image_data: &[u8]) -> Result<String, BarcodeError> {
        if image_data.is_empty() {
            return Err(BarcodeError::Unreadable("Empty image data".into()));
        }

        // Write image data to virtual file
        self.ffmpeg.write_file("input", image_data.to_vec());

        // Process image with ffmpeg using AV1/Opus/WebM
        let args = [
            "-i", "input",
            "-c:v", "av1",
            "-c:a", "opus",
            "-f", "webm",
            "-pix_fmt", "gray",
            "output"
        ];
        self.ffmpeg.run(&args).await.map_err(|e| BarcodeError::FfmpegInit(e.to_string()))?;

        // Read processed image data
        let raw_data = self.ffmpeg.read_file("output")
            .map_err(|e| BarcodeError::ImageProcessing(e.to_string()))?;

        // Decode barcode
        let img = image::load_from_memory(&raw_data)
            .map_err(|e| BarcodeError::ImageProcessing(e.to_string()))?;
        
        // Create decoder with all supported formats
        let mut builder = bardecoder::DecoderBuilder::new();
        builder.with_formats(vec![
            Format::EAN13,
            Format::EAN8,
            Format::UPCA,
            Format::UPCE,
            Format::CODE39,
            Format::CODE128,
            Format::QR,
            Format::DATAMATRIX,
            Format::PDF417
        ]);
        let decoder = builder.build();
        
        let results = decoder.decode(&img);
        
        if results.is_empty() {
            return Err(BarcodeError::Unreadable("No barcode detected".into()));
        }
        
        // Return first detected barcode
        Ok(results[0].to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::barcode::tests::*;
}