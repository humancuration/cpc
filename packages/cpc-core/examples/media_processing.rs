use cpc_core::media::*;
use std::path::PathBuf;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    println!("Media Processing Example");
    println!("========================");
    
    // Create media service
    let base_dir = PathBuf::from("./example_media");
    let media_service = MediaService::new_with_local_storage(base_dir);
    
    // Initialize the service
    media_service.initialize().await?;
    println!("✓ Media service initialized");
    
    // Example 1: Process a sample image
    println!("\n1. Processing sample image...");
    let sample_image_data = create_sample_image_data();
    
    let result = media_service.upload_and_process(
        "sample.png",
        "image/png",
        &sample_image_data,
        Some(MediaProcessingConfig {
            container_format: ContainerFormat::PNG,
            quality: Some(85),
            resolution: Some((800, 600)),
            ..Default::default()
        }),
        Some(ThumbnailConfig {
            width: 200,
            height: 150,
            quality: 80,
            timestamp: None,
        }),
    ).await?;
    
    println!("✓ Image processed successfully!");
    println!("  Upload ID: {}", result.upload_id);
    println!("  Storage ID: {}", result.storage_id);
    if let Some(thumb_id) = &result.thumbnail_storage_id {
        println!("  Thumbnail ID: {}", thumb_id);
    }
    
    // Example 2: Generate additional thumbnails
    println!("\n2. Generating additional thumbnails...");
    let thumbnail_sizes = vec![(64, 64), (128, 128), (320, 240)];
    let thumbnail_ids = media_service.generate_additional_thumbnails(
        &result.storage_id,
        &thumbnail_sizes,
    ).await?;
    
    println!("✓ Generated {} additional thumbnails", thumbnail_ids.len());
    for (i, thumb_id) in thumbnail_ids.iter().enumerate() {
        let size = thumbnail_sizes[i];
        println!("  {}x{}: {}", size.0, size.1, thumb_id);
    }
    
    // Example 3: Retrieve media metadata
    println!("\n3. Retrieving media metadata...");
    if let Some(metadata) = media_service.get_media_metadata(&result.storage_id).await? {
        println!("✓ Media metadata:");
        println!("  File name: {}", metadata.file_name);
        println!("  File size: {} bytes", metadata.file_size);
        println!("  Media type: {:?}", metadata.media_type);
        println!("  Checksum: {}", metadata.checksum);
        if let Some((w, h)) = metadata.width.zip(metadata.height) {
            println!("  Dimensions: {}x{}", w, h);
        }
    }
    
    // Example 4: Get storage statistics
    println!("\n4. Storage statistics...");
    let stats = media_service.get_storage_stats().await?;
    println!("✓ Storage stats:");
    println!("  Total uploads: {}", stats.total_uploads);
    println!("  Upload size: {} bytes", stats.upload_size);
    println!("  Processed size: {} bytes", stats.processed_size);
    println!("  Thumbnail size: {} bytes", stats.thumbnail_size);
    println!("  Total size: {} bytes", stats.total_size);
    
    // Example 5: Test supported media types
    println!("\n5. Supported media types...");
    let supported_types = integration::utils::get_supported_media_types();
    println!("✓ Supported extensions: {:?}", supported_types);
    
    // Test some filenames
    let test_files = vec![
        "video.mp4",
        "audio.opus", 
        "image.png",
        "document.pdf", // Should be unsupported
    ];
    
    for filename in test_files {
        let supported = integration::utils::is_supported_media_type(filename);
        println!("  {}: {}", filename, if supported { "✓" } else { "✗" });
    }
    
    println!("\n✓ Media processing example completed successfully!");
    
    Ok(())
}

/// Create sample image data for testing
fn create_sample_image_data() -> Vec<u8> {
    // Create a simple 100x100 PNG image
    use image::{ImageBuffer, Rgb};
    
    let img = ImageBuffer::from_fn(100, 100, |x, y| {
        let intensity = ((x + y) % 255) as u8;
        Rgb([intensity, intensity / 2, 255 - intensity])
    });
    
    let mut buffer = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut buffer);
    
    img.write_to(&mut cursor, image::ImageOutputFormat::Png)
        .expect("Failed to write sample image");
    
    buffer
}