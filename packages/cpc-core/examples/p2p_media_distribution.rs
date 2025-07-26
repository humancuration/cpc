use cpc_core::media::*;
use std::path::PathBuf;
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    println!("P2P Media Distribution Example");
    println!("==============================");
    
    // Create distributed media service
    let config = MediaServiceConfig {
        base_directory: PathBuf::from("./distributed_media"),
        use_p2p_storage: true,
        ..Default::default()
    };
    
    let p2p_config = serde_json::json!({
        "bootstrap_node": "127.0.0.1:4001"
    }).to_string();
    
    let distributed_service = integration::utils::create_distributed_media_service(config, p2p_config);
    
    // Initialize the service
    distributed_service.initialize().await?;
    println!("✓ Distributed media service initialized");
    
    // Example 1: Upload and distribute content
    println!("\n1. Uploading and distributing content...");
    let sample_video_data = create_sample_video_data();
    
    let result = distributed_service.upload_and_distribute(
        "sample_video.webm",
        "video/webm",
        &sample_video_data,
        3, // Replicate to 3 nodes
        Some(MediaProcessingConfig {
            video_codec: VideoCodec::AV1,
            audio_codec: AudioCodec::Opus,
            container_format: ContainerFormat::WebM,
            video_bitrate: Some(1000000), // 1 Mbps
            audio_bitrate: Some(128000),  // 128 kbps
            quality: Some(80),
            ..Default::default()
        }),
        Some(ThumbnailConfig {
            width: 320,
            height: 240,
            quality: 85,
            timestamp: Some(2.0), // 2 seconds into video
        }),
    ).await?;
    
    println!("✓ Content uploaded and distributed!");
    println!("  Upload ID: {}", result.processed_result.upload_id);
    println!("  Distribution ID: {}", result.distribution_id);
    println!("  Replication factor: {}", result.replication_factor);
    
    // Example 2: Retrieve distributed content
    println!("\n2. Retrieving distributed content...");
    let retrieved_data = distributed_service.get_distributed_content(&result.distribution_id).await?;
    println!("✓ Retrieved {} bytes from distributed network", retrieved_data.len());
    
    // Verify data integrity
    if retrieved_data == sample_video_data {
        println!("✓ Data integrity verified - content matches original");
    } else {
        println!("✗ Data integrity check failed");
    }
    
    // Example 3: Get distribution statistics
    println!("\n3. Distribution statistics...");
    let stats = distributed_service.get_distribution_stats();
    println!("✓ Distribution stats:");
    println!("  Total nodes: {}", stats.total_nodes);
    println!("  Total content: {}", stats.total_content);
    println!("  Cache entries: {}", stats.cache_entries);
    println!("  Cache size: {} bytes", stats.cache_size);
    println!("  Max cache size: {} bytes", stats.max_cache_size);
    println!("  P2P connected peers: {}", stats.p2p_stats.connected_peers);
    println!("  P2P total content: {}", stats.p2p_stats.total_content);
    println!("  P2P total size: {} bytes", stats.p2p_stats.total_size);
    
    // Example 4: Content integrity verification
    println!("\n4. Verifying content integrity...");
    match distributed_service.verify_content_integrity(&result.distribution_id).await {
        Ok(integrity_report) => {
            println!("✓ Content integrity report:");
            println!("  Content ID: {}", integrity_report.content_id);
            println!("  Expected hash: {}", integrity_report.expected_hash);
            println!("  Integrity score: {:.2}", integrity_report.integrity_score);
            println!("  Node reports: {}", integrity_report.node_reports.len());
            
            for node_report in &integrity_report.node_reports {
                let status = if node_report.hash_matches { "✓" } else { "✗" };
                println!("    {} Node {}: exists={}, hash_matches={}", 
                        status, node_report.node_id, node_report.content_exists, node_report.hash_matches);
            }
        }
        Err(e) => {
            println!("✗ Content integrity verification failed: {}", e);
        }
    }
    
    // Example 5: Simulate multiple retrievals to test caching
    println!("\n5. Testing cache performance...");
    let start_time = std::time::Instant::now();
    
    for i in 0..5 {
        let _data = distributed_service.get_distributed_content(&result.distribution_id).await?;
        println!("  Retrieval {}: {} bytes", i + 1, _data.len());
    }
    
    let total_time = start_time.elapsed();
    println!("✓ 5 retrievals completed in {:?} (avg: {:?})", 
             total_time, total_time / 5);
    
    // Example 6: Cleanup old content
    println!("\n6. Cleaning up old content...");
    let cleanup_stats = distributed_service.cleanup(30).await?; // 30 days
    println!("✓ Cleanup completed:");
    println!("  Media files cleaned: {}", cleanup_stats.media_files_cleaned);
    println!("  Cache entries cleaned: {}", cleanup_stats.cache_entries_cleaned);
    
    println!("\n✓ P2P media distribution example completed successfully!");
    
    Ok(())
}

/// Create sample video data for testing
fn create_sample_video_data() -> Vec<u8> {
    // Create a simple "video" file (just some structured binary data)
    let mut data = Vec::new();
    
    // Add a simple header
    data.extend_from_slice(b"WEBM");
    data.extend_from_slice(&[0x1A, 0x45, 0xDF, 0xA3]); // WebM signature
    
    // Add some dummy video data
    for i in 0..1000 {
        let frame_data = format!("FRAME_{:04}", i);
        data.extend_from_slice(frame_data.as_bytes());
        
        // Add some binary data to simulate video content
        for j in 0..100 {
            data.push(((i + j) % 256) as u8);
        }
    }
    
    data
}

/// Create sample audio data for testing
fn create_sample_audio_data() -> Vec<u8> {
    // Create a simple "audio" file (structured binary data)
    let mut data = Vec::new();
    
    // Add Opus header
    data.extend_from_slice(b"OpusHead");
    data.push(1); // Version
    data.push(2); // Channel count
    
    // Add some dummy audio data
    for i in 0..500 {
        let sample = (i as f32 * 0.1).sin() * 127.0;
        data.push((sample as i8 + 128) as u8);
    }
    
    data
}